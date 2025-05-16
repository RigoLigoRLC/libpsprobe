use probe_rs::probe::list::Lister;
use probe_rs::probe::{DebugProbeInfo, Probe, WireProtocol};

pub struct ExtProbeList {
    probes: Vec<DebugProbeInfo>,
}

pub struct ExtProbeInfo {
    pub probe: DebugProbeInfo,
}

pub struct ExtProbe {
    pub probe: Probe,
    pub dropped: bool,
}

#[no_mangle]
pub extern "C" fn psprobe_probe_list_get(
    probes_out: *mut *mut ExtProbeList,
    size_out: *mut usize,
) -> u32 {
    let probes = Lister::new().list_all();

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
    probe_out: *mut *mut ExtProbeInfo,
) -> u32 {
    if probes.is_null() {
        return 1;
    }

    let probes = unsafe { &*probes };

    if index >= probes.probes.len() {
        return 1;
    }

    unsafe {
        *probe_out = Box::into_raw(Box::new(ExtProbeInfo {
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
    probe: *mut ExtProbeInfo,
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
    probe: *mut ExtProbeInfo,
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
    probe: *mut ExtProbeInfo,
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
pub extern "C" fn psprobe_probe_destroy(probe: *mut ExtProbeInfo) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let _ = unsafe { Box::from_raw(probe) };

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_open(
    probe_info: *mut ExtProbeInfo,
    probe_out: *mut *mut ExtProbe,
) -> u32 {
    if probe_info.is_null() {
        return 1;
    }

    let probe_info = unsafe { &*probe_info };
    let probe = match probe_info.probe.open() {
        Ok(probe) => probe,
        Err(_) => return 2,
    };

    unsafe { *probe_out = Box::into_raw(Box::new(ExtProbe { probe, dropped: false })); };

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_set_connection_speed(probe: *mut ExtProbe, speed: u32) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let probe = unsafe { &mut (*probe).probe };
    match probe.set_speed(speed) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

#[no_mangle]
pub extern "C" fn psprobe_probe_get_connection_speed(probe: *mut ExtProbe, speed_out: *mut u32) -> u32 {
    if probe.is_null() || speed_out.is_null() {
        return 1;
    };

    let probe = unsafe { &(*probe).probe };
    unsafe { *speed_out = probe.speed_khz() };

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_set_protocol(probe: *mut ExtProbe, protocol: u32) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let probe = unsafe { &mut (*probe).probe };
    let protocol = match protocol {
        1 => WireProtocol::Swd,
        _ => WireProtocol::Jtag,
    };

    match probe.select_protocol(protocol) {
        Ok(_) => 0,
        Err(_) => 2,
    }
}

#[no_mangle]
pub extern "C" fn psprobe_probe_get_protocol(probe: *mut ExtProbe, protocol_out: *mut u32) -> u32 {
    if probe.is_null() || protocol_out.is_null() {
        return 1;
    }

    let probe = unsafe { &(*probe).probe };
    unsafe {
        *protocol_out = match probe.protocol() {
            Some(proto) => match proto {
                WireProtocol::Swd => 1,
                WireProtocol::Jtag => 2,
                _ => 0,
            }
            _ => 0,
        }
    }

    0
}

#[no_mangle]
pub extern "C" fn psprobe_probe_close(probe: *mut ExtProbe) -> u32 {
    if probe.is_null() {
        return 1;
    }

    let extprobe = unsafe { Box::from_raw(probe) };

    if extprobe.dropped {
        std::mem::forget(extprobe.probe);
    }

    0
}

