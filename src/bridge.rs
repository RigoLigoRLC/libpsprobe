struct VecU8Wrapper {
    vec_obj: Vec<u8>,
}

#[repr(C)]
pub struct ExtDynStr {
    length: usize,
    data: *const u8,
    box_ptr: *mut VecU8Wrapper,
}

impl Into<ExtDynStr> for String {
    fn into(self) -> ExtDynStr {
        let length = self.len();
        let bytes = self.into_bytes();
        let data = bytes.as_ptr();
        let box_ptr = Box::into_raw(Box::new(VecU8Wrapper { vec_obj: bytes }));

        ExtDynStr { length, data, box_ptr }
    }
}

#[no_mangle]
pub fn psprobe_extdynstr_dispose(extdynstr: *mut ExtDynStr) {
    if extdynstr.is_null() {
        return;
    }

    unsafe {
        let _ = Box::from_raw((*extdynstr).box_ptr);
        (*extdynstr).data = std::ptr::null_mut();
        (*extdynstr).length = 0;
    }
}
