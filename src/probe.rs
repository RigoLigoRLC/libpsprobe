use probe_rs::{DebugProbeInfo, Probe};

pub struct ExtProbeList {
    probes: Vec<DebugProbeInfo>,
}

pub struct ExtProbe {
    probe: DebugProbeInfo,
}

#[no_mangle]
pub extern "C" fn psprobe_probe_list_get(
    probes_out: *mut *mut ExtProbeList,
    size_out: *mut usize,
) -> u32 {
    let probes = Probe::list_all();

    unsafe {
        *size_out = probes.len();
        *probes_out = Box::into_raw(Box::new(ExtProbeList { probes }));
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_list_get_probe(
    probes: *mut ExtProbeList,
    index: usize,
    probe_out: *mut *mut ExtProbe,
) -> u32 {
    if probes.is_null() {
        return 1;
    }

    let probes = unsafe { &*probes };

    if index >= probes.probes.len() {
        return 1;
    }

    unsafe {
        *probe_out = Box::into_raw(Box::new(ExtProbe {
            probe: probes.probes[index].clone(),
        }));
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_list_destroy(probes: *mut ExtProbeList) -> u32 {
    if probes.is_null() {
        return 1;
    }

    let _ = unsafe { Box::from_raw(probes) };

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_get_name(
    probe: *mut ExtProbe,
    name_out: *mut *const u8,
    name_len_out: *mut usize,
) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let probe = unsafe { &((*probe).probe) };

    unsafe {
        *name_out = probe.identifier.as_ptr();
        *name_len_out = probe.identifier.len();
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_get_vid_pid(
    probe: *mut ExtProbe,
    vid_out: *mut u16,
    pid_out: *mut u16,
) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let probe = unsafe { &((*probe).probe) };

    unsafe {
        *vid_out = probe.vendor_id;
        *pid_out = probe.product_id;
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_get_serial_number(
    probe: *mut ExtProbe,
    sn_out: *mut *const u8,
    sn_len_out: *mut usize,
) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let probe = unsafe { &((*probe).probe) };
    let serial = &probe.serial_number;

    match serial {
        Some(sn_string) => unsafe {
            *sn_out = sn_string.as_ptr();
            *sn_len_out = sn_string.len();
        },

        None => unsafe {
            *sn_out = std::ptr::null_mut();
            *sn_len_out = 0;
        },
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_destroy(probe: *mut ExtProbe) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let _ = unsafe { Box::from_raw(probe) };

    0
}
