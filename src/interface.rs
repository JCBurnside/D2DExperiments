use windows::Win32::{Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, PWSTR, WPARAM}, UI::WindowsAndMessaging::{self, DefWindowProcW, CREATESTRUCTW, GWLP_USERDATA}};

use crate::support::{GetWindowLong, SetWindowLong};


trait IWindow {

    /// handle any messages from the win32 api.
    fn handle_message(&mut self, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT;

    /// This will be called before any other operation can occur.  It should call init on any children it may have.
    /// target and instance will be [None] if root.  should init all childern.
    fn init(self: Box<Self>, target:Option<HWND>, instance:Option<HINSTANCE>) -> windows::runtime::Result<()>;
    
    // set the HWND handle of the window here.
    fn set_handle(&mut self, handle: HWND);

    /// This is called as the final step of `windows::Windows::Win32::WindowsAndMessaging::WM_NCCREATE`
    /// you should use this to create the initial factories and other resources you need post creation.
    /// # Example
    ///
    /// ```
    /// use doc::IWindow;
    /// use doc::windows::Windows::Win32::Graphics::Direct2D as D2D;
    /// use doc::windows::runtime::Result;
    /// struct Foo {
    ///     //--snip--
    ///     d2d_factory : Option<D2D::IFactory>
    /// }
    /// impl IWindow for Foo {
    ///     fn on_create(&mut self) -> Result<()> {
    ///         self.d2d_factory = Some(create_factory()?);
    ///     }
    /// 
    ///     //--snip--
    /// }
    /// ```
    fn on_create(&mut self) -> windows::runtime::Result<()> {
        Ok(())
    }

    /// do your drawing here. should call [draw](IWindow::draw) on any childern
    fn draw(&mut self) -> windows::runtime::Result<()>;

    fn get_wc_name(&self)  -> PWSTR;
    /// In general you should not override this function
    /// you should override [on_create](IWindow::on_create) instead for things needed to be done when WM_NCCREATE is triggered.
    /// for all other events you should use [handle_message](IWindow::handle_message).
    unsafe extern "system" fn wnd_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT
    where
        Self: Sized,
    {
        if msg == WindowsAndMessaging::WM_NCCREATE {
            let cs = lparam.0 as *const CREATESTRUCTW;
            let this = (*cs).lpCreateParams as *mut Self;
            SetWindowLong(hwnd, GWLP_USERDATA, this as _);
            let this = &mut (*this);
            this.set_handle(hwnd);
            this.on_create().unwrap();
        } else {
            let this = GetWindowLong(hwnd, GWLP_USERDATA) as *mut Self;
            if !this.is_null() {
                return (*this).handle_message(msg, wparam, lparam);
            }
        }
        DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

