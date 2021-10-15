use std::sync::{Arc, Mutex};

use bindings::Windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, PSTR, WPARAM},
    Graphics::{
        Direct2D::{
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1RenderTarget, ID2D1SolidColorBrush,
            D2D1_DRAW_TEXT_OPTIONS_NONE, D2D_RECT_F,
        },
        DirectWrite::{IDWriteFactory, IDWriteTextFormat, DWRITE_MEASURING_MODE_NATURAL},
    },
    System::LibraryLoader::GetModuleHandleA,
    UI::WindowsAndMessaging::{
        self, CreateWindowExA, DefWindowProcA, DispatchMessageA, LoadCursorW, PeekMessageA,
        PostQuitMessage, RegisterClassA, CREATESTRUCTA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
        GWLP_USERDATA, IDC_ARROW, MSG, PM_REMOVE, WNDCLASSA, WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW,
        WS_VISIBLE,
    },
};

use windows::{Handle, Interface};

use crate::{input::TextBox, support::*};

pub struct MainWindow {
    handle: HWND,
    factory: ID2D1Factory1,
    text_factory: IDWriteFactory,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    text_format: Option<IDWriteTextFormat>,
    text_boxes: Vec<TextBox>,
}

impl MainWindow {
    pub fn new() -> windows::Result<Self> {
        let factory = create_factory()?;
        let text_factory = create_text_factory()?;
        Ok(Self {
            handle: HWND::default(),
            factory: factory,
            text_factory,
            target: None,
            brush: None,
            black: None,
            text_format: None,
            text_boxes: vec![],
        })
    }

    pub fn run(&mut self) -> windows::Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None);
            debug_assert!(!instance.is_invalid());
            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_ARROW),
                hInstance: instance,
                lpszClassName: PSTR(b"window\0".as_ptr() as _),

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),

                ..Default::default()
            };

            let atom = RegisterClassA(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExA(
                Default::default(),
                wc.lpszClassName,
                "Sample Window",
                WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                self as *mut _ as _,
            );

            debug_assert!(!handle.is_invalid());
            debug_assert!(handle == self.handle);
            if self.target.is_none() {
                let target = create_render_target(&self.factory, self.handle)?;

                self.text_format = Some(create_formater(&self.text_factory)?);
                self.brush = Some(create_brush(&target, &Color::RGB(1.0, 0.0, 0.0))?);
                self.black = Some(create_brush(&target, &Color::RGB(0.0, 0.0, 0.0))?);
                self.target = Some(target);
            }
            self.text_boxes.push(TextBox::new(handle, instance)?);
            let mut msg = MSG::default();
            loop {
                self.render()?;

                while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    if msg.message == WindowsAndMessaging::WM_QUIT {
                        return Ok(());
                    }
                    DispatchMessageA(&msg);
                }
            }
        }
    }

    fn render(&mut self) -> windows::Result<()> {
        let target = self.target.as_ref().unwrap();
        self.draw(target)?;

        Ok(())
    }

    unsafe extern "system" fn wnd_proc(
        window: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        if msg == WindowsAndMessaging::WM_NCCREATE {
            let cs = lparam.0 as *const CREATESTRUCTA;
            let this = (*cs).lpCreateParams as *mut Self;
            (*this).handle = window;
            SetWindowLong(window, GWLP_USERDATA, this as _);
        } else {
            let this = GetWindowLong(window, GWLP_USERDATA) as *mut Self;
            if !this.is_null() {
                return (*this).message_handler(msg, wparam, lparam);
            }
        }
        DefWindowProcA(window, msg, wparam, lparam)
    }

    unsafe fn message_handler(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WindowsAndMessaging::WM_PAINT => {
                self.render().unwrap();
                LRESULT(0)
            }
            WindowsAndMessaging::WM_DISPLAYCHANGE => {
                self.render().unwrap();
                LRESULT(0)
            }
            WindowsAndMessaging::WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }

            _ => DefWindowProcA(self.handle, msg, wparam, lparam),
        }
    }

    fn draw(&self, target: &ID2D1HwndRenderTarget) -> windows::Result<()> {
        let text = "FooBar";
        let len = text.len();
        unsafe {
            target.BeginDraw();
            target.Clear(&Color::RGB(0.0, 0.0, 0.0).into());
            let size = target.GetSize();

            let rect = D2D_RECT_F {
                left: 100.0,
                top: 100.0,
                right: size.width - 100.0,
                bottom: size.height - 100.0,
            };
            target.FillRectangle(&rect, self.brush.as_ref().unwrap());
            let rect = D2D_RECT_F {
                left: size.width / 2.0 - 50.0,
                top: size.height / 2.0 - 15.0,
                right: size.width / 2.0 + 50.0,
                bottom: size.height / 2.0 + 15.0,
            };
            target.DrawText(
                text,
                len as u32,
                self.text_format.as_ref().unwrap(),
                &rect,
                self.black.as_ref().unwrap(),
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        }
        Ok(())
    }
}
