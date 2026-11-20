#![windows_subsystem = "windows"]



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

fn main()->Result<(),Error> {


	
    std::panic::set_hook(Box::new(move |e| {
        hata_goster(e);
    }));

	let instance = SingleInstance::new("123456789").unwrap();
    assert!(instance.is_single(),"Already Running...");

    let orj_speed = unsafe {
        let mut orj_speed: i32 = 0;
        SystemParametersInfoW(
            SPI_GETMOUSESPEED,
            0,
            Some(&mut orj_speed as *mut i32 as *mut std::ffi::c_void),
            Default::default(),
        )
        .context("Error while reading original speed of mouse")?;
        orj_speed
    };
    #[cfg(feature = "hold")]
    Keyboard::LeftControl.act_on(Action {
        callback: Box::new(move |_event, state| {
            if state == State::Pressed {
                let desired_speed = 2;
                let current_speed = unsafe {
                    let mut current_speed: i32 = 0;
                    SystemParametersInfoW(
                        SPI_GETMOUSESPEED,
                        0,
                        Some(&mut current_speed as *mut i32 as *mut std::ffi::c_void),
                        Default::default(),
                    )
                    .context("Error while reading current speed of mouse")
                    .unwrap_or_else(|e| {
                        panic!("Unrecoverable Error\n{}", e);
                    });
                    current_speed
                };
                if current_speed != desired_speed {
                    unsafe {
                        SystemParametersInfoW(
                            SPI_SETMOUSESPEED,
                            0,
                            Some(desired_speed as *mut std::ffi::c_void),
                            Default::default(),
                        )
                        .context("Error while setting slow down speed to")
                        .unwrap_or_else(|e| {
                            panic!("Unrecoverable Error\n{} {}", e, desired_speed);
                        });
                    }
                    return;
                }
            } else {
                {
                    unsafe {
                        SystemParametersInfoW(
                            SPI_SETMOUSESPEED,
                            0,
                            Some(orj_speed as *mut std::ffi::c_void),
                            Default::default(),
                        )
                        .context("Error while returning to original speed of mouse to which is")
                        .unwrap_or_else(|e| {
                            panic!("Unrecoverable Error\n{} {}", e, orj_speed);
                        });
                    }
                    return;
                }
            }
        }),
        inhibit: mki::InhibitEvent::No,
        sequencer: false,
        defer: true,
    });
    #[cfg(feature = "toggle")]
    mki::register_hotkey(&[Keyboard::LeftControl], move || {
        let desired_speed = 2;
        let current_speed = unsafe {
            let mut current_speed: i32 = 0;
            SystemParametersInfoW(
                SPI_GETMOUSESPEED,
                0,
                Some(&mut current_speed as *mut i32 as *mut std::ffi::c_void),
                Default::default(),
            )
            .context("Error while reading current speed of mouse which is")
            .unwrap_or_else(|e| {
                panic!("Unrecoverable Error\n{} {}", e, current_speed);
            });
            current_speed
        };

        if current_speed != desired_speed {
            unsafe {
                SystemParametersInfoW(
                    SPI_SETMOUSESPEED,
                    0,
                    Some(desired_speed as *mut std::ffi::c_void),
                    Default::default(),
                )
                .context("Error while setting slow down speed to")
                .unwrap_or_else(|e| {
                    panic!("Unrecoverable Error\n{} {}", e, desired_speed);
                });
            }
        } else {
            unsafe {
                SystemParametersInfoW(
                    SPI_SETMOUSESPEED,
                    0,
                    Some(orj_speed as *mut std::ffi::c_void),
                    Default::default(),
                )
                .context("Error while returning to original speed of mouse to which is")
                .unwrap_or_else(|e| {
                    panic!("Unrecoverable Error\n{} {}", e, orj_speed);
                });
            }
        }
    });

    let event_loop = EventLoop::<UserEvent>::with_user_event().build().unwrap();

    // set a tray event handler that forwards the event and wakes up the event loop

    // let proxy = event_loop.create_proxy();
    // TrayIconEvent::set_event_handler(Some(move |event| {
    //     proxy.send_event(UserEvent::TrayIconEvent(event)).unwrap();
    // }));

    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |event| {
        proxy
            .send_event(UserEvent::MenuEvent(event))
            .context("Unrecoverable Error.")
            .unwrap();
    }));

    let mut app = Application::new();

    let _menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();

    if let Err(err) = event_loop.run_app(&mut app) {
        println!("Error: {:?}", err);
    }
	Ok(())
}



#[derive(Debug)]
enum UserEvent {
    _TrayIconEvent(tray_icon::TrayIconEvent),
    MenuEvent(tray_icon::menu::MenuEvent),
}

struct Application {
    tray_icon: Option<TrayIcon>,
}

impl Application {
    fn new() -> Application {
        Application { tray_icon: None }
    }

    fn new_tray_icon() -> TrayIcon {
        let icon = load_icon();

        TrayIconBuilder::new()
            .with_menu(Box::new(Self::new_tray_menu()))
            .with_tooltip("Set Mouse Speed")
            .with_icon(icon)
            .with_title("x")
            .with_guid(0u128)
            .build()
            .unwrap()
    }

    fn new_tray_menu() -> Menu {
        let menu = Menu::new();
        let item1 = MenuItem::new("Quit", true, None);
        if let Err(err) = menu.append_items(&[
            &PredefinedMenuItem::section_header("=== Set Mouse Speed ==="),
            &PredefinedMenuItem::separator(),
            &item1,
        ]) {
            println!("{err:?}");
        }
        menu
    }
}

impl ApplicationHandler<UserEvent> for Application {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) {
    }

    fn new_events(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        cause: winit::event::StartCause,
    ) {
        // We create the icon once the event loop is actually running
        // to prevent issues like https://github.com/tauri-apps/tray-icon/issues/90
        if winit::event::StartCause::Init == cause {
            {
                self.tray_icon = Some(Self::new_tray_icon());
            }
        }
    }

    fn user_event(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::MenuEvent(MenuEvent { id: MenuId(id) }) => {
                if id == "1".to_string() {
                    std::process::exit(0)
                }
            }
            _ => {}
        }
    }
}



fn load_icon() -> tray_icon::Icon {
    const ICON_BYTES: &[u8] = include_bytes!("../tray_icon.png");
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(ICON_BYTES)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height)
        .context("Failed to set icon")
        .unwrap()
}

fn hata_goster(e: &std::panic::PanicHookInfo) {
    // hata mesajlarını panic!("unknown error") olarak değil, panic!("unknown error {}",err) ya da panic!("unknown error {}",String::new()) şeklinde yaz.
    // let msg=e.payload().downcast_ref::<String>().unwrap();
    let msg = e.to_string();
    let h_title = HSTRING::from(format!("{}", "Set Mouse Speed"));
    let h_message = HSTRING::from(format!("{}", msg));
    let title = PCWSTR::from_raw(h_title.as_ptr());
    let message = PCWSTR::from_raw(h_message.as_ptr());
    unsafe {
        ShellMessageBoxW(None, None, message, title, MB_ICONHAND | MB_OK);
    };
    std::process::exit(99);

    // println!("{}", e);

    // sender.send("=== 10sn içinde çıkılıyor... ===".to_string());
    // thread::sleep(Duration::from_millis(10000));
    // std::process::exit(0);
}
