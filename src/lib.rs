#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;

    #[test]
    fn new_ipset() {
        unsafe {
            let my_ipset = ipset_init();
            let null_ipset: *mut ipset = ptr::null_mut();

            assert_ne!(my_ipset, null_ipset)
        }
    }

    #[test]
    fn ipset_session_not_null() {
        unsafe {
            let my_ipset = ipset_init();
            let null_ipset: *mut ipset = ptr::null_mut();

            assert_ne!(my_ipset, null_ipset);

            let my_ipset_session = ipset_session(my_ipset);
            let null_ipset_session: *mut ipset_session = ptr::null_mut();

            assert_ne!(my_ipset_session, null_ipset_session);
        }
    }
}
