
use std::{ffi::CString, ptr};

use nwg::CheckBoxState;
use winapi::um::shellapi::ShellExecuteA;
extern crate native_windows_gui as nwg;


pub fn create_run_window(){
    let mut icon = nwg::Icon::default();
    nwg::Icon::builder()
        .source_file(Some("assets/run.ico"))
        .build(&mut icon)
        .expect("Failure");
    let mut run_window = nwg::Window::default();
    nwg::Window::builder()
        .size((400, 170))
        .title("Run")
        .icon(Some(&icon))
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
        .position((10, 130))
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

    let mut admincheck = nwg::CheckBox::default();
    nwg::CheckBox::builder()
        .parent(&run_window)
        .text("Run As Administrator")
        .size((250, 21))
        .font(Some(&font))
        .position((10, 75))
        .build(&mut admincheck)
        .expect("error");
    let mut trustedinstacheck = nwg::CheckBox::default();
    nwg::CheckBox::builder()
        .parent(&run_window)
        .text("Run As TrustedInstaller")
        .size((250, 21))
        .font(Some(&font))
        .position((10, 100))
        .build(&mut trustedinstacheck)
        .expect("error");
    let handler = nwg::full_bind_event_handler(&run_window.handle, move |evt, _evt_data, handle| {
        if evt == nwg::Event::OnButtonClick && handle == button.handle {
            if trustedinstacheck.check_state() == CheckBoxState::Checked{
                nwg::simple_message("System Sirdoon", "Coming Soon!");
                return
            }
            let mut types = CString::new("open").expect("Failed to convert to CString");
            if admincheck.check_state() == CheckBoxState::Checked {
                types = CString::new("runas").expect("Failed to convert to CString");
            }
            let text = inputbox.text();
            let mut vecc: Vec<&str> = text.split_whitespace().collect();
            if vecc.len() <= 1 {
                vecc.push("explorer");
            }
            let command = CString::new(vecc[0]).expect("Failed to convert to CString");
            let args = CString::new(vecc[1..].join(" ")).expect("Failed To Convert To CString");    
            unsafe {
                ShellExecuteA(
                    ptr::null_mut(),
                    types.as_ptr(),
                    command.as_ptr(),
                    args.as_ptr(),
                    ptr::null_mut(),
                    5
            )};
        }
    });  
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler); 
}



