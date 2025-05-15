/* See LICENSE file for copyright and license details. */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZError {
    ErrnoSet,
    ZeroPowZero,
    ZeroDivZero,
    DivZero,
    Negative,
}

const ZERROR_0_POW_0: i32 = -1;
const ZERROR_0_DIV_0: i32 = -2;
const ZERROR_DIV_0: i32 = -3;
const ZERROR_NEGATIVE: i32 = -4;

pub fn zerror(desc: Option<&mut &'static str>) -> Result<ZError, std::io::Error> {
    if libzahl_error >= 0 {
        if let Some(desc) = desc {
            *desc = std::io::Error::from_raw_os_error(libzahl_error)
                .to_string()
                .leak();
        }
        return Err(std::io::Error::from_raw_os_error(libzahl_error));
    }

    if let Some(desc) = desc {
        match -libzahl_error {
            ZERROR_0_POW_0 => *desc = "indeterminate form: 0:th power of 0",
            ZERROR_0_DIV_0 => *desc = "indeterminate form: 0 divided by 0",
            ZERROR_DIV_0 => *desc = "undefined result: division by 0",
            ZERROR_NEGATIVE => *desc = "argument must be non-negative",
            _ => panic!("Unknown error code"),
        }
    }

    match -libzahl_error {
        ZERROR_0_POW_0 => Ok(ZError::ZeroPowZero),
        ZERROR_0_DIV_0 => Ok(ZError::ZeroDivZero),
        ZERROR_DIV_0 => Ok(ZError::DivZero),
        ZERROR_NEGATIVE => Ok(ZError::Negative),
        _ => panic!("Unknown error code"),
    }
}