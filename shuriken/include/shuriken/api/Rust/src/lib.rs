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
        unsafe {
            let c_str = CString::new("/home/jgamba/dev/Shuriken-Analyzer/shuriken/tests/compiled/DexParserTest.dex").unwrap();
            let c_world = c_str.as_ptr() as *const c_char;
            let dex =  parse_dex(c_world);
            assert_eq!(dex.is_null(), false);
        }
    }

    #[test]
    fn check_get_number_of_strings() {
        unsafe {
            let c_str = CString::new("/home/jgamba/dev/Shuriken-Analyzer/shuriken/tests/compiled/DexParserTest.dex").unwrap();
            let c_world = c_str.as_ptr() as *const c_char;
            let dex =  parse_dex(c_world);
            assert_eq!(get_number_of_strings(dex), 33);
        }
    }

    #[test]
    fn check_get_string_by_id() {
        todo!();
    }

    #[test]
    fn check_get_number_of_classes() {
        unsafe {
            let c_str = CString::new("/home/jgamba/dev/Shuriken-Analyzer/shuriken/tests/compiled/DexParserTest.dex").unwrap();
            let c_world = c_str.as_ptr() as *const c_char;
            let dex =  parse_dex(c_world);
            assert_eq!(get_number_of_classes(dex), 1);
        }
    }

    #[test]
    fn check_get_class_by_id() {
        todo!();
    }

    #[test]
    fn check_get_class_by_name() {
        assert!(true);
    }

    #[test]
    fn check_get_method_by_name() {
        todo!();
    }
}
