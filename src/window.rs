use std::iter::once;

use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        self, CreateWindowExW, DefWindowProcW, DispatchMessageW, PeekMessageW, PostQuitMessage,
        RegisterClassW, SetWindowPos, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, MSG,
        PM_REMOVE, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOZORDER, WNDCLASSW, WS_CLIPCHILDREN,
        WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

use crate::{
    input::TextBox,
    interface::*,
    panels::stack::{Orientation, StackPanel},
    support::{hiword, loword, Fill},
    test::Test,
};

pub struct MainWindow {
    handle: HWND,
    init: bool,
    child: Box<dyn IWindow>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            handle: HWND::default(),
            init: false,
            child: StackPanel::new(
                Orientation::Vertical,
                vec![
                    Box::new(TextBox::new((0, 0), Fill::Fill, Fill::Fixed(100))),
                    StackPanel::new(Orientation::Horizontal, vec![Test::new((Fill::Percent(0.25),Fill::Fill)),Test::new((Fill::Percent(0.75),Fill::Fill))],(Fill::Fill,Fill::Fill))
                ],
                (Fill::Fill, Fill::Fill),
            ),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        unsafe {
            self.init(None, None)?;

            let mut msg = MSG::default();
            loop {
                self.draw()?;

                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    if msg.message == WindowsAndMessaging::WM_QUIT {
                        return Ok(());
                    }
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
}
impl crate::interface::IWindow for MainWindow {
    fn handle_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        unsafe {
            match msg {
                WindowsAndMessaging::WM_SIZE => {
                    self.resize(loword(lparam.0) as u32, hiword(lparam.0) as u32)
                        .unwrap();
                    LRESULT(0)
                }
                WindowsAndMessaging::WM_DISPLAYCHANGE => {
                    self.draw().unwrap();
                    LRESULT(0)
                }
                WindowsAndMessaging::WM_DESTROY => {
                    PostQuitMessage(0);
                    LRESULT(0)
                }
                _ => DefWindowProcW(self.handle, msg, wparam, lparam),
            }
        }
    }

    fn init(&self, target: Option<HWND>, instance: Option<HINSTANCE>) -> anyhow::Result<()> {
        if target.is_some() || instance.is_some() {
            panic!("THIS SHOULD NOT BE REACHED. TRIED TO CREATE A WINDOW AS A CHILD");
        }

        if self.init {
            return Ok(());
        }
        unsafe {
            let mut wc_name = self
                .get_wc_name()
                .encode_utf16()
                .chain(once(0))
                .collect::<Vec<_>>();
            debug_assert!(wc_name.len() < 256);
            let wc_name = PWSTR(wc_name.as_mut_ptr());
            let instance = GetModuleHandleW(None);
            debug_assert!(instance.0 != 0);
            let wc = WNDCLASSW {
                // hCursor: LoadCursorW(None, IDC_ARROW),
                // hInstance: instance,
                lpszClassName: wc_name,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),

                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);
            let handle = CreateWindowExW(
                Default::default(),
                wc_name,
                "Sample Window",
                WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                self as *const _ as _,
            );
            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);

            self.child.init(Some(handle), Some(instance))?;
        }
        Ok(())
    }

    fn set_handle(&mut self, handle: HWND) {
        self.handle = handle;
    }

    fn on_create(&mut self) -> anyhow::Result<()> {
        self.init = true;

        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        self.child.draw()?;
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if self.handle.0 == 0 || self.child.get_handle().0 == 0 {
            return Ok(());
        }
        let (fill_width, fill_height) = self.child.get_fill();
        let new_width = match fill_width {
            Fill::Fill => width,
            Fill::Fixed(width) => width,
            Fill::Percent(perc) => (width as f32 * perc).floor() as u32,
        };
        let new_height = match fill_height {
            Fill::Fill => height,
            Fill::Fixed(height) => height,
            Fill::Percent(perc) => (height as f32 * perc).floor() as u32,
        };
        if !unsafe {
            SetWindowPos(
                self.child.get_handle(),
                None,
                0,
                0,
                dbg!(new_width) as i32,
                dbg!(new_height) as i32,
                SWP_NOOWNERZORDER | SWP_NOZORDER | SWP_NOACTIVATE,
            )
            .as_bool()
        } {
            anyhow::bail!("couldn't set window pos");
        }

        self.draw()
    }

    fn get_handle(&self) -> HWND {
        self.handle
    }
}
