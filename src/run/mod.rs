use std::{process, sync::{Arc, Mutex}};

use nwg::{Window, WindowFlags};
use CreateProcessW::Command;
extern crate native_windows_gui as nwg;

static mut SHOULD_EXIT: bool = false;

pub fn create_run_window(){
    let mut run_window = nwg::Window::default();
    nwg::Window::builder()
        .size((400, 150))
        .title("Run")
        .build(&mut run_window)
        .expect("Error Creating the Run Window");
    let mut font = nwg::Font::default();
    nwg::Font::builder()
        .family("Microsoft JhengHei Light")
        .size(16)       
        .weight(400)     
        .build(&mut font)
        .expect("Failed to create font");

    let mut textfont = nwg::Font::default();
    nwg::Font::builder()
        .family("Microsoft JhengHei Light")
        .size(20)       
        .weight(400)     
        .build(&mut textfont)
        .expect("Failed to create font");

    let mut label = nwg::Label::default();
    nwg::Label::builder()
        .parent(&run_window)
        .font(Some(&textfont))
        .position((10, 10))
        .size((300, 50))
        .text("Enter The Application You Want to Run")
        .build(&mut label)
        .expect("f");

    let mut button = nwg::Button::default();
    nwg::Button::builder()
        .parent(&run_window)
        .size((70, 30))
        .text("Run")
        .position((10, 100))
        .font(Some(&font))
        .build(&mut button)
        .expect("bad");

    let mut inputbox = nwg::TextInput::default();
    nwg::TextInput::builder()
        .parent(&run_window)
        .size((350, 21))
        .position((10, 50))
        .font(Some(&font))
        .build(&mut inputbox)
        .expect("bad");

    inputbox.set_focus();

    
    let handler = nwg::full_bind_event_handler(&run_window.handle, move |evt, _evt_data, handle| {
        if evt == nwg::Event::OnButtonClick && handle == button.handle {
            let command = Command::new(inputbox.text()).spawn();
            match command {
                Ok(_) => {
                }
                Err(_) => {
                    nwg::simple_message("Error", "Couldn't Create Process");
                }
            }
        }
    });
    
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler); 

}