use bindings::Windows::Win32::{
    Foundation::{HWND, LPARAM, LRESULT, PSTR, WPARAM},
    Graphics::Direct2D::{
        ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush, ID2D1StrokeStyle, D2D_RECT_F,
    },
    System::LibraryLoader::GetModuleHandleA,
    UI::WindowsAndMessaging::{
        self, CreateWindowExA, DefWindowProcA, DispatchMessageA, LoadCursorW, PeekMessageA,
        PostQuitMessage, RegisterClassA, CREATESTRUCTA, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT,
        GWLP_USERDATA, IDC_HAND, MSG, PM_REMOVE, WNDCLASSA, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

use crate::support::*;

pub struct MainWindow {
    handle: HWND,
    factory: ID2D1Factory1,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    stroke: Option<ID2D1StrokeStyle>,
}

impl MainWindow {
    pub fn new() -> anyhow::Result<Self> {
        let factory = create_factory()?;

        Ok(Self {
            handle: HWND(0),
            factory,
            target: None,
            brush: None,
            stroke: None,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        unsafe {
            let instance = GetModuleHandleA(None);
            debug_assert!(!instance.is_null());
            let wc = WNDCLASSA {
                hCursor: LoadCursorW(None, IDC_HAND),
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
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                None,
                None,
                instance,
                self as *mut _ as _,
            );

            debug_assert!(!handle.is_null());
            debug_assert!(handle == self.handle);
            let mut msg = MSG::default();
            loop {
                match self.render() {
                    Ok(()) => (),
                    Err(err) => {
                        eprintln!("Error {:?}", err);
                        return Err(err);
                    }
                }

                while PeekMessageA(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    if msg.message == WindowsAndMessaging::WM_QUIT {
                        return Ok(());
                    }
                    DispatchMessageA(&msg);
                }
            }
        }
    }

    fn render(&mut self) -> anyhow::Result<()> {
        if self.target.is_none() {
            let target = match create_render_target(&self.factory, self.handle) {
                Ok(rt) => rt,
                Err(err) => {
                    eprintln!("Error creating RenderTarget {:?}", err);
                    return Err(err.into());
                }
            };

            self.brush = Some(create_brush(&target)?);
            self.stroke = Some(create_stroke(&self.factory)?);
            self.target = Some(target);
        }

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
                // self.render().unwrap();ca
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
        unsafe {
            target.BeginDraw();
            // target.Clear(&Color::RGB(1.0, 0.0, 0.0).into());
            let size = target.GetSize();

            let rect = D2D_RECT_F {
                left: 10.0,
                top: 10.0,
                right: size.width - 10.0,
                bottom: size.height - 10.0,
            };
            target.DrawRectangle(&rect, self.brush.as_ref().unwrap(), 1000.0, None);
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;

        }
        Ok(())
    }
}
