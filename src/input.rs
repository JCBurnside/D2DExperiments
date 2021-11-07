use std::ffi::OsStr;

use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
    Graphics::{
        Direct2D::{
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
            D2D1_DRAW_TEXT_OPTIONS_NONE, D2D_RECT_F,
        },
        DirectWrite::{IDWriteFactory, IDWriteTextFormat, DWRITE_MEASURING_MODE_NATURAL},
    },
    UI::{
        Input::KeyboardAndMouse,
        WindowsAndMessaging::{
            self, CreateWindowExW, DefWindowProcW, LoadCursorW, RegisterClassW, CREATESTRUCTA,
            CREATESTRUCTW, CS_HREDRAW, CS_VREDRAW, GWLP_USERDATA, IDC_IBEAM, WNDCLASSW, WS_BORDER,
            WS_CHILD, WS_CHILDWINDOW, WS_VISIBLE,
        },
    },
};

use crate::support::{
    self, create_brush, create_factory, create_formater, create_render_target, create_text_factory,
    Color, GetWindowLong, SetWindowLong,
};

pub struct TextBox {
    factory: Option<ID2D1Factory1>,
    target: Option<ID2D1HwndRenderTarget>,
    text_factory: Option<IDWriteFactory>,
    text_format: Option<IDWriteTextFormat>,
    green: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    handle: HWND,
    data: String,
    caret_pos: usize,
    pos: (i32, i32),
    width: i32,
    height: i32,
}

impl TextBox {
    pub fn new(pos: (i32, i32), width: i32, height: i32) -> Self {
        Self {
            factory: None,
            target: None,
            green: None,
            black: None,
            text_factory: None,
            text_format: None,
            handle: HWND::default(),
            data: String::from("TEXTBOX"),
            caret_pos: 0,
            pos,
            width,
            height,
        }
    }

    pub fn init(&mut self, target: HWND, instance: HINSTANCE) -> windows::runtime::Result<()> {
        if self.factory.is_some() {
            return Ok(());
        }
        #[allow(non_upper_case_globals)]
        const wc_name : PWSTR = PWSTR(utf16_literal::utf16!("TextBox\0").as_ptr() as _);
        let wc = WNDCLASSW {
            style: CS_VREDRAW | CS_HREDRAW,
            hCursor: unsafe { LoadCursorW(None, IDC_IBEAM) },
            lpfnWndProc: Some(Self::wnd_proc),
            lpszClassName: wc_name,
            ..Default::default()
        };
        let atom = unsafe { RegisterClassW(&wc) };
        debug_assert!(atom != 0, "FAILED TO REGISTER CLASS");
        let name_os = OsStr::new("foo");
        let name = PWSTR(utf16_literal::utf16!("foo").as_ptr() as _);
        let handle = unsafe {
            CreateWindowExW(
                Default::default(),
                wc_name,
                name_os,
                WS_CHILDWINDOW | WS_VISIBLE | WS_BORDER,
                self.pos.0,
                self.pos.1,
                self.width,
                self.height,
                target,
                None,
                instance,
                self as *const Self as _,
            )
        };

        debug_assert!(handle.0 != 0);
        debug_assert!(handle == self.handle);

        self.factory = Some(create_factory()?);
        self.text_factory = Some(create_text_factory()?);
        Ok(())
    }

    pub(crate) fn draw(&mut self) {
        if self.target.is_none() {
            let target =
                create_render_target(&self.factory.as_ref().unwrap(), self.handle).unwrap();
            self.green = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 0.0)).unwrap());
            self.black = Some(create_brush(&target, &Color::RGB(0.0, 0.0, 0.0)).unwrap());
            self.target = Some(target);
            self.text_format = Some(create_formater(self.text_factory.as_ref().unwrap()).unwrap());
        }
        let target = self.target.as_ref().unwrap();
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
            target.DrawText(
                self.data.clone(),
                self.data.len() as u32,
                self.text_format.as_ref().unwrap(),
                &rect,
                self.black.as_ref().unwrap(),
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
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
            // WindowsAndMessaging::WM_PAINT | WindowsAndMessaging::WM_DISPLAYCHANGE => {
            //     self.draw();
            //     LRESULT(0)
            // },
            // WindowsAndMessaging::WM_SIZE if wparam.0 as u32 == WindowsAndMessaging::SIZE_MAXIMIZED => {
            //     self.draw();
            //     DefWindowProcW(self.handle, msg, wparam, lparam)
            // },
            WindowsAndMessaging::WM_LBUTTONDOWN => {
                KeyboardAndMouse::SetFocus(self.handle);
                LRESULT(0)
            }
            WindowsAndMessaging::WM_CHAR => {
                if wparam.0 == 0x08 {
                    //backspace char
                    self.data.pop();
                    self.draw();
                    return LRESULT(0);
                }
                if wparam.0 == 0x0B {
                    //enter is VT?!?!
                    KeyboardAndMouse::SetFocus(HWND::default());
                }
                let meta = support::Keymeta::from_bytes(u32::to_ne_bytes(lparam.0 as u32));
                let repeat = meta.count();
                println!("{:#04x}", wparam.0);
                let to_add = char::from_u32(wparam.0 as u32);
                match to_add {
                    Some(c) => {
                        println!("adding {:#} {} time(s) raw {}", c, repeat, wparam.0);
                        for _ in 0..repeat {
                            self.data.push(c);
                        }
                        self.draw();
                    }
                    None => {
                        return DefWindowProcW(self.handle, msg, wparam, lparam);
                    }
                }
                LRESULT(0)
            }
            _ => {
                println!("NOT HANDLED {}", msg);
                DefWindowProcW(self.handle, msg, wparam, lparam)
            }
        }
    }

    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WindowsAndMessaging::WM_NCCREATE {
            let cs = lparam.0 as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;
            (*this).set_handle(hwnd);

            SetWindowLong(hwnd, GWLP_USERDATA, this as _);
        } else {
            let this = GetWindowLong(hwnd, GWLP_USERDATA) as *mut Self;
            if !this.is_null() {
                return (*this).handle_message(msg, wparam, lparam);
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}
