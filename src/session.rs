use crate::bridge::ExtDynStr;
use crate::probe::ExtProbe;
use probe_rs::{CoreType, Error, MemoryInterface, Permissions, Session};

pub struct ExtSession {
    session: Session,
    error: Option<Error>,
}

pub struct ExtCoreInfo {
    core_info: Vec<(usize, CoreType)>,
}

#[no_mangle]
pub fn psprobe_session_open(
    probe: *mut ExtProbe,
    device_name: *const u8,
    device_name_len: usize,
    session_out: *mut *mut ExtSession,
) -> u32 {
    if session_out.is_null() || probe.is_null() {
        return 1;
    }

    // Converting a probe to a session is a move, this "probe" will have to be dropped here
    // Remember to clean up in C++ part, and get a new probe object
    let mut ext_probe = unsafe { Box::from_raw(probe) };
    ext_probe.dropped = true;

    let probe = ext_probe.probe;
    let name = match unsafe { std::str::from_utf8(std::slice::from_raw_parts(device_name, device_name_len)) } {
        Ok(name) => name,
        Err(_) => return 2,
    };

    let session = match probe.attach(name, Permissions::default()) {
        Ok(session) => session,
        Err(_) => return 3,
    };

    unsafe { *session_out = Box::into_raw(Box::new(ExtSession { session, error: None })); }

    0
}

#[no_mangle]
pub fn psprobe_session_close(session: *mut ExtSession) -> u32 {
    if session.is_null() {
        return 1;
    }

    let _ = unsafe { Box::from_raw(session) };

    0
}

#[no_mangle]
pub fn psprobe_core_info_get(session: *mut ExtSession, core_info_out: *mut *mut ExtCoreInfo, length: *mut usize) -> u32 {
    if session.is_null() {
        return 1;
    };

    let session = unsafe { &(*session).session };
    let core_info = session.list_cores();
    unsafe {
        *length = core_info.len();
        *core_info_out = Box::into_raw(Box::new(ExtCoreInfo { core_info }));
    }

    0
}

#[no_mangle]
pub fn psprobe_core_info_get_entry(core_info: *mut ExtCoreInfo, index: usize, core_index_out: *mut usize, core_desc_out: *mut ExtDynStr) -> u32 {
    if core_info.is_null() || core_desc_out.is_null() {
        return 1;
    }

    let core_info = unsafe { &(*core_info).core_info };
    if index >= core_info.len() {
        return 1;
    }

    let (core_index, core_type) = &core_info[index];
    let core_info_str = format!("{:?}", core_type);
    unsafe {
        *core_index_out = core_index.clone();
        *core_desc_out = core_info_str.into();
    }

    0
}

#[no_mangle]
pub fn psprobe_core_info_destroy(core_info: *mut ExtCoreInfo) -> u32 {
    if core_info.is_null() {
        return 1;
    }

    let _ = unsafe { Box::from_raw(core_info) };

    0
}

#[no_mangle]
pub fn psprobe_session_read_memory_8(session: *mut ExtSession, core: usize, address: u64, size: usize, dest: *mut u8) -> u32 {
    if session.is_null() {
        return 1;
    }

    match unsafe {
        (*session).session.core(core).unwrap().read_8(address, std::slice::from_raw_parts_mut(dest, size))
    } {
        Ok(_) => 0,
        Err(e) => {
            unsafe { (*session).error = Some(e); }
            2
        }
    }
}

#[no_mangle]
pub fn psprobe_session_read_memory_16(session: *mut ExtSession, core: usize, address: u64, size: usize, dest: *mut u8) -> u32 {
    if session.is_null() {
        return 1;
    }

    match unsafe {
        (*session).session.core(core).unwrap().read_8(address, std::slice::from_raw_parts_mut(dest, size * 2))
    } {
        Ok(_) => 0,
        Err(e) => {
            unsafe { (*session).error = Some(e); }
            2
        }
    }
}

#[no_mangle]
pub fn psprobe_session_read_memory_32(session: *mut ExtSession, core: usize, address: u64, size: usize, dest: *mut u32) -> u32 {
    if session.is_null() {
        return 1;
    }

    match unsafe {
        (*session).session.core(core).unwrap().read_32(address, std::slice::from_raw_parts_mut(dest, size))
    } {
        Ok(_) => 0,
        Err(e) => {
            unsafe { (*session).error = Some(e); }
            2
        }
    }
}

#[no_mangle]
pub fn psprobe_session_read_memory_64(session: *mut ExtSession, core: usize, address: u64, size: usize, dest: *mut u64) -> u32 {
    if session.is_null() {
        return 1;
    }

    match unsafe {
        (*session).session.core(core).unwrap().read_64(address, std::slice::from_raw_parts_mut(dest, size))
    } {
        Ok(_) => 0,
        Err(e) => {
            unsafe { (*session).error = Some(e); }
            2
        }
    }
}
