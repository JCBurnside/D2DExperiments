use std::sync::{Arc, Mutex};

use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PSTR, WPARAM},
    Graphics::Direct2D::{
        ID2D1DeviceContext, ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush, D2D_RECT_F,
    },
    UI::WindowsAndMessaging::{
        self, CreateWindowExA, DefWindowProcA, LoadCursorW, RegisterClassA, CREATESTRUCTA,
        CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_IBEAM, WNDCLASSA, WS_CHILD,
        WS_CHILDWINDOW, WS_CLIPCHILDREN, WS_VISIBLE,
    },
};
use windows::Handle;

use crate::support::{
    create_brush, create_factory, create_render_target, Color, GetWindowLong, SetWindowLong,
};

pub struct TextBox {
    class: WNDCLASSA,
    factory: ID2D1Factory1,
    target: Option<ID2D1HwndRenderTarget>,
    green: Option<ID2D1SolidColorBrush>,
    wc_name: PSTR,
    handle: HWND,
}

impl TextBox {
    pub fn new(target: HWND, instance: HINSTANCE) -> windows::Result<TextBox> {
        let wc_name = PSTR(b"TextBox".as_ptr() as _);
        let wc = WNDCLASSA {
            style: CS_VREDRAW | CS_HREDRAW,
            hCursor: unsafe { LoadCursorW(None, IDC_IBEAM) },
            lpfnWndProc: Some(Self::wnd_proc),
            lpszClassName: wc_name,
            ..Default::default()
        };
        let atom = unsafe { RegisterClassA(&wc) };
        debug_assert!(atom != 0, "FAILED TO REGISTER CLASS");

        let mut this = Self {
            class: wc,
            factory: create_factory()?,
            target: None,
            green: None,
            wc_name: wc_name.clone(),
            handle: HWND::default(),
        };
        let handle = unsafe {
            CreateWindowExA(
                Default::default(),
                wc_name,
                "foo",
                WS_CHILD | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                100,
                10,
                target,
                None,
                instance,
                &this as *const _ as _,
            )
        };

        debug_assert!(!handle.is_invalid());
        debug_assert!(handle == this.handle);
        if this.target.is_none() {
            let target = create_render_target(&this.factory, this.handle).unwrap();
            this.green = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 0.0)).unwrap());
            this.target = Some(target);
        }
        Ok(this)
    }

    fn draw(&mut self) {
        if self.target.is_none() {
            self.target = Some(create_render_target(&self.factory, self.handle).unwrap());
        }
        let target = self.target.as_ref().unwrap();
        if self.green.is_none() {
            self.green = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 0.0)).unwrap());
        }
        let green = self.green.as_ref().unwrap();
        unsafe {
            let size = target.GetSize();
            let rect = D2D_RECT_F {
                top: 0.0,
                left: 0.0,
                right: size.width,
                bottom: size.height,
            };

            target.BeginDraw();

            target.FillRectangle(&rect, green);
            target
                .EndDraw(std::ptr::null_mut(), std::ptr::null_mut())
                .unwrap();
        }
    }
    fn set_handle(&mut self, hwnd: HWND) {
        self.handle = hwnd;
    }

    unsafe fn handle_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WindowsAndMessaging::WM_PAINT => {
                self.draw();
                LRESULT(0)
            }
            WindowsAndMessaging::WM_DISPLAYCHANGE => {
                self.draw();
                LRESULT(0)
            }
            _ => DefWindowProcA(self.handle, msg, wparam, lparam),
        }
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WindowsAndMessaging::WM_NCCREATE {
            let cs = lparam.0 as *const CREATESTRUCTA;
            let this = (*cs).lpCreateParams as *mut Self;
            (*this).set_handle(hwnd);

            SetWindowLong(hwnd, GWLP_USERDATA, this as _);
        } else {
            let this = GetWindowLong(hwnd, GWLP_USERDATA) as *mut Self;
            if !this.is_null() {
                return (*this).handle_message(msg, wparam, lparam);
            }
        }
        DefWindowProcA(hwnd, msg, wparam, lparam)
    }
}
