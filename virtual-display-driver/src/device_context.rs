use log::info;
use wdf_umdf::WDF_DECLARE_CONTEXT_TYPE;
use wdf_umdf_sys::{
    IDARG_IN_ADAPTER_INIT_FINISHED, IDARG_IN_COMMITMODES, IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    IDARG_IN_PARSEMONITORDESCRIPTION, IDARG_IN_QUERYTARGETMODES, IDARG_IN_SETSWAPCHAIN,
    IDARG_OUT_GETDEFAULTDESCRIPTIONMODES, IDARG_OUT_PARSEMONITORDESCRIPTION,
    IDARG_OUT_QUERYTARGETMODES, IDDCX_ADAPTER__, IDDCX_MONITOR__, NTSTATUS, WDFDEVICE,
    WDF_POWER_DEVICE_STATE,
};
use windows::Win32::Foundation::STATUS_NOT_IMPLEMENTED;

// Taken from
// https://github.com/ge9/IddSampleDriver/blob/fe98ccff703b5c1e578a0d627aeac2fa77ac58e2/IddSampleDriver/Driver.cpp#L403
static MONITOR_EDID: &[u8] = &[
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x31, 0xD8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x05, 0x16, 0x01, 0x03, 0x6D, 0x32, 0x1C, 0x78, 0xEA, 0x5E, 0xC0, 0xA4, 0x59, 0x4A, 0x98, 0x25,
    0x20, 0x50, 0x54, 0x00, 0x00, 0x00, 0xD1, 0xC0, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
    0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x02, 0x3A, 0x80, 0x18, 0x71, 0x38, 0x2D, 0x40, 0x58, 0x2C,
    0x45, 0x00, 0xF4, 0x19, 0x11, 0x00, 0x00, 0x1E, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x4C, 0x69, 0x6E,
    0x75, 0x78, 0x20, 0x23, 0x30, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFD, 0x00, 0x3B,
    0x3D, 0x42, 0x44, 0x0F, 0x00, 0x0A, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0xFC,
    0x00, 0x4C, 0x69, 0x6E, 0x75, 0x78, 0x20, 0x46, 0x48, 0x44, 0x0A, 0x20, 0x20, 0x20, 0x00, 0x05,
];

pub struct DeviceContext {
    device: WDFDEVICE,
}

unsafe impl Sync for DeviceContext {}

impl DeviceContext {
    pub fn new(device: WDFDEVICE) -> Self {
        Self { device }
    }
}

WDF_DECLARE_CONTEXT_TYPE!(pub DeviceContext);

pub extern "C-unwind" fn adapter_init_finished(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_ADAPTER_INIT_FINISHED,
) -> NTSTATUS {
    info!("adapter_init_finished");
    todo!()
}

pub extern "C-unwind" fn device_d0_entry(
    device: WDFDEVICE,
    previous_state: WDF_POWER_DEVICE_STATE,
) -> NTSTATUS {
    info!("device_d0_entry");
    todo!()
}

pub extern "C-unwind" fn parse_monitor_description(
    p_in_args: *const IDARG_IN_PARSEMONITORDESCRIPTION,
    p_out_args: *mut IDARG_OUT_PARSEMONITORDESCRIPTION,
) -> NTSTATUS {
    info!("parse_monitor_description");
    todo!()
}

pub extern "C-unwind" fn monitor_get_default_modes(
    _monitor_object: *mut IDDCX_MONITOR__,
    _p_in_args: *const IDARG_IN_GETDEFAULTDESCRIPTIONMODES,
    _p_out_args: *mut IDARG_OUT_GETDEFAULTDESCRIPTIONMODES,
) -> NTSTATUS {
    info!("monitor_get_default_modes");
    STATUS_NOT_IMPLEMENTED.0.into()
}

pub extern "C-unwind" fn monitor_query_modes(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_QUERYTARGETMODES,
    p_out_args: *mut IDARG_OUT_QUERYTARGETMODES,
) -> NTSTATUS {
    info!("monitor_query_modes");
    todo!()
}

pub extern "C-unwind" fn adapter_commit_modes(
    adapter_object: *mut IDDCX_ADAPTER__,
    p_in_args: *const IDARG_IN_COMMITMODES,
) -> NTSTATUS {
    info!("adapter_commit_modes");
    todo!()
}

pub extern "C-unwind" fn assign_swap_chain(
    monitor_object: *mut IDDCX_MONITOR__,
    p_in_args: *const IDARG_IN_SETSWAPCHAIN,
) -> NTSTATUS {
    info!("assign_swap_chain");
    todo!()
}

pub extern "C-unwind" fn unassign_swap_chain(monitor_object: *mut IDDCX_MONITOR__) -> NTSTATUS {
    info!("unassign_swap_chain");
    todo!()
}
