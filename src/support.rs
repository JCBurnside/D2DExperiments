use windows::{
    runtime::Interface,
    Win32::{
        Foundation::{HWND, PWSTR},
        Graphics::{
            Direct2D::{
                Common::*, D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget,
                ID2D1SolidColorBrush, D2D1_BRUSH_PROPERTIES, D2D1_DEBUG_LEVEL_INFORMATION,
                D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_SINGLE_THREADED,
                D2D1_HWND_RENDER_TARGET_PROPERTIES,
            },
            DirectWrite::{
                DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat,
                DWRITE_FACTORY_TYPE_ISOLATED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_WEIGHT_NORMAL,
            },
        },
        UI::WindowsAndMessaging::{GetClientRect, WINDOW_LONG_PTR_INDEX},
    },
};

pub fn create_factory() -> windows::runtime::Result<ID2D1Factory1> {
    let options = {
        let mut out = D2D1_FACTORY_OPTIONS::default();
        if cfg!(debug_assertions) {
            out.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        }
        out
    };
    let mut result: Option<ID2D1Factory1> = None;
    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            &mut result as *mut _ as *mut *mut _,
        )
        .map(move |()| result.unwrap())
    }
}

pub fn create_render_target(
    factory: &ID2D1Factory1,
    hwnd: HWND,
) -> windows::runtime::Result<ID2D1HwndRenderTarget> {
    let options = {
        let mut rc = Default::default();
        unsafe { GetClientRect(hwnd, &mut rc) };
        let mut out = D2D1_HWND_RENDER_TARGET_PROPERTIES::default();
        out.hwnd = hwnd;
        out.pixelSize = D2D_SIZE_U {
            width: (rc.right - rc.left) as _,
            height: (rc.bottom - rc.top) as _,
        };
        out
    };
    unsafe { factory.CreateHwndRenderTarget(&Default::default(), &options) }
}

pub fn create_text_factory() -> windows::runtime::Result<IDWriteFactory> {
    unsafe {
        DWriteCreateFactory(DWRITE_FACTORY_TYPE_ISOLATED, &IDWriteFactory::IID)
            .and_then(|it| it.cast())
    }
}

pub fn create_formater(factory: &IDWriteFactory) -> windows::runtime::Result<IDWriteTextFormat> {
    const FONT: PWSTR = PWSTR(utf16_literal::utf16!("calibri\0").as_ptr() as _);
    let mut family = None;
    let family = unsafe {
        factory
            .GetSystemFontCollection(&mut family, false)
            .map(|()| family.unwrap())?
    };
    unsafe {
        factory.CreateTextFormat(
            FONT,
            family,
            DWRITE_FONT_WEIGHT_NORMAL,
            DWRITE_FONT_STYLE_NORMAL,
            DWRITE_FONT_STRETCH_NORMAL,
            16.0,
            FONT,
        )
    }
}

#[derive(Clone, Copy)]
#[allow(unused)]
pub enum Fill {
    Fill,
    Percent(f32),
    Fixed(u32),
}

pub fn loword(i: isize) -> u16 {
    (i & 0xffff) as u16
}

pub fn hiword(i: isize) -> u16 {
    ((i >> 16) & 0xffff) as u16
}

#[derive(Clone, Copy)]
pub enum Color {
    RGB(f32, f32, f32),
    RGBA(f32, f32, f32, f32),
}

impl Color {
    pub const BLACK: Color = Color::RGB(0.0, 0.0, 0.0);
}

impl Into<D2D1_COLOR_F> for &Color {
    fn into(self) -> D2D1_COLOR_F {
        match self {
            &Color::RGB(r, g, b) => D2D1_COLOR_F { r, g, b, a: 1.0 },
            &Color::RGBA(r, g, b, a) => D2D1_COLOR_F { r, g, b, a },
        }
    }
}

impl Into<D2D1_COLOR_F> for Color {
    fn into(self) -> D2D1_COLOR_F {
        match self {
            Color::RGB(r, g, b) => D2D1_COLOR_F { r, g, b, a: 1.0 },
            Color::RGBA(r, g, b, a) => D2D1_COLOR_F { r, g, b, a },
        }
    }
}

pub fn create_brush(
    target: &ID2D1HwndRenderTarget,
    color: &Color,
) -> windows::runtime::Result<ID2D1SolidColorBrush> {
    let props = D2D1_BRUSH_PROPERTIES {
        opacity: 1.0,
        ..Default::default()
    };
    unsafe { target.CreateSolidColorBrush(&color.into(), &props) }
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    use windows::Win32::UI::WindowsAndMessaging::SetWindowLongW;

    SetWindowLongW(window, index, value as _) as _
}
#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    use windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrW;

    SetWindowLongPtrW(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrA;
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    use windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW;

    GetWindowLongPtrW(window, index)
}

mod keymeta {
    #![allow(unused)]
    use modular_bitfield::prelude::*;
    #[bitfield]
    pub struct Keymeta {
        pub count: u16,
        scan_code: u8,
        extended: bool,
        #[allow(unused)]
        reserved: B4,
        context: bool,
        prev: bool,
        transition: bool,
    }
}

pub use keymeta::*;
