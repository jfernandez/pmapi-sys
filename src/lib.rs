#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::{
        alloc::{alloc, Layout},
        ffi::{CStr, CString},
        os::raw::{c_char, c_int},
    };

    use super::*;

    fn get_error(sts: c_int) -> String {
        unsafe {
            let err_str = pmErrStr(sts);
            let c_str = CStr::from_ptr(err_str);
            c_str.to_string_lossy().into_owned()
        }
    }

    #[test]
    fn test_query_metric() {
        unsafe {
            let host = CString::new("localhost").unwrap();
            let sts = pmNewContext(PM_CONTEXT_HOST.try_into().unwrap(), host.as_ptr());
            assert!(sts >= 0, "pmNewContext error: {}", get_error(sts));

            let mut metric_id = pmID::default();
            let metric_name = CString::new("hinv.ncpu").unwrap();

            let sts = pmLookupName(
                1,
                &mut metric_name.as_ptr() as *mut *const c_char,
                &mut metric_id,
            );
            assert!(sts >= 0, "pmLookupName error: {}", get_error(sts));

            let layout = Layout::new::<pmResult>();
            let mut ptr = alloc(layout) as *mut pmResult;
            let sts = pmFetch(1, &mut metric_id, &mut ptr);
            assert!(sts >= 0, "pmFetch error: {}", get_error(sts));

            let vset = *(*ptr).vset[0];
            assert_eq!(vset.vlist[0].value.lval as usize, num_cpus::get());
        }
    }
}
