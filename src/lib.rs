
use probe_rs::config::ChipFamily;

pub struct ExtDeviceFamilies {
    families: Vec<ChipFamily>
}

#[no_mangle]
pub extern "C" fn lsprobe_families_get(families_out: *mut *mut ExtDeviceFamilies, families_count_out: *mut usize) -> u32 {
    match probe_rs::config::families() {
        Ok(families) => {
            let families_count = families.len();
            let boxed_families = Box::new(ExtDeviceFamilies { families });

            unsafe {
                *families_count_out = families_count;
                *families_out = Box::into_raw(boxed_families);
            }

            0
        }

        Err(_) => {
            unsafe {
                *families_count_out = 0;
                *families_out = std::ptr::null_mut();
            }

            1
        }
    }
}

#[no_mangle]
pub extern "C" fn lsprobe_families_get_name(families: *const ExtDeviceFamilies, index: usize, name_out: *mut *const u8, name_length_out: *mut usize) -> u32 {
    let families = unsafe { &*families };

    if index >= families.families.len() {
        return 1;
    }

    let family = &families.families[index];

    unsafe {
        *name_out = family.name.as_ptr();
        *name_length_out = family.name.len();
    }

    0
}

#[no_mangle]
pub extern "C" fn lsprobe_families_destroy(families: *mut ExtDeviceFamilies) -> u32 {
    if families.is_null() {
        return 1;
    }

    unsafe {
        let _ = Box::from_raw(families);
    }

    0
}
