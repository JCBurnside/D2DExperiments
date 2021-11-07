#![feature(backtrace)]
#![feature(once_cell)]
#![feature(type_name_of_val)]
#![allow(non_snake_case)]

use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

mod input;
mod interface;
mod support;
mod window;

fn main() -> windows::runtime::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    };
    let mut app = window::MainWindow::new();
    app.run()
}
