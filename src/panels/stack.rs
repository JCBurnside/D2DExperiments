use std::{
    iter::once,
    sync::atomic::{AtomicBool, Ordering},
};

use windows::Win32::{
    Foundation::{HWND, PWSTR, RECT},
    UI::WindowsAndMessaging::{
        CreateWindowExW, GetClientRect, LoadCursorW, RegisterClassW, SetWindowPos, CS_HREDRAW,
        CS_VREDRAW, IDC_ARROW, SWP_DRAWFRAME, SWP_NOACTIVATE, SWP_NOOWNERZORDER, SWP_NOZORDER,
        WNDCLASSW, WS_CHILDWINDOW, WS_CLIPCHILDREN, WS_VISIBLE,
    },
};

use crate::{interface::IWindow, support::Fill};
pub struct StackPanel {
    orientation: Orientation,
    children: Vec<Box<dyn IWindow>>,
    fill: (Fill, Fill),
    pos: (i32, i32),
    handle: HWND,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

impl StackPanel {
    pub fn new(
        orientation: Orientation,
        children: Vec<Box<dyn IWindow>>,
        fill: (Fill, Fill),
    ) -> Box<Self> {
        Box::new(Self {
            orientation,
            children,
            fill,
            pos: (0, 0),
            handle: HWND(0),
        })
    }
}

impl IWindow for StackPanel {
    fn handle_message(
        &mut self,
        _msg: u32,
        _wparam: windows::Win32::Foundation::WPARAM,
        _lparam: windows::Win32::Foundation::LPARAM,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn init(
        &self,
        parent: Option<windows::Win32::Foundation::HWND>,
        instance: Option<windows::Win32::Foundation::HINSTANCE>,
    ) -> anyhow::Result<()> {
        let parent = parent.expect("Must be child");
        let instance = instance.expect("Must be child");
        unsafe {
            let mut wc_name = self
                .get_wc_name()
                .encode_utf16()
                .chain(once(0))
                .collect::<Vec<_>>();
            let wc_name = PWSTR(wc_name.as_mut_ptr());
            lazy_static::lazy_static! {
                static ref REGISTERED : AtomicBool = AtomicBool::new(false);
                static ref LOCK : std::sync::Mutex<()> = std::sync::Mutex::new(());
            }
            {
                //lock to make sure you can't register it twice
                let _key = LOCK.lock().unwrap();
                if !REGISTERED.load(Ordering::Acquire) {
                    let wc = WNDCLASSW {
                        style: CS_VREDRAW | CS_HREDRAW,
                        hCursor: LoadCursorW(None, IDC_ARROW),
                        lpfnWndProc: Some(Self::wnd_proc),
                        hInstance: instance,
                        lpszClassName: wc_name,
                        ..Default::default()
                    };
                    let atom = RegisterClassW(&wc);
                    debug_assert!(atom != 0, "Failed to register class");
                    REGISTERED.store(true, Ordering::Release);
                }
            }

            let mut rect = RECT::default();
            if !GetClientRect(parent, &mut rect).as_bool() {
                anyhow::bail!("Failed to get parent client rect");
            }
            let width = match self.fill.0 {
                Fill::Fill => rect.right.checked_sub(self.pos.0).unwrap_or(0),
                Fill::Fixed(width) => width as i32,
                Fill::Percent(perc) => {
                    (rect.right.checked_sub(self.pos.0).unwrap_or(0) as f32 * perc).floor() as i32
                }
            };
            let height = match self.fill.1 {
                Fill::Fill => rect.bottom.checked_sub(self.pos.1).unwrap_or(0),
                Fill::Fixed(height) => height as i32,
                Fill::Percent(perc) => {
                    (rect.bottom.checked_sub(self.pos.1).unwrap_or(0) as f32 * perc).floor() as i32
                }
            };

            let handle = CreateWindowExW(
                Default::default(),
                wc_name,
                None,
                WS_CHILDWINDOW | WS_CLIPCHILDREN | WS_VISIBLE,
                self.pos.0,
                self.pos.1,
                width,
                height,
                parent,
                None,
                instance,
                self as *const Self as _,
            );
            debug_assert!(handle.0 != 0, "Invalid handle");
            debug_assert!(handle == self.handle, "Handle doesn't match");

            for child in self.children.iter() {
                child.init(Some(handle), Some(instance))?;
            }
        }
        Ok(())
    }

    fn set_handle(&mut self, handle: windows::Win32::Foundation::HWND) {
        self.handle = handle;
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        for child in self.children.iter_mut() {
            child.draw()?;
        }
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if self.handle.0 == 0 || self.children.iter().any(|c| c.get_handle().0 == 0) {
            return Ok(());
        }
        match self.orientation {
            Orientation::Horizontal => {
                let mut used_width = 0;
                for child in self.children.iter_mut() {
                    let (width_fill, height_fill) = child.get_fill();
                    let width = match width_fill {
                        Fill::Fill => width.saturating_sub(used_width) as u32,
                        Fill::Fixed(width) => width,
                        Fill::Percent(perc) => (width as f32 * perc).floor() as u32,
                    };
                    let height = match height_fill {
                        Fill::Fill => height as u32,
                        Fill::Fixed(height) => height,
                        Fill::Percent(perc) => (height as f32 * perc).floor() as u32,
                    };
                    let x = used_width;
                    used_width += width;
                    unsafe {
                        SetWindowPos(
                            child.get_handle(),
                            None,
                            x as i32,
                            0,
                            width as i32,
                            height as i32,
                            SWP_DRAWFRAME | SWP_NOOWNERZORDER | SWP_NOZORDER | SWP_NOACTIVATE,
                        )
                    };
                }
            }
            Orientation::Vertical => {
                let mut used_height = 0;
                for child in self.children.iter_mut() {
                    let (width_fill, height_fill) = child.get_fill();
                    let width = match width_fill {
                        Fill::Fill => width as u32,
                        Fill::Fixed(width) => width,
                        Fill::Percent(perc) => (width as f32 * perc).floor() as u32,
                    };
                    let height = match height_fill {
                        Fill::Fill => height.saturating_sub(used_height) as u32,
                        Fill::Fixed(height) => height,
                        Fill::Percent(perc) => (height as f32 * perc).floor() as u32,
                    };
                    let y = used_height;
                    used_height += height;
                    unsafe {
                        SetWindowPos(
                            child.get_handle(),
                            None,
                            0,
                            y as i32,
                            width as i32,
                            height as i32,
                            SWP_NOOWNERZORDER | SWP_NOZORDER | SWP_NOACTIVATE,
                        )
                    };
                }
            }
        }
        Ok(())
    }

    fn get_handle(&self) -> HWND {
        self.handle
    }

    fn get_fill(&self) -> (Fill, Fill) {
        self.fill
    }
}
