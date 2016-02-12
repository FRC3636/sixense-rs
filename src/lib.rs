extern crate libc;
extern crate nalgebra;
extern crate sixense_sys;

use std::mem::zeroed;
use nalgebra::{Pnt3, Vec2, Mat3, Rot3, UnitQuat, Quat};
use libc::c_int;

pub struct Sixense;

#[derive(Copy, Clone, Debug)]
pub struct ControllerData {
    pub pos: Pnt3<f64>,
    pub rot: Rot3<f64>,
    pub rot_quat: UnitQuat<f64>,
    pub joystick: Vec2<f64>,
    pub trigger: f64
}

impl Sixense {
    pub fn new() -> Sixense {
        unsafe {
            sixense_sys::sixenseInit();
            Sixense
        }
    }

    fn convert_data(data: sixense_sys::ControllerData) -> ControllerData {
        ControllerData {
            pos: Pnt3::new(data.pos[0] as f64, data.pos[1] as f64, data.pos[2] as f64),
            rot: unsafe {
                Rot3::new_with_mat(Mat3::new(
                    data.rot_mat[0][0] as f64,
                    data.rot_mat[0][1] as f64,
                    data.rot_mat[0][2] as f64,
                    data.rot_mat[1][0] as f64,
                    data.rot_mat[1][1] as f64,
                    data.rot_mat[1][2] as f64,
                    data.rot_mat[2][0] as f64,
                    data.rot_mat[2][1] as f64,
                    data.rot_mat[2][2] as f64))
            },
            rot_quat: UnitQuat::new_with_quat(Quat::new(
                data.rot_quat[0] as f64,
                data.rot_quat[1] as f64,
                data.rot_quat[2] as f64,
                data.rot_quat[3] as f64,
            )),
            joystick: Vec2::new(data.joystick_x as f64, data.joystick_y as f64),
            trigger: data.trigger as f64
        }
    }

    pub fn newest_data(&self, index: u8) -> ControllerData {
        let data = unsafe {
            let mut data = zeroed();
            sixense_sys::sixenseGetNewestData(
                index as c_int, &mut data as *mut sixense_sys::ControllerData);
            data
        };
        Sixense::convert_data(data)
    }

    pub fn all_newest_data(&self) -> [ControllerData; 4] {
        let data = unsafe {
            let mut data: sixense_sys::AllControllerData = zeroed();
            sixense_sys::sixenseGetAllNewestData(&mut data as *mut sixense_sys::AllControllerData);
            data
        };
        [Sixense::convert_data(data.controllers[0]),
         Sixense::convert_data(data.controllers[1]),
         Sixense::convert_data(data.controllers[2]),
         Sixense::convert_data(data.controllers[3])]

    }
}

impl Drop for Sixense {
    fn drop(&mut self) {
        unsafe {
            sixense_sys::sixenseExit();
        }
    }
}
