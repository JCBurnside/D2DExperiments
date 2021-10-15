#![feature(backtrace)]
#![allow(non_snake_case)]

use bindings::Windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

mod input;
mod support;
mod window;

fn main() -> windows::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    };
    let mut app = window::MainWindow::new()?;
    app.run()
}
