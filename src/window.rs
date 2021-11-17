use std::{
    iter::once,
    sync::atomic::{AtomicBool, Ordering},
};

use windows::Win32::{
    Foundation::{GetLastError, HINSTANCE, HWND, LPARAM, PWSTR, RECT, WPARAM},
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        self, CreateWindowExW, DispatchMessageW, GetMessageW, GetWindowRect, LoadCursorW,
        PostQuitMessage, RegisterClassW, SetWindowPos, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
        CW_USEDEFAULT, IDC_ARROW, MSG, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOZORDER, WNDCLASSW,
        WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

use crate::{
    input::TextBox,
    interface::IWindow,
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
                    TextBox::new((0, 0), Fill::Fill, Fill::Fixed(100)),
                    StackPanel::new(
                        Orientation::Horizontal,
                        vec![
                            Test::new((Fill::Percent(0.25), Fill::Fill)),
                            Test::new((Fill::Percent(0.50), Fill::Fill)),
                        ],
                        (Fill::Fill, Fill::Fill),
                    ),
                ],
                (Fill::Fill, Fill::Fill),
            ),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        unsafe {
            self.init(None, None)?;

            let mut msg = MSG::default();
            let mut state;
            while {
                state = GetMessageW(&mut msg, None, 0, 0);
                state.0 != 0
            } {
                if state.0 == -1 {
                    let err = GetLastError();
                    anyhow::bail!(format!("GetMessage {:?}", err));
                }

                if msg.message == WindowsAndMessaging::WM_QUIT {
                    return Ok(());
                }
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
            Ok(())
        }
    }
}
impl IWindow for MainWindow {
    fn handle_message(
        &mut self,
        msg: u32,
        #[allow(unused)] wparam: WPARAM,
        lparam: LPARAM,
    ) -> anyhow::Result<()> {
        unsafe {
            match msg {
                WindowsAndMessaging::WM_SIZE => {
                    self.resize(loword(lparam.0) as u32, hiword(lparam.0) as u32)
                        .unwrap();
                    Ok(())
                }
                WindowsAndMessaging::WM_DISPLAYCHANGE => {
                    self.draw().unwrap();
                    Ok(())
                }
                WindowsAndMessaging::WM_DESTROY => {
                    PostQuitMessage(0);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }

    fn init(&self, target: Option<HWND>, instance: Option<HINSTANCE>) -> anyhow::Result<()> {
        assert!(
            !(target.is_some() || instance.is_some()),
            "THIS SHOULD NOT BE REACHED. TRIED TO CREATE A WINDOW AS A CHILD"
        );

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
            lazy_static::lazy_static! {
                static ref REGISTERED : AtomicBool = AtomicBool::new(false);
                static ref LOCK : std::sync::Mutex<()> = std::sync::Mutex::new(());
            }
            {
                let _key = LOCK.lock().unwrap();
                if !REGISTERED.load(Ordering::Acquire) {
                    let wc = WNDCLASSW {
                        hCursor: LoadCursorW(None, IDC_ARROW),
                        hInstance: instance,
                        lpszClassName: wc_name,

                        style: CS_HREDRAW | CS_VREDRAW,
                        lpfnWndProc: Some(Self::wnd_proc),

                        ..WNDCLASSW::default()
                    };
                    let atom = RegisterClassW(&wc);
                    debug_assert!(atom != 0);
                    REGISTERED.store(true, Ordering::Release);
                }
            }

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
            let mut rect = RECT::default();
            GetWindowRect(handle, &mut rect);

            SetWindowPos(
                handle,
                None,
                rect.left,
                rect.top,
                rect.right - rect.left + 1,
                rect.bottom - rect.top + 1,
                SWP_NOOWNERZORDER | SWP_NOZORDER | SWP_NOACTIVATE,
            );
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
                new_width as i32,
                new_height as i32,
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
