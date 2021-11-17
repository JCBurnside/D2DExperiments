#![allow(non_snake_case)]
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

mod caret;
mod input;
mod interface;
mod panels;
mod support;
mod test;
mod window;

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    };
    let mut app = window::MainWindow::new();
    app.run()
}
