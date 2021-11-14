use std::{iter::once, sync::atomic::{AtomicBool, Ordering}};

use crate::{interface::IWindow, support::*};
use windows::Win32::{Foundation::{HWND, PWSTR, RECT}, Graphics::{
        Direct2D::{
            Common::{D2D_RECT_F, D2D_SIZE_U},
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
            D2D1_DRAW_TEXT_OPTIONS_NONE,
        },
        DirectWrite::{IDWriteFactory, IDWriteTextFormat, DWRITE_MEASURING_MODE_NATURAL},
    }, UI::WindowsAndMessaging::{CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, GetClientRect, IDC_ARROW, LoadCursorW, RegisterClassW, WNDCLASSW, WS_CHILDWINDOW, WS_VISIBLE}};
pub struct Test {
    handle: HWND,
    factory: Option<ID2D1Factory1>,
    text_factory: Option<IDWriteFactory>,
    target: Option<ID2D1HwndRenderTarget>,
    brush: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    text_format: Option<IDWriteTextFormat>,
    fill : (Fill,Fill),
}

impl Test {
    pub fn new(fill:(Fill,Fill)) -> Box<Self> {
        Box::new(Self {
            handle: HWND::default(),
            factory: None,
            text_factory: None,
            target: None,
            brush: None,
            black: None,
            text_format: None,
            fill,
        })
    }
}

impl IWindow for Test {
    fn handle_message(
        &mut self,
        msg: u32,
        wparam: windows::Win32::Foundation::WPARAM,
        lparam: windows::Win32::Foundation::LPARAM,
    ) -> windows::Win32::Foundation::LRESULT {
        unsafe { DefWindowProcW(self.handle, msg, wparam, lparam) }
    }

    fn init(
        &self,
        parent: Option<windows::Win32::Foundation::HWND>,
        instance: Option<windows::Win32::Foundation::HINSTANCE>,
    ) -> anyhow::Result<()> {
        let parent = parent.expect("SHOULD BE A CHILD");
        let instance = instance.expect("SHOULD BE A CHILD");
        unsafe {
            let mut wc_name = self
                .get_wc_name()
                .encode_utf16()
                .chain(once(0))
                .collect::<Vec<_>>();
            let mut rect = RECT::default();
            if !GetClientRect(parent, &mut rect).as_bool() {
                anyhow::bail!("Failed to get rect of parent");
            }
            debug_assert!(wc_name.len() < 256);
            let wc_name = PWSTR(wc_name.as_mut_ptr());
            lazy_static::lazy_static! {
                static ref REGISTERED : AtomicBool = AtomicBool::new(false);
            }
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
            let handle = CreateWindowExW(
                Default::default(),
                wc_name,
                None,
                WS_CHILDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                rect.right,
                rect.bottom,
                parent,
                None,
                instance,
                self as *const Self as _,
            );
            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
        }

        Ok(())
    }

    fn set_handle(&mut self, handle: windows::Win32::Foundation::HWND) {
        self.handle = handle;
    }

    fn on_create(&mut self) -> anyhow::Result<()> {
        let factory = create_factory()?;
        let text_factory = create_text_factory()?;

        self.factory = Some(factory);
        self.text_factory = Some(text_factory);

        let target = create_render_target(self.factory.as_ref().unwrap(), self.handle)?;
        self.black = Some(create_brush(&target, &Color::BLACK)?);
        self.brush = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 1.0))?);
        self.text_format = Some(create_formater(self.text_factory.as_ref().unwrap())?);
        self.target = Some(target);
        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        if self.target.is_none() {
            return Ok(());
        }
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
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if let Some(target) = self.target.as_ref() {
            dbg!(unsafe { target.GetSize() });
            let size = D2D_SIZE_U {
                width: width,
                height: height,
            };
            unsafe { target.Resize(&dbg!(size)) }?;
        }
        self.draw()
    }

    fn get_handle(&self) -> HWND {
        self.handle
    }

    fn get_fill(&self) -> (Fill, Fill) {
        self.fill
    }
}
