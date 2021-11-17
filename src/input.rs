use std::{
    iter::once,
    sync::atomic::{AtomicBool, Ordering},
};

use windows::Win32::{Foundation::{BOOL, GetLastError, HINSTANCE, HWND, LPARAM, PWSTR, RECT, WPARAM}, Graphics::{Direct2D::{
            Common::{D2D_POINT_2F, D2D_RECT_F, D2D_SIZE_U},
            ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
        }, DirectWrite::{DWRITE_HIT_TEST_METRICS, IDWriteFactory, IDWriteTextFormat, IDWriteTextLayout}, Gdi::{RedrawWindow, RDW_INVALIDATE}}, UI::{Input::KeyboardAndMouse::{self, SetFocus}, WindowsAndMessaging::{
            self, CreateWindowExW, GetClientRect, LoadCursorW, RegisterClassW, SetCaretPos,
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_IBEAM, WNDCLASSW, WS_BORDER, WS_CHILDWINDOW,
            WS_VISIBLE,
        }}};

use crate::{
    caret::Caret,
    interface::*,
    support::{self, create_brush, create_factory, create_render_target, text, Color, Fill},
};

pub struct TextBox {
    factory: Option<ID2D1Factory1>,
    target: Option<ID2D1HwndRenderTarget>,
    text_factory: Option<IDWriteFactory>,
    text_format: Option<IDWriteTextFormat>,
    text_layout: Option<IDWriteTextLayout>,
    green: Option<ID2D1SolidColorBrush>,
    black: Option<ID2D1SolidColorBrush>,
    handle: HWND,
    data: String,
    caret_pos: usize,
    pos: (i32, i32),
    width: Fill,
    height: Fill,
    caret: Option<Caret>,
}

impl TextBox {
    pub fn new(pos: (i32, i32), width: Fill, height: Fill) -> Box<Self> {
        Box::new(Self {
            factory: None,
            target: None,
            green: None,
            black: None,
            text_factory: None,
            text_format: None,
            text_layout: None,
            handle: HWND::default(),
            data: String::from("TEXTBOX"),
            caret_pos: 0,
            pos,
            width,
            height,
            caret: None,
        })
    }
    fn update_caret(&mut self) {
        unsafe { 
            if self.caret_pos == 0 {
                SetCaretPos(0, 0);
            } else {
                let size = self.target.as_ref().unwrap().GetSize();
                let layout = text::create_layout(self.text_factory.as_ref().unwrap(), self.text_format.as_ref().unwrap(),(size.width,size.height), &self.data[..self.caret_pos]).unwrap();
                let width = layout.GetMetrics().unwrap().width;
                SetCaretPos(width as i32, 0);
            }
        }
    }
}

impl IWindow for TextBox {
    fn on_click(&mut self, pos: (i32, i32)) -> anyhow::Result<()> {
        unsafe { SetFocus(self.handle) };
        let pos = self
            .text_layout
            .as_ref()
            .and_then(|layout| unsafe { 
                let mut trailing = BOOL::default();
                let mut inside = BOOL::default();
                let mut hittestmetrics = DWRITE_HIT_TEST_METRICS::default();
                layout.HitTestPoint(pos.0 as f32, pos.1 as f32, &mut trailing, &mut inside, &mut hittestmetrics).unwrap();

                Some(if inside.as_bool() || trailing.as_bool() { hittestmetrics.textPosition + 1} else { hittestmetrics.textPosition } as usize)
             })
            .unwrap();
        self.caret_pos = pos;
        self.update_caret();
        Ok(())
    }
    fn handle_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> anyhow::Result<()> {
        unsafe {
            match msg {
                WindowsAndMessaging::WM_SETFOCUS => {
                    let height = self
                        .text_layout
                        .as_ref()
                        .and_then(|layout| layout.GetMetrics().ok())
                        .and_then(|metrics| Some(metrics.height))
                        .unwrap_or(20.0) as _;
                    let mut caret = Caret::new(self.handle, 2, height);
                    caret.show();
                    self.caret = Some(caret);
                }

                WindowsAndMessaging::WM_KILLFOCUS => {
                    self.caret = None;
                }
                WindowsAndMessaging::WM_CHAR => {
                    if wparam.0 as u16 == KeyboardAndMouse::VK_BACK.0 && self.data.len() != 0 {
                        self.data.remove(self.caret_pos-1);
                        self.caret_pos-=1;
                        self.update_caret();
                    } else if wparam.0 as u16 == KeyboardAndMouse::VK_RETURN.0
                        || wparam.0 as u16 == KeyboardAndMouse::VK_ESCAPE.0
                    {
                        KeyboardAndMouse::SetFocus(HWND::default());
                    } else {
                        let meta = support::Keymeta::from_bytes(u32::to_ne_bytes(lparam.0 as u32));
                        let repeat = meta.count();
                        if let Some(c) = char::from_u32(wparam.0 as u32) {
                            for _ in 0..repeat {
                                self.data.insert(self.caret_pos,c);
                                self.caret_pos+=1;
                            }
                        }
                        self.update_caret();
                    }
                    self.text_layout = None;
                    RedrawWindow(self.handle, std::ptr::null_mut(), None, RDW_INVALIDATE);
                } 
                WindowsAndMessaging::WM_KEYDOWN => {
                    if self.caret.is_some() {

                        const LEFT : u16 = KeyboardAndMouse::VK_LEFT.0;
                        const RIGHT : u16 = KeyboardAndMouse::VK_RIGHT.0;
                        const DELETE : u16 = KeyboardAndMouse::VK_DELETE.0;
                        const HOME : u16 = KeyboardAndMouse::VK_HOME.0;
                        const END : u16 = KeyboardAndMouse::VK_END.0;
                        match wparam.0 as u16 {
                            LEFT => {
                                self.caret_pos = self.caret_pos.saturating_sub(1);
                                self.update_caret();
                            }
                            RIGHT => {
                                self.caret_pos = (self.caret_pos + 1).min(self.data.len());
                                self.update_caret();
                            }
                            DELETE if self.caret_pos < self.data.len() => {
                                self.data.remove(self.caret_pos);
                                self.text_layout = None;
                                RedrawWindow(self.handle, std::ptr::null_mut(), None, RDW_INVALIDATE);
                            }
                            HOME => {
                                self.caret_pos = 0;
                                self.update_caret();
                            }
                            END => {
                                self.caret_pos = self.data.len();
                                self.update_caret();
                            }
                            _=>()
                        }
                    }
                }
                _ => (),
            }

            Ok(())
        }
    }

