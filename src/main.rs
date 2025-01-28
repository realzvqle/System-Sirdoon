#![windows_subsystem = "windows"]


use std::process;



mod run;
extern crate native_windows_gui as nwg;

fn main(){
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

    let mut menu = nwg::Menu::default();
    nwg::Menu::builder()
        .parent(&window)
        .text("System")
        .build(&mut menu)
        .expect("Bad");

    let mut run = nwg::MenuItem::default();
    nwg::MenuItem::builder()
        .parent(&menu)
        .text("Run")
        .build(&mut run)
        .expect("Bad");
    
    
    

    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        if evt == nwg::Event::OnWindowClose && handle == window.handle {
            process::exit(0); 
        }
        if evt == nwg::Event::OnMenuItemSelected {
            if handle == run.handle {
                run::create_run_window();
            }
        }
    });
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler); 

}





// let mut input = nwg::TextInput::default();
    // nwg::TextInput::builder()
    //     .parent(&window)
    //     .size((400, 300))
    //     .text("Enter Something")
    //     .position((100, 400))
    //     .font(Some(&font))
    //     .build(&mut input)
    //     .expect("Bad");


//let mut button = nwg::Button::default();
    // nwg::Button::builder()
    //     .parent(&window)
    //     .size((100, 100))
    //     .text("Hi")
    //     .position((100, 100))
    //     .font(Some(&font))
    //     .build(&mut button)
    //     .expect("bad");