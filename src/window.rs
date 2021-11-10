use windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM},
    Graphics::{
        Direct2D::{
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
            D2D1_DRAW_TEXT_OPTIONS_NONE, D2D_RECT_F,
        },
        DirectWrite::{IDWriteFactory, IDWriteTextFormat, DWRITE_MEASURING_MODE_NATURAL},
    },
    System::LibraryLoader::GetModuleHandleW,
    UI::WindowsAndMessaging::{
        self, CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, PeekMessageW,
        PostQuitMessage, RegisterClassW, TranslateMessage, CS_HREDRAW, CS_VREDRAW,
        CW_USEDEFAULT, IDC_ARROW, MSG, PM_REMOVE, WNDCLASSW,
        WS_CLIPCHILDREN, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
    },
};

use crate::{input::TextBox, interface::*, support::*};

pub struct MainWindow {
    handle: HWND,
    factory: Option<ID2D1Factory1>,
    text_factory: Option<IDWriteFactory>,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    text_format: Option<IDWriteTextFormat>,
    text_box: Box<dyn IWindow>,
}

impl MainWindow {
    pub fn new() -> Self {
        Self {
            handle: HWND::default(),
            factory: None,
            text_factory: None,
            target: None,
            brush: None,
            black: None,
            text_format: None,
            text_box: Box::new(TextBox::new((0, 0), 500, 100)),
        }
    }

    pub fn run(&mut self) -> windows::runtime::Result<()> {
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
    fn on_create(&mut self) -> windows::runtime::Result<()> {
        //create factories
        let factory = create_factory()?;
        let text_factory = create_text_factory()?;
        self.factory = Some(factory);
        self.text_factory = Some(text_factory);

        //create local resources
        let target = create_render_target(self.factory.as_ref().unwrap(), self.handle)?;
        self.black = Some(create_brush(&target, &Color::RGB(0.0, 0.0, 0.0))?);
        self.brush = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 1.0))?);
        self.target = Some(target);
        self.text_format = Some(create_formater(self.text_factory.as_ref().unwrap())?);

        Ok(())
    }
    fn init(
        &self,
        target: Option<HWND>,
        instance: Option<HINSTANCE>,
    ) -> windows::runtime::Result<()> {
        if target.is_some() || instance.is_some() {
            panic!("THIS SHOULD NOT BE REACHED. TRIED TO CREATE A WINDOW AS A CHILD");
        }

        if self.factory.is_some() {
            return Ok(());
        }
        unsafe {
            let mut WC_NAME = self.get_wc_name().encode_utf16().collect::<Vec<u16>>();
            let WC_NAME = PWSTR(WC_NAME.as_mut_ptr());
            let instance = GetModuleHandleW(None);
            debug_assert!(instance.0 != 0);
            let wc = WNDCLASSW {
                hCursor: LoadCursorW(None, IDC_ARROW),
                hInstance: instance,
                lpszClassName: WC_NAME,

                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(Self::wnd_proc),

                ..Default::default()
            };

            let atom = RegisterClassW(&wc);
            debug_assert!(atom != 0);

            let handle = CreateWindowExW(
                Default::default(),
                WC_NAME,
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

            self.text_box.init(Some(self.handle), Some(instance))?;
        }
        Ok(())
    }

    fn set_handle(&mut self, handle: HWND) {
        self.handle = handle;
    }

    fn draw(&mut self) -> windows::runtime::Result<()> {
        let text = "FooBar";
        let len: usize = text.len();
        let target = self.target.as_ref().unwrap();
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
        self.text_box.draw()?;
        Ok(())
    }
}
