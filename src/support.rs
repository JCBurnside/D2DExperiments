use bindings::Windows::Win32::{
    Foundation::HWND,
    Graphics::Direct2D::{
        D2D1CreateFactory, ID2D1Factory1, ID2D1HwndRenderTarget, ID2D1SolidColorBrush,
        ID2D1StrokeStyle, D2D1_CAP_STYLE_FLAT, D2D1_COLOR_F, D2D1_DASH_STYLE_SOLID,
        D2D1_DEBUG_LEVEL_INFORMATION, D2D1_FACTORY_OPTIONS, D2D1_FACTORY_TYPE_SINGLE_THREADED,
        D2D1_HWND_RENDER_TARGET_PROPERTIES, D2D1_LINE_JOIN_MITER, D2D1_PRESENT_OPTIONS,
        D2D1_STROKE_STYLE_PROPERTIES, D2D1_STROKE_STYLE_PROPERTIES1, D2D_SIZE_U,
    },
    UI::WindowsAndMessaging::{GetClientRect, WINDOW_LONG_PTR_INDEX},
};
use windows::{Abi, Interface};

pub fn create_brush(target: &ID2D1HwndRenderTarget) -> windows::Result<ID2D1SolidColorBrush> {
    unsafe { target.CreateSolidColorBrush(&Color::RGB(0.0, 1.0, 0.0).into(), &Default::default()) }
}

pub fn create_stroke(factory: &ID2D1Factory1) -> windows::Result<ID2D1StrokeStyle> {
    let props = {
        let mut out = D2D1_STROKE_STYLE_PROPERTIES::default();
        out.startCap = D2D1_CAP_STYLE_FLAT;
        out.endCap = D2D1_CAP_STYLE_FLAT;
        out.dashCap = D2D1_CAP_STYLE_FLAT;
        out.dashStyle = D2D1_DASH_STYLE_SOLID;
        out.miterLimit = 1.0;
        out.dashOffset = 0.0;
        out.lineJoin = D2D1_LINE_JOIN_MITER;
        out
    };
    unsafe { factory.CreateStrokeStyle(&props, std::ptr::null(), 0) }
}

#[derive(Clone, Copy)]
pub enum Color {
    RGB(f32, f32, f32),
    RGBA(f32, f32, f32, f32),
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

pub fn create_render_target(
    factory: &ID2D1Factory1,
    hwnd: HWND,
) -> windows::Result<ID2D1HwndRenderTarget> {
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

pub fn create_factory() -> windows::Result<ID2D1Factory1> {
    let options = {
        let mut out = D2D1_FACTORY_OPTIONS::default();
        if cfg!(debug_assertions) {
            out.debugLevel = D2D1_DEBUG_LEVEL_INFORMATION;
        }
        out
    };
    let mut result = None;
    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &ID2D1Factory1::IID,
            &options,
            result.set_abi(),
        )
        .map(|()| result.unwrap())
    }
}
#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    use bindings::Windows::Win32::UI::WindowsAndMessaging::SetWindowLongA;

    SetWindowLongA(window, index, value as _) as _
}
#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn SetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX, value: isize) -> isize {
    use bindings::Windows::Win32::UI::WindowsAndMessaging::SetWindowLongPtrA;

    SetWindowLongPtrA(window, index, value)
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "32")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    GetWindowLongA(window, index) as _
}

#[allow(non_snake_case)]
#[cfg(target_pointer_width = "64")]
pub unsafe fn GetWindowLong(window: HWND, index: WINDOW_LONG_PTR_INDEX) -> isize {
    use bindings::Windows::Win32::UI::WindowsAndMessaging::GetWindowLongPtrA;

    GetWindowLongPtrA(window, index)
}
