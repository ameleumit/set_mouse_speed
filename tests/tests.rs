use mki::{self, Action, Keyboard, State};

use anyhow::{Context, Error};
use tray_icon::{
    TrayIcon, TrayIconBuilder, TrayIconEvent,
    menu::{Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem},
};
use windows::Win32::UI::Shell::ShellMessageBoxW;
use windows::Win32::UI::WindowsAndMessaging::{
    MB_ICONHAND, MB_OK, SPI_GETMOUSESPEED, SPI_SETMOUSESPEED, SystemParametersInfoW,
};
use windows::core::{HSTRING, PCWSTR};
use winit::{application::ApplicationHandler, event_loop::EventLoop};
use single_instance::SingleInstance;

#[test]
fn read_mouse_speed() {
   unsafe {
        let mut current_speed: i32 = 0;
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            Some(&mut current_speed as *mut i32 as *mut std::ffi::c_void),
            Default::default(),
        ).expect("error while reading mouse speed");
   
    };
	
}
#[test]
fn ensure_mouse_speed_between_0_and_20() {
    let current_speed = unsafe {
        let mut current_speed: i32 = 0;
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            Some(&mut current_speed as *mut i32 as *mut std::ffi::c_void),
            Default::default(),
        ).expect("error while reading mouse speed");
        current_speed
    };
	assert!((0<=current_speed)&&(current_speed<=20));
	
}
#[test]
fn set_mouse_speed() {
    let current_speed=unsafe {
        let mut current_speed: i32 = 0;
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            Some(&mut current_speed as *mut i32 as *mut std::ffi::c_void),
            Default::default(),
        ).expect("error while reading mouse speed");
		current_speed
    };

	unsafe {
                        SystemParametersInfoW(
                            SPI_SETMOUSESPEED,
                            0,
                            Some(current_speed as *mut std::ffi::c_void),
                            Default::default(),
                        )
                        .context("Error while setting slow down speed to")
                        .expect("error while setting mouse speed");
                    }
	
}