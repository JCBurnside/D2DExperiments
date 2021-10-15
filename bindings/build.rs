fn main() {
    windows::build! {
        Windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx},
        Windows::Win32::Graphics::Direct2D::{D2D1CreateFactory,ID2D1Effect, ID2D1DeviceContext, ID2D1Factory1, ID2D1SolidColorBrush,D2D1CreateDevice, ID2D1HwndRenderTarget, },
        Windows::Win32::System::LibraryLoader::GetModuleHandleA,
        Windows::Win32::Graphics::DirectWrite::{DWriteCreateFactory,IDWriteFactory},
        Windows::Win32::UI::WindowsAndMessaging::{
            CreateWindowExA, DefWindowProcA, DispatchMessageA, GetMessageA, RegisterClassExA,
            GetWindowLongPtrA, LoadCursorW, PeekMessageA, PostQuitMessage, RegisterClassA,
            SetWindowLongA, SetWindowLongPtrA, CREATESTRUCTA, CW_USEDEFAULT, IDC_ARROW, IDC_IBEAM, MSG,
            SIZE_MINIMIZED, WINDOW_LONG_PTR_INDEX, WNDCLASSA,WM_ACTIVATE, WM_DESTROY,
            WM_DISPLAYCHANGE, WM_NCCREATE, WM_PAINT, WM_QUIT, WM_SIZE, WM_USER, WM_KEYDOWN,
            GetClientRect, WNDCLASSEXA,
        },
        Windows::Win32::UI::Shell::GetCurrentProcessExplicitAppUserModelID,
    }
}