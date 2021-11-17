use windows::runtime::Interface;
use windows::Win32::{
    Foundation::PWSTR,
    Graphics::DirectWrite::{
        DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, IDWriteTextLayout,
        DWRITE_FACTORY_TYPE_ISOLATED, DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL,
        DWRITE_FONT_WEIGHT_NORMAL,
    },
};

pub fn create_factory() -> windows::runtime::Result<IDWriteFactory> {
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

pub fn create_layout(
    factory: &IDWriteFactory,
    format: &IDWriteTextFormat,
    size: (f32, f32),
    text: &str,
) -> windows::runtime::Result<IDWriteTextLayout> {
    unsafe { factory.CreateTextLayout(text, text.len() as u32, format, size.0, size.1) }
}
