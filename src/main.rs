#![feature(backtrace)]
use bindings::Windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

mod support;
mod window;

fn main() -> anyhow::Result<()> {
    unsafe {
        CoInitializeEx(std::ptr::null_mut(), COINIT_MULTITHREADED)?;
    };
    let mut app = window::MainWindow::new()?;
    app.run()
}
