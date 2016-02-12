extern crate libc;

use libc::{c_float, c_uint, c_int, c_ushort, c_uchar};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ControllerData {
    pub pos: [c_float; 3],
    pub rot_mat: [[c_float; 3]; 3],
    pub joystick_x: c_float,
    pub joystick_y: c_float,
    pub trigger: c_float,
    pub buttons: c_uint,
    pub sequence_number: c_uchar,
    pub rot_quat: [c_float; 4],
    pub firmware_revision: c_ushort,
    pub hardware_revision: c_ushort,
    pub packet_type: c_ushort,
    pub magnetic_frequency: c_ushort,
    pub enabled: c_int,
    pub controller_index: c_int,
    pub is_docked: c_uchar,
    pub which_hand: c_uchar,
    pub hemi_tracking_enabled: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllControllerData {
    pub controllers: [ControllerData; 4]
}

#[link(name="sixense_x64")]
extern "C" {
    pub fn sixenseInit();
    pub fn sixenseExit();
    pub fn sixenseGetNewestData(which: c_int, data: *mut ControllerData);
    pub fn sixenseGetAllNewestData(data: *mut AllControllerData);
}
