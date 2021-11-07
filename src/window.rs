use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
    Graphics::{
        Direct2D::{
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
            D2D1_DRAW_TEXT_OPTIONS_NONE, D2D_RECT_F,
        },
        DirectWrite::{IDWriteFactory, IDWriteTextFormat, DWRITE_MEASURING_MODE_NATURAL},
    },
    System::LibraryLoader::GetModuleHandleA,
    UI::{
        Input::KeyboardAndMouse::SetFocus,
        WindowsAndMessaging::{
            self, CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, PeekMessageW,
            PostQuitMessage, RegisterClassW, TranslateMessage, CREATESTRUCTA, CS_HREDRAW,
            CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, MSG, PM_REMOVE, WM_KEYUP,
            WNDCLASSW, WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

use crate::{input::TextBox, support::*};

pub struct MainWindow {
    handle: HWND,
    instance: HINSTANCE,
    factory: Option<ID2D1Factory1>,
    text_factory: Option<IDWriteFactory>,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    text_format: Option<IDWriteTextFormat>,
    text_box: TextBox,
}

impl MainWindow {
    const WC_NAME: PWSTR = PWSTR(utf16_literal::utf16!("window\0").as_ptr() as _);
    pub fn new() -> Self {
        let instance = unsafe { GetModuleHandleA(None) };
        debug_assert!(instance.0 != 0);
        Self {
            handle: HWND::default(),
            instance,
            factory: None,
            text_factory: None,
            target: None,
            brush: None,
            black: None,
            text_format: None,
            text_box: TextBox::new((0, 0), 500, 100),
        }
    }

    fn init(&mut self) -> windows::runtime::Result<()> {
        if self.factory.is_some() {
            return Ok(());
        }
        unsafe {
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW),
                hInstance: self.instance,
                lpszClassName: Self::WC_NAME,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),

                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExW(
                Default::default(),
                Self::WC_NAME,
                "Sample Window",
                WS_OVERLAPPEDWINDOW | WS_VISIBLE | WS_CLIPCHILDREN,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                self.instance,
                self as *mut _ as _,
            );
            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
            let factory = create_factory()?;
            let text_factory = create_text_factory()?;

            self.factory = Some(factory);
            self.text_factory = Some(text_factory);
            self.text_box.init(self.handle, self.instance)?;
            self.text_box.draw();
        }
        Ok(())
    }

    pub fn run(&mut self) -> windows::runtime::Result<()> {
        unsafe {
            self.init()?;

            let mut msg = MSG::default();
            loop {
                self.render()?;

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

    fn render(&mut self) -> windows::runtime::Result<()> {
        if self.target.is_none() {
            let target = create_render_target(self.factory.as_ref().unwrap(), self.handle)?;
            self.black = Some(create_brush(&target, &Color::RGB(0.0, 0.0, 0.0))?);
            self.brush = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 1.0))?);
            self.target = Some(target);
        }

        if self.text_format.is_none() {
            self.text_format = Some(create_formater(self.text_factory.as_ref().unwrap())?);
        }
        let target = self.target.as_ref().unwrap();
        self.draw(target)?;
        self.text_box.draw();
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
        DefWindowProcW(window, msg, wparam, lparam)
    }

    unsafe fn message_handler(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WindowsAndMessaging::WM_LBUTTONDOWN => {
                SetFocus(self.handle);
                DefWindowProcW(self.handle, msg, wparam, lparam)
            }
            // WindowsAndMessaging::WM_PAINT => {
            //     self.render().unwrap();
            //     LRESULT(0)
            // }
            WindowsAndMessaging::WM_DISPLAYCHANGE => {
                self.render().unwrap();
                LRESULT(0)
            }
            WindowsAndMessaging::WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WindowsAndMessaging::WM_ACTIVATE => {
                println!("PARENT ACTIVED");
                DefWindowProcW(self.handle, msg, wparam, lparam)
            }
            _ => DefWindowProcW(self.handle, msg, wparam, lparam),
        }
    }

    fn draw(&self, target: &ID2D1HwndRenderTarget) -> windows::runtime::Result<()> {
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
