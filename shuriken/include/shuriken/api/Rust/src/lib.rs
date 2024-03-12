#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/shuriken_core.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_char;
    use std::ffi::CString;

    #[test]
    fn check_parse_dex() {
        let c_str = CString::new("/home/jgamba/dev/Shuriken-Analyzer/shuriken/tests/compiled/DexParserTest.dex").unwrap();
        let c_world = c_str.as_ptr() as *const c_char;
        let dex = unsafe { parse_dex(c_world); };
        assert_eq!(true, true)
    }
}
