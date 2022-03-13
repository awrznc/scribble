extern crate libc;

#[no_mangle]
pub extern "C" fn galm_sort(sort_target: *mut *const libc::c_char, size: libc::size_t, sort_key: *const libc::c_char) {

    // sort key
    let c_str_sort_key = unsafe { std::ffi::CStr::from_ptr(sort_key) };
    let sort_key: &str = c_str_sort_key.to_str().expect("std::ffi::CStr::from_ptr.to_str() failed.");

    // sort target
    let raw_sort_target: &mut [*const libc::c_char] = unsafe {
        assert!(!sort_target.is_null());
        std::slice::from_raw_parts_mut(sort_target, size as usize)
    };

    // sort
    let galm: galm::Database = galm::new();
    raw_sort_target.sort_by_cached_key( |candidate| {
        let c_str = unsafe {
            assert!(!(*candidate).is_null());
            std::ffi::CStr::from_ptr(*candidate)
        };
        let sort_target_element: &str = c_str.to_str().expect("std::ffi::CStr::from_ptr.to_str() failed.");
        galm.get_word_distance(sort_key, &sort_target_element)
    });
}
