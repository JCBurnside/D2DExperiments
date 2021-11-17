pub mod text;
use windows::{
    runtime::Interface,
    Win32::{
        Foundation::HWND,
        Graphics::Direct2D::{
            Common::*, D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget,
            ID2D1SolidColorBrush, D2D1_BRUSH_PROPERTIES, D2D1_DEBUG_LEVEL_INFORMATION,
            D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_SINGLE_THREADED,
            D2D1_HWND_RENDER_TARGET_PROPERTIES,
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
        D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd,
            pixelSize: D2D_SIZE_U {
                width: (rc.right - rc.left) as _,
                height: (rc.bottom - rc.top) as _,
            },
            ..Default::default()
        }
    };
    unsafe { factory.CreateHwndRenderTarget(&Default::default(), &options) }
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
#[allow(clippy::upper_case_acronyms)]
pub enum Color {
    RGB(f32, f32, f32),
    RGBA(f32, f32, f32, f32),
}

impl Color {
    pub const BLACK: Color = Color::RGB(0.0, 0.0, 0.0);
}

impl From<&Color> for D2D1_COLOR_F {
    fn from(color: &Color) -> D2D1_COLOR_F {
        match *color {
            Color::RGB(r, g, b) => D2D1_COLOR_F { r, g, b, a: 1.0 },
            Color::RGBA(r, g, b, a) => D2D1_COLOR_F { r, g, b, a },
        }
    }
}

impl From<Color> for D2D1_COLOR_F {
    fn from(color: Color) -> D2D1_COLOR_F {
        match color {
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
    #[allow(unused)]
    pub struct Keymeta {
        pub count: u16,
        scan_code: u8,
        extended: bool,
        reserved: B4,
        context: bool,
        prev: bool,
        transition: bool,
    }
}

pub use keymeta::*;
