use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;
use std::str;

static mut FAILURES: i32 = 0;
static mut SUCCESSES: i32 = 0;

extern "C" {
    fn hash_one_at_a_time(key: *const c_char, len: usize) -> u32;
    fn hash_md5(key: *const c_char, len: usize) -> u32;
    fn hash_crc16(key: *const c_char, len: usize) -> u32;
    fn hash_crc32(key: *const c_char, len: usize) -> u32;
    fn hash_crc32a(key: *const c_char, len: usize) -> u32;
    fn hash_fnv1_32(key: *const c_char, len: usize) -> u32;
    fn hash_fnv1a_32(key: *const c_char, len: usize) -> u32;
    fn hash_fnv1_64(key: *const c_char, len: usize) -> u32;
    fn hash_fnv1a_64(key: *const c_char, len: usize) -> u32;
    fn hash_hsieh(key: *const c_char, len: usize) -> u32;
    fn hash_jenkins(key: *const c_char, len: usize) -> u32;
    fn hash_murmur(key: *const c_char, len: usize) -> u32;
    fn ketama_hash(key: *const c_char, len: usize, index: u32) -> u32;
    fn conf_create(file: *const c_char) -> *mut c_void;
    fn conf_destroy(conf: *mut c_void);
}

fn expect_same_int(expected: i32, actual: i32, message: &str) {
    unsafe {
        if expected != actual {
            println!("FAIL Expected {}, got {} ({})", expected, actual, message);
            FAILURES += 1;
        } else {
            SUCCESSES += 1;
        }
    }
}

fn expect_same_uint32(expected: u32, actual: u32, message: &str) {
    unsafe {
        if expected != actual {
            println!("FAIL Expected {}, got {} ({})", expected, actual, message);
            FAILURES += 1;
        } else {
            SUCCESSES += 1;
        }
    }
}

fn expect_same_ptr(expected: *const c_void, actual: *const c_void, message: &str) {
    unsafe {
        if expected != actual {
            println!("FAIL Expected {:p}, got {:p} ({})", expected, actual, message);
            FAILURES += 1;
        } else {
            SUCCESSES += 1;
        }
    }
}

fn test_hash_algorithms() {
    let key = CString::new("apple").unwrap();
    unsafe {
        expect_same_uint32(2297466611, hash_one_at_a_time(key.as_ptr(), 5), "should have expected one_at_a_time hash for key \"apple\"");
        expect_same_uint32(3195025439, hash_md5(key.as_ptr(), 5), "should have expected md5 hash for key \"apple\"");
        
        expect_same_uint32(3662830516, hash_crc16(key.as_ptr(), 5), "should have expected crc16 hash for key \"apple\"");
        expect_same_uint32(10542, hash_crc32(key.as_ptr(), 5), "should have expected crc32 hash for key \"apple\"");
        expect_same_uint32(2838417488, hash_crc32a(key.as_ptr(), 5), "should have expected crc32a hash for key \"apple\"");
        expect_same_uint32(67176023, hash_fnv1_32(key.as_ptr(), 5), "should have expected fnv1_32 hash for key \"apple\"");
        expect_same_uint32(280767167, hash_fnv1a_32(key.as_ptr(), 5), "should have expected fnv1a_32 hash for key \"apple\"");
        expect_same_uint32(473199127, hash_fnv1_64(key.as_ptr(), 5), "should have expected fnv1_64 hash for key \"apple\"");
        expect_same_uint32(1488911807, hash_fnv1a_64(key.as_ptr(), 5), "should have expected fnv1a_64 hash for key \"apple\"");
        expect_same_uint32(3738850110, hash_hsieh(key.as_ptr(), 5), "should have expected hsieh hash for key \"apple\"");
        expect_same_uint32(1442444624, hash_jenkins(key.as_ptr(), 5), "should have expected jenkins hash for key \"apple\"");
        expect_same_uint32(4142305122, hash_murmur(key.as_ptr(), 5), "should have expected murmur hash for key \"apple\"");
        
        let server_key = CString::new("server1-8").unwrap();
        expect_same_uint32(3853726576, ketama_hash(server_key.as_ptr(), server_key.as_bytes().len(), 0), "should have expected ketama_hash for server1-8 index 0");
        expect_same_uint32(2667054752, ketama_hash(server_key.as_ptr(), server_key.as_bytes().len(), 3), "should have expected ketama_hash for server1-8 index 3");
    }
}

fn test_config_parsing() {
    let conf_file = CString::new("../conf/nutcracker.yml").unwrap();
    unsafe {
        let conf = conf_create(conf_file.as_ptr());
        if conf.is_null() {
            println!("FAIL could not parse {} (this test should be run within src/ folder)", conf_file.to_str().unwrap());
            FAILURES += 1;
        } else {
            println!("PASS parsed {}", conf_file.to_str().unwrap());
            conf_destroy(conf);
            SUCCESSES += 1;
        }
    }
}

fn main() {
    test_hash_algorithms();
    test_config_parsing();
    
    unsafe {
        println!("{} successes, {} failures", SUCCESSES, FAILURES);
    }
    
    std::process::exit(unsafe { if FAILURES > 0 { 1 } else { 0 } });
}