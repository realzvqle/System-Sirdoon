#![windows_subsystem = "windows"]


use std::process;

use phnt::ffi::NtShutdownSystem;
use winapi::{shared::ntdef::NT_SUCCESS, um::{powrprof::SetSuspendState, reason::SHTDN_REASON_MAJOR_SYSTEM, winuser::{ExitWindowsEx, EWX_REBOOT, EWX_SHUTDOWN}}};



mod run;
mod presetup;
extern crate native_windows_gui as nwg;

fn main(){
    presetup::give_power_permissions();
    nwg::init().expect("Error");

    let mut window = nwg::Window::default();
    nwg::Window::builder()
        .size((800, 700))
        .title("System Sirdoon")
        .build(&mut window)
        .expect("Error with the Window");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Microsoft JhengHei Light")
        .size(20)       
        .weight(400)     
        .build(&mut font)
        .expect("Failed to create font");

    let mut sysmenu = nwg::Menu::default();
    nwg::Menu::builder()
        .parent(&window)
        .text("System")
        .build(&mut sysmenu)
        .expect("Bad");
    let mut run = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&sysmenu)
        .text("Run")
        .build(&mut run)
        .expect("Bad");

    let mut powermenu = nwg::Menu::default();
    nwg::Menu::builder()
        .parent(&window)
        .text("Power")
        .build(&mut powermenu)
        .expect("Bad");

    let mut shutdown: nwg::Menu = nwg::Menu::default();
    nwg::Menu::builder()
        .parent(&powermenu)
        .text("Shutdown")
        .build(&mut shutdown)
        .expect("Bad");

    let mut restart: nwg::Menu = nwg::Menu::default();
    nwg::Menu::builder()
        .parent(&powermenu)
        .text("Restart")
        .build(&mut restart)
        .expect("Bad");

    let mut normalshutdown: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&shutdown)
        .text("Normal")
        .build(&mut normalshutdown)
        .expect("Bad");

    let mut forceshutdown: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&shutdown)
        .text("Force")
        .build(&mut forceshutdown)
        .expect("Bad");

    let mut normalrestart: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&restart)
        .text("Normal")
        .build(&mut normalrestart)
        .expect("Bad");

    let mut forcerestart: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&restart)
        .text("Force")
        .build(&mut forcerestart)
        .expect("Bad");


    let mut suspend: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&powermenu)
        .text("Suspend")
        .build(&mut suspend)
        .expect("Bad");

    let mut hibernate: nwg::MenuItem = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&powermenu)
        .text("Hibernate")
        .build(&mut hibernate)
        .expect("Bad");
    

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        if evt == nwg::Event::OnWindowClose && handle == window.handle {
            process::exit(0); 
        }
        if evt == nwg::Event::OnMenuItemSelected {
            if handle == run.handle {
                run::create_run_window();
            }
            if handle == normalshutdown.handle {
                let status = unsafe{ExitWindowsEx(EWX_SHUTDOWN, SHTDN_REASON_MAJOR_SYSTEM)};
                if status == 0 {
                    nwg::simple_message("System Sirdoon","Failed Shutting Down The Machine");
                    return;
                }
            }
            if handle == normalrestart.handle {
                let status = unsafe{ExitWindowsEx(EWX_REBOOT, SHTDN_REASON_MAJOR_SYSTEM)};
                if status == 0 {
                    nwg::simple_message("System Sirdoon","Failed Restarting The Machine");
                    return;
                }
            }

            if handle == forceshutdown.handle {
                unsafe {
                    let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownPowerOff);
                    if !NT_SUCCESS(status) {
                        nwg::simple_message("System Sirdoon", "Failed Shutting Down The Machine");
                        return;
                    }
                }
            }
            if handle == forcerestart.handle {
                unsafe {
                    let status = NtShutdownSystem(phnt::ffi::SHUTDOWN_ACTION::ShutdownReboot);
                    if !NT_SUCCESS(status) {
                        nwg::simple_message("System Sirdoon", "Failed Restarting The Machine");
                        return;
                    }
                }
            }
            if handle == suspend.handle {
                let status = unsafe{SetSuspendState(0, 1, 0)};
                if status == 0 {
                    nwg::simple_message("System Sirdoon","Failed Suspending The Machine");
                    return;
                }
            }
            if handle == hibernate.handle {
                let status = unsafe{SetSuspendState(1, 1, 0)};
                if status == 0 {
                    nwg::simple_message("System Sirdoon","Failed Suspending The Machine");
                    return;
                }
            }
            
        }
    });
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler); 

}