    fn init(&self, parent: Option<HWND>, instance: Option<HINSTANCE>) -> anyhow::Result<()> {
        let target = parent.expect("CAN ONLY BE A CHILD WINDOW");
        let instance = instance.expect("CAN ONLY BE A CHILD WINDOW");
        if self.factory.is_some() {
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
            lazy_static::lazy_static! {
                static ref REGISTERED : AtomicBool = AtomicBool::new(false);
                static ref LOCK : std::sync::Mutex<()> = std::sync::Mutex::new(());
            }
            {
                let _key = LOCK.lock().unwrap();
                if !REGISTERED.load(Ordering::Acquire) {
                    let wc = WNDCLASSW {
                        style: CS_VREDRAW | CS_HREDRAW,
                        hCursor: LoadCursorW(None, IDC_IBEAM),
                        lpfnWndProc: Some(Self::wnd_proc),
                        hInstance: instance,
                        lpszClassName: wc_name,
                        ..Default::default()
                    };
                    let atom = RegisterClassW(&wc);
                    debug_assert!(atom != 0, "FAILED TO REGISTER CLASS");
                    REGISTERED.store(true, Ordering::Release);
                }
            }
            let mut rect = RECT::default();
            if !GetClientRect(parent, &mut rect).as_bool() {
                anyhow::bail!("Couldn't get parent rect");
            }
            let handle = CreateWindowExW(
                Default::default(),
                wc_name,
                None,
                WS_CHILDWINDOW | WS_VISIBLE | WS_BORDER,
                self.pos.0,
                self.pos.1,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                target,
                None,
                instance,
                self as *const Self as _,
            );

            debug_assert!(handle.0 != 0);
            debug_assert!(handle == self.handle);
        }

        Ok(())
    }

    fn set_handle(&mut self, handle: HWND) {
        self.handle = handle;
    }

    fn on_create(&mut self) -> anyhow::Result<()> {
        self.factory = Some(create_factory()?);
        self.text_factory = Some(text::create_factory()?);
        let target = create_render_target(self.factory.as_ref().unwrap(), self.handle).unwrap();
        self.green = Some(create_brush(&target, &Color::RGB(0.0, 1.0, 0.0)).unwrap());
        self.black = Some(create_brush(&target, &Color::RGB(0.0, 0.0, 0.0)).unwrap());
        self.target = Some(target);
        self.text_format =
            Some(text::create_formater(self.text_factory.as_ref().unwrap()).unwrap());
        Ok(())
    }

    fn draw(&mut self) -> anyhow::Result<()> {
        if self.target.is_none() {
            return Ok(());
        }
        let target = self.target.as_ref().unwrap();
        let green = self.green.as_ref().unwrap();
        if self.text_layout.is_none() {
            let size = unsafe { target.GetSize() };
            self.text_layout = Some(
                text::create_layout(
                    self.text_factory.as_ref().unwrap(),
                    self.text_format.as_ref().unwrap(),
                    (size.width, size.height),
                    &self.data,
                )
                .expect("Faild to create layout"),
            )
        }
        unsafe {
            let _caret_key = self.caret.as_mut().map(|c| c.pause());
            let size = target.GetSize();
            let rect = D2D_RECT_F {
                top: 0.0,
                left: 0.0,
                right: size.width,
                bottom: size.height,
            };

            target.BeginDraw();
            target.FillRectangle(&rect, green);
            target.DrawTextLayout(
                &D2D_POINT_2F { x: 0.0, y: 0.0 },
                self.text_layout.as_ref().unwrap(),
                self.black.as_ref().unwrap(),
                Default::default(),
            );
            target.EndDraw(std::ptr::null_mut(), std::ptr::null_mut())?;
        }
        Ok(())
    }

    fn resize(&mut self, width: u32, height: u32) -> anyhow::Result<()> {
        if let Some(target) = self.target.as_ref() {
            let size = D2D_SIZE_U { width, height };
            unsafe { target.Resize(&size)? };
        }
        Ok(())
    }

    fn get_handle(&self) -> HWND {
        self.handle
    }

    fn get_fill(&self) -> (Fill, Fill) {
        (self.width, self.height)
    }
}
