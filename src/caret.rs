use windows::Win32::{
    Foundation::{GetLastError, HWND},
    UI::WindowsAndMessaging::{CreateCaret, DestroyCaret, HideCaret, SetCaretBlinkTime, ShowCaret},
};

pub struct Caret {
    is_showing: bool,
    handle: HWND,
}

pub struct CaretPauseGaurd<'a> {
    owner: &'a mut Caret,
}

impl<'a> CaretPauseGaurd<'a> {
    fn new(owner: &'a mut Caret) -> Self {
        owner.hide();
        Self { owner }
    }
}

impl Drop for CaretPauseGaurd<'_> {
    fn drop(&mut self) {
        self.owner.show();
    }
}

impl Caret {
    pub fn new(handle: HWND, width: i32, height: i32) -> Self {
        if !unsafe { CreateCaret(handle, None, width, height) }.as_bool() {
            let error = unsafe { GetLastError() };
            if error.0 != 5 && error.0 != 0 {
                println!("{:?}", error);
            }
        }
        // if !unsafe { SetCaretBlinkTime(30).as_bool() } {
        //     let error = unsafe { GetLastError() };
        //     if error.0 != 5 && error.0 != 0 {
        //         println!("{:?}", error);
        //     }
        // }
        Self {
            is_showing: false,
            handle,
        }
    }

    pub fn pause<'a>(&'a mut self) -> CaretPauseGaurd<'a> {
        CaretPauseGaurd::new(self)
    }

    pub fn show(&mut self) {
        if self.is_showing {
            return;
        }
        if !unsafe { ShowCaret(self.handle) }.as_bool() {
            let error = unsafe { GetLastError() };
            if error.0 != 5 && error.0 != 0 {
                println!("{:?}", error);
            }
        }
        self.is_showing = true;
    }

    pub fn hide(&mut self) {
        if !self.is_showing {
            return;
        }
        if !unsafe { HideCaret(self.handle) }.as_bool() {
            let error = unsafe { GetLastError() };
            if error.0 != 5 && error.0 != 0 {
                println!("{:?}", error);
            }
        }
        self.is_showing = false;
    }
}

impl Drop for Caret {
    fn drop(&mut self) {
        unsafe { DestroyCaret() };
    }
}
