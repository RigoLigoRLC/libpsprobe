use crate::bridge::ExtDynStr;
use crate::probe::ExtProbe;
use probe_rs::{CoreType, Permissions, Session};

pub struct ExtSession {
    session: Session,
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

    // Converting a probe to a session is a move, we will recycle this "probe"
    // Remember to clean up in C++ part
    let ext_probe = unsafe { Box::from_raw(probe) };

    let probe = ext_probe.probe;
    let name = match unsafe { std::str::from_utf8(std::slice::from_raw_parts(device_name, device_name_len)) } {
        Ok(name) => name,
        Err(_) => return 2,
    };

    let session = match probe.attach(name, Permissions::default()) {
        Ok(session) => session,
        Err(_) => return 3,
    };

    unsafe { *session_out = Box::into_raw(Box::new(ExtSession { session })); }

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
