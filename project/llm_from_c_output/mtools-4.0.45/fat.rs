use std::io::{self, Read, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::convert::TryInto;

const SECT_PER_ENTRY: usize = mem::size_of::<u64>() * 8;
const ONE: u64 = 1;
const MAX32: u32 = 0xFFFFFFFF;
const FAT12: u32 = 4085;
const FAT16: u32 = 65525;
const INFOSECT_SIGNATURE1: u32 = 0x41615252;
const INFOSECT_SIGNATURE2: u32 = 0x61417272;

#[derive(Debug)]
struct FatMap {
    data: Vec<u8>,
    dirty: u64,
    valid: u64,
}

#[derive(Debug)]
struct Fs {
    fat_error: u32,
    fat_dirty: bool,
    last: u32,
    free_space: u32,
    last_fat_sector_nr: u32,
    last_fat_sector_data: Vec<u8>,
    last_fat_access_mode: FatAccessMode,
    fat_map: Vec<FatMap>,
    fat_bits: u32,
    end_fat: u32,
    last_fat: u32,
    num_clus: u32,
    sector_size: u32,
    sector_shift: u32,
    sector_mask: u32,
    fat_len: u32,
    num_fat: u32,
    primary_fat: u32,
    write_all_fats: bool,
    root_cluster: u32,
    info_sector_loc: u32,
    cluster_size: u32,
    fat_start: u32,
    serialized: bool,
    serial_number: u32,
    preallocated_clusters: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum FatAccessMode {
    Read,
    Write,
}

impl Fs {
    fn new() -> Self {
        Fs {
            fat_error: 0,
            fat_dirty: false,
            last: MAX32,
            free_space: MAX32,
            last_fat_sector_nr: 0,
            last_fat_sector_data: Vec::new(),
            last_fat_access_mode: FatAccessMode::Read,
            fat_map: Vec::new(),
            fat_bits: 0,
            end_fat: 0,
            last_fat: 0,
            num_clus: 0,
            sector_size: 0,
            sector_shift: 0,
            sector_mask: 0,
            fat_len: 0,
            num_fat: 0,
            primary_fat: 0,
            write_all_fats: false,
            root_cluster: 0,
            info_sector_loc: MAX32,
            cluster_size: 0,
            fat_start: 0,
            serialized: false,
            serial_number: 0,
            preallocated_clusters: 0,
        }
    }

    fn get_fat_map(&mut self) -> io::Result<()> {
        self.fat_error = 0;
        let nr_entries = (self.fat_len + SECT_PER_ENTRY as u32 - 1) / SECT_PER_ENTRY as u32;
        self.fat_map = Vec::with_capacity(nr_entries as usize);
        
        for _ in 0..nr_entries {
            self.fat_map.push(FatMap {
                data: Vec::new(),
                valid: 0,
                dirty: 0,
            });
        }
        Ok(())
    }

    fn locate(&self, offset: u32, slot: &mut u32, bit: &mut u32) -> io::Result<()> {
        if offset >= self.fat_len {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Offset out of bounds"));
        }
        *slot = offset / SECT_PER_ENTRY as u32;
        *bit = offset % SECT_PER_ENTRY as u32;
        Ok(())
    }

    fn read_sector(&self, buf: &mut [u8], off: u32, size: usize) -> io::Result<usize> {
        // Implementation depends on your actual sector reading logic
        unimplemented!()
    }

    fn force_read_sector(&self, buf: &mut [u8], off: u32, size: usize) -> io::Result<usize> {
        // Implementation depends on your actual sector reading logic
        unimplemented!()
    }

    fn force_write_sector(&self, buf: &[u8], off: u32, size: usize) -> io::Result<usize> {
        // Implementation depends on your actual sector writing logic
        unimplemented!()
    }

    fn fat_read_sector(
        &mut self,
        sector: u32,
        slot: u32,
        bit: u32,
        dupe: u32,
        bitmap: u64,
    ) -> io::Result<usize> {
        let dupe = (dupe + self.primary_fat) % self.num_fat;
        let fat_start = self.fat_start + self.fat_len * dupe;

        let nr_sectors = if bitmap == 0 {
            SECT_PER_ENTRY as u32 - bit % SECT_PER_ENTRY as u32
        } else {
            1
        };

        let offset = bit << self.sector_shift;
        let buf = &mut self.fat_map[slot as usize].data[offset as usize..];
        
        let ret = self.read_sector(buf, fat_start + sector, nr_sectors as usize)?;
        if ret < self.sector_size as usize {
            let ret = self.force_read_sector(buf, fat_start + sector, 1)?;
            if ret < self.sector_size as usize {
                return Ok(0);
            }
            return Ok(1);
        }
        Ok(ret >> self.sector_shift)
    }

    fn fat_write_sector(
        &mut self,
        sector: u32,
        slot: u32,
        bit: u32,
        dupe: u32,
    ) -> io::Result<usize> {
        let dupe = (dupe + self.primary_fat) % self.num_fat;
        if dupe != 0 && !self.write_all_fats {
            return Ok(self.sector_size as usize);
        }

        let fat_start = self.fat_start + self.fat_len * dupe;
        let offset = bit * self.sector_size;
        let buf = &self.fat_map[slot as usize].data[offset as usize..];
        
        self.force_write_sector(buf, fat_start + sector, 1)
    }

    fn load_sector(
        &mut self,
        sector: u32,
        mode: FatAccessMode,
        recurs: bool,
    ) -> io::Result<&[u8]> {
        let mut slot = 0;
        let mut bit = 0;
        self.locate(sector, &mut slot, &mut bit)?;

        if self.fat_map[slot as usize].data.is_empty() {
            self.fat_map[slot as usize].data = vec![0xEE; (self.sector_size * SECT_PER_ENTRY as u32) as usize];
        }

        if (self.fat_map[slot as usize].valid & (ONE << bit)) == 0 {
            let mut ret = 0;
            for i in 0..self.num_fat {
                ret = self.fat_read_sector(sector, slot, bit, i, self.fat_map[slot as usize].valid)?;
                if ret == 0 {
                    eprintln!("Error reading fat number {}", i);
                    continue;
                }
                if self.fat_map[slot as usize].valid != 0 {
                    break;
                }
            }

            if ret == 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "All FAT copies bad"));
            }

            for i in 0..ret {
                self.fat_map[slot as usize].valid |= ONE << (bit + i as u32);
            }

            if !recurs && ret == 1 {
                self.load_sector(sector + 1, mode, true)?;
            }
        }

        if mode == FatAccessMode::Write {
            self.fat_map[slot as usize].dirty |= ONE << bit;
            self.fat_dirty = true;
        }

        let offset = (bit << self.sector_shift) as usize;
        Ok(&self.fat_map[slot as usize].data[offset..offset + self.sector_size as usize])
    }

    fn get_address(&mut self, num: u32, mode: FatAccessMode) -> io::Result<&[u8]> {
        let sector = num >> self.sector_shift;
        let mut ret = None;

        if sector == self.last_fat_sector_nr && self.last_fat_access_mode >= mode {
            ret = Some(&self.last_fat_sector_data);
        }

        let ret = match ret {
            Some(r) => r,
            None => {
                let data = self.load_sector(sector, mode, false)?;
                self.last_fat_sector_nr = sector;
                self.last_fat_sector_data = data.to_vec();
                self.last_fat_access_mode = mode;
                &self.last_fat_sector_data
            }
        };

        let offset = (num & self.sector_mask) as usize;
        Ok(&ret[offset..])
    }

    fn read_byte(&mut self, start: u32) -> io::Result<u8> {
        let address = self.get_address(start, FatAccessMode::Read)?;
        Ok(address[0])
    }

    fn fat12_decode(&mut self, num: u32) -> io::Result<u32> {
        let start = num * 3 / 2;
        let byte0 = self.read_byte(start)?;
        let byte1 = self.read_byte(start + 1)?;

        if num < 2 || num > self.num_clus + 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Bad address"));
        }

        if num & 1 != 0 {
            Ok(((byte1 as u32) << 4) | (((byte0 as u32) & 0xF0) >> 4)
        } else {
            Ok((((byte1 as u32) & 0xF) << 8) | (byte0 as u32)
        }
    }

    fn fat12_encode(&mut self, num: u32, code: u32) -> io::Result<()> {
        let start = num * 3 / 2;
        let address0 = self.get_address(start, FatAccessMode::Write)?;
        let address1 = self.get_address(start + 1, FatAccessMode::Write)?;

        if num & 1 != 0 {
            address0[0] = (address0[0] & 0x0F) | ((code << 4) & 0xF0) as u8;
            address1[0] = (code >> 4) as u8;
        } else {
            address0[0] = code as u8;
            address1[0] = (address1[0] & 0xF0) | ((code >> 8) & 0x0F) as u8;
        }
        Ok(())
    }

    fn fat16_decode(&mut self, num: u32) -> io::Result<u32> {
        let address = self.get_address(num << 1, FatAccessMode::Read)?;
        Ok(u16::from_le_bytes([address[0], address[1]]) as u32)
    }

    fn fat16_encode(&mut self, num: u32, code: u32) -> io::Result<()> {
        if code > u16::MAX as u32 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "FAT16 code too big"));
        }
        let address = self.get_address(num << 1, FatAccessMode::Write)?;
        let bytes = (code as u16).to_le_bytes();
        address[0] = bytes[0];
        address[1] = bytes[1];
        Ok(())
    }

    fn fat32_decode(&mut self, num: u32) -> io::Result<u32> {
        let address = self.get_address(num << 2, FatAccessMode::Read)?;
        Ok(u32::from_le_bytes([address[0], address[1], address[2], address[3]]) & 0x0FFFFFFF)
    }

    fn fat32_encode(&mut self, num: u32, code: u32) -> io::Result<()> {
        let address = self.get_address(num << 2, FatAccessMode::Write)?;
        let current = u32::from_le_bytes([address[0], address[1], address[2], address[3]]);
        let new_value = (current & 0xF0000000) | (code & 0x0FFFFFFF);
        address.copy_from_slice(&new_value.to_le_bytes());
        Ok(())
    }

    fn fat_write(&mut self) -> io::Result<()> {
        if !self.fat_dirty {
            return Ok(());
        }

        let dups = if self.fat_error != 0 { 1 } else { self.num_fat };

        for i in 0..dups {
            let mut j = 0;
            for slot in 0..self.fat_map.len() {
                if self.fat_map[slot].dirty == 0 {
                    j += SECT_PER_ENTRY as u32;
                    continue;
                }

                for bit in 0..SECT_PER_ENTRY as u32 {
                    if j >= self.fat_len {
                        break;
                    }
                    if (self.fat_map[slot].dirty & (ONE << bit)) == 0 {
                        j += 1;
                        continue;
                    }

                    let ret = self.fat_write_sector(j, slot as u32, bit, i)?;
                    if ret < self.sector_size as usize {
                        return Err(io::Error::new(io::ErrorKind::Other, "Error in fat_write"));
                    }

                    if i == dups - 1 {
                        self.fat_map[slot].dirty &= !(ONE << bit);
                    }
                    j += 1;
                }
            }
        }

        if self.info_sector_loc != 0 && self.info_sector_loc != MAX32 {
            let mut info_sector = vec![0; self.sector_size as usize];
            if self.force_read_sector(&mut info_sector, self.info_sector_loc, 1)? != self.sector_size as usize {
                info_sector[..4].copy_from_slice(&INFOSECT_SIGNATURE1.to_le_bytes());
                info_sector[4..8].copy_from_slice(&INFOSECT_SIGNATURE2.to_le_bytes());
                info_sector[8..12].copy_from_slice(&self.last.to_le_bytes());
                info_sector[12..16].copy_from_slice(&self.free_space.to_le_bytes());
                info_sector[510..512].copy_from_slice(&[0x55, 0xAA]);
            }

            if self.force_write_sector(&info_sector, self.info_sector_loc, 1)? != self.sector_size as usize {
                eprintln!("Trouble writing the info sector");
            }
        }

        self.fat_dirty = false;
        self.last_fat_access_mode = FatAccessMode::Read;
        Ok(())
    }

    fn zero_fat(&mut self, media_descriptor: u8) -> io::Result<()> {
        let mut buf = vec![0; self.sector_size as usize];
        
        for i in 0..self.num_fat {
            let fat_start = self.fat_start + i * self.fat_len;
            for j in 0..self.fat_len {
                if j <= 1 {
                    buf.fill(0);
                }
                if j == 0 {
                    buf[0] = media_descriptor;
                    buf[1] = 0xFF;
                    buf[2] = 0xFF;
                    if self.fat_bits > 12 {
                        buf[3] = 0xFF;
                    }
                    if self.fat_bits > 16 {
                        buf[3] = 0x0F;
                        buf[4] = 0xFF;
                        buf[5] = 0xFF;
                        buf[6] = 0xFF;
                        buf[7] = 0xFF;
                    }
                }

                if self.force_write_sector(&buf, fat_start + j, 1)? != self.sector_size as usize {
                    return Err(io::Error::new(io::ErrorKind::Other, "Trouble initializing a FAT sector"));
                }
            }
        }

        self.get_fat_map()?;
        Ok(())
    }

    fn set_fat12(&mut self) {
        self.fat_bits = 12;
        self.end_fat = 0xFFF;
        self.last_fat = 0xFF6;
        // Set decode/encode functions (would need function pointers in Rust)
    }

    fn set_fat16(&mut self) {
        self.fat_bits = 16;
        self.end_fat = 0xFFFF;
        self.last_fat = 0xFFF6;
        // Set decode/encode functions based on endianness
    }

    fn set_fat32(&mut self) {
        self.fat_bits = 32;
        self.end_fat = 0x0FFFFFFF;
        self.last_fat = 0x0FFFFFF6;
        // Set decode/encode functions based on endianness
    }

    fn set_fat(&mut self, have_big_fat_len: bool) {
        if have_big_fat_len {
            self.set_fat32();
        } else if self.num_clus < FAT12 {
            self.set_fat12();
        } else if self.num_clus < FAT16 {
            self.set_fat16();
        } else {
            self.set_fat32();
        }
    }

    fn fat_decode(&mut self, pos: u32) -> u32 {
        match self.fat_bits {
            12 => self.fat12_decode(pos).unwrap_or(1),
            16 => self.fat16_decode(pos).unwrap_or(1),
            32 => self.fat32_decode(pos).unwrap_or(1