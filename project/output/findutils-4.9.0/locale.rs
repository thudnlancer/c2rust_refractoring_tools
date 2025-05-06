#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn setlocale(__category: i32, __locale: *const i8) -> *mut i8;
    fn printf(_: *const i8, _: ...) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
}
unsafe extern "C" fn defaulted_getenv(mut variable: *const i8) -> *const i8 {
    let mut value: *const i8 = getenv(variable);
    return if !value.is_null() { value } else { b"\0" as *const u8 as *const i8 };
}
unsafe extern "C" fn print_category(mut category: i32, mut variable: *const i8) {
    let mut value: *const i8 = defaulted_getenv(variable);
    if *value.offset(0 as i32 as isize) as i32 != '\0' as i32
        && *(defaulted_getenv(b"LC_ALL\0" as *const u8 as *const i8))
            .offset(0 as i32 as isize) as i32 == '\0' as i32
    {
        printf(b"%s=%s\n\0" as *const u8 as *const i8, variable, value);
    } else {
        printf(
            b"%s=\"%s\"\n\0" as *const u8 as *const i8,
            variable,
            setlocale(category, 0 as *const i8),
        );
    };
}
unsafe fn main_0() -> i32 {
    setlocale(6 as i32, b"\0" as *const u8 as *const i8);
    printf(
        b"LANG=%s\n\0" as *const u8 as *const i8,
        defaulted_getenv(b"LANG\0" as *const u8 as *const i8),
    );
    print_category(0 as i32, b"LC_CTYPE\0" as *const u8 as *const i8);
    print_category(1 as i32, b"LC_NUMERIC\0" as *const u8 as *const i8);
    print_category(2 as i32, b"LC_TIME\0" as *const u8 as *const i8);
    print_category(3 as i32, b"LC_COLLATE\0" as *const u8 as *const i8);
    print_category(4 as i32, b"LC_MONETARY\0" as *const u8 as *const i8);
    print_category(5 as i32, b"LC_MESSAGES\0" as *const u8 as *const i8);
    print_category(7 as i32, b"LC_PAPER\0" as *const u8 as *const i8);
    print_category(8 as i32, b"LC_NAME\0" as *const u8 as *const i8);
    print_category(9 as i32, b"LC_ADDRESS\0" as *const u8 as *const i8);
    print_category(10 as i32, b"LC_TELEPHONE\0" as *const u8 as *const i8);
    print_category(11 as i32, b"LC_MEASUREMENT\0" as *const u8 as *const i8);
    print_category(12 as i32, b"LC_IDENTIFICATION\0" as *const u8 as *const i8);
    printf(
        b"LC_ALL=%s\n\0" as *const u8 as *const i8,
        defaulted_getenv(b"LC_ALL\0" as *const u8 as *const i8),
    );
    return 0 as i32;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}