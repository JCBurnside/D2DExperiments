#[allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Windows {
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Foundation {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Numerics {
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct Matrix3x2 {
                pub M11: f32,
                pub M12: f32,
                pub M21: f32,
                pub M22: f32,
                pub M31: f32,
                pub M32: f32,
            }
            impl Matrix3x2 {}
            impl ::std::default::Default for Matrix3x2 {
                fn default() -> Self {
                    Self {
                        M11: 0.0,
                        M12: 0.0,
                        M21: 0.0,
                        M22: 0.0,
                        M31: 0.0,
                        M32: 0.0,
                    }
                }
            }
            impl ::std::fmt::Debug for Matrix3x2 {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("Matrix3x2")
                        .field("M11", &self.M11)
                        .field("M12", &self.M12)
                        .field("M21", &self.M21)
                        .field("M22", &self.M22)
                        .field("M31", &self.M31)
                        .field("M32", &self.M32)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for Matrix3x2 {
                fn eq(&self, other: &Self) -> bool {
                    self.M11 == other.M11
                        && self.M12 == other.M12
                        && self.M21 == other.M21
                        && self.M22 == other.M22
                        && self.M31 == other.M31
                        && self.M32 == other.M32
                }
            }
            impl ::std::cmp::Eq for Matrix3x2 {}
            unsafe impl ::windows::Abi for Matrix3x2 {
                type Abi = Self;
                type DefaultType = Self;
            }
            unsafe impl ::windows::RuntimeType for Matrix3x2 {
                const SIGNATURE: ::windows::ConstBuffer = ::windows::ConstBuffer::from_slice(
                    b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)",
                );
            }
            impl Matrix3x2 {
                pub fn identity() -> Self {
                    Self {
                        M11: 1.0,
                        M12: 0.0,
                        M21: 0.0,
                        M22: 1.0,
                        M31: 0.0,
                        M32: 0.0,
                    }
                }
                pub fn translation(x: f32, y: f32) -> Self {
                    Self {
                        M11: 1.0,
                        M12: 0.0,
                        M21: 0.0,
                        M22: 1.0,
                        M31: x,
                        M32: y,
                    }
                }
                pub fn rotation(angle: f32, x: f32, y: f32) -> Self {
                    let mut matrix = Self::default();
                    unsafe {
                        super::super::Win32::Graphics::Direct2D::D2D1MakeRotateMatrix(
                            angle,
                            super::super::Win32::Graphics::Direct2D::D2D_POINT_2F { x, y },
                            &mut matrix,
                        );
                    }
                    matrix
                }
                fn impl_add(&self, rhs: &Self) -> Self {
                    Self {
                        M11: self.M11 + rhs.M11,
                        M12: self.M12 + rhs.M12,
                        M21: self.M21 + rhs.M21,
                        M22: self.M22 + rhs.M22,
                        M31: self.M31 + rhs.M31,
                        M32: self.M32 + rhs.M32,
                    }
                }
                fn impl_sub(&self, rhs: &Self) -> Self {
                    Self {
                        M11: self.M11 - rhs.M11,
                        M12: self.M12 - rhs.M12,
                        M21: self.M21 - rhs.M21,
                        M22: self.M22 - rhs.M22,
                        M31: self.M31 - rhs.M31,
                        M32: self.M32 - rhs.M32,
                    }
                }
                fn impl_mul(&self, rhs: &Self) -> Self {
                    Self {
                        M11: self.M11 * rhs.M11 + self.M12 * rhs.M21,
                        M12: self.M11 * rhs.M12 + self.M12 * rhs.M22,
                        M21: self.M21 * rhs.M11 + self.M22 * rhs.M21,
                        M22: self.M21 * rhs.M12 + self.M22 * rhs.M22,
                        M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + rhs.M31,
                        M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + rhs.M32,
                    }
                }
                fn impl_mul_f32(&self, rhs: f32) -> Self {
                    Self {
                        M11: self.M11 * rhs,
                        M12: self.M12 * rhs,
                        M21: self.M21 * rhs,
                        M22: self.M22 * rhs,
                        M31: self.M31 * rhs,
                        M32: self.M32 * rhs,
                    }
                }
            }
            impl ::std::ops::Add<Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_add(&rhs)
                }
            }
            impl ::std::ops::Add<&Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_add(rhs)
                }
            }
            impl ::std::ops::Add<Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_add(&rhs)
                }
            }
            impl ::std::ops::Add<&Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_add(rhs)
                }
            }
            impl ::std::ops::Sub<Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_sub(&rhs)
                }
            }
            impl ::std::ops::Sub<&Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_sub(rhs)
                }
            }
            impl ::std::ops::Sub<Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_sub(&rhs)
                }
            }
            impl ::std::ops::Sub<&Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_sub(rhs)
                }
            }
            impl ::std::ops::Mul<Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_mul(&rhs)
                }
            }
            impl ::std::ops::Mul<&Matrix3x2> for Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_mul(rhs)
                }
            }
            impl ::std::ops::Mul<Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                    self.impl_mul(&rhs)
                }
            }
            impl ::std::ops::Mul<&Matrix3x2> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                    self.impl_mul(rhs)
                }
            }
            impl ::std::ops::Mul<f32> for Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: f32) -> Matrix3x2 {
                    self.impl_mul_f32(rhs)
                }
            }
            impl ::std::ops::Mul<f32> for &Matrix3x2 {
                type Output = Matrix3x2;
                fn mul(self, rhs: f32) -> Matrix3x2 {
                    self.impl_mul_f32(rhs)
                }
            }
        }
    }
    #[allow(
        unused_variables,
        non_upper_case_globals,
        non_snake_case,
        unused_unsafe,
        non_camel_case_types,
        dead_code,
        clippy::all
    )]
    pub mod Win32 {
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Foundation {
            #[repr(transparent)]
            #[derive(
                :: std :: default :: Default,
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: PartialEq,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct BOOL(pub i32);
            unsafe impl ::windows::Abi for BOOL {
                type Abi = Self;
                type DefaultType = Self;
            }
            impl BOOL {
                #[inline]
                pub fn as_bool(self) -> bool {
                    !(self.0 == 0)
                }
                #[inline]
                pub fn ok(self) -> ::windows::Result<()> {
                    if self.as_bool() {
                        Ok(())
                    } else {
                        Err(::windows::HRESULT::from_thread().into())
                    }
                }
                #[inline]
                #[track_caller]
                pub fn unwrap(self) {
                    self.ok().unwrap();
                }
                #[inline]
                #[track_caller]
                pub fn expect(self, msg: &str) {
                    self.ok().expect(msg);
                }
            }
            impl ::std::convert::From<BOOL> for bool {
                fn from(value: BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<&BOOL> for bool {
                fn from(value: &BOOL) -> Self {
                    value.as_bool()
                }
            }
            impl ::std::convert::From<bool> for BOOL {
                fn from(value: bool) -> Self {
                    if value {
                        BOOL(1)
                    } else {
                        BOOL(0)
                    }
                }
            }
            impl ::std::convert::From<&bool> for BOOL {
                fn from(value: &bool) -> Self {
                    (*value).into()
                }
            }
            impl ::std::cmp::PartialEq<bool> for BOOL {
                fn eq(&self, other: &bool) -> bool {
                    self.as_bool() == *other
                }
            }
            impl ::std::cmp::PartialEq<BOOL> for bool {
                fn eq(&self, other: &BOOL) -> bool {
                    *self == other.as_bool()
                }
            }
            impl std::ops::Not for BOOL {
                type Output = Self;
                fn not(self) -> Self::Output {
                    if self.as_bool() {
                        BOOL(0)
                    } else {
                        BOOL(1)
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, BOOL> for bool {
                fn into_param(self) -> ::windows::Param<'a, BOOL> {
                    ::windows::Param::Owned(self.into())
                }
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct HINSTANCE(pub isize);
            impl HINSTANCE {}
            impl ::std::default::Default for HINSTANCE {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HINSTANCE {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HINSTANCE {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HINSTANCE")
                        .field("Value", &self.0)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for HINSTANCE {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HINSTANCE {}
            unsafe impl ::windows::Abi for HINSTANCE {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct HWND(pub isize);
            impl HWND {}
            impl ::std::default::Default for HWND {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl HWND {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for HWND {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("HWND").field("Value", &self.0).finish()
                }
            }
            impl ::std::cmp::PartialEq for HWND {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for HWND {}
            unsafe impl ::windows::Abi for HWND {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct LPARAM(pub isize);
            impl LPARAM {}
            impl ::std::default::Default for LPARAM {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl LPARAM {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for LPARAM {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("LPARAM").field("Value", &self.0).finish()
                }
            }
            impl ::std::cmp::PartialEq for LPARAM {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for LPARAM {}
            unsafe impl ::windows::Abi for LPARAM {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct LRESULT(pub i32);
            impl LRESULT {}
            impl ::std::default::Default for LRESULT {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl LRESULT {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for LRESULT {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("LRESULT").field("Value", &self.0).finish()
                }
            }
            impl ::std::cmp::PartialEq for LRESULT {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for LRESULT {}
            unsafe impl ::windows::Abi for LRESULT {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct POINT {
                pub x: i32,
                pub y: i32,
            }
            impl POINT {}
            impl ::std::default::Default for POINT {
                fn default() -> Self {
                    Self { x: 0, y: 0 }
                }
            }
            impl ::std::fmt::Debug for POINT {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("POINT")
                        .field("x", &self.x)
                        .field("y", &self.y)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for POINT {
                fn eq(&self, other: &Self) -> bool {
                    self.x == other.x && self.y == other.y
                }
            }
            impl ::std::cmp::Eq for POINT {}
            unsafe impl ::windows::Abi for POINT {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PSTR(pub *mut u8);
            impl PSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PSTR {
                type Abi = Self;
                type DefaultType = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for &'a str {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PSTR> {
                    ::windows::Param::Boxed(PSTR(::std::boxed::Box::<[u8]>::into_raw(
                        self.bytes()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u8>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[repr(transparent)]
            #[derive(
                :: std :: clone :: Clone,
                :: std :: marker :: Copy,
                :: std :: cmp :: Eq,
                :: std :: fmt :: Debug,
            )]
            pub struct PWSTR(pub *mut u16);
            impl PWSTR {
                pub const NULL: Self = Self(::std::ptr::null_mut());
                pub fn is_null(&self) -> bool {
                    self.0.is_null()
                }
            }
            impl ::std::default::Default for PWSTR {
                fn default() -> Self {
                    Self(::std::ptr::null_mut())
                }
            }
            impl ::std::cmp::PartialEq for PWSTR {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            unsafe impl ::windows::Abi for PWSTR {
                type Abi = Self;
                type DefaultType = Self;
                fn drop_param(param: &mut ::windows::Param<'_, Self>) {
                    if let ::windows::Param::Boxed(value) = param {
                        if !value.0.is_null() {
                            unsafe {
                                ::std::boxed::Box::from_raw(value.0);
                            }
                        }
                    }
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for &'a str {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            impl<'a> ::windows::IntoParam<'a, PWSTR> for String {
                fn into_param(self) -> ::windows::Param<'a, PWSTR> {
                    ::windows::Param::Boxed(PWSTR(::std::boxed::Box::<[u16]>::into_raw(
                        self.encode_utf16()
                            .chain(::std::iter::once(0))
                            .collect::<std::vec::Vec<u16>>()
                            .into_boxed_slice(),
                    ) as _))
                }
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(C)]
            pub struct RECT {
                pub left: i32,
                pub top: i32,
                pub right: i32,
                pub bottom: i32,
            }
            impl RECT {}
            impl ::std::default::Default for RECT {
                fn default() -> Self {
                    Self {
                        left: 0,
                        top: 0,
                        right: 0,
                        bottom: 0,
                    }
                }
            }
            impl ::std::fmt::Debug for RECT {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("RECT")
                        .field("left", &self.left)
                        .field("top", &self.top)
                        .field("right", &self.right)
                        .field("bottom", &self.bottom)
                        .finish()
                }
            }
            impl ::std::cmp::PartialEq for RECT {
                fn eq(&self, other: &Self) -> bool {
                    self.left == other.left
                        && self.top == other.top
                        && self.right == other.right
                        && self.bottom == other.bottom
                }
            }
            impl ::std::cmp::Eq for RECT {}
            unsafe impl ::windows::Abi for RECT {
                type Abi = Self;
                type DefaultType = Self;
            }
            #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
            #[repr(transparent)]
            pub struct WPARAM(pub usize);
            impl WPARAM {}
            impl ::std::default::Default for WPARAM {
                fn default() -> Self {
                    Self(0)
                }
            }
            impl WPARAM {
                pub const NULL: Self = Self(0);
                pub fn is_null(&self) -> bool {
                    self.0 == 0
                }
            }
            impl ::std::fmt::Debug for WPARAM {
                fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    fmt.debug_struct("WPARAM").field("Value", &self.0).finish()
                }
            }
            impl ::std::cmp::PartialEq for WPARAM {
                fn eq(&self, other: &Self) -> bool {
                    self.0 == other.0
                }
            }
            impl ::std::cmp::Eq for WPARAM {}
            unsafe impl ::windows::Abi for WPARAM {
                type Abi = Self;
                type DefaultType = Self;
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Graphics {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Direct2D {
                pub unsafe fn D2D1CreateDevice<'a>(
                    dxgidevice: impl ::windows::IntoParam<'a, super::Dxgi::IDXGIDevice>,
                    creationproperties: *const D2D1_CREATION_PROPERTIES,
                ) -> ::windows::Result<ID2D1Device> {
                    #[cfg(windows)]
                    {
                        #[link(name = "d2d1")]
                        extern "system" {
                            fn D2D1CreateDevice(
                                dxgidevice: ::windows::RawPtr,
                                creationproperties: *const D2D1_CREATION_PROPERTIES,
                                d2ddevice: *mut ::windows::RawPtr,
                            ) -> ::windows::HRESULT;
                        }
                        let mut result__: <ID2D1Device as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        D2D1CreateDevice(
                            dxgidevice.into_param().abi(),
                            ::std::mem::transmute(creationproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Device>(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn D2D1CreateFactory(
                    factorytype: D2D1_FACTORY_TYPE,
                    riid: *const ::windows::Guid,
                    pfactoryoptions: *const D2D1_FACTORY_OPTIONS,
                    ppifactory: *mut *mut ::std::ffi::c_void,
                ) -> ::windows::Result<()> {
                    #[cfg(windows)]
                    {
                        #[link(name = "d2d1")]
                        extern "system" {
                            fn D2D1CreateFactory(
                                factorytype: D2D1_FACTORY_TYPE,
                                riid: *const ::windows::Guid,
                                pfactoryoptions: *const D2D1_FACTORY_OPTIONS,
                                ppifactory: *mut *mut ::std::ffi::c_void,
                            ) -> ::windows::HRESULT;
                        }
                        D2D1CreateFactory(
                            ::std::mem::transmute(factorytype),
                            ::std::mem::transmute(riid),
                            ::std::mem::transmute(pfactoryoptions),
                            ::std::mem::transmute(ppifactory),
                        )
                        .ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn D2D1MakeRotateMatrix<'a>(
                    angle: f32,
                    center: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                    matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                ) {
                    #[cfg(windows)]
                    {
                        #[link(name = "d2d1")]
                        extern "system" {
                            fn D2D1MakeRotateMatrix(
                                angle: f32,
                                center: D2D_POINT_2F,
                                matrix: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                            );
                        }
                        D2D1MakeRotateMatrix(
                            ::std::mem::transmute(angle),
                            center.into_param().abi(),
                            ::std::mem::transmute(matrix),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_ALPHA_MODE(pub u32);
                pub const D2D1_ALPHA_MODE_UNKNOWN: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(0u32);
                pub const D2D1_ALPHA_MODE_PREMULTIPLIED: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(1u32);
                pub const D2D1_ALPHA_MODE_STRAIGHT: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(2u32);
                pub const D2D1_ALPHA_MODE_IGNORE: D2D1_ALPHA_MODE = D2D1_ALPHA_MODE(3u32);
                pub const D2D1_ALPHA_MODE_FORCE_DWORD: D2D1_ALPHA_MODE =
                    D2D1_ALPHA_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_ALPHA_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_ALPHA_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_ALPHA_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_ALPHA_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_ALPHA_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_ALPHA_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_ANTIALIAS_MODE(pub u32);
                pub const D2D1_ANTIALIAS_MODE_PER_PRIMITIVE: D2D1_ANTIALIAS_MODE =
                    D2D1_ANTIALIAS_MODE(0u32);
                pub const D2D1_ANTIALIAS_MODE_ALIASED: D2D1_ANTIALIAS_MODE =
                    D2D1_ANTIALIAS_MODE(1u32);
                pub const D2D1_ANTIALIAS_MODE_FORCE_DWORD: D2D1_ANTIALIAS_MODE =
                    D2D1_ANTIALIAS_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_ANTIALIAS_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_ANTIALIAS_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_ANTIALIAS_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_ANTIALIAS_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_ANTIALIAS_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_ANTIALIAS_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_BITMAP_BRUSH_PROPERTIES {
                    pub extendModeX: D2D1_EXTEND_MODE,
                    pub extendModeY: D2D1_EXTEND_MODE,
                    pub interpolationMode: D2D1_BITMAP_INTERPOLATION_MODE,
                }
                impl D2D1_BITMAP_BRUSH_PROPERTIES {}
                impl ::std::default::Default for D2D1_BITMAP_BRUSH_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            extendModeX: ::std::default::Default::default(),
                            extendModeY: ::std::default::Default::default(),
                            interpolationMode: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_BITMAP_BRUSH_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_BITMAP_BRUSH_PROPERTIES")
                            .field("extendModeX", &self.extendModeX)
                            .field("extendModeY", &self.extendModeY)
                            .field("interpolationMode", &self.interpolationMode)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_BITMAP_BRUSH_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.extendModeX == other.extendModeX
                            && self.extendModeY == other.extendModeY
                            && self.interpolationMode == other.interpolationMode
                    }
                }
                impl ::std::cmp::Eq for D2D1_BITMAP_BRUSH_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_BITMAP_BRUSH_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_BITMAP_INTERPOLATION_MODE(pub u32);
                pub const D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR:
                    D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(0u32);
                pub const D2D1_BITMAP_INTERPOLATION_MODE_LINEAR: D2D1_BITMAP_INTERPOLATION_MODE =
                    D2D1_BITMAP_INTERPOLATION_MODE(1u32);
                pub const D2D1_BITMAP_INTERPOLATION_MODE_FORCE_DWORD:
                    D2D1_BITMAP_INTERPOLATION_MODE = D2D1_BITMAP_INTERPOLATION_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_BITMAP_INTERPOLATION_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_BITMAP_INTERPOLATION_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_BITMAP_INTERPOLATION_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_BITMAP_INTERPOLATION_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_BITMAP_INTERPOLATION_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_BITMAP_INTERPOLATION_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_BITMAP_PROPERTIES {
                    pub pixelFormat: D2D1_PIXEL_FORMAT,
                    pub dpiX: f32,
                    pub dpiY: f32,
                }
                impl D2D1_BITMAP_PROPERTIES {}
                impl ::std::default::Default for D2D1_BITMAP_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            pixelFormat: ::std::default::Default::default(),
                            dpiX: 0.0,
                            dpiY: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_BITMAP_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_BITMAP_PROPERTIES")
                            .field("pixelFormat", &self.pixelFormat)
                            .field("dpiX", &self.dpiX)
                            .field("dpiY", &self.dpiY)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_BITMAP_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.pixelFormat == other.pixelFormat
                            && self.dpiX == other.dpiX
                            && self.dpiY == other.dpiY
                    }
                }
                impl ::std::cmp::Eq for D2D1_BITMAP_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_BITMAP_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_BRUSH_PROPERTIES {
                    pub opacity: f32,
                    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
                }
                impl D2D1_BRUSH_PROPERTIES {}
                impl ::std::default::Default for D2D1_BRUSH_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            opacity: 0.0,
                            transform: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_BRUSH_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_BRUSH_PROPERTIES")
                            .field("opacity", &self.opacity)
                            .field("transform", &self.transform)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_BRUSH_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.opacity == other.opacity && self.transform == other.transform
                    }
                }
                impl ::std::cmp::Eq for D2D1_BRUSH_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_BRUSH_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_CAP_STYLE(pub u32);
                pub const D2D1_CAP_STYLE_FLAT: D2D1_CAP_STYLE = D2D1_CAP_STYLE(0u32);
                pub const D2D1_CAP_STYLE_SQUARE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(1u32);
                pub const D2D1_CAP_STYLE_ROUND: D2D1_CAP_STYLE = D2D1_CAP_STYLE(2u32);
                pub const D2D1_CAP_STYLE_TRIANGLE: D2D1_CAP_STYLE = D2D1_CAP_STYLE(3u32);
                pub const D2D1_CAP_STYLE_FORCE_DWORD: D2D1_CAP_STYLE =
                    D2D1_CAP_STYLE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_CAP_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_CAP_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_CAP_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_CAP_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_CAP_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_CAP_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_COLOR_F {
                    pub r: f32,
                    pub g: f32,
                    pub b: f32,
                    pub a: f32,
                }
                impl D2D1_COLOR_F {}
                impl ::std::default::Default for D2D1_COLOR_F {
                    fn default() -> Self {
                        Self {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_COLOR_F {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_COLOR_F")
                            .field("r", &self.r)
                            .field("g", &self.g)
                            .field("b", &self.b)
                            .field("a", &self.a)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_COLOR_F {
                    fn eq(&self, other: &Self) -> bool {
                        self.r == other.r
                            && self.g == other.g
                            && self.b == other.b
                            && self.a == other.a
                    }
                }
                impl ::std::cmp::Eq for D2D1_COLOR_F {}
                unsafe impl ::windows::Abi for D2D1_COLOR_F {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(pub u32);
                pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE:
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS =
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(0u32);
                pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE:
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS =
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(1u32);
                pub const D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_FORCE_DWORD:
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS =
                    D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_CREATION_PROPERTIES {
                    pub threadingMode: D2D1_THREADING_MODE,
                    pub debugLevel: D2D1_DEBUG_LEVEL,
                    pub options: D2D1_DEVICE_CONTEXT_OPTIONS,
                }
                impl D2D1_CREATION_PROPERTIES {}
                impl ::std::default::Default for D2D1_CREATION_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            threadingMode: ::std::default::Default::default(),
                            debugLevel: ::std::default::Default::default(),
                            options: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_CREATION_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_CREATION_PROPERTIES")
                            .field("threadingMode", &self.threadingMode)
                            .field("debugLevel", &self.debugLevel)
                            .field("options", &self.options)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_CREATION_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.threadingMode == other.threadingMode
                            && self.debugLevel == other.debugLevel
                            && self.options == other.options
                    }
                }
                impl ::std::cmp::Eq for D2D1_CREATION_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_CREATION_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_DASH_STYLE(pub u32);
                pub const D2D1_DASH_STYLE_SOLID: D2D1_DASH_STYLE = D2D1_DASH_STYLE(0u32);
                pub const D2D1_DASH_STYLE_DASH: D2D1_DASH_STYLE = D2D1_DASH_STYLE(1u32);
                pub const D2D1_DASH_STYLE_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(2u32);
                pub const D2D1_DASH_STYLE_DASH_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(3u32);
                pub const D2D1_DASH_STYLE_DASH_DOT_DOT: D2D1_DASH_STYLE = D2D1_DASH_STYLE(4u32);
                pub const D2D1_DASH_STYLE_CUSTOM: D2D1_DASH_STYLE = D2D1_DASH_STYLE(5u32);
                pub const D2D1_DASH_STYLE_FORCE_DWORD: D2D1_DASH_STYLE =
                    D2D1_DASH_STYLE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_DASH_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_DASH_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_DASH_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_DASH_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_DASH_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_DASH_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_DEBUG_LEVEL(pub u32);
                pub const D2D1_DEBUG_LEVEL_NONE: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(0u32);
                pub const D2D1_DEBUG_LEVEL_ERROR: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(1u32);
                pub const D2D1_DEBUG_LEVEL_WARNING: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(2u32);
                pub const D2D1_DEBUG_LEVEL_INFORMATION: D2D1_DEBUG_LEVEL = D2D1_DEBUG_LEVEL(3u32);
                pub const D2D1_DEBUG_LEVEL_FORCE_DWORD: D2D1_DEBUG_LEVEL =
                    D2D1_DEBUG_LEVEL(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_DEBUG_LEVEL {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_DEBUG_LEVEL {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_DEBUG_LEVEL {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_DEBUG_LEVEL {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_DEBUG_LEVEL {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_DEBUG_LEVEL {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_DEVICE_CONTEXT_OPTIONS(pub u32);
                pub const D2D1_DEVICE_CONTEXT_OPTIONS_NONE: D2D1_DEVICE_CONTEXT_OPTIONS =
                    D2D1_DEVICE_CONTEXT_OPTIONS(0u32);
                pub const D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS:
                    D2D1_DEVICE_CONTEXT_OPTIONS = D2D1_DEVICE_CONTEXT_OPTIONS(1u32);
                pub const D2D1_DEVICE_CONTEXT_OPTIONS_FORCE_DWORD: D2D1_DEVICE_CONTEXT_OPTIONS =
                    D2D1_DEVICE_CONTEXT_OPTIONS(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_DEVICE_CONTEXT_OPTIONS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_DEVICE_CONTEXT_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_DEVICE_CONTEXT_OPTIONS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_DEVICE_CONTEXT_OPTIONS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_DEVICE_CONTEXT_OPTIONS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_DRAWING_STATE_DESCRIPTION {
                    pub antialiasMode: D2D1_ANTIALIAS_MODE,
                    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
                    pub tag1: u64,
                    pub tag2: u64,
                    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
                }
                impl D2D1_DRAWING_STATE_DESCRIPTION {}
                impl ::std::default::Default for D2D1_DRAWING_STATE_DESCRIPTION {
                    fn default() -> Self {
                        Self {
                            antialiasMode: ::std::default::Default::default(),
                            textAntialiasMode: ::std::default::Default::default(),
                            tag1: 0,
                            tag2: 0,
                            transform: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_DRAWING_STATE_DESCRIPTION {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_DRAWING_STATE_DESCRIPTION")
                            .field("antialiasMode", &self.antialiasMode)
                            .field("textAntialiasMode", &self.textAntialiasMode)
                            .field("tag1", &self.tag1)
                            .field("tag2", &self.tag2)
                            .field("transform", &self.transform)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_DRAWING_STATE_DESCRIPTION {
                    fn eq(&self, other: &Self) -> bool {
                        self.antialiasMode == other.antialiasMode
                            && self.textAntialiasMode == other.textAntialiasMode
                            && self.tag1 == other.tag1
                            && self.tag2 == other.tag2
                            && self.transform == other.transform
                    }
                }
                impl ::std::cmp::Eq for D2D1_DRAWING_STATE_DESCRIPTION {}
                unsafe impl ::windows::Abi for D2D1_DRAWING_STATE_DESCRIPTION {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_DRAWING_STATE_DESCRIPTION1 {
                    pub antialiasMode: D2D1_ANTIALIAS_MODE,
                    pub textAntialiasMode: D2D1_TEXT_ANTIALIAS_MODE,
                    pub tag1: u64,
                    pub tag2: u64,
                    pub transform: super::super::super::Foundation::Numerics::Matrix3x2,
                    pub primitiveBlend: D2D1_PRIMITIVE_BLEND,
                    pub unitMode: D2D1_UNIT_MODE,
                }
                impl D2D1_DRAWING_STATE_DESCRIPTION1 {}
                impl ::std::default::Default for D2D1_DRAWING_STATE_DESCRIPTION1 {
                    fn default() -> Self {
                        Self {
                            antialiasMode: ::std::default::Default::default(),
                            textAntialiasMode: ::std::default::Default::default(),
                            tag1: 0,
                            tag2: 0,
                            transform: ::std::default::Default::default(),
                            primitiveBlend: ::std::default::Default::default(),
                            unitMode: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_DRAWING_STATE_DESCRIPTION1 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_DRAWING_STATE_DESCRIPTION1")
                            .field("antialiasMode", &self.antialiasMode)
                            .field("textAntialiasMode", &self.textAntialiasMode)
                            .field("tag1", &self.tag1)
                            .field("tag2", &self.tag2)
                            .field("transform", &self.transform)
                            .field("primitiveBlend", &self.primitiveBlend)
                            .field("unitMode", &self.unitMode)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_DRAWING_STATE_DESCRIPTION1 {
                    fn eq(&self, other: &Self) -> bool {
                        self.antialiasMode == other.antialiasMode
                            && self.textAntialiasMode == other.textAntialiasMode
                            && self.tag1 == other.tag1
                            && self.tag2 == other.tag2
                            && self.transform == other.transform
                            && self.primitiveBlend == other.primitiveBlend
                            && self.unitMode == other.unitMode
                    }
                }
                impl ::std::cmp::Eq for D2D1_DRAWING_STATE_DESCRIPTION1 {}
                unsafe impl ::windows::Abi for D2D1_DRAWING_STATE_DESCRIPTION1 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_DRAW_TEXT_OPTIONS(pub u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_NO_SNAP: D2D1_DRAW_TEXT_OPTIONS =
                    D2D1_DRAW_TEXT_OPTIONS(1u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_CLIP: D2D1_DRAW_TEXT_OPTIONS =
                    D2D1_DRAW_TEXT_OPTIONS(2u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT: D2D1_DRAW_TEXT_OPTIONS =
                    D2D1_DRAW_TEXT_OPTIONS(4u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_DISABLE_COLOR_BITMAP_SNAPPING:
                    D2D1_DRAW_TEXT_OPTIONS = D2D1_DRAW_TEXT_OPTIONS(8u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_NONE: D2D1_DRAW_TEXT_OPTIONS =
                    D2D1_DRAW_TEXT_OPTIONS(0u32);
                pub const D2D1_DRAW_TEXT_OPTIONS_FORCE_DWORD: D2D1_DRAW_TEXT_OPTIONS =
                    D2D1_DRAW_TEXT_OPTIONS(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_DRAW_TEXT_OPTIONS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_DRAW_TEXT_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_DRAW_TEXT_OPTIONS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_DRAW_TEXT_OPTIONS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_DRAW_TEXT_OPTIONS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_DRAW_TEXT_OPTIONS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_ELLIPSE {
                    pub point: D2D_POINT_2F,
                    pub radiusX: f32,
                    pub radiusY: f32,
                }
                impl D2D1_ELLIPSE {}
                impl ::std::default::Default for D2D1_ELLIPSE {
                    fn default() -> Self {
                        Self {
                            point: ::std::default::Default::default(),
                            radiusX: 0.0,
                            radiusY: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_ELLIPSE {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_ELLIPSE")
                            .field("point", &self.point)
                            .field("radiusX", &self.radiusX)
                            .field("radiusY", &self.radiusY)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_ELLIPSE {
                    fn eq(&self, other: &Self) -> bool {
                        self.point == other.point
                            && self.radiusX == other.radiusX
                            && self.radiusY == other.radiusY
                    }
                }
                impl ::std::cmp::Eq for D2D1_ELLIPSE {}
                unsafe impl ::windows::Abi for D2D1_ELLIPSE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_EXTEND_MODE(pub u32);
                pub const D2D1_EXTEND_MODE_CLAMP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(0u32);
                pub const D2D1_EXTEND_MODE_WRAP: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(1u32);
                pub const D2D1_EXTEND_MODE_MIRROR: D2D1_EXTEND_MODE = D2D1_EXTEND_MODE(2u32);
                pub const D2D1_EXTEND_MODE_FORCE_DWORD: D2D1_EXTEND_MODE =
                    D2D1_EXTEND_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_EXTEND_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_EXTEND_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_EXTEND_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_EXTEND_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_EXTEND_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_EXTEND_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_FACTORY_OPTIONS {
                    pub debugLevel: D2D1_DEBUG_LEVEL,
                }
                impl D2D1_FACTORY_OPTIONS {}
                impl ::std::default::Default for D2D1_FACTORY_OPTIONS {
                    fn default() -> Self {
                        Self {
                            debugLevel: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_FACTORY_OPTIONS {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_FACTORY_OPTIONS")
                            .field("debugLevel", &self.debugLevel)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_FACTORY_OPTIONS {
                    fn eq(&self, other: &Self) -> bool {
                        self.debugLevel == other.debugLevel
                    }
                }
                impl ::std::cmp::Eq for D2D1_FACTORY_OPTIONS {}
                unsafe impl ::windows::Abi for D2D1_FACTORY_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_FACTORY_TYPE(pub u32);
                pub const D2D1_FACTORY_TYPE_SINGLE_THREADED: D2D1_FACTORY_TYPE =
                    D2D1_FACTORY_TYPE(0u32);
                pub const D2D1_FACTORY_TYPE_MULTI_THREADED: D2D1_FACTORY_TYPE =
                    D2D1_FACTORY_TYPE(1u32);
                pub const D2D1_FACTORY_TYPE_FORCE_DWORD: D2D1_FACTORY_TYPE =
                    D2D1_FACTORY_TYPE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_FACTORY_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_FACTORY_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_FACTORY_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_FACTORY_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_FACTORY_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_FACTORY_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_FEATURE_LEVEL(pub u32);
                pub const D2D1_FEATURE_LEVEL_DEFAULT: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(0u32);
                pub const D2D1_FEATURE_LEVEL_9: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(37120u32);
                pub const D2D1_FEATURE_LEVEL_10: D2D1_FEATURE_LEVEL = D2D1_FEATURE_LEVEL(40960u32);
                pub const D2D1_FEATURE_LEVEL_FORCE_DWORD: D2D1_FEATURE_LEVEL =
                    D2D1_FEATURE_LEVEL(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_FEATURE_LEVEL {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_FEATURE_LEVEL {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_FEATURE_LEVEL {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_FEATURE_LEVEL {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_FEATURE_LEVEL {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_FEATURE_LEVEL {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_FILL_MODE(pub u32);
                pub const D2D1_FILL_MODE_ALTERNATE: D2D1_FILL_MODE = D2D1_FILL_MODE(0u32);
                pub const D2D1_FILL_MODE_WINDING: D2D1_FILL_MODE = D2D1_FILL_MODE(1u32);
                pub const D2D1_FILL_MODE_FORCE_DWORD: D2D1_FILL_MODE =
                    D2D1_FILL_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_FILL_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_FILL_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_FILL_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_FILL_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_FILL_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_FILL_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_GAMMA(pub u32);
                pub const D2D1_GAMMA_2_2: D2D1_GAMMA = D2D1_GAMMA(0u32);
                pub const D2D1_GAMMA_1_0: D2D1_GAMMA = D2D1_GAMMA(1u32);
                pub const D2D1_GAMMA_FORCE_DWORD: D2D1_GAMMA = D2D1_GAMMA(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_GAMMA {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_GAMMA {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_GAMMA {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_GAMMA {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_GAMMA {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_GAMMA {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_GRADIENT_STOP {
                    pub position: f32,
                    pub color: D2D1_COLOR_F,
                }
                impl D2D1_GRADIENT_STOP {}
                impl ::std::default::Default for D2D1_GRADIENT_STOP {
                    fn default() -> Self {
                        Self {
                            position: 0.0,
                            color: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_GRADIENT_STOP {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_GRADIENT_STOP")
                            .field("position", &self.position)
                            .field("color", &self.color)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_GRADIENT_STOP {
                    fn eq(&self, other: &Self) -> bool {
                        self.position == other.position && self.color == other.color
                    }
                }
                impl ::std::cmp::Eq for D2D1_GRADIENT_STOP {}
                unsafe impl ::windows::Abi for D2D1_GRADIENT_STOP {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    pub hwnd: super::super::Foundation::HWND,
                    pub pixelSize: D2D_SIZE_U,
                    pub presentOptions: D2D1_PRESENT_OPTIONS,
                }
                impl D2D1_HWND_RENDER_TARGET_PROPERTIES {}
                impl ::std::default::Default for D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            hwnd: ::std::default::Default::default(),
                            pixelSize: ::std::default::Default::default(),
                            presentOptions: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_HWND_RENDER_TARGET_PROPERTIES")
                            .field("hwnd", &self.hwnd)
                            .field("pixelSize", &self.pixelSize)
                            .field("presentOptions", &self.presentOptions)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.hwnd == other.hwnd
                            && self.pixelSize == other.pixelSize
                            && self.presentOptions == other.presentOptions
                    }
                }
                impl ::std::cmp::Eq for D2D1_HWND_RENDER_TARGET_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_HWND_RENDER_TARGET_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_LAYER_OPTIONS(pub u32);
                pub const D2D1_LAYER_OPTIONS_NONE: D2D1_LAYER_OPTIONS = D2D1_LAYER_OPTIONS(0u32);
                pub const D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE: D2D1_LAYER_OPTIONS =
                    D2D1_LAYER_OPTIONS(1u32);
                pub const D2D1_LAYER_OPTIONS_FORCE_DWORD: D2D1_LAYER_OPTIONS =
                    D2D1_LAYER_OPTIONS(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_LAYER_OPTIONS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_LAYER_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_LAYER_OPTIONS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_LAYER_OPTIONS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_LAYER_OPTIONS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_LAYER_OPTIONS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone)]
                #[repr(C)]
                pub struct D2D1_LAYER_PARAMETERS {
                    pub contentBounds: D2D_RECT_F,
                    pub geometricMask: ::std::option::Option<ID2D1Geometry>,
                    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
                    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
                    pub opacity: f32,
                    pub opacityBrush: ::std::option::Option<ID2D1Brush>,
                    pub layerOptions: D2D1_LAYER_OPTIONS,
                }
                impl D2D1_LAYER_PARAMETERS {}
                impl ::std::default::Default for D2D1_LAYER_PARAMETERS {
                    fn default() -> Self {
                        Self {
                            contentBounds: ::std::default::Default::default(),
                            geometricMask: ::std::default::Default::default(),
                            maskAntialiasMode: ::std::default::Default::default(),
                            maskTransform: ::std::default::Default::default(),
                            opacity: 0.0,
                            opacityBrush: ::std::default::Default::default(),
                            layerOptions: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_LAYER_PARAMETERS {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_LAYER_PARAMETERS")
                            .field("contentBounds", &self.contentBounds)
                            .field("geometricMask", &self.geometricMask)
                            .field("maskAntialiasMode", &self.maskAntialiasMode)
                            .field("maskTransform", &self.maskTransform)
                            .field("opacity", &self.opacity)
                            .field("opacityBrush", &self.opacityBrush)
                            .field("layerOptions", &self.layerOptions)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_LAYER_PARAMETERS {
                    fn eq(&self, other: &Self) -> bool {
                        self.contentBounds == other.contentBounds
                            && self.geometricMask == other.geometricMask
                            && self.maskAntialiasMode == other.maskAntialiasMode
                            && self.maskTransform == other.maskTransform
                            && self.opacity == other.opacity
                            && self.opacityBrush == other.opacityBrush
                            && self.layerOptions == other.layerOptions
                    }
                }
                impl ::std::cmp::Eq for D2D1_LAYER_PARAMETERS {}
                #[repr(C)]
                #[doc(hidden)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct D2D1_LAYER_PARAMETERS_abi {
                    pub contentBounds: D2D_RECT_F,
                    pub geometricMask: ::windows::RawPtr,
                    pub maskAntialiasMode: D2D1_ANTIALIAS_MODE,
                    pub maskTransform: super::super::super::Foundation::Numerics::Matrix3x2,
                    pub opacity: f32,
                    pub opacityBrush: ::windows::RawPtr,
                    pub layerOptions: D2D1_LAYER_OPTIONS,
                }
                unsafe impl ::windows::Abi for D2D1_LAYER_PARAMETERS {
                    type Abi = D2D1_LAYER_PARAMETERS_abi;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    pub startPoint: D2D_POINT_2F,
                    pub endPoint: D2D_POINT_2F,
                }
                impl D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
                impl ::std::default::Default for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            startPoint: ::std::default::Default::default(),
                            endPoint: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES")
                            .field("startPoint", &self.startPoint)
                            .field("endPoint", &self.endPoint)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.startPoint == other.startPoint && self.endPoint == other.endPoint
                    }
                }
                impl ::std::cmp::Eq for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_LINE_JOIN(pub u32);
                pub const D2D1_LINE_JOIN_MITER: D2D1_LINE_JOIN = D2D1_LINE_JOIN(0u32);
                pub const D2D1_LINE_JOIN_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(1u32);
                pub const D2D1_LINE_JOIN_ROUND: D2D1_LINE_JOIN = D2D1_LINE_JOIN(2u32);
                pub const D2D1_LINE_JOIN_MITER_OR_BEVEL: D2D1_LINE_JOIN = D2D1_LINE_JOIN(3u32);
                pub const D2D1_LINE_JOIN_FORCE_DWORD: D2D1_LINE_JOIN =
                    D2D1_LINE_JOIN(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_LINE_JOIN {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_LINE_JOIN {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_LINE_JOIN {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_LINE_JOIN {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_LINE_JOIN {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_LINE_JOIN {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_OPACITY_MASK_CONTENT(pub u32);
                pub const D2D1_OPACITY_MASK_CONTENT_GRAPHICS: D2D1_OPACITY_MASK_CONTENT =
                    D2D1_OPACITY_MASK_CONTENT(0u32);
                pub const D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL: D2D1_OPACITY_MASK_CONTENT =
                    D2D1_OPACITY_MASK_CONTENT(1u32);
                pub const D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE: D2D1_OPACITY_MASK_CONTENT =
                    D2D1_OPACITY_MASK_CONTENT(2u32);
                pub const D2D1_OPACITY_MASK_CONTENT_FORCE_DWORD: D2D1_OPACITY_MASK_CONTENT =
                    D2D1_OPACITY_MASK_CONTENT(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_OPACITY_MASK_CONTENT {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_OPACITY_MASK_CONTENT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_OPACITY_MASK_CONTENT {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_OPACITY_MASK_CONTENT {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_OPACITY_MASK_CONTENT {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_OPACITY_MASK_CONTENT {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_PIXEL_FORMAT {
                    pub format: super::Dxgi::DXGI_FORMAT,
                    pub alphaMode: D2D1_ALPHA_MODE,
                }
                impl D2D1_PIXEL_FORMAT {}
                impl ::std::default::Default for D2D1_PIXEL_FORMAT {
                    fn default() -> Self {
                        Self {
                            format: ::std::default::Default::default(),
                            alphaMode: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_PIXEL_FORMAT {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_PIXEL_FORMAT")
                            .field("format", &self.format)
                            .field("alphaMode", &self.alphaMode)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_PIXEL_FORMAT {
                    fn eq(&self, other: &Self) -> bool {
                        self.format == other.format && self.alphaMode == other.alphaMode
                    }
                }
                impl ::std::cmp::Eq for D2D1_PIXEL_FORMAT {}
                unsafe impl ::windows::Abi for D2D1_PIXEL_FORMAT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_PRESENT_OPTIONS(pub u32);
                pub const D2D1_PRESENT_OPTIONS_NONE: D2D1_PRESENT_OPTIONS =
                    D2D1_PRESENT_OPTIONS(0u32);
                pub const D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS: D2D1_PRESENT_OPTIONS =
                    D2D1_PRESENT_OPTIONS(1u32);
                pub const D2D1_PRESENT_OPTIONS_IMMEDIATELY: D2D1_PRESENT_OPTIONS =
                    D2D1_PRESENT_OPTIONS(2u32);
                pub const D2D1_PRESENT_OPTIONS_FORCE_DWORD: D2D1_PRESENT_OPTIONS =
                    D2D1_PRESENT_OPTIONS(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_PRESENT_OPTIONS {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_PRESENT_OPTIONS {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_PRESENT_OPTIONS {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_PRESENT_OPTIONS {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_PRESENT_OPTIONS {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_PRESENT_OPTIONS {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_PRIMITIVE_BLEND(pub u32);
                pub const D2D1_PRIMITIVE_BLEND_SOURCE_OVER: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(0u32);
                pub const D2D1_PRIMITIVE_BLEND_COPY: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(1u32);
                pub const D2D1_PRIMITIVE_BLEND_MIN: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(2u32);
                pub const D2D1_PRIMITIVE_BLEND_ADD: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(3u32);
                pub const D2D1_PRIMITIVE_BLEND_MAX: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(4u32);
                pub const D2D1_PRIMITIVE_BLEND_FORCE_DWORD: D2D1_PRIMITIVE_BLEND =
                    D2D1_PRIMITIVE_BLEND(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_PRIMITIVE_BLEND {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_PRIMITIVE_BLEND {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_PRIMITIVE_BLEND {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_PRIMITIVE_BLEND {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_PRIMITIVE_BLEND {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_PRIMITIVE_BLEND {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone)]
                #[repr(C)]
                pub struct D2D1_PROPERTY_BINDING {
                    pub propertyName: super::super::Foundation::PWSTR,
                    pub setFunction: ::std::option::Option<PD2D1_PROPERTY_SET_FUNCTION>,
                    pub getFunction: ::std::option::Option<PD2D1_PROPERTY_GET_FUNCTION>,
                }
                impl D2D1_PROPERTY_BINDING {}
                impl ::std::default::Default for D2D1_PROPERTY_BINDING {
                    fn default() -> Self {
                        Self {
                            propertyName: ::std::default::Default::default(),
                            setFunction: ::std::default::Default::default(),
                            getFunction: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_PROPERTY_BINDING {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_PROPERTY_BINDING")
                            .field("propertyName", &self.propertyName)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_PROPERTY_BINDING {
                    fn eq(&self, other: &Self) -> bool {
                        self.propertyName == other.propertyName
                            && self.setFunction.map(|f| f as usize)
                                == other.setFunction.map(|f| f as usize)
                            && self.getFunction.map(|f| f as usize)
                                == other.getFunction.map(|f| f as usize)
                    }
                }
                impl ::std::cmp::Eq for D2D1_PROPERTY_BINDING {}
                #[repr(C)]
                #[doc(hidden)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct D2D1_PROPERTY_BINDING_abi {
                    pub propertyName: super::super::Foundation::PWSTR,
                    pub setFunction: ::windows::RawPtr,
                    pub getFunction: ::windows::RawPtr,
                }
                unsafe impl ::windows::Abi for D2D1_PROPERTY_BINDING {
                    type Abi = D2D1_PROPERTY_BINDING_abi;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_PROPERTY_TYPE(pub u32);
                pub const D2D1_PROPERTY_TYPE_UNKNOWN: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(0u32);
                pub const D2D1_PROPERTY_TYPE_STRING: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(1u32);
                pub const D2D1_PROPERTY_TYPE_BOOL: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(2u32);
                pub const D2D1_PROPERTY_TYPE_UINT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(3u32);
                pub const D2D1_PROPERTY_TYPE_INT32: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(4u32);
                pub const D2D1_PROPERTY_TYPE_FLOAT: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(5u32);
                pub const D2D1_PROPERTY_TYPE_VECTOR2: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(6u32);
                pub const D2D1_PROPERTY_TYPE_VECTOR3: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(7u32);
                pub const D2D1_PROPERTY_TYPE_VECTOR4: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(8u32);
                pub const D2D1_PROPERTY_TYPE_BLOB: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(9u32);
                pub const D2D1_PROPERTY_TYPE_IUNKNOWN: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(10u32);
                pub const D2D1_PROPERTY_TYPE_ENUM: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(11u32);
                pub const D2D1_PROPERTY_TYPE_ARRAY: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(12u32);
                pub const D2D1_PROPERTY_TYPE_CLSID: D2D1_PROPERTY_TYPE = D2D1_PROPERTY_TYPE(13u32);
                pub const D2D1_PROPERTY_TYPE_MATRIX_3X2: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(14u32);
                pub const D2D1_PROPERTY_TYPE_MATRIX_4X3: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(15u32);
                pub const D2D1_PROPERTY_TYPE_MATRIX_4X4: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(16u32);
                pub const D2D1_PROPERTY_TYPE_MATRIX_5X4: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(17u32);
                pub const D2D1_PROPERTY_TYPE_COLOR_CONTEXT: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(18u32);
                pub const D2D1_PROPERTY_TYPE_FORCE_DWORD: D2D1_PROPERTY_TYPE =
                    D2D1_PROPERTY_TYPE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_PROPERTY_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_PROPERTY_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_PROPERTY_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_PROPERTY_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_PROPERTY_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_PROPERTY_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    pub center: D2D_POINT_2F,
                    pub gradientOriginOffset: D2D_POINT_2F,
                    pub radiusX: f32,
                    pub radiusY: f32,
                }
                impl D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
                impl ::std::default::Default for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            center: ::std::default::Default::default(),
                            gradientOriginOffset: ::std::default::Default::default(),
                            radiusX: 0.0,
                            radiusY: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES")
                            .field("center", &self.center)
                            .field("gradientOriginOffset", &self.gradientOriginOffset)
                            .field("radiusX", &self.radiusX)
                            .field("radiusY", &self.radiusY)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.center == other.center
                            && self.gradientOriginOffset == other.gradientOriginOffset
                            && self.radiusX == other.radiusX
                            && self.radiusY == other.radiusY
                    }
                }
                impl ::std::cmp::Eq for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_RENDER_TARGET_PROPERTIES {
                    pub r#type: D2D1_RENDER_TARGET_TYPE,
                    pub pixelFormat: D2D1_PIXEL_FORMAT,
                    pub dpiX: f32,
                    pub dpiY: f32,
                    pub usage: D2D1_RENDER_TARGET_USAGE,
                    pub minLevel: D2D1_FEATURE_LEVEL,
                }
                impl D2D1_RENDER_TARGET_PROPERTIES {}
                impl ::std::default::Default for D2D1_RENDER_TARGET_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            r#type: ::std::default::Default::default(),
                            pixelFormat: ::std::default::Default::default(),
                            dpiX: 0.0,
                            dpiY: 0.0,
                            usage: ::std::default::Default::default(),
                            minLevel: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_RENDER_TARGET_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_RENDER_TARGET_PROPERTIES")
                            .field("r#type", &self.r#type)
                            .field("pixelFormat", &self.pixelFormat)
                            .field("dpiX", &self.dpiX)
                            .field("dpiY", &self.dpiY)
                            .field("usage", &self.usage)
                            .field("minLevel", &self.minLevel)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_RENDER_TARGET_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.r#type == other.r#type
                            && self.pixelFormat == other.pixelFormat
                            && self.dpiX == other.dpiX
                            && self.dpiY == other.dpiY
                            && self.usage == other.usage
                            && self.minLevel == other.minLevel
                    }
                }
                impl ::std::cmp::Eq for D2D1_RENDER_TARGET_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_RENDER_TARGET_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_RENDER_TARGET_TYPE(pub u32);
                pub const D2D1_RENDER_TARGET_TYPE_DEFAULT: D2D1_RENDER_TARGET_TYPE =
                    D2D1_RENDER_TARGET_TYPE(0u32);
                pub const D2D1_RENDER_TARGET_TYPE_SOFTWARE: D2D1_RENDER_TARGET_TYPE =
                    D2D1_RENDER_TARGET_TYPE(1u32);
                pub const D2D1_RENDER_TARGET_TYPE_HARDWARE: D2D1_RENDER_TARGET_TYPE =
                    D2D1_RENDER_TARGET_TYPE(2u32);
                pub const D2D1_RENDER_TARGET_TYPE_FORCE_DWORD: D2D1_RENDER_TARGET_TYPE =
                    D2D1_RENDER_TARGET_TYPE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_RENDER_TARGET_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_RENDER_TARGET_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_RENDER_TARGET_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_RENDER_TARGET_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_RENDER_TARGET_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_RENDER_TARGET_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_RENDER_TARGET_USAGE(pub u32);
                pub const D2D1_RENDER_TARGET_USAGE_NONE: D2D1_RENDER_TARGET_USAGE =
                    D2D1_RENDER_TARGET_USAGE(0u32);
                pub const D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING: D2D1_RENDER_TARGET_USAGE =
                    D2D1_RENDER_TARGET_USAGE(1u32);
                pub const D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE: D2D1_RENDER_TARGET_USAGE =
                    D2D1_RENDER_TARGET_USAGE(2u32);
                pub const D2D1_RENDER_TARGET_USAGE_FORCE_DWORD: D2D1_RENDER_TARGET_USAGE =
                    D2D1_RENDER_TARGET_USAGE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_RENDER_TARGET_USAGE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_RENDER_TARGET_USAGE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_RENDER_TARGET_USAGE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_RENDER_TARGET_USAGE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_RENDER_TARGET_USAGE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_RENDER_TARGET_USAGE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_ROUNDED_RECT {
                    pub rect: D2D_RECT_F,
                    pub radiusX: f32,
                    pub radiusY: f32,
                }
                impl D2D1_ROUNDED_RECT {}
                impl ::std::default::Default for D2D1_ROUNDED_RECT {
                    fn default() -> Self {
                        Self {
                            rect: ::std::default::Default::default(),
                            radiusX: 0.0,
                            radiusY: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_ROUNDED_RECT {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_ROUNDED_RECT")
                            .field("rect", &self.rect)
                            .field("radiusX", &self.radiusX)
                            .field("radiusY", &self.radiusY)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_ROUNDED_RECT {
                    fn eq(&self, other: &Self) -> bool {
                        self.rect == other.rect
                            && self.radiusX == other.radiusX
                            && self.radiusY == other.radiusY
                    }
                }
                impl ::std::cmp::Eq for D2D1_ROUNDED_RECT {}
                unsafe impl ::windows::Abi for D2D1_ROUNDED_RECT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_STROKE_STYLE_PROPERTIES {
                    pub startCap: D2D1_CAP_STYLE,
                    pub endCap: D2D1_CAP_STYLE,
                    pub dashCap: D2D1_CAP_STYLE,
                    pub lineJoin: D2D1_LINE_JOIN,
                    pub miterLimit: f32,
                    pub dashStyle: D2D1_DASH_STYLE,
                    pub dashOffset: f32,
                }
                impl D2D1_STROKE_STYLE_PROPERTIES {}
                impl ::std::default::Default for D2D1_STROKE_STYLE_PROPERTIES {
                    fn default() -> Self {
                        Self {
                            startCap: ::std::default::Default::default(),
                            endCap: ::std::default::Default::default(),
                            dashCap: ::std::default::Default::default(),
                            lineJoin: ::std::default::Default::default(),
                            miterLimit: 0.0,
                            dashStyle: ::std::default::Default::default(),
                            dashOffset: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_STROKE_STYLE_PROPERTIES {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_STROKE_STYLE_PROPERTIES")
                            .field("startCap", &self.startCap)
                            .field("endCap", &self.endCap)
                            .field("dashCap", &self.dashCap)
                            .field("lineJoin", &self.lineJoin)
                            .field("miterLimit", &self.miterLimit)
                            .field("dashStyle", &self.dashStyle)
                            .field("dashOffset", &self.dashOffset)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_STROKE_STYLE_PROPERTIES {
                    fn eq(&self, other: &Self) -> bool {
                        self.startCap == other.startCap
                            && self.endCap == other.endCap
                            && self.dashCap == other.dashCap
                            && self.lineJoin == other.lineJoin
                            && self.miterLimit == other.miterLimit
                            && self.dashStyle == other.dashStyle
                            && self.dashOffset == other.dashOffset
                    }
                }
                impl ::std::cmp::Eq for D2D1_STROKE_STYLE_PROPERTIES {}
                unsafe impl ::windows::Abi for D2D1_STROKE_STYLE_PROPERTIES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D1_STROKE_STYLE_PROPERTIES1 {
                    pub startCap: D2D1_CAP_STYLE,
                    pub endCap: D2D1_CAP_STYLE,
                    pub dashCap: D2D1_CAP_STYLE,
                    pub lineJoin: D2D1_LINE_JOIN,
                    pub miterLimit: f32,
                    pub dashStyle: D2D1_DASH_STYLE,
                    pub dashOffset: f32,
                    pub transformType: D2D1_STROKE_TRANSFORM_TYPE,
                }
                impl D2D1_STROKE_STYLE_PROPERTIES1 {}
                impl ::std::default::Default for D2D1_STROKE_STYLE_PROPERTIES1 {
                    fn default() -> Self {
                        Self {
                            startCap: ::std::default::Default::default(),
                            endCap: ::std::default::Default::default(),
                            dashCap: ::std::default::Default::default(),
                            lineJoin: ::std::default::Default::default(),
                            miterLimit: 0.0,
                            dashStyle: ::std::default::Default::default(),
                            dashOffset: 0.0,
                            transformType: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D1_STROKE_STYLE_PROPERTIES1 {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D1_STROKE_STYLE_PROPERTIES1")
                            .field("startCap", &self.startCap)
                            .field("endCap", &self.endCap)
                            .field("dashCap", &self.dashCap)
                            .field("lineJoin", &self.lineJoin)
                            .field("miterLimit", &self.miterLimit)
                            .field("dashStyle", &self.dashStyle)
                            .field("dashOffset", &self.dashOffset)
                            .field("transformType", &self.transformType)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D1_STROKE_STYLE_PROPERTIES1 {
                    fn eq(&self, other: &Self) -> bool {
                        self.startCap == other.startCap
                            && self.endCap == other.endCap
                            && self.dashCap == other.dashCap
                            && self.lineJoin == other.lineJoin
                            && self.miterLimit == other.miterLimit
                            && self.dashStyle == other.dashStyle
                            && self.dashOffset == other.dashOffset
                            && self.transformType == other.transformType
                    }
                }
                impl ::std::cmp::Eq for D2D1_STROKE_STYLE_PROPERTIES1 {}
                unsafe impl ::windows::Abi for D2D1_STROKE_STYLE_PROPERTIES1 {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_STROKE_TRANSFORM_TYPE(pub u32);
                pub const D2D1_STROKE_TRANSFORM_TYPE_NORMAL: D2D1_STROKE_TRANSFORM_TYPE =
                    D2D1_STROKE_TRANSFORM_TYPE(0u32);
                pub const D2D1_STROKE_TRANSFORM_TYPE_FIXED: D2D1_STROKE_TRANSFORM_TYPE =
                    D2D1_STROKE_TRANSFORM_TYPE(1u32);
                pub const D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE: D2D1_STROKE_TRANSFORM_TYPE =
                    D2D1_STROKE_TRANSFORM_TYPE(2u32);
                pub const D2D1_STROKE_TRANSFORM_TYPE_FORCE_DWORD: D2D1_STROKE_TRANSFORM_TYPE =
                    D2D1_STROKE_TRANSFORM_TYPE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_STROKE_TRANSFORM_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_STROKE_TRANSFORM_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_STROKE_TRANSFORM_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_STROKE_TRANSFORM_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_STROKE_TRANSFORM_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_STROKE_TRANSFORM_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_TEXT_ANTIALIAS_MODE(pub u32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_DEFAULT: D2D1_TEXT_ANTIALIAS_MODE =
                    D2D1_TEXT_ANTIALIAS_MODE(0u32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE: D2D1_TEXT_ANTIALIAS_MODE =
                    D2D1_TEXT_ANTIALIAS_MODE(1u32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE: D2D1_TEXT_ANTIALIAS_MODE =
                    D2D1_TEXT_ANTIALIAS_MODE(2u32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_ALIASED: D2D1_TEXT_ANTIALIAS_MODE =
                    D2D1_TEXT_ANTIALIAS_MODE(3u32);
                pub const D2D1_TEXT_ANTIALIAS_MODE_FORCE_DWORD: D2D1_TEXT_ANTIALIAS_MODE =
                    D2D1_TEXT_ANTIALIAS_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_TEXT_ANTIALIAS_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_TEXT_ANTIALIAS_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_TEXT_ANTIALIAS_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_TEXT_ANTIALIAS_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_TEXT_ANTIALIAS_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_TEXT_ANTIALIAS_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_THREADING_MODE(pub u32);
                pub const D2D1_THREADING_MODE_SINGLE_THREADED: D2D1_THREADING_MODE =
                    D2D1_THREADING_MODE(0u32);
                pub const D2D1_THREADING_MODE_MULTI_THREADED: D2D1_THREADING_MODE =
                    D2D1_THREADING_MODE(1u32);
                pub const D2D1_THREADING_MODE_FORCE_DWORD: D2D1_THREADING_MODE =
                    D2D1_THREADING_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_THREADING_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_THREADING_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_THREADING_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_THREADING_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_THREADING_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_THREADING_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_UNIT_MODE(pub u32);
                pub const D2D1_UNIT_MODE_DIPS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(0u32);
                pub const D2D1_UNIT_MODE_PIXELS: D2D1_UNIT_MODE = D2D1_UNIT_MODE(1u32);
                pub const D2D1_UNIT_MODE_FORCE_DWORD: D2D1_UNIT_MODE =
                    D2D1_UNIT_MODE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_UNIT_MODE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_UNIT_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_UNIT_MODE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_UNIT_MODE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_UNIT_MODE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_UNIT_MODE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct D2D1_WINDOW_STATE(pub u32);
                pub const D2D1_WINDOW_STATE_NONE: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(0u32);
                pub const D2D1_WINDOW_STATE_OCCLUDED: D2D1_WINDOW_STATE = D2D1_WINDOW_STATE(1u32);
                pub const D2D1_WINDOW_STATE_FORCE_DWORD: D2D1_WINDOW_STATE =
                    D2D1_WINDOW_STATE(4294967295u32);
                impl ::std::convert::From<u32> for D2D1_WINDOW_STATE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for D2D1_WINDOW_STATE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for D2D1_WINDOW_STATE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for D2D1_WINDOW_STATE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for D2D1_WINDOW_STATE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for D2D1_WINDOW_STATE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D_POINT_2F {
                    pub x: f32,
                    pub y: f32,
                }
                impl D2D_POINT_2F {}
                impl ::std::default::Default for D2D_POINT_2F {
                    fn default() -> Self {
                        Self { x: 0.0, y: 0.0 }
                    }
                }
                impl ::std::fmt::Debug for D2D_POINT_2F {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D_POINT_2F")
                            .field("x", &self.x)
                            .field("y", &self.y)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D_POINT_2F {
                    fn eq(&self, other: &Self) -> bool {
                        self.x == other.x && self.y == other.y
                    }
                }
                impl ::std::cmp::Eq for D2D_POINT_2F {}
                unsafe impl ::windows::Abi for D2D_POINT_2F {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D_RECT_F {
                    pub left: f32,
                    pub top: f32,
                    pub right: f32,
                    pub bottom: f32,
                }
                impl D2D_RECT_F {}
                impl ::std::default::Default for D2D_RECT_F {
                    fn default() -> Self {
                        Self {
                            left: 0.0,
                            top: 0.0,
                            right: 0.0,
                            bottom: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D_RECT_F {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D_RECT_F")
                            .field("left", &self.left)
                            .field("top", &self.top)
                            .field("right", &self.right)
                            .field("bottom", &self.bottom)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D_RECT_F {
                    fn eq(&self, other: &Self) -> bool {
                        self.left == other.left
                            && self.top == other.top
                            && self.right == other.right
                            && self.bottom == other.bottom
                    }
                }
                impl ::std::cmp::Eq for D2D_RECT_F {}
                unsafe impl ::windows::Abi for D2D_RECT_F {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D_SIZE_F {
                    pub width: f32,
                    pub height: f32,
                }
                impl D2D_SIZE_F {}
                impl ::std::default::Default for D2D_SIZE_F {
                    fn default() -> Self {
                        Self {
                            width: 0.0,
                            height: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D_SIZE_F {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D_SIZE_F")
                            .field("width", &self.width)
                            .field("height", &self.height)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D_SIZE_F {
                    fn eq(&self, other: &Self) -> bool {
                        self.width == other.width && self.height == other.height
                    }
                }
                impl ::std::cmp::Eq for D2D_SIZE_F {}
                unsafe impl ::windows::Abi for D2D_SIZE_F {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct D2D_SIZE_U {
                    pub width: u32,
                    pub height: u32,
                }
                impl D2D_SIZE_U {}
                impl ::std::default::Default for D2D_SIZE_U {
                    fn default() -> Self {
                        Self {
                            width: 0,
                            height: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for D2D_SIZE_U {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("D2D_SIZE_U")
                            .field("width", &self.width)
                            .field("height", &self.height)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for D2D_SIZE_U {
                    fn eq(&self, other: &Self) -> bool {
                        self.width == other.width && self.height == other.height
                    }
                }
                impl ::std::cmp::Eq for D2D_SIZE_U {}
                unsafe impl ::windows::Abi for D2D_SIZE_U {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Bitmap(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Bitmap {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2720620631,
                        59970,
                        16537,
                        [152, 59, 83, 159, 182, 80, 84, 38],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1BitmapBrush(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1BitmapBrush {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420522,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1BitmapRenderTarget(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1BitmapRenderTarget {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420501,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Brush(::windows::IUnknown);
                impl ID2D1Brush {
                    pub unsafe fn GetFactory(
                        &self,
                        factory: *mut ::std::option::Option<ID2D1Factory>,
                    ) {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(factory),
                        )
                    }
                    pub unsafe fn SetOpacity(&self, opacity: f32) {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(opacity),
                        )
                    }
                    pub unsafe fn SetTransform(
                        &self,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn GetOpacity(&self) -> f32 {
                        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn GetTransform(
                        &self,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Brush {
                    type Vtable = ID2D1Brush_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420520,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                impl ::std::convert::From<ID2D1Brush> for ::windows::IUnknown {
                    fn from(value: ID2D1Brush) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Brush> for ::windows::IUnknown {
                    fn from(value: &ID2D1Brush) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Brush {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Brush {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1Brush> for ID2D1Resource {
                    fn from(value: ID2D1Brush) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Brush> for ID2D1Resource {
                    fn from(value: &ID2D1Brush) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for ID2D1Brush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for &'a ID2D1Brush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Brush_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        factory: *mut ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, opacity: f32),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> f32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1DCRenderTarget(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1DCRenderTarget {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        475118692,
                        56929,
                        18173,
                        [152, 153, 99, 165, 216, 240, 57, 80],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Device(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Device {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1205688157,
                        44037,
                        19677,
                        [128, 73, 155, 2, 205, 22, 244, 76],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1DrawingStateBlock(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1DrawingStateBlock {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        676359737,
                        60406,
                        18081,
                        [187, 71, 253, 133, 86, 90, 185, 87],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1DrawingStateBlock1(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1DrawingStateBlock1 {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1755258757,
                        50990,
                        20019,
                        [143, 25, 133, 117, 78, 253, 90, 206],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Effect(::windows::IUnknown);
                impl ID2D1Effect {
                    pub unsafe fn GetPropertyCount(&self) -> u32 {
                        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn GetPropertyName(
                        &self,
                        index: u32,
                        name: super::super::Foundation::PWSTR,
                        namecount: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(name),
                            ::std::mem::transmute(namecount),
                        )
                        .ok()
                    }
                    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetPropertyIndex<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                    ) -> u32 {
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                        )
                    }
                    pub unsafe fn SetValueByName<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn SetValue(
                        &self,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValueByName<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValue(
                        &self,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetSubProperties(
                        &self,
                        index: u32,
                    ) -> ::windows::Result<ID2D1Properties> {
                        let mut result__: <ID2D1Properties as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Properties>(result__)
                    }
                    pub unsafe fn SetInput<'a>(
                        &self,
                        index: u32,
                        input: impl ::windows::IntoParam<'a, ID2D1Image>,
                        invalidate: impl ::windows::IntoParam<'a, super::super::Foundation::BOOL>,
                    ) {
                        (::windows::Interface::vtable(self).14)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            input.into_param().abi(),
                            invalidate.into_param().abi(),
                        )
                    }
                    pub unsafe fn SetInputCount(&self, inputcount: u32) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).15)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(inputcount),
                        )
                        .ok()
                    }
                    pub unsafe fn GetInput(
                        &self,
                        index: u32,
                        input: *mut ::std::option::Option<ID2D1Image>,
                    ) {
                        (::windows::Interface::vtable(self).16)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(input),
                        )
                    }
                    pub unsafe fn GetInputCount(&self) -> u32 {
                        (::windows::Interface::vtable(self).17)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn GetOutput(
                        &self,
                        outputimage: *mut ::std::option::Option<ID2D1Image>,
                    ) {
                        (::windows::Interface::vtable(self).18)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(outputimage),
                        )
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Effect {
                    type Vtable = ID2D1Effect_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        673258051,
                        32137,
                        18287,
                        [129, 129, 45, 97, 89, 178, 32, 173],
                    );
                }
                impl ::std::convert::From<ID2D1Effect> for ::windows::IUnknown {
                    fn from(value: ID2D1Effect) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Effect> for ::windows::IUnknown {
                    fn from(value: &ID2D1Effect) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Effect {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Effect {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1Effect> for ID2D1Properties {
                    fn from(value: ID2D1Effect) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Effect> for ID2D1Properties {
                    fn from(value: &ID2D1Effect) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Properties> for ID2D1Effect {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Properties> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Properties>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Properties> for &'a ID2D1Effect {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Properties> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Properties>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Effect_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        name: super::super::Foundation::PWSTR,
                        namecount: u32,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, index: u32) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                    ) -> D2D1_PROPERTY_TYPE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                    ) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, index: u32) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        subproperties: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        input: ::windows::RawPtr,
                        invalidate: super::super::Foundation::BOOL,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        inputcount: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        input: *mut ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        outputimage: *mut ::windows::RawPtr,
                    ),
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1EllipseGeometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1EllipseGeometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420516,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Factory(::windows::IUnknown);
                impl ID2D1Factory {
                    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self)).ok()
                    }
                    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn CreateRectangleGeometry(
                        &self,
                        rectangle: *const D2D_RECT_F,
                    ) -> ::windows::Result<ID2D1RectangleGeometry> {
                        let mut result__: <ID2D1RectangleGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rectangle),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RectangleGeometry>(result__)
                    }
                    pub unsafe fn CreateRoundedRectangleGeometry(
                        &self,
                        roundedrectangle: *const D2D1_ROUNDED_RECT,
                    ) -> ::windows::Result<ID2D1RoundedRectangleGeometry> {
                        let mut result__: <ID2D1RoundedRectangleGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrectangle),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RoundedRectangleGeometry>(result__)
                    }
                    pub unsafe fn CreateEllipseGeometry(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                    ) -> ::windows::Result<ID2D1EllipseGeometry> {
                        let mut result__: <ID2D1EllipseGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            &mut result__,
                        )
                        .from_abi::<ID2D1EllipseGeometry>(result__)
                    }
                    pub unsafe fn CreateGeometryGroup(
                        &self,
                        fillmode: D2D1_FILL_MODE,
                        geometries: *mut ::std::option::Option<ID2D1Geometry>,
                        geometriescount: u32,
                    ) -> ::windows::Result<ID2D1GeometryGroup> {
                        let mut result__: <ID2D1GeometryGroup as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(fillmode),
                            ::std::mem::transmute(geometries),
                            ::std::mem::transmute(geometriescount),
                            &mut result__,
                        )
                        .from_abi::<ID2D1GeometryGroup>(result__)
                    }
                    pub unsafe fn CreateTransformedGeometry<'a>(
                        &self,
                        sourcegeometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) -> ::windows::Result<ID2D1TransformedGeometry> {
                        let mut result__: <ID2D1TransformedGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            sourcegeometry.into_param().abi(),
                            ::std::mem::transmute(transform),
                            &mut result__,
                        )
                        .from_abi::<ID2D1TransformedGeometry>(result__)
                    }
                    pub unsafe fn CreatePathGeometry(
                        &self,
                    ) -> ::windows::Result<ID2D1PathGeometry> {
                        let mut result__: <ID2D1PathGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ID2D1PathGeometry>(result__)
                    }
                    pub unsafe fn CreateStrokeStyle(
                        &self,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
                        dashes: *const f32,
                        dashescount: u32,
                    ) -> ::windows::Result<ID2D1StrokeStyle> {
                        let mut result__: <ID2D1StrokeStyle as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(strokestyleproperties),
                            ::std::mem::transmute(dashes),
                            ::std::mem::transmute(dashescount),
                            &mut result__,
                        )
                        .from_abi::<ID2D1StrokeStyle>(result__)
                    }
                    pub unsafe fn CreateDrawingStateBlock<'a>(
                        &self,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
                        textrenderingparams: impl ::windows::IntoParam<
                            'a,
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) -> ::windows::Result<ID2D1DrawingStateBlock> {
                        let mut result__: <ID2D1DrawingStateBlock as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(drawingstatedescription),
                            textrenderingparams.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1DrawingStateBlock>(result__)
                    }
                    pub unsafe fn CreateWicBitmapRenderTarget<'a>(
                        &self,
                        target: impl ::windows::IntoParam<'a, super::Imaging::IWICBitmap>,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1RenderTarget> {
                        let mut result__: <ID2D1RenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            target.into_param().abi(),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RenderTarget>(result__)
                    }
                    pub unsafe fn CreateHwndRenderTarget(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1HwndRenderTarget> {
                        let mut result__: <ID2D1HwndRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).14)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                            ::std::mem::transmute(hwndrendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1HwndRenderTarget>(result__)
                    }
                    pub unsafe fn CreateDxgiSurfaceRenderTarget<'a>(
                        &self,
                        dxgisurface: impl ::windows::IntoParam<'a, super::Dxgi::IDXGISurface>,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1RenderTarget> {
                        let mut result__: <ID2D1RenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).15)(
                            ::windows::Abi::abi(self),
                            dxgisurface.into_param().abi(),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RenderTarget>(result__)
                    }
                    pub unsafe fn CreateDCRenderTarget(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1DCRenderTarget> {
                        let mut result__: <ID2D1DCRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).16)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1DCRenderTarget>(result__)
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Factory {
                    type Vtable = ID2D1Factory_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        102048327,
                        28496,
                        18010,
                        [146, 69, 17, 139, 253, 59, 96, 7],
                    );
                }
                impl ::std::convert::From<ID2D1Factory> for ::windows::IUnknown {
                    fn from(value: ID2D1Factory) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Factory> for ::windows::IUnknown {
                    fn from(value: &ID2D1Factory) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Factory {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Factory {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Factory_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dpix: *mut f32,
                        dpiy: *mut f32,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rectangle: *const D2D_RECT_F,
                        rectanglegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrectangle: *const D2D1_ROUNDED_RECT,
                        roundedrectanglegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        ellipsegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        fillmode: D2D1_FILL_MODE,
                        geometries: *mut ::windows::RawPtr,
                        geometriescount: u32,
                        geometrygroup: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        sourcegeometry: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                        transformedgeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pathgeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
                        dashes: *const f32,
                        dashescount: u32,
                        strokestyle: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
                        textrenderingparams: ::windows::RawPtr,
                        drawingstateblock: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        target: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        rendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
                        hwndrendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dxgisurface: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        rendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        dcrendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Factory1(::windows::IUnknown);
                impl ID2D1Factory1 {
                    pub unsafe fn ReloadSystemMetrics(&self) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self)).ok()
                    }
                    pub unsafe fn GetDesktopDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn CreateRectangleGeometry(
                        &self,
                        rectangle: *const D2D_RECT_F,
                    ) -> ::windows::Result<ID2D1RectangleGeometry> {
                        let mut result__: <ID2D1RectangleGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rectangle),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RectangleGeometry>(result__)
                    }
                    pub unsafe fn CreateRoundedRectangleGeometry(
                        &self,
                        roundedrectangle: *const D2D1_ROUNDED_RECT,
                    ) -> ::windows::Result<ID2D1RoundedRectangleGeometry> {
                        let mut result__: <ID2D1RoundedRectangleGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrectangle),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RoundedRectangleGeometry>(result__)
                    }
                    pub unsafe fn CreateEllipseGeometry(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                    ) -> ::windows::Result<ID2D1EllipseGeometry> {
                        let mut result__: <ID2D1EllipseGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            &mut result__,
                        )
                        .from_abi::<ID2D1EllipseGeometry>(result__)
                    }
                    pub unsafe fn CreateGeometryGroup(
                        &self,
                        fillmode: D2D1_FILL_MODE,
                        geometries: *mut ::std::option::Option<ID2D1Geometry>,
                        geometriescount: u32,
                    ) -> ::windows::Result<ID2D1GeometryGroup> {
                        let mut result__: <ID2D1GeometryGroup as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(fillmode),
                            ::std::mem::transmute(geometries),
                            ::std::mem::transmute(geometriescount),
                            &mut result__,
                        )
                        .from_abi::<ID2D1GeometryGroup>(result__)
                    }
                    pub unsafe fn CreateTransformedGeometry<'a>(
                        &self,
                        sourcegeometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) -> ::windows::Result<ID2D1TransformedGeometry> {
                        let mut result__: <ID2D1TransformedGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            sourcegeometry.into_param().abi(),
                            ::std::mem::transmute(transform),
                            &mut result__,
                        )
                        .from_abi::<ID2D1TransformedGeometry>(result__)
                    }
                    pub unsafe fn CreatePathGeometry(
                        &self,
                    ) -> ::windows::Result<ID2D1PathGeometry> {
                        let mut result__: <ID2D1PathGeometry as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ID2D1PathGeometry>(result__)
                    }
                    pub unsafe fn CreateStrokeStyle(
                        &self,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
                        dashes: *const f32,
                        dashescount: u32,
                    ) -> ::windows::Result<ID2D1StrokeStyle> {
                        let mut result__: <ID2D1StrokeStyle as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(strokestyleproperties),
                            ::std::mem::transmute(dashes),
                            ::std::mem::transmute(dashescount),
                            &mut result__,
                        )
                        .from_abi::<ID2D1StrokeStyle>(result__)
                    }
                    pub unsafe fn CreateDrawingStateBlock<'a>(
                        &self,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
                        textrenderingparams: impl ::windows::IntoParam<
                            'a,
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) -> ::windows::Result<ID2D1DrawingStateBlock> {
                        let mut result__: <ID2D1DrawingStateBlock as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(drawingstatedescription),
                            textrenderingparams.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1DrawingStateBlock>(result__)
                    }
                    pub unsafe fn CreateWicBitmapRenderTarget<'a>(
                        &self,
                        target: impl ::windows::IntoParam<'a, super::Imaging::IWICBitmap>,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1RenderTarget> {
                        let mut result__: <ID2D1RenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            target.into_param().abi(),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RenderTarget>(result__)
                    }
                    pub unsafe fn CreateHwndRenderTarget(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1HwndRenderTarget> {
                        let mut result__: <ID2D1HwndRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).14)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                            ::std::mem::transmute(hwndrendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1HwndRenderTarget>(result__)
                    }
                    pub unsafe fn CreateDxgiSurfaceRenderTarget<'a>(
                        &self,
                        dxgisurface: impl ::windows::IntoParam<'a, super::Dxgi::IDXGISurface>,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1RenderTarget> {
                        let mut result__: <ID2D1RenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).15)(
                            ::windows::Abi::abi(self),
                            dxgisurface.into_param().abi(),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RenderTarget>(result__)
                    }
                    pub unsafe fn CreateDCRenderTarget(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> ::windows::Result<ID2D1DCRenderTarget> {
                        let mut result__: <ID2D1DCRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).16)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1DCRenderTarget>(result__)
                    }
                    pub unsafe fn CreateDevice<'a>(
                        &self,
                        dxgidevice: impl ::windows::IntoParam<'a, super::Dxgi::IDXGIDevice>,
                    ) -> ::windows::Result<ID2D1Device> {
                        let mut result__: <ID2D1Device as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).17)(
                            ::windows::Abi::abi(self),
                            dxgidevice.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Device>(result__)
                    }
                    pub unsafe fn CreateStrokeStyle2(
                        &self,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1,
                        dashes: *const f32,
                        dashescount: u32,
                    ) -> ::windows::Result<ID2D1StrokeStyle1> {
                        let mut result__: <ID2D1StrokeStyle1 as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).18)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(strokestyleproperties),
                            ::std::mem::transmute(dashes),
                            ::std::mem::transmute(dashescount),
                            &mut result__,
                        )
                        .from_abi::<ID2D1StrokeStyle1>(result__)
                    }
                    pub unsafe fn CreatePathGeometry2(
                        &self,
                    ) -> ::windows::Result<ID2D1PathGeometry1> {
                        let mut result__: <ID2D1PathGeometry1 as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).19)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ID2D1PathGeometry1>(result__)
                    }
                    pub unsafe fn CreateDrawingStateBlock2<'a>(
                        &self,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1,
                        textrenderingparams: impl ::windows::IntoParam<
                            'a,
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) -> ::windows::Result<ID2D1DrawingStateBlock1> {
                        let mut result__: <ID2D1DrawingStateBlock1 as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).20)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(drawingstatedescription),
                            textrenderingparams.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1DrawingStateBlock1>(result__)
                    }
                    pub unsafe fn CreateGdiMetafile<'a>(
                        &self,
                        metafilestream: impl ::windows::IntoParam<
                            'a,
                            super::super::Storage::StructuredStorage::IStream,
                        >,
                    ) -> ::windows::Result<ID2D1GdiMetafile> {
                        let mut result__: <ID2D1GdiMetafile as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).21)(
                            ::windows::Abi::abi(self),
                            metafilestream.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1GdiMetafile>(result__)
                    }
                    pub unsafe fn RegisterEffectFromStream<'a>(
                        &self,
                        classid: *const ::windows::Guid,
                        propertyxml: impl ::windows::IntoParam<
                            'a,
                            super::super::Storage::StructuredStorage::IStream,
                        >,
                        bindings: *const D2D1_PROPERTY_BINDING,
                        bindingscount: u32,
                        effectfactory: ::std::option::Option<PD2D1_EFFECT_FACTORY>,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).22)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(classid),
                            propertyxml.into_param().abi(),
                            ::std::mem::transmute(bindings),
                            ::std::mem::transmute(bindingscount),
                            ::std::mem::transmute(effectfactory),
                        )
                        .ok()
                    }
                    pub unsafe fn RegisterEffectFromString<'a>(
                        &self,
                        classid: *const ::windows::Guid,
                        propertyxml: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        bindings: *const D2D1_PROPERTY_BINDING,
                        bindingscount: u32,
                        effectfactory: ::std::option::Option<PD2D1_EFFECT_FACTORY>,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).23)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(classid),
                            propertyxml.into_param().abi(),
                            ::std::mem::transmute(bindings),
                            ::std::mem::transmute(bindingscount),
                            ::std::mem::transmute(effectfactory),
                        )
                        .ok()
                    }
                    pub unsafe fn UnregisterEffect(
                        &self,
                        classid: *const ::windows::Guid,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).24)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(classid),
                        )
                        .ok()
                    }
                    pub unsafe fn GetRegisteredEffects(
                        &self,
                        effects: *mut ::windows::Guid,
                        effectscount: u32,
                        effectsreturned: *mut u32,
                        effectsregistered: *mut u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).25)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(effects),
                            ::std::mem::transmute(effectscount),
                            ::std::mem::transmute(effectsreturned),
                            ::std::mem::transmute(effectsregistered),
                        )
                        .ok()
                    }
                    pub unsafe fn GetEffectProperties(
                        &self,
                        effectid: *const ::windows::Guid,
                    ) -> ::windows::Result<ID2D1Properties> {
                        let mut result__: <ID2D1Properties as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).26)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(effectid),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Properties>(result__)
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Factory1 {
                    type Vtable = ID2D1Factory1_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3138573154,
                        56046,
                        19354,
                        [170, 29, 20, 186, 64, 28, 250, 31],
                    );
                }
                impl ::std::convert::From<ID2D1Factory1> for ::windows::IUnknown {
                    fn from(value: ID2D1Factory1) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Factory1> for ::windows::IUnknown {
                    fn from(value: &ID2D1Factory1) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Factory1 {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Factory1 {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1Factory1> for ID2D1Factory {
                    fn from(value: ID2D1Factory1) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Factory1> for ID2D1Factory {
                    fn from(value: &ID2D1Factory1) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Factory> for ID2D1Factory1 {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Factory> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Factory>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Factory> for &'a ID2D1Factory1 {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Factory> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Factory>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Factory1_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dpix: *mut f32,
                        dpiy: *mut f32,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rectangle: *const D2D_RECT_F,
                        rectanglegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrectangle: *const D2D1_ROUNDED_RECT,
                        roundedrectanglegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        ellipsegeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        fillmode: D2D1_FILL_MODE,
                        geometries: *mut ::windows::RawPtr,
                        geometriescount: u32,
                        geometrygroup: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        sourcegeometry: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                        transformedgeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pathgeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES,
                        dashes: *const f32,
                        dashescount: u32,
                        strokestyle: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION,
                        textrenderingparams: ::windows::RawPtr,
                        drawingstateblock: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        target: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        rendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        hwndrendertargetproperties: *const D2D1_HWND_RENDER_TARGET_PROPERTIES,
                        hwndrendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dxgisurface: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        rendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                        dcrendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dxgidevice: ::windows::RawPtr,
                        d2ddevice: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        strokestyleproperties: *const D2D1_STROKE_STYLE_PROPERTIES1,
                        dashes: *const f32,
                        dashescount: u32,
                        strokestyle: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pathgeometry: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstatedescription: *const D2D1_DRAWING_STATE_DESCRIPTION1,
                        textrenderingparams: ::windows::RawPtr,
                        drawingstateblock: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        metafilestream: ::windows::RawPtr,
                        metafile: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        classid: *const ::windows::Guid,
                        propertyxml: ::windows::RawPtr,
                        bindings: *const D2D1_PROPERTY_BINDING_abi,
                        bindingscount: u32,
                        effectfactory: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        classid: *const ::windows::Guid,
                        propertyxml: super::super::Foundation::PWSTR,
                        bindings: *const D2D1_PROPERTY_BINDING_abi,
                        bindingscount: u32,
                        effectfactory: ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        classid: *const ::windows::Guid,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        effects: *mut ::windows::Guid,
                        effectscount: u32,
                        effectsreturned: *mut u32,
                        effectsregistered: *mut u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        effectid: *const ::windows::Guid,
                        properties: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1GdiMetafile(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1GdiMetafile {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        794049987,
                        53185,
                        16913,
                        [134, 79, 207, 217, 28, 111, 51, 149],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Geometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Geometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420513,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1GeometryGroup(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1GeometryGroup {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420518,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1GradientStopCollection(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1GradientStopCollection {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420519,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1HwndRenderTarget(::windows::IUnknown);
                impl ID2D1HwndRenderTarget {
                    pub unsafe fn GetFactory(
                        &self,
                        factory: *mut ::std::option::Option<ID2D1Factory>,
                    ) {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(factory),
                        )
                    }
                    pub unsafe fn CreateBitmap<'a>(
                        &self,
                        size: impl ::windows::IntoParam<'a, D2D_SIZE_U>,
                        srcdata: *const ::std::ffi::c_void,
                        pitch: u32,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                    ) -> ::windows::Result<ID2D1Bitmap> {
                        let mut result__: <ID2D1Bitmap as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            size.into_param().abi(),
                            ::std::mem::transmute(srcdata),
                            ::std::mem::transmute(pitch),
                            ::std::mem::transmute(bitmapproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Bitmap>(result__)
                    }
                    pub unsafe fn CreateBitmapFromWicBitmap<'a>(
                        &self,
                        wicbitmapsource: impl ::windows::IntoParam<'a, super::Imaging::IWICBitmapSource>,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                    ) -> ::windows::Result<ID2D1Bitmap> {
                        let mut result__: <ID2D1Bitmap as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            wicbitmapsource.into_param().abi(),
                            ::std::mem::transmute(bitmapproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Bitmap>(result__)
                    }
                    pub unsafe fn CreateSharedBitmap(
                        &self,
                        riid: *const ::windows::Guid,
                        data: *mut ::std::ffi::c_void,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::std::option::Option<ID2D1Bitmap>,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(riid),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(bitmapproperties),
                            ::std::mem::transmute(bitmap),
                        )
                        .ok()
                    }
                    pub unsafe fn CreateBitmapBrush<'a>(
                        &self,
                        bitmap: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                    ) -> ::windows::Result<ID2D1BitmapBrush> {
                        let mut result__: <ID2D1BitmapBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            bitmap.into_param().abi(),
                            ::std::mem::transmute(bitmapbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1BitmapBrush>(result__)
                    }
                    pub unsafe fn CreateSolidColorBrush(
                        &self,
                        color: *const D2D1_COLOR_F,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                    ) -> ::windows::Result<ID2D1SolidColorBrush> {
                        let mut result__: <ID2D1SolidColorBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(color),
                            ::std::mem::transmute(brushproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1SolidColorBrush>(result__)
                    }
                    pub unsafe fn CreateGradientStopCollection(
                        &self,
                        gradientstops: *const D2D1_GRADIENT_STOP,
                        gradientstopscount: u32,
                        colorinterpolationgamma: D2D1_GAMMA,
                        extendmode: D2D1_EXTEND_MODE,
                    ) -> ::windows::Result<ID2D1GradientStopCollection> {
                        let mut result__: <ID2D1GradientStopCollection as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(gradientstops),
                            ::std::mem::transmute(gradientstopscount),
                            ::std::mem::transmute(colorinterpolationgamma),
                            ::std::mem::transmute(extendmode),
                            &mut result__,
                        )
                        .from_abi::<ID2D1GradientStopCollection>(result__)
                    }
                    pub unsafe fn CreateLinearGradientBrush<'a>(
                        &self,
                        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: impl ::windows::IntoParam<
                            'a,
                            ID2D1GradientStopCollection,
                        >,
                    ) -> ::windows::Result<ID2D1LinearGradientBrush> {
                        let mut result__: <ID2D1LinearGradientBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(lineargradientbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            gradientstopcollection.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1LinearGradientBrush>(result__)
                    }
                    pub unsafe fn CreateRadialGradientBrush<'a>(
                        &self,
                        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: impl ::windows::IntoParam<
                            'a,
                            ID2D1GradientStopCollection,
                        >,
                    ) -> ::windows::Result<ID2D1RadialGradientBrush> {
                        let mut result__: <ID2D1RadialGradientBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(radialgradientbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            gradientstopcollection.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RadialGradientBrush>(result__)
                    }
                    pub unsafe fn CreateCompatibleRenderTarget(
                        &self,
                        desiredsize: *const D2D_SIZE_F,
                        desiredpixelsize: *const D2D_SIZE_U,
                        desiredformat: *const D2D1_PIXEL_FORMAT,
                        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
                    ) -> ::windows::Result<ID2D1BitmapRenderTarget> {
                        let mut result__: <ID2D1BitmapRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(desiredsize),
                            ::std::mem::transmute(desiredpixelsize),
                            ::std::mem::transmute(desiredformat),
                            ::std::mem::transmute(options),
                            &mut result__,
                        )
                        .from_abi::<ID2D1BitmapRenderTarget>(result__)
                    }
                    pub unsafe fn CreateLayer(
                        &self,
                        size: *const D2D_SIZE_F,
                    ) -> ::windows::Result<ID2D1Layer> {
                        let mut result__: <ID2D1Layer as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(size),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Layer>(result__)
                    }
                    pub unsafe fn CreateMesh(&self) -> ::windows::Result<ID2D1Mesh> {
                        let mut result__: <ID2D1Mesh as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).14)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Mesh>(result__)
                    }
                    pub unsafe fn DrawLine<'a>(
                        &self,
                        point0: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        point1: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).15)(
                            ::windows::Abi::abi(self),
                            point0.into_param().abi(),
                            point1.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawRectangle<'a>(
                        &self,
                        rect: *const D2D_RECT_F,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).16)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rect),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillRectangle<'a>(
                        &self,
                        rect: *const D2D_RECT_F,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).17)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rect),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawRoundedRectangle<'a>(
                        &self,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).18)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrect),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillRoundedRectangle<'a>(
                        &self,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).19)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrect),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawEllipse<'a>(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).20)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillEllipse<'a>(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).21)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawGeometry<'a>(
                        &self,
                        geometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).22)(
                            ::windows::Abi::abi(self),
                            geometry.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillGeometry<'a>(
                        &self,
                        geometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        opacitybrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).23)(
                            ::windows::Abi::abi(self),
                            geometry.into_param().abi(),
                            brush.into_param().abi(),
                            opacitybrush.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillMesh<'a>(
                        &self,
                        mesh: impl ::windows::IntoParam<'a, ID2D1Mesh>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).24)(
                            ::windows::Abi::abi(self),
                            mesh.into_param().abi(),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillOpacityMask<'a>(
                        &self,
                        opacitymask: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        content: D2D1_OPACITY_MASK_CONTENT,
                        destinationrectangle: *const D2D_RECT_F,
                        sourcerectangle: *const D2D_RECT_F,
                    ) {
                        (::windows::Interface::vtable(self).25)(
                            ::windows::Abi::abi(self),
                            opacitymask.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(content),
                            ::std::mem::transmute(destinationrectangle),
                            ::std::mem::transmute(sourcerectangle),
                        )
                    }
                    pub unsafe fn DrawBitmap<'a>(
                        &self,
                        bitmap: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        destinationrectangle: *const D2D_RECT_F,
                        opacity: f32,
                        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
                        sourcerectangle: *const D2D_RECT_F,
                    ) {
                        (::windows::Interface::vtable(self).26)(
                            ::windows::Abi::abi(self),
                            bitmap.into_param().abi(),
                            ::std::mem::transmute(destinationrectangle),
                            ::std::mem::transmute(opacity),
                            ::std::mem::transmute(interpolationmode),
                            ::std::mem::transmute(sourcerectangle),
                        )
                    }
                    pub unsafe fn DrawText<'a>(
                        &self,
                        string: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        stringlength: u32,
                        textformat: impl ::windows::IntoParam<'a, super::DirectWrite::IDWriteTextFormat>,
                        layoutrect: *const D2D_RECT_F,
                        defaultfillbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ) {
                        (::windows::Interface::vtable(self).27)(
                            ::windows::Abi::abi(self),
                            string.into_param().abi(),
                            ::std::mem::transmute(stringlength),
                            textformat.into_param().abi(),
                            ::std::mem::transmute(layoutrect),
                            defaultfillbrush.into_param().abi(),
                            ::std::mem::transmute(options),
                            ::std::mem::transmute(measuringmode),
                        )
                    }
                    pub unsafe fn DrawTextLayout<'a>(
                        &self,
                        origin: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        textlayout: impl ::windows::IntoParam<'a, super::DirectWrite::IDWriteTextLayout>,
                        defaultfillbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                    ) {
                        (::windows::Interface::vtable(self).28)(
                            ::windows::Abi::abi(self),
                            origin.into_param().abi(),
                            textlayout.into_param().abi(),
                            defaultfillbrush.into_param().abi(),
                            ::std::mem::transmute(options),
                        )
                    }
                    pub unsafe fn DrawGlyphRun<'a>(
                        &self,
                        baselineorigin: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN,
                        foregroundbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ) {
                        (::windows::Interface::vtable(self).29)(
                            ::windows::Abi::abi(self),
                            baselineorigin.into_param().abi(),
                            ::std::mem::transmute(glyphrun),
                            foregroundbrush.into_param().abi(),
                            ::std::mem::transmute(measuringmode),
                        )
                    }
                    pub unsafe fn SetTransform(
                        &self,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).30)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn GetTransform(
                        &self,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).31)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
                        (::windows::Interface::vtable(self).32)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(antialiasmode),
                        )
                    }
                    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
                        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn SetTextAntialiasMode(
                        &self,
                        textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE,
                    ) {
                        (::windows::Interface::vtable(self).34)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(textantialiasmode),
                        )
                    }
                    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
                        (::windows::Interface::vtable(self).35)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn SetTextRenderingParams<'a>(
                        &self,
                        textrenderingparams: impl ::windows::IntoParam<
                            'a,
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) {
                        (::windows::Interface::vtable(self).36)(
                            ::windows::Abi::abi(self),
                            textrenderingparams.into_param().abi(),
                        )
                    }
                    pub unsafe fn GetTextRenderingParams(
                        &self,
                        textrenderingparams: *mut ::std::option::Option<
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) {
                        (::windows::Interface::vtable(self).37)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(textrenderingparams),
                        )
                    }
                    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
                        (::windows::Interface::vtable(self).38)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                    }
                    pub unsafe fn GetTags(&self, tag1: *mut u64, tag2: *mut u64) {
                        (::windows::Interface::vtable(self).39)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                    }
                    pub unsafe fn PushLayer<'a>(
                        &self,
                        layerparameters: *const D2D1_LAYER_PARAMETERS,
                        layer: impl ::windows::IntoParam<'a, ID2D1Layer>,
                    ) {
                        (::windows::Interface::vtable(self).40)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(layerparameters),
                            layer.into_param().abi(),
                        )
                    }
                    pub unsafe fn PopLayer(&self) {
                        (::windows::Interface::vtable(self).41)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn Flush(
                        &self,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).42)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                        .ok()
                    }
                    pub unsafe fn SaveDrawingState<'a>(
                        &self,
                        drawingstateblock: impl ::windows::IntoParam<'a, ID2D1DrawingStateBlock>,
                    ) {
                        (::windows::Interface::vtable(self).43)(
                            ::windows::Abi::abi(self),
                            drawingstateblock.into_param().abi(),
                        )
                    }
                    pub unsafe fn RestoreDrawingState<'a>(
                        &self,
                        drawingstateblock: impl ::windows::IntoParam<'a, ID2D1DrawingStateBlock>,
                    ) {
                        (::windows::Interface::vtable(self).44)(
                            ::windows::Abi::abi(self),
                            drawingstateblock.into_param().abi(),
                        )
                    }
                    pub unsafe fn PushAxisAlignedClip(
                        &self,
                        cliprect: *const D2D_RECT_F,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ) {
                        (::windows::Interface::vtable(self).45)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(cliprect),
                            ::std::mem::transmute(antialiasmode),
                        )
                    }
                    pub unsafe fn PopAxisAlignedClip(&self) {
                        (::windows::Interface::vtable(self).46)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn Clear(&self, clearcolor: *const D2D1_COLOR_F) {
                        (::windows::Interface::vtable(self).47)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(clearcolor),
                        )
                    }
                    pub unsafe fn BeginDraw(&self) {
                        (::windows::Interface::vtable(self).48)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn EndDraw(
                        &self,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).49)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                        .ok()
                    }
                    pub unsafe fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT {
                        let mut result__: D2D1_PIXEL_FORMAT = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).50)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
                        (::windows::Interface::vtable(self).51)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
                        (::windows::Interface::vtable(self).52)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn GetSize(&self) -> D2D_SIZE_F {
                        let mut result__: D2D_SIZE_F = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).53)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn GetPixelSize(&self) -> D2D_SIZE_U {
                        let mut result__: D2D_SIZE_U = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).54)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
                        (::windows::Interface::vtable(self).55)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn IsSupported(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> super::super::Foundation::BOOL {
                        (::windows::Interface::vtable(self).56)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                        )
                    }
                    pub unsafe fn CheckWindowState(&self) -> D2D1_WINDOW_STATE {
                        (::windows::Interface::vtable(self).57)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn Resize(
                        &self,
                        pixelsize: *const D2D_SIZE_U,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).58)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(pixelsize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetHwnd(&self) -> super::super::Foundation::HWND {
                        (::windows::Interface::vtable(self).59)(::windows::Abi::abi(self))
                    }
                }
                unsafe impl ::windows::Interface for ID2D1HwndRenderTarget {
                    type Vtable = ID2D1HwndRenderTarget_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420504,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                impl ::std::convert::From<ID2D1HwndRenderTarget> for ::windows::IUnknown {
                    fn from(value: ID2D1HwndRenderTarget) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1HwndRenderTarget> for ::windows::IUnknown {
                    fn from(value: &ID2D1HwndRenderTarget) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1HwndRenderTarget> for ID2D1RenderTarget {
                    fn from(value: ID2D1HwndRenderTarget) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1HwndRenderTarget> for ID2D1RenderTarget {
                    fn from(value: &ID2D1HwndRenderTarget) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1RenderTarget> for ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1RenderTarget> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1RenderTarget>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1RenderTarget> for &'a ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1RenderTarget> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1RenderTarget>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1HwndRenderTarget> for ID2D1Resource {
                    fn from(value: ID2D1HwndRenderTarget) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1HwndRenderTarget> for ID2D1Resource {
                    fn from(value: &ID2D1HwndRenderTarget) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for &'a ID2D1HwndRenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1HwndRenderTarget_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        factory: *mut ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        size: D2D_SIZE_U,
                        srcdata: *const ::std::ffi::c_void,
                        pitch: u32,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        wicbitmapsource: ::windows::RawPtr,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        riid: *const ::windows::Guid,
                        data: *mut ::std::ffi::c_void,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        bitmap: ::windows::RawPtr,
                        bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        bitmapbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        color: *const D2D1_COLOR_F,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        solidcolorbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        gradientstops: *const D2D1_GRADIENT_STOP,
                        gradientstopscount: u32,
                        colorinterpolationgamma: D2D1_GAMMA,
                        extendmode: D2D1_EXTEND_MODE,
                        gradientstopcollection: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: ::windows::RawPtr,
                        lineargradientbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: ::windows::RawPtr,
                        radialgradientbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        desiredsize: *const D2D_SIZE_F,
                        desiredpixelsize: *const D2D_SIZE_U,
                        desiredformat: *const D2D1_PIXEL_FORMAT,
                        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
                        bitmaprendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        size: *const D2D_SIZE_F,
                        layer: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        mesh: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        point0: D2D_POINT_2F,
                        point1: D2D_POINT_2F,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rect: *const D2D_RECT_F,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rect: *const D2D_RECT_F,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        geometry: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        geometry: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        opacitybrush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        mesh: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        opacitymask: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        content: D2D1_OPACITY_MASK_CONTENT,
                        destinationrectangle: *const D2D_RECT_F,
                        sourcerectangle: *const D2D_RECT_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        bitmap: ::windows::RawPtr,
                        destinationrectangle: *const D2D_RECT_F,
                        opacity: f32,
                        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
                        sourcerectangle: *const D2D_RECT_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        string: super::super::Foundation::PWSTR,
                        stringlength: u32,
                        textformat: ::windows::RawPtr,
                        layoutrect: *const D2D_RECT_F,
                        defaultfillbrush: ::windows::RawPtr,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        origin: D2D_POINT_2F,
                        textlayout: ::windows::RawPtr,
                        defaultfillbrush: ::windows::RawPtr,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        baselineorigin: D2D_POINT_2F,
                        glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN_abi,
                        foregroundbrush: ::windows::RawPtr,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D2D1_ANTIALIAS_MODE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                    ) -> D2D1_TEXT_ANTIALIAS_MODE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textrenderingparams: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textrenderingparams: *mut ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, tag1: u64, tag2: u64),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        layerparameters: *const D2D1_LAYER_PARAMETERS_abi,
                        layer: ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstateblock: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstateblock: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        cliprect: *const D2D_RECT_F,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        clearcolor: *const D2D1_COLOR_F,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D1_PIXEL_FORMAT,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, dpix: f32, dpiy: f32),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dpix: *mut f32,
                        dpiy: *mut f32,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D_SIZE_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D_SIZE_U,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    )
                        -> super::super::Foundation::BOOL,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D2D1_WINDOW_STATE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        pixelsize: *const D2D_SIZE_U,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                    )
                        -> super::super::Foundation::HWND,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Image(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Image {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1694605173,
                        36258,
                        18812,
                        [179, 44, 223, 163, 78, 72, 237, 230],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Layer(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Layer {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420507,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1LinearGradientBrush(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1LinearGradientBrush {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420523,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1Mesh(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1Mesh {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420546,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1PathGeometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1PathGeometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420517,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1PathGeometry1(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1PathGeometry1 {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1656398546,
                        43860,
                        16823,
                        [184, 114, 120, 126, 1, 6, 164, 33],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Properties(::windows::IUnknown);
                impl ID2D1Properties {
                    pub unsafe fn GetPropertyCount(&self) -> u32 {
                        (::windows::Interface::vtable(self).3)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn GetPropertyName(
                        &self,
                        index: u32,
                        name: super::super::Foundation::PWSTR,
                        namecount: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(name),
                            ::std::mem::transmute(namecount),
                        )
                        .ok()
                    }
                    pub unsafe fn GetPropertyNameLength(&self, index: u32) -> u32 {
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetType(&self, index: u32) -> D2D1_PROPERTY_TYPE {
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetPropertyIndex<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                    ) -> u32 {
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                        )
                    }
                    pub unsafe fn SetValueByName<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn SetValue(
                        &self,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValueByName<'a>(
                        &self,
                        name: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            name.into_param().abi(),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValue(
                        &self,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            ::std::mem::transmute(r#type),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(datasize),
                        )
                        .ok()
                    }
                    pub unsafe fn GetValueSize(&self, index: u32) -> u32 {
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                        )
                    }
                    pub unsafe fn GetSubProperties(
                        &self,
                        index: u32,
                    ) -> ::windows::Result<ID2D1Properties> {
                        let mut result__: <ID2D1Properties as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(index),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Properties>(result__)
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Properties {
                    type Vtable = ID2D1Properties_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1211397079,
                        52550,
                        20381,
                        [157, 58, 49, 18, 170, 128, 21, 157],
                    );
                }
                impl ::std::convert::From<ID2D1Properties> for ::windows::IUnknown {
                    fn from(value: ID2D1Properties) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Properties> for ::windows::IUnknown {
                    fn from(value: &ID2D1Properties) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Properties {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Properties {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Properties_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        name: super::super::Foundation::PWSTR,
                        namecount: u32,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, index: u32) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                    ) -> D2D1_PROPERTY_TYPE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                    ) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        name: super::super::Foundation::PWSTR,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        r#type: D2D1_PROPERTY_TYPE,
                        data: *mut u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, index: u32) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        index: u32,
                        subproperties: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1RadialGradientBrush(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1RadialGradientBrush {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420524,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1RectangleGeometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1RectangleGeometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420514,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1RenderTarget(::windows::IUnknown);
                impl ID2D1RenderTarget {
                    pub unsafe fn GetFactory(
                        &self,
                        factory: *mut ::std::option::Option<ID2D1Factory>,
                    ) {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(factory),
                        )
                    }
                    pub unsafe fn CreateBitmap<'a>(
                        &self,
                        size: impl ::windows::IntoParam<'a, D2D_SIZE_U>,
                        srcdata: *const ::std::ffi::c_void,
                        pitch: u32,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                    ) -> ::windows::Result<ID2D1Bitmap> {
                        let mut result__: <ID2D1Bitmap as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            size.into_param().abi(),
                            ::std::mem::transmute(srcdata),
                            ::std::mem::transmute(pitch),
                            ::std::mem::transmute(bitmapproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Bitmap>(result__)
                    }
                    pub unsafe fn CreateBitmapFromWicBitmap<'a>(
                        &self,
                        wicbitmapsource: impl ::windows::IntoParam<'a, super::Imaging::IWICBitmapSource>,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                    ) -> ::windows::Result<ID2D1Bitmap> {
                        let mut result__: <ID2D1Bitmap as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            wicbitmapsource.into_param().abi(),
                            ::std::mem::transmute(bitmapproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Bitmap>(result__)
                    }
                    pub unsafe fn CreateSharedBitmap(
                        &self,
                        riid: *const ::windows::Guid,
                        data: *mut ::std::ffi::c_void,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::std::option::Option<ID2D1Bitmap>,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).6)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(riid),
                            ::std::mem::transmute(data),
                            ::std::mem::transmute(bitmapproperties),
                            ::std::mem::transmute(bitmap),
                        )
                        .ok()
                    }
                    pub unsafe fn CreateBitmapBrush<'a>(
                        &self,
                        bitmap: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                    ) -> ::windows::Result<ID2D1BitmapBrush> {
                        let mut result__: <ID2D1BitmapBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            bitmap.into_param().abi(),
                            ::std::mem::transmute(bitmapbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1BitmapBrush>(result__)
                    }
                    pub unsafe fn CreateSolidColorBrush(
                        &self,
                        color: *const D2D1_COLOR_F,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                    ) -> ::windows::Result<ID2D1SolidColorBrush> {
                        let mut result__: <ID2D1SolidColorBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(color),
                            ::std::mem::transmute(brushproperties),
                            &mut result__,
                        )
                        .from_abi::<ID2D1SolidColorBrush>(result__)
                    }
                    pub unsafe fn CreateGradientStopCollection(
                        &self,
                        gradientstops: *const D2D1_GRADIENT_STOP,
                        gradientstopscount: u32,
                        colorinterpolationgamma: D2D1_GAMMA,
                        extendmode: D2D1_EXTEND_MODE,
                    ) -> ::windows::Result<ID2D1GradientStopCollection> {
                        let mut result__: <ID2D1GradientStopCollection as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(gradientstops),
                            ::std::mem::transmute(gradientstopscount),
                            ::std::mem::transmute(colorinterpolationgamma),
                            ::std::mem::transmute(extendmode),
                            &mut result__,
                        )
                        .from_abi::<ID2D1GradientStopCollection>(result__)
                    }
                    pub unsafe fn CreateLinearGradientBrush<'a>(
                        &self,
                        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: impl ::windows::IntoParam<
                            'a,
                            ID2D1GradientStopCollection,
                        >,
                    ) -> ::windows::Result<ID2D1LinearGradientBrush> {
                        let mut result__: <ID2D1LinearGradientBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).10)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(lineargradientbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            gradientstopcollection.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1LinearGradientBrush>(result__)
                    }
                    pub unsafe fn CreateRadialGradientBrush<'a>(
                        &self,
                        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: impl ::windows::IntoParam<
                            'a,
                            ID2D1GradientStopCollection,
                        >,
                    ) -> ::windows::Result<ID2D1RadialGradientBrush> {
                        let mut result__: <ID2D1RadialGradientBrush as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).11)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(radialgradientbrushproperties),
                            ::std::mem::transmute(brushproperties),
                            gradientstopcollection.into_param().abi(),
                            &mut result__,
                        )
                        .from_abi::<ID2D1RadialGradientBrush>(result__)
                    }
                    pub unsafe fn CreateCompatibleRenderTarget(
                        &self,
                        desiredsize: *const D2D_SIZE_F,
                        desiredpixelsize: *const D2D_SIZE_U,
                        desiredformat: *const D2D1_PIXEL_FORMAT,
                        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
                    ) -> ::windows::Result<ID2D1BitmapRenderTarget> {
                        let mut result__: <ID2D1BitmapRenderTarget as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).12)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(desiredsize),
                            ::std::mem::transmute(desiredpixelsize),
                            ::std::mem::transmute(desiredformat),
                            ::std::mem::transmute(options),
                            &mut result__,
                        )
                        .from_abi::<ID2D1BitmapRenderTarget>(result__)
                    }
                    pub unsafe fn CreateLayer(
                        &self,
                        size: *const D2D_SIZE_F,
                    ) -> ::windows::Result<ID2D1Layer> {
                        let mut result__: <ID2D1Layer as ::windows::Abi>::Abi =
                            ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).13)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(size),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Layer>(result__)
                    }
                    pub unsafe fn CreateMesh(&self) -> ::windows::Result<ID2D1Mesh> {
                        let mut result__: <ID2D1Mesh as ::windows::Abi>::Abi = ::std::mem::zeroed();
                        (::windows::Interface::vtable(self).14)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        )
                        .from_abi::<ID2D1Mesh>(result__)
                    }
                    pub unsafe fn DrawLine<'a>(
                        &self,
                        point0: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        point1: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).15)(
                            ::windows::Abi::abi(self),
                            point0.into_param().abi(),
                            point1.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawRectangle<'a>(
                        &self,
                        rect: *const D2D_RECT_F,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).16)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rect),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillRectangle<'a>(
                        &self,
                        rect: *const D2D_RECT_F,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).17)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rect),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawRoundedRectangle<'a>(
                        &self,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).18)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrect),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillRoundedRectangle<'a>(
                        &self,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).19)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(roundedrect),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawEllipse<'a>(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).20)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillEllipse<'a>(
                        &self,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).21)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(ellipse),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn DrawGeometry<'a>(
                        &self,
                        geometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        strokewidth: f32,
                        strokestyle: impl ::windows::IntoParam<'a, ID2D1StrokeStyle>,
                    ) {
                        (::windows::Interface::vtable(self).22)(
                            ::windows::Abi::abi(self),
                            geometry.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(strokewidth),
                            strokestyle.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillGeometry<'a>(
                        &self,
                        geometry: impl ::windows::IntoParam<'a, ID2D1Geometry>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        opacitybrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).23)(
                            ::windows::Abi::abi(self),
                            geometry.into_param().abi(),
                            brush.into_param().abi(),
                            opacitybrush.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillMesh<'a>(
                        &self,
                        mesh: impl ::windows::IntoParam<'a, ID2D1Mesh>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                    ) {
                        (::windows::Interface::vtable(self).24)(
                            ::windows::Abi::abi(self),
                            mesh.into_param().abi(),
                            brush.into_param().abi(),
                        )
                    }
                    pub unsafe fn FillOpacityMask<'a>(
                        &self,
                        opacitymask: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        brush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        content: D2D1_OPACITY_MASK_CONTENT,
                        destinationrectangle: *const D2D_RECT_F,
                        sourcerectangle: *const D2D_RECT_F,
                    ) {
                        (::windows::Interface::vtable(self).25)(
                            ::windows::Abi::abi(self),
                            opacitymask.into_param().abi(),
                            brush.into_param().abi(),
                            ::std::mem::transmute(content),
                            ::std::mem::transmute(destinationrectangle),
                            ::std::mem::transmute(sourcerectangle),
                        )
                    }
                    pub unsafe fn DrawBitmap<'a>(
                        &self,
                        bitmap: impl ::windows::IntoParam<'a, ID2D1Bitmap>,
                        destinationrectangle: *const D2D_RECT_F,
                        opacity: f32,
                        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
                        sourcerectangle: *const D2D_RECT_F,
                    ) {
                        (::windows::Interface::vtable(self).26)(
                            ::windows::Abi::abi(self),
                            bitmap.into_param().abi(),
                            ::std::mem::transmute(destinationrectangle),
                            ::std::mem::transmute(opacity),
                            ::std::mem::transmute(interpolationmode),
                            ::std::mem::transmute(sourcerectangle),
                        )
                    }
                    pub unsafe fn DrawText<'a>(
                        &self,
                        string: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                        stringlength: u32,
                        textformat: impl ::windows::IntoParam<'a, super::DirectWrite::IDWriteTextFormat>,
                        layoutrect: *const D2D_RECT_F,
                        defaultfillbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ) {
                        (::windows::Interface::vtable(self).27)(
                            ::windows::Abi::abi(self),
                            string.into_param().abi(),
                            ::std::mem::transmute(stringlength),
                            textformat.into_param().abi(),
                            ::std::mem::transmute(layoutrect),
                            defaultfillbrush.into_param().abi(),
                            ::std::mem::transmute(options),
                            ::std::mem::transmute(measuringmode),
                        )
                    }
                    pub unsafe fn DrawTextLayout<'a>(
                        &self,
                        origin: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        textlayout: impl ::windows::IntoParam<'a, super::DirectWrite::IDWriteTextLayout>,
                        defaultfillbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                    ) {
                        (::windows::Interface::vtable(self).28)(
                            ::windows::Abi::abi(self),
                            origin.into_param().abi(),
                            textlayout.into_param().abi(),
                            defaultfillbrush.into_param().abi(),
                            ::std::mem::transmute(options),
                        )
                    }
                    pub unsafe fn DrawGlyphRun<'a>(
                        &self,
                        baselineorigin: impl ::windows::IntoParam<'a, D2D_POINT_2F>,
                        glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN,
                        foregroundbrush: impl ::windows::IntoParam<'a, ID2D1Brush>,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ) {
                        (::windows::Interface::vtable(self).29)(
                            ::windows::Abi::abi(self),
                            baselineorigin.into_param().abi(),
                            ::std::mem::transmute(glyphrun),
                            foregroundbrush.into_param().abi(),
                            ::std::mem::transmute(measuringmode),
                        )
                    }
                    pub unsafe fn SetTransform(
                        &self,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).30)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn GetTransform(
                        &self,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).31)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn SetAntialiasMode(&self, antialiasmode: D2D1_ANTIALIAS_MODE) {
                        (::windows::Interface::vtable(self).32)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(antialiasmode),
                        )
                    }
                    pub unsafe fn GetAntialiasMode(&self) -> D2D1_ANTIALIAS_MODE {
                        (::windows::Interface::vtable(self).33)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn SetTextAntialiasMode(
                        &self,
                        textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE,
                    ) {
                        (::windows::Interface::vtable(self).34)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(textantialiasmode),
                        )
                    }
                    pub unsafe fn GetTextAntialiasMode(&self) -> D2D1_TEXT_ANTIALIAS_MODE {
                        (::windows::Interface::vtable(self).35)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn SetTextRenderingParams<'a>(
                        &self,
                        textrenderingparams: impl ::windows::IntoParam<
                            'a,
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) {
                        (::windows::Interface::vtable(self).36)(
                            ::windows::Abi::abi(self),
                            textrenderingparams.into_param().abi(),
                        )
                    }
                    pub unsafe fn GetTextRenderingParams(
                        &self,
                        textrenderingparams: *mut ::std::option::Option<
                            super::DirectWrite::IDWriteRenderingParams,
                        >,
                    ) {
                        (::windows::Interface::vtable(self).37)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(textrenderingparams),
                        )
                    }
                    pub unsafe fn SetTags(&self, tag1: u64, tag2: u64) {
                        (::windows::Interface::vtable(self).38)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                    }
                    pub unsafe fn GetTags(&self, tag1: *mut u64, tag2: *mut u64) {
                        (::windows::Interface::vtable(self).39)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                    }
                    pub unsafe fn PushLayer<'a>(
                        &self,
                        layerparameters: *const D2D1_LAYER_PARAMETERS,
                        layer: impl ::windows::IntoParam<'a, ID2D1Layer>,
                    ) {
                        (::windows::Interface::vtable(self).40)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(layerparameters),
                            layer.into_param().abi(),
                        )
                    }
                    pub unsafe fn PopLayer(&self) {
                        (::windows::Interface::vtable(self).41)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn Flush(
                        &self,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).42)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                        .ok()
                    }
                    pub unsafe fn SaveDrawingState<'a>(
                        &self,
                        drawingstateblock: impl ::windows::IntoParam<'a, ID2D1DrawingStateBlock>,
                    ) {
                        (::windows::Interface::vtable(self).43)(
                            ::windows::Abi::abi(self),
                            drawingstateblock.into_param().abi(),
                        )
                    }
                    pub unsafe fn RestoreDrawingState<'a>(
                        &self,
                        drawingstateblock: impl ::windows::IntoParam<'a, ID2D1DrawingStateBlock>,
                    ) {
                        (::windows::Interface::vtable(self).44)(
                            ::windows::Abi::abi(self),
                            drawingstateblock.into_param().abi(),
                        )
                    }
                    pub unsafe fn PushAxisAlignedClip(
                        &self,
                        cliprect: *const D2D_RECT_F,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ) {
                        (::windows::Interface::vtable(self).45)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(cliprect),
                            ::std::mem::transmute(antialiasmode),
                        )
                    }
                    pub unsafe fn PopAxisAlignedClip(&self) {
                        (::windows::Interface::vtable(self).46)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn Clear(&self, clearcolor: *const D2D1_COLOR_F) {
                        (::windows::Interface::vtable(self).47)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(clearcolor),
                        )
                    }
                    pub unsafe fn BeginDraw(&self) {
                        (::windows::Interface::vtable(self).48)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn EndDraw(
                        &self,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::Result<()> {
                        (::windows::Interface::vtable(self).49)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(tag1),
                            ::std::mem::transmute(tag2),
                        )
                        .ok()
                    }
                    pub unsafe fn GetPixelFormat(&self) -> D2D1_PIXEL_FORMAT {
                        let mut result__: D2D1_PIXEL_FORMAT = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).50)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn SetDpi(&self, dpix: f32, dpiy: f32) {
                        (::windows::Interface::vtable(self).51)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn GetDpi(&self, dpix: *mut f32, dpiy: *mut f32) {
                        (::windows::Interface::vtable(self).52)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(dpix),
                            ::std::mem::transmute(dpiy),
                        )
                    }
                    pub unsafe fn GetSize(&self) -> D2D_SIZE_F {
                        let mut result__: D2D_SIZE_F = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).53)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn GetPixelSize(&self) -> D2D_SIZE_U {
                        let mut result__: D2D_SIZE_U = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).54)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                    pub unsafe fn GetMaximumBitmapSize(&self) -> u32 {
                        (::windows::Interface::vtable(self).55)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn IsSupported(
                        &self,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    ) -> super::super::Foundation::BOOL {
                        (::windows::Interface::vtable(self).56)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(rendertargetproperties),
                        )
                    }
                }
                unsafe impl ::windows::Interface for ID2D1RenderTarget {
                    type Vtable = ID2D1RenderTarget_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420500,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                impl ::std::convert::From<ID2D1RenderTarget> for ::windows::IUnknown {
                    fn from(value: ID2D1RenderTarget) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1RenderTarget> for ::windows::IUnknown {
                    fn from(value: &ID2D1RenderTarget) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1RenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1RenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1RenderTarget> for ID2D1Resource {
                    fn from(value: ID2D1RenderTarget) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1RenderTarget> for ID2D1Resource {
                    fn from(value: &ID2D1RenderTarget) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for ID2D1RenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for &'a ID2D1RenderTarget {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1RenderTarget_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        factory: *mut ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        size: D2D_SIZE_U,
                        srcdata: *const ::std::ffi::c_void,
                        pitch: u32,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        wicbitmapsource: ::windows::RawPtr,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        riid: *const ::windows::Guid,
                        data: *mut ::std::ffi::c_void,
                        bitmapproperties: *const D2D1_BITMAP_PROPERTIES,
                        bitmap: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        bitmap: ::windows::RawPtr,
                        bitmapbrushproperties: *const D2D1_BITMAP_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        bitmapbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        color: *const D2D1_COLOR_F,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        solidcolorbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        gradientstops: *const D2D1_GRADIENT_STOP,
                        gradientstopscount: u32,
                        colorinterpolationgamma: D2D1_GAMMA,
                        extendmode: D2D1_EXTEND_MODE,
                        gradientstopcollection: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        lineargradientbrushproperties: *const D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: ::windows::RawPtr,
                        lineargradientbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        radialgradientbrushproperties: *const D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES,
                        brushproperties: *const D2D1_BRUSH_PROPERTIES,
                        gradientstopcollection: ::windows::RawPtr,
                        radialgradientbrush: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        desiredsize: *const D2D_SIZE_F,
                        desiredpixelsize: *const D2D_SIZE_U,
                        desiredformat: *const D2D1_PIXEL_FORMAT,
                        options: D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS,
                        bitmaprendertarget: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        size: *const D2D_SIZE_F,
                        layer: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        mesh: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        point0: D2D_POINT_2F,
                        point1: D2D_POINT_2F,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rect: *const D2D_RECT_F,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rect: *const D2D_RECT_F,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        roundedrect: *const D2D1_ROUNDED_RECT,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        ellipse: *const D2D1_ELLIPSE,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        geometry: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        strokewidth: f32,
                        strokestyle: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        geometry: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        opacitybrush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        mesh: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        opacitymask: ::windows::RawPtr,
                        brush: ::windows::RawPtr,
                        content: D2D1_OPACITY_MASK_CONTENT,
                        destinationrectangle: *const D2D_RECT_F,
                        sourcerectangle: *const D2D_RECT_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        bitmap: ::windows::RawPtr,
                        destinationrectangle: *const D2D_RECT_F,
                        opacity: f32,
                        interpolationmode: D2D1_BITMAP_INTERPOLATION_MODE,
                        sourcerectangle: *const D2D_RECT_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        string: super::super::Foundation::PWSTR,
                        stringlength: u32,
                        textformat: ::windows::RawPtr,
                        layoutrect: *const D2D_RECT_F,
                        defaultfillbrush: ::windows::RawPtr,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        origin: D2D_POINT_2F,
                        textlayout: ::windows::RawPtr,
                        defaultfillbrush: ::windows::RawPtr,
                        options: D2D1_DRAW_TEXT_OPTIONS,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        baselineorigin: D2D_POINT_2F,
                        glyphrun: *const super::DirectWrite::DWRITE_GLYPH_RUN_abi,
                        foregroundbrush: ::windows::RawPtr,
                        measuringmode: super::DirectWrite::DWRITE_MEASURING_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> D2D1_ANTIALIAS_MODE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textantialiasmode: D2D1_TEXT_ANTIALIAS_MODE,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                    ) -> D2D1_TEXT_ANTIALIAS_MODE,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textrenderingparams: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        textrenderingparams: *mut ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, tag1: u64, tag2: u64),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        layerparameters: *const D2D1_LAYER_PARAMETERS_abi,
                        layer: ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstateblock: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        drawingstateblock: ::windows::RawPtr,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        cliprect: *const D2D_RECT_F,
                        antialiasmode: D2D1_ANTIALIAS_MODE,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        clearcolor: *const D2D1_COLOR_F,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        tag1: *mut u64,
                        tag2: *mut u64,
                    ) -> ::windows::HRESULT,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D1_PIXEL_FORMAT,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, dpix: f32, dpiy: f32),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        dpix: *mut f32,
                        dpiy: *mut f32,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D_SIZE_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D_SIZE_U,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        rendertargetproperties: *const D2D1_RENDER_TARGET_PROPERTIES,
                    )
                        -> super::super::Foundation::BOOL,
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1Resource(::windows::IUnknown);
                impl ID2D1Resource {
                    pub unsafe fn GetFactory(
                        &self,
                        factory: *mut ::std::option::Option<ID2D1Factory>,
                    ) {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(factory),
                        )
                    }
                }
                unsafe impl ::windows::Interface for ID2D1Resource {
                    type Vtable = ID2D1Resource_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420497,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                impl ::std::convert::From<ID2D1Resource> for ::windows::IUnknown {
                    fn from(value: ID2D1Resource) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1Resource> for ::windows::IUnknown {
                    fn from(value: &ID2D1Resource) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1Resource {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1Resource {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1Resource_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        factory: *mut ::windows::RawPtr,
                    ),
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1RoundedRectangleGeometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1RoundedRectangleGeometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420515,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                pub struct ID2D1SolidColorBrush(::windows::IUnknown);
                impl ID2D1SolidColorBrush {
                    pub unsafe fn GetFactory(
                        &self,
                        factory: *mut ::std::option::Option<ID2D1Factory>,
                    ) {
                        (::windows::Interface::vtable(self).3)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(factory),
                        )
                    }
                    pub unsafe fn SetOpacity(&self, opacity: f32) {
                        (::windows::Interface::vtable(self).4)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(opacity),
                        )
                    }
                    pub unsafe fn SetTransform(
                        &self,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).5)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn GetOpacity(&self) -> f32 {
                        (::windows::Interface::vtable(self).6)(::windows::Abi::abi(self))
                    }
                    pub unsafe fn GetTransform(
                        &self,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ) {
                        (::windows::Interface::vtable(self).7)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(transform),
                        )
                    }
                    pub unsafe fn SetColor(&self, color: *const D2D1_COLOR_F) {
                        (::windows::Interface::vtable(self).8)(
                            ::windows::Abi::abi(self),
                            ::std::mem::transmute(color),
                        )
                    }
                    pub unsafe fn GetColor(&self) -> D2D1_COLOR_F {
                        let mut result__: D2D1_COLOR_F = ::std::default::Default::default();
                        (::windows::Interface::vtable(self).9)(
                            ::windows::Abi::abi(self),
                            &mut result__,
                        );
                        result__
                    }
                }
                unsafe impl ::windows::Interface for ID2D1SolidColorBrush {
                    type Vtable = ID2D1SolidColorBrush_abi;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420521,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                impl ::std::convert::From<ID2D1SolidColorBrush> for ::windows::IUnknown {
                    fn from(value: ID2D1SolidColorBrush) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1SolidColorBrush> for ::windows::IUnknown {
                    fn from(value: &ID2D1SolidColorBrush) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            self,
                        ))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ::windows::IUnknown> for &'a ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ::windows::IUnknown> {
                        ::windows::Param::Owned(::std::convert::Into::<::windows::IUnknown>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1SolidColorBrush> for ID2D1Brush {
                    fn from(value: ID2D1SolidColorBrush) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1SolidColorBrush> for ID2D1Brush {
                    fn from(value: &ID2D1SolidColorBrush) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Brush> for ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Brush> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Brush>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Brush> for &'a ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Brush> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Brush>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                impl ::std::convert::From<ID2D1SolidColorBrush> for ID2D1Resource {
                    fn from(value: ID2D1SolidColorBrush) -> Self {
                        unsafe { ::std::mem::transmute(value) }
                    }
                }
                impl ::std::convert::From<&ID2D1SolidColorBrush> for ID2D1Resource {
                    fn from(value: &ID2D1SolidColorBrush) -> Self {
                        ::std::convert::From::from(::std::clone::Clone::clone(value))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(self))
                    }
                }
                impl<'a> ::windows::IntoParam<'a, ID2D1Resource> for &'a ID2D1SolidColorBrush {
                    fn into_param(self) -> ::windows::Param<'a, ID2D1Resource> {
                        ::windows::Param::Owned(::std::convert::Into::<ID2D1Resource>::into(
                            ::std::clone::Clone::clone(self),
                        ))
                    }
                }
                #[repr(C)]
                #[doc(hidden)]
                pub struct ID2D1SolidColorBrush_abi(
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        iid: &::windows::Guid,
                        interface: *mut ::windows::RawPtr,
                    ) -> ::windows::HRESULT,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> u32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        factory: *mut ::windows::RawPtr,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr, opacity: f32),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *const super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub unsafe extern "system" fn(this: ::windows::RawPtr) -> f32,
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        transform: *mut super::super::super::Foundation::Numerics::Matrix3x2,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        color: *const D2D1_COLOR_F,
                    ),
                    pub  unsafe extern "system" fn(
                        this: ::windows::RawPtr,
                        result__: *mut D2D1_COLOR_F,
                    ),
                );
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1StrokeStyle(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1StrokeStyle {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420509,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1StrokeStyle1(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1StrokeStyle1 {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        279390822,
                        59676,
                        17396,
                        [153, 63, 221, 244, 184, 43, 11, 74],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct ID2D1TransformedGeometry(::windows::IUnknown);
                unsafe impl ::windows::Interface for ID2D1TransformedGeometry {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        752420539,
                        4834,
                        4572,
                        [159, 237, 0, 17, 67, 160, 85, 249],
                    );
                }
                pub type PD2D1_EFFECT_FACTORY = unsafe extern "system" fn(
                    effectimpl: *mut ::windows::RawPtr,
                )
                    -> ::windows::HRESULT;
                pub type PD2D1_PROPERTY_GET_FUNCTION =
                    unsafe extern "system" fn(
                        effect: ::windows::RawPtr,
                        data: *mut u8,
                        datasize: u32,
                        actualsize: *mut u32,
                    ) -> ::windows::HRESULT;
                pub type PD2D1_PROPERTY_SET_FUNCTION =
                    unsafe extern "system" fn(
                        effect: ::windows::RawPtr,
                        data: *const u8,
                        datasize: u32,
                    ) -> ::windows::HRESULT;
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod DirectWrite {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct DWRITE_GLYPH_OFFSET {
                    pub advanceOffset: f32,
                    pub ascenderOffset: f32,
                }
                impl DWRITE_GLYPH_OFFSET {}
                impl ::std::default::Default for DWRITE_GLYPH_OFFSET {
                    fn default() -> Self {
                        Self {
                            advanceOffset: 0.0,
                            ascenderOffset: 0.0,
                        }
                    }
                }
                impl ::std::fmt::Debug for DWRITE_GLYPH_OFFSET {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("DWRITE_GLYPH_OFFSET")
                            .field("advanceOffset", &self.advanceOffset)
                            .field("ascenderOffset", &self.ascenderOffset)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for DWRITE_GLYPH_OFFSET {
                    fn eq(&self, other: &Self) -> bool {
                        self.advanceOffset == other.advanceOffset
                            && self.ascenderOffset == other.ascenderOffset
                    }
                }
                impl ::std::cmp::Eq for DWRITE_GLYPH_OFFSET {}
                unsafe impl ::windows::Abi for DWRITE_GLYPH_OFFSET {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone)]
                #[repr(C)]
                pub struct DWRITE_GLYPH_RUN {
                    pub fontFace: ::std::option::Option<IDWriteFontFace>,
                    pub fontEmSize: f32,
                    pub glyphCount: u32,
                    pub glyphIndices: *mut u16,
                    pub glyphAdvances: *mut f32,
                    pub glyphOffsets: *mut DWRITE_GLYPH_OFFSET,
                    pub isSideways: super::super::Foundation::BOOL,
                    pub bidiLevel: u32,
                }
                impl DWRITE_GLYPH_RUN {}
                impl ::std::default::Default for DWRITE_GLYPH_RUN {
                    fn default() -> Self {
                        Self {
                            fontFace: ::std::default::Default::default(),
                            fontEmSize: 0.0,
                            glyphCount: 0,
                            glyphIndices: ::std::ptr::null_mut(),
                            glyphAdvances: ::std::ptr::null_mut(),
                            glyphOffsets: ::std::ptr::null_mut(),
                            isSideways: ::std::default::Default::default(),
                            bidiLevel: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for DWRITE_GLYPH_RUN {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("DWRITE_GLYPH_RUN")
                            .field("fontFace", &self.fontFace)
                            .field("fontEmSize", &self.fontEmSize)
                            .field("glyphCount", &self.glyphCount)
                            .field("glyphIndices", &self.glyphIndices)
                            .field("glyphAdvances", &self.glyphAdvances)
                            .field("glyphOffsets", &self.glyphOffsets)
                            .field("isSideways", &self.isSideways)
                            .field("bidiLevel", &self.bidiLevel)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for DWRITE_GLYPH_RUN {
                    fn eq(&self, other: &Self) -> bool {
                        self.fontFace == other.fontFace
                            && self.fontEmSize == other.fontEmSize
                            && self.glyphCount == other.glyphCount
                            && self.glyphIndices == other.glyphIndices
                            && self.glyphAdvances == other.glyphAdvances
                            && self.glyphOffsets == other.glyphOffsets
                            && self.isSideways == other.isSideways
                            && self.bidiLevel == other.bidiLevel
                    }
                }
                impl ::std::cmp::Eq for DWRITE_GLYPH_RUN {}
                #[repr(C)]
                #[doc(hidden)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct DWRITE_GLYPH_RUN_abi {
                    pub fontFace: ::windows::RawPtr,
                    pub fontEmSize: f32,
                    pub glyphCount: u32,
                    pub glyphIndices: *mut u16,
                    pub glyphAdvances: *mut f32,
                    pub glyphOffsets: *mut DWRITE_GLYPH_OFFSET,
                    pub isSideways: super::super::Foundation::BOOL,
                    pub bidiLevel: u32,
                }
                unsafe impl ::windows::Abi for DWRITE_GLYPH_RUN {
                    type Abi = DWRITE_GLYPH_RUN_abi;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct DWRITE_MEASURING_MODE(pub i32);
                pub const DWRITE_MEASURING_MODE_NATURAL: DWRITE_MEASURING_MODE =
                    DWRITE_MEASURING_MODE(0i32);
                pub const DWRITE_MEASURING_MODE_GDI_CLASSIC: DWRITE_MEASURING_MODE =
                    DWRITE_MEASURING_MODE(1i32);
                pub const DWRITE_MEASURING_MODE_GDI_NATURAL: DWRITE_MEASURING_MODE =
                    DWRITE_MEASURING_MODE(2i32);
                impl ::std::convert::From<i32> for DWRITE_MEASURING_MODE {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for DWRITE_MEASURING_MODE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDWriteFontFace(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDWriteFontFace {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1598652493,
                        28708,
                        19779,
                        [191, 169, 210, 89, 132, 245, 56, 73],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDWriteRenderingParams(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDWriteRenderingParams {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        789423418,
                        10973,
                        18381,
                        [130, 238, 217, 236, 52, 104, 142, 117],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDWriteTextFormat(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDWriteTextFormat {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        2626709528,
                        12759,
                        20435,
                        [161, 81, 124, 94, 34, 93, 181, 90],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDWriteTextLayout(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDWriteTextLayout {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1400074295,
                        27924,
                        16651,
                        [155, 254, 11, 24, 43, 183, 9, 97],
                    );
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Dxgi {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct DXGI_FORMAT(pub u32);
                impl ::std::convert::From<u32> for DXGI_FORMAT {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for DXGI_FORMAT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for DXGI_FORMAT {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for DXGI_FORMAT {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for DXGI_FORMAT {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for DXGI_FORMAT {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDXGIDevice(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDXGIDevice {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        1424783354,
                        4983,
                        17638,
                        [140, 50, 136, 253, 95, 68, 200, 76],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IDXGISurface(::windows::IUnknown);
                unsafe impl ::windows::Interface for IDXGISurface {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        3405559148,
                        27331,
                        18569,
                        [191, 71, 158, 35, 187, 210, 96, 236],
                    );
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Gdi {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HBRUSH(pub isize);
                impl HBRUSH {}
                impl ::std::default::Default for HBRUSH {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HBRUSH {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HBRUSH {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HBRUSH").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for HBRUSH {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HBRUSH {}
                unsafe impl ::windows::Abi for HBRUSH {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl<'a> ::windows::IntoParam<'a, HGDIOBJ> for HBRUSH {
                    fn into_param(self) -> ::windows::Param<'a, HGDIOBJ> {
                        ::windows::Param::Owned(HGDIOBJ(self.0))
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HGDIOBJ(pub isize);
                impl HGDIOBJ {}
                impl ::std::default::Default for HGDIOBJ {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HGDIOBJ {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HGDIOBJ {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HGDIOBJ").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for HGDIOBJ {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HGDIOBJ {}
                unsafe impl ::windows::Abi for HGDIOBJ {
                    type Abi = Self;
                    type DefaultType = Self;
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Imaging {
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IWICBitmap(::windows::IUnknown);
                unsafe impl ::windows::Interface for IWICBitmap {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        289,
                        43250,
                        18551,
                        [186, 10, 253, 43, 102, 69, 251, 148],
                    );
                }
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IWICBitmapSource(::windows::IUnknown);
                unsafe impl ::windows::Interface for IWICBitmapSource {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid = ::windows::Guid::from_values(
                        288,
                        43250,
                        18551,
                        [186, 10, 253, 43, 102, 69, 251, 148],
                    );
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod Storage {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod StructuredStorage {
                #[repr(transparent)]
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: clone :: Clone,
                    :: std :: fmt :: Debug,
                )]
                #[doc(hidden)]
                pub struct IStream(::windows::IUnknown);
                unsafe impl ::windows::Interface for IStream {
                    type Vtable = <::windows::IUnknown as ::windows::Interface>::Vtable;
                    const IID: ::windows::Guid =
                        ::windows::Guid::from_values(12, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod System {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod Com {
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct CLSCTX(pub u32);
                pub const CLSCTX_INPROC_SERVER: CLSCTX = CLSCTX(1u32);
                pub const CLSCTX_INPROC_HANDLER: CLSCTX = CLSCTX(2u32);
                pub const CLSCTX_LOCAL_SERVER: CLSCTX = CLSCTX(4u32);
                pub const CLSCTX_INPROC_SERVER16: CLSCTX = CLSCTX(8u32);
                pub const CLSCTX_REMOTE_SERVER: CLSCTX = CLSCTX(16u32);
                pub const CLSCTX_INPROC_HANDLER16: CLSCTX = CLSCTX(32u32);
                pub const CLSCTX_RESERVED1: CLSCTX = CLSCTX(64u32);
                pub const CLSCTX_RESERVED2: CLSCTX = CLSCTX(128u32);
                pub const CLSCTX_RESERVED3: CLSCTX = CLSCTX(256u32);
                pub const CLSCTX_RESERVED4: CLSCTX = CLSCTX(512u32);
                pub const CLSCTX_NO_CODE_DOWNLOAD: CLSCTX = CLSCTX(1024u32);
                pub const CLSCTX_RESERVED5: CLSCTX = CLSCTX(2048u32);
                pub const CLSCTX_NO_CUSTOM_MARSHAL: CLSCTX = CLSCTX(4096u32);
                pub const CLSCTX_ENABLE_CODE_DOWNLOAD: CLSCTX = CLSCTX(8192u32);
                pub const CLSCTX_NO_FAILURE_LOG: CLSCTX = CLSCTX(16384u32);
                pub const CLSCTX_DISABLE_AAA: CLSCTX = CLSCTX(32768u32);
                pub const CLSCTX_ENABLE_AAA: CLSCTX = CLSCTX(65536u32);
                pub const CLSCTX_FROM_DEFAULT_CONTEXT: CLSCTX = CLSCTX(131072u32);
                pub const CLSCTX_ACTIVATE_X86_SERVER: CLSCTX = CLSCTX(262144u32);
                pub const CLSCTX_ACTIVATE_32_BIT_SERVER: CLSCTX = CLSCTX(262144u32);
                pub const CLSCTX_ACTIVATE_64_BIT_SERVER: CLSCTX = CLSCTX(524288u32);
                pub const CLSCTX_ENABLE_CLOAKING: CLSCTX = CLSCTX(1048576u32);
                pub const CLSCTX_APPCONTAINER: CLSCTX = CLSCTX(4194304u32);
                pub const CLSCTX_ACTIVATE_AAA_AS_IU: CLSCTX = CLSCTX(8388608u32);
                pub const CLSCTX_RESERVED6: CLSCTX = CLSCTX(16777216u32);
                pub const CLSCTX_ACTIVATE_ARM32_SERVER: CLSCTX = CLSCTX(33554432u32);
                pub const CLSCTX_PS_DLL: CLSCTX = CLSCTX(2147483648u32);
                pub const CLSCTX_ALL: CLSCTX = CLSCTX(23u32);
                pub const CLSCTX_SERVER: CLSCTX = CLSCTX(21u32);
                impl ::std::convert::From<u32> for CLSCTX {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for CLSCTX {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for CLSCTX {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for CLSCTX {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for CLSCTX {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for CLSCTX {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct COINIT(pub u32);
                pub const COINIT_APARTMENTTHREADED: COINIT = COINIT(2u32);
                pub const COINIT_MULTITHREADED: COINIT = COINIT(0u32);
                pub const COINIT_DISABLE_OLE1DDE: COINIT = COINIT(4u32);
                pub const COINIT_SPEED_OVER_MEMORY: COINIT = COINIT(8u32);
                impl ::std::convert::From<u32> for COINIT {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for COINIT {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for COINIT {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for COINIT {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for COINIT {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for COINIT {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn CoCreateInstance<'a, T: ::windows::Interface>(
                    rclsid: *const ::windows::Guid,
                    punkouter: impl ::windows::IntoParam<'a, ::windows::IUnknown>,
                    dwclscontext: CLSCTX,
                ) -> ::windows::Result<T> {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLE32")]
                        extern "system" {
                            fn CoCreateInstance(
                                rclsid: *const ::windows::Guid,
                                punkouter: ::windows::RawPtr,
                                dwclscontext: CLSCTX,
                                riid: *const ::windows::Guid,
                                ppv: *mut *mut ::std::ffi::c_void,
                            ) -> ::windows::HRESULT;
                        }
                        let mut result__ = ::std::option::Option::None;
                        CoCreateInstance(
                            ::std::mem::transmute(rclsid),
                            punkouter.into_param().abi(),
                            ::std::mem::transmute(dwclscontext),
                            &<T as ::windows::Interface>::IID,
                            ::windows::Abi::set_abi(&mut result__),
                        )
                        .and_some(result__)
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn CoInitializeEx(
                    pvreserved: *mut ::std::ffi::c_void,
                    dwcoinit: COINIT,
                ) -> ::windows::Result<()> {
                    #[cfg(windows)]
                    {
                        #[link(name = "OLE32")]
                        extern "system" {
                            fn CoInitializeEx(
                                pvreserved: *mut ::std::ffi::c_void,
                                dwcoinit: COINIT,
                            ) -> ::windows::HRESULT;
                        }
                        CoInitializeEx(
                            ::std::mem::transmute(pvreserved),
                            ::std::mem::transmute(dwcoinit),
                        )
                        .ok()
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod LibraryLoader {
                pub unsafe fn GetModuleHandleA<'a>(
                    lpmodulename: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                ) -> super::super::Foundation::HINSTANCE {
                    #[cfg(windows)]
                    {
                        #[link(name = "KERNEL32")]
                        extern "system" {
                            fn GetModuleHandleA(
                                lpmodulename: super::super::Foundation::PSTR,
                            ) -> super::super::Foundation::HINSTANCE;
                        }
                        GetModuleHandleA(lpmodulename.into_param().abi())
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
            }
        }
        #[allow(
            unused_variables,
            non_upper_case_globals,
            non_snake_case,
            unused_unsafe,
            non_camel_case_types,
            dead_code,
            clippy::all
        )]
        pub mod UI {
            #[allow(
                unused_variables,
                non_upper_case_globals,
                non_snake_case,
                unused_unsafe,
                non_camel_case_types,
                dead_code,
                clippy::all
            )]
            pub mod WindowsAndMessaging {
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct CREATESTRUCTA {
                    pub lpCreateParams: *mut ::std::ffi::c_void,
                    pub hInstance: super::super::Foundation::HINSTANCE,
                    pub hMenu: HMENU,
                    pub hwndParent: super::super::Foundation::HWND,
                    pub cy: i32,
                    pub cx: i32,
                    pub y: i32,
                    pub x: i32,
                    pub style: i32,
                    pub lpszName: super::super::Foundation::PSTR,
                    pub lpszClass: super::super::Foundation::PSTR,
                    pub dwExStyle: u32,
                }
                impl CREATESTRUCTA {}
                impl ::std::default::Default for CREATESTRUCTA {
                    fn default() -> Self {
                        Self {
                            lpCreateParams: ::std::ptr::null_mut(),
                            hInstance: ::std::default::Default::default(),
                            hMenu: ::std::default::Default::default(),
                            hwndParent: ::std::default::Default::default(),
                            cy: 0,
                            cx: 0,
                            y: 0,
                            x: 0,
                            style: 0,
                            lpszName: ::std::default::Default::default(),
                            lpszClass: ::std::default::Default::default(),
                            dwExStyle: 0,
                        }
                    }
                }
                impl ::std::fmt::Debug for CREATESTRUCTA {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("CREATESTRUCTA")
                            .field("lpCreateParams", &self.lpCreateParams)
                            .field("hInstance", &self.hInstance)
                            .field("hMenu", &self.hMenu)
                            .field("hwndParent", &self.hwndParent)
                            .field("cy", &self.cy)
                            .field("cx", &self.cx)
                            .field("y", &self.y)
                            .field("x", &self.x)
                            .field("style", &self.style)
                            .field("lpszName", &self.lpszName)
                            .field("lpszClass", &self.lpszClass)
                            .field("dwExStyle", &self.dwExStyle)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for CREATESTRUCTA {
                    fn eq(&self, other: &Self) -> bool {
                        self.lpCreateParams == other.lpCreateParams
                            && self.hInstance == other.hInstance
                            && self.hMenu == other.hMenu
                            && self.hwndParent == other.hwndParent
                            && self.cy == other.cy
                            && self.cx == other.cx
                            && self.y == other.y
                            && self.x == other.x
                            && self.style == other.style
                            && self.lpszName == other.lpszName
                            && self.lpszClass == other.lpszClass
                            && self.dwExStyle == other.dwExStyle
                    }
                }
                impl ::std::cmp::Eq for CREATESTRUCTA {}
                unsafe impl ::windows::Abi for CREATESTRUCTA {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub const CW_USEDEFAULT: i32 = -2147483648i32;
                pub unsafe fn CreateWindowExA<'a>(
                    dwexstyle: WINDOW_EX_STYLE,
                    lpclassname: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                    lpwindowname: impl ::windows::IntoParam<'a, super::super::Foundation::PSTR>,
                    dwstyle: WINDOW_STYLE,
                    x: i32,
                    y: i32,
                    nwidth: i32,
                    nheight: i32,
                    hwndparent: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    hmenu: impl ::windows::IntoParam<'a, HMENU>,
                    hinstance: impl ::windows::IntoParam<'a, super::super::Foundation::HINSTANCE>,
                    lpparam: *mut ::std::ffi::c_void,
                ) -> super::super::Foundation::HWND {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn CreateWindowExA(
                                dwexstyle: WINDOW_EX_STYLE,
                                lpclassname: super::super::Foundation::PSTR,
                                lpwindowname: super::super::Foundation::PSTR,
                                dwstyle: WINDOW_STYLE,
                                x: i32,
                                y: i32,
                                nwidth: i32,
                                nheight: i32,
                                hwndparent: super::super::Foundation::HWND,
                                hmenu: HMENU,
                                hinstance: super::super::Foundation::HINSTANCE,
                                lpparam: *mut ::std::ffi::c_void,
                            ) -> super::super::Foundation::HWND;
                        }
                        CreateWindowExA(
                            ::std::mem::transmute(dwexstyle),
                            lpclassname.into_param().abi(),
                            lpwindowname.into_param().abi(),
                            ::std::mem::transmute(dwstyle),
                            ::std::mem::transmute(x),
                            ::std::mem::transmute(y),
                            ::std::mem::transmute(nwidth),
                            ::std::mem::transmute(nheight),
                            hwndparent.into_param().abi(),
                            hmenu.into_param().abi(),
                            hinstance.into_param().abi(),
                            ::std::mem::transmute(lpparam),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn DefWindowProcA<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    msg: u32,
                    wparam: impl ::windows::IntoParam<'a, super::super::Foundation::WPARAM>,
                    lparam: impl ::windows::IntoParam<'a, super::super::Foundation::LPARAM>,
                ) -> super::super::Foundation::LRESULT {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn DefWindowProcA(
                                hwnd: super::super::Foundation::HWND,
                                msg: u32,
                                wparam: super::super::Foundation::WPARAM,
                                lparam: super::super::Foundation::LPARAM,
                            ) -> super::super::Foundation::LRESULT;
                        }
                        DefWindowProcA(
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(msg),
                            wparam.into_param().abi(),
                            lparam.into_param().abi(),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn DispatchMessageA(
                    lpmsg: *const MSG,
                ) -> super::super::Foundation::LRESULT {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn DispatchMessageA(
                                lpmsg: *const MSG,
                            ) -> super::super::Foundation::LRESULT;
                        }
                        DispatchMessageA(::std::mem::transmute(lpmsg))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetClientRect<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    lprect: *mut super::super::Foundation::RECT,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn GetClientRect(
                                hwnd: super::super::Foundation::HWND,
                                lprect: *mut super::super::Foundation::RECT,
                            ) -> super::super::Foundation::BOOL;
                        }
                        GetClientRect(hwnd.into_param().abi(), ::std::mem::transmute(lprect))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetMessageA<'a>(
                    lpmsg: *mut MSG,
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    wmsgfiltermin: u32,
                    wmsgfiltermax: u32,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn GetMessageA(
                                lpmsg: *mut MSG,
                                hwnd: super::super::Foundation::HWND,
                                wmsgfiltermin: u32,
                                wmsgfiltermax: u32,
                            ) -> super::super::Foundation::BOOL;
                        }
                        GetMessageA(
                            ::std::mem::transmute(lpmsg),
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(wmsgfiltermin),
                            ::std::mem::transmute(wmsgfiltermax),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn GetWindowLongPtrA<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    nindex: WINDOW_LONG_PTR_INDEX,
                ) -> isize {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn GetWindowLongPtrA(
                                hwnd: super::super::Foundation::HWND,
                                nindex: WINDOW_LONG_PTR_INDEX,
                            ) -> isize;
                        }
                        GetWindowLongPtrA(hwnd.into_param().abi(), ::std::mem::transmute(nindex))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HCURSOR(pub isize);
                impl HCURSOR {}
                impl ::std::default::Default for HCURSOR {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HCURSOR {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HCURSOR {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HCURSOR").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for HCURSOR {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HCURSOR {}
                unsafe impl ::windows::Abi for HCURSOR {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl<'a> ::windows::IntoParam<'a, HICON> for HCURSOR {
                    fn into_param(self) -> ::windows::Param<'a, HICON> {
                        ::windows::Param::Owned(HICON(self.0))
                    }
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HICON(pub isize);
                impl HICON {}
                impl ::std::default::Default for HICON {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HICON {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HICON {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HICON").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for HICON {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HICON {}
                unsafe impl ::windows::Abi for HICON {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(transparent)]
                pub struct HMENU(pub isize);
                impl HMENU {}
                impl ::std::default::Default for HMENU {
                    fn default() -> Self {
                        Self(0)
                    }
                }
                impl HMENU {
                    pub const NULL: Self = Self(0);
                    pub fn is_null(&self) -> bool {
                        self.0 == 0
                    }
                }
                impl ::std::fmt::Debug for HMENU {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("HMENU").field("Value", &self.0).finish()
                    }
                }
                impl ::std::cmp::PartialEq for HMENU {
                    fn eq(&self, other: &Self) -> bool {
                        self.0 == other.0
                    }
                }
                impl ::std::cmp::Eq for HMENU {}
                unsafe impl ::windows::Abi for HMENU {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                pub const IDC_HAND: super::super::Foundation::PWSTR =
                    super::super::Foundation::PWSTR(32649i32 as _);
                pub unsafe fn LoadCursorW<'a>(
                    hinstance: impl ::windows::IntoParam<'a, super::super::Foundation::HINSTANCE>,
                    lpcursorname: impl ::windows::IntoParam<'a, super::super::Foundation::PWSTR>,
                ) -> HCURSOR {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn LoadCursorW(
                                hinstance: super::super::Foundation::HINSTANCE,
                                lpcursorname: super::super::Foundation::PWSTR,
                            ) -> HCURSOR;
                        }
                        LoadCursorW(
                            hinstance.into_param().abi(),
                            lpcursorname.into_param().abi(),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                #[repr(C)]
                pub struct MSG {
                    pub hwnd: super::super::Foundation::HWND,
                    pub message: u32,
                    pub wParam: super::super::Foundation::WPARAM,
                    pub lParam: super::super::Foundation::LPARAM,
                    pub time: u32,
                    pub pt: super::super::Foundation::POINT,
                }
                impl MSG {}
                impl ::std::default::Default for MSG {
                    fn default() -> Self {
                        Self {
                            hwnd: ::std::default::Default::default(),
                            message: 0,
                            wParam: ::std::default::Default::default(),
                            lParam: ::std::default::Default::default(),
                            time: 0,
                            pt: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for MSG {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("MSG")
                            .field("hwnd", &self.hwnd)
                            .field("message", &self.message)
                            .field("wParam", &self.wParam)
                            .field("lParam", &self.lParam)
                            .field("time", &self.time)
                            .field("pt", &self.pt)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for MSG {
                    fn eq(&self, other: &Self) -> bool {
                        self.hwnd == other.hwnd
                            && self.message == other.message
                            && self.wParam == other.wParam
                            && self.lParam == other.lParam
                            && self.time == other.time
                            && self.pt == other.pt
                    }
                }
                impl ::std::cmp::Eq for MSG {}
                unsafe impl ::windows::Abi for MSG {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct PEEK_MESSAGE_REMOVE_TYPE(pub u32);
                pub const PM_NOREMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(0u32);
                pub const PM_REMOVE: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(1u32);
                pub const PM_NOYIELD: PEEK_MESSAGE_REMOVE_TYPE = PEEK_MESSAGE_REMOVE_TYPE(2u32);
                pub const PM_QS_INPUT: PEEK_MESSAGE_REMOVE_TYPE =
                    PEEK_MESSAGE_REMOVE_TYPE(67567616u32);
                pub const PM_QS_POSTMESSAGE: PEEK_MESSAGE_REMOVE_TYPE =
                    PEEK_MESSAGE_REMOVE_TYPE(9961472u32);
                pub const PM_QS_PAINT: PEEK_MESSAGE_REMOVE_TYPE =
                    PEEK_MESSAGE_REMOVE_TYPE(2097152u32);
                pub const PM_QS_SENDMESSAGE: PEEK_MESSAGE_REMOVE_TYPE =
                    PEEK_MESSAGE_REMOVE_TYPE(4194304u32);
                impl ::std::convert::From<u32> for PEEK_MESSAGE_REMOVE_TYPE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for PEEK_MESSAGE_REMOVE_TYPE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for PEEK_MESSAGE_REMOVE_TYPE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for PEEK_MESSAGE_REMOVE_TYPE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for PEEK_MESSAGE_REMOVE_TYPE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for PEEK_MESSAGE_REMOVE_TYPE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub unsafe fn PeekMessageA<'a>(
                    lpmsg: *mut MSG,
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    wmsgfiltermin: u32,
                    wmsgfiltermax: u32,
                    wremovemsg: PEEK_MESSAGE_REMOVE_TYPE,
                ) -> super::super::Foundation::BOOL {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn PeekMessageA(
                                lpmsg: *mut MSG,
                                hwnd: super::super::Foundation::HWND,
                                wmsgfiltermin: u32,
                                wmsgfiltermax: u32,
                                wremovemsg: PEEK_MESSAGE_REMOVE_TYPE,
                            ) -> super::super::Foundation::BOOL;
                        }
                        PeekMessageA(
                            ::std::mem::transmute(lpmsg),
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(wmsgfiltermin),
                            ::std::mem::transmute(wmsgfiltermax),
                            ::std::mem::transmute(wremovemsg),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn PostQuitMessage(nexitcode: i32) {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn PostQuitMessage(nexitcode: i32);
                        }
                        PostQuitMessage(::std::mem::transmute(nexitcode))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn RegisterClassA(lpwndclass: *const WNDCLASSA) -> u16 {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn RegisterClassA(lpwndclass: *const WNDCLASSA_abi) -> u16;
                        }
                        RegisterClassA(::std::mem::transmute(lpwndclass))
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub const SIZE_MINIMIZED: u32 = 1u32;
                pub unsafe fn SetWindowLongA<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    nindex: WINDOW_LONG_PTR_INDEX,
                    dwnewlong: i32,
                ) -> i32 {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn SetWindowLongA(
                                hwnd: super::super::Foundation::HWND,
                                nindex: WINDOW_LONG_PTR_INDEX,
                                dwnewlong: i32,
                            ) -> i32;
                        }
                        SetWindowLongA(
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(nindex),
                            ::std::mem::transmute(dwnewlong),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                pub unsafe fn SetWindowLongPtrA<'a>(
                    hwnd: impl ::windows::IntoParam<'a, super::super::Foundation::HWND>,
                    nindex: WINDOW_LONG_PTR_INDEX,
                    dwnewlong: isize,
                ) -> isize {
                    #[cfg(windows)]
                    {
                        #[link(name = "USER32")]
                        extern "system" {
                            fn SetWindowLongPtrA(
                                hwnd: super::super::Foundation::HWND,
                                nindex: WINDOW_LONG_PTR_INDEX,
                                dwnewlong: isize,
                            ) -> isize;
                        }
                        SetWindowLongPtrA(
                            hwnd.into_param().abi(),
                            ::std::mem::transmute(nindex),
                            ::std::mem::transmute(dwnewlong),
                        )
                    }
                    #[cfg(not(windows))]
                    unimplemented!("Unsupported target OS");
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WINDOW_EX_STYLE(pub u32);
                pub const WS_EX_DLGMODALFRAME: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1u32);
                pub const WS_EX_NOPARENTNOTIFY: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4u32);
                pub const WS_EX_TOPMOST: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8u32);
                pub const WS_EX_ACCEPTFILES: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16u32);
                pub const WS_EX_TRANSPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(32u32);
                pub const WS_EX_MDICHILD: WINDOW_EX_STYLE = WINDOW_EX_STYLE(64u32);
                pub const WS_EX_TOOLWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(128u32);
                pub const WS_EX_WINDOWEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(256u32);
                pub const WS_EX_CLIENTEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(512u32);
                pub const WS_EX_CONTEXTHELP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1024u32);
                pub const WS_EX_RIGHT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4096u32);
                pub const WS_EX_LEFT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
                pub const WS_EX_RTLREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(8192u32);
                pub const WS_EX_LTRREADING: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
                pub const WS_EX_LEFTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(16384u32);
                pub const WS_EX_RIGHTSCROLLBAR: WINDOW_EX_STYLE = WINDOW_EX_STYLE(0u32);
                pub const WS_EX_CONTROLPARENT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(65536u32);
                pub const WS_EX_STATICEDGE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(131072u32);
                pub const WS_EX_APPWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(262144u32);
                pub const WS_EX_OVERLAPPEDWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(768u32);
                pub const WS_EX_PALETTEWINDOW: WINDOW_EX_STYLE = WINDOW_EX_STYLE(392u32);
                pub const WS_EX_LAYERED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(524288u32);
                pub const WS_EX_NOINHERITLAYOUT: WINDOW_EX_STYLE = WINDOW_EX_STYLE(1048576u32);
                pub const WS_EX_NOREDIRECTIONBITMAP: WINDOW_EX_STYLE = WINDOW_EX_STYLE(2097152u32);
                pub const WS_EX_LAYOUTRTL: WINDOW_EX_STYLE = WINDOW_EX_STYLE(4194304u32);
                pub const WS_EX_COMPOSITED: WINDOW_EX_STYLE = WINDOW_EX_STYLE(33554432u32);
                pub const WS_EX_NOACTIVATE: WINDOW_EX_STYLE = WINDOW_EX_STYLE(134217728u32);
                impl ::std::convert::From<u32> for WINDOW_EX_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WINDOW_EX_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WINDOW_EX_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WINDOW_EX_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WINDOW_EX_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WINDOW_EX_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WINDOW_LONG_PTR_INDEX(pub i32);
                pub const GWL_EXSTYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-20i32);
                pub const GWLP_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
                pub const GWLP_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
                pub const GWLP_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
                pub const GWL_STYLE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-16i32);
                pub const GWLP_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
                pub const GWLP_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
                pub const GWL_HINSTANCE: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-6i32);
                pub const GWL_ID: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-12i32);
                pub const GWL_USERDATA: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-21i32);
                pub const GWL_WNDPROC: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-4i32);
                pub const GWL_HWNDPARENT: WINDOW_LONG_PTR_INDEX = WINDOW_LONG_PTR_INDEX(-8i32);
                impl ::std::convert::From<i32> for WINDOW_LONG_PTR_INDEX {
                    fn from(value: i32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WINDOW_LONG_PTR_INDEX {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WINDOW_STYLE(pub u32);
                pub const WS_OVERLAPPED: WINDOW_STYLE = WINDOW_STYLE(0u32);
                pub const WS_POPUP: WINDOW_STYLE = WINDOW_STYLE(2147483648u32);
                pub const WS_CHILD: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
                pub const WS_MINIMIZE: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
                pub const WS_VISIBLE: WINDOW_STYLE = WINDOW_STYLE(268435456u32);
                pub const WS_DISABLED: WINDOW_STYLE = WINDOW_STYLE(134217728u32);
                pub const WS_CLIPSIBLINGS: WINDOW_STYLE = WINDOW_STYLE(67108864u32);
                pub const WS_CLIPCHILDREN: WINDOW_STYLE = WINDOW_STYLE(33554432u32);
                pub const WS_MAXIMIZE: WINDOW_STYLE = WINDOW_STYLE(16777216u32);
                pub const WS_CAPTION: WINDOW_STYLE = WINDOW_STYLE(12582912u32);
                pub const WS_BORDER: WINDOW_STYLE = WINDOW_STYLE(8388608u32);
                pub const WS_DLGFRAME: WINDOW_STYLE = WINDOW_STYLE(4194304u32);
                pub const WS_VSCROLL: WINDOW_STYLE = WINDOW_STYLE(2097152u32);
                pub const WS_HSCROLL: WINDOW_STYLE = WINDOW_STYLE(1048576u32);
                pub const WS_SYSMENU: WINDOW_STYLE = WINDOW_STYLE(524288u32);
                pub const WS_THICKFRAME: WINDOW_STYLE = WINDOW_STYLE(262144u32);
                pub const WS_GROUP: WINDOW_STYLE = WINDOW_STYLE(131072u32);
                pub const WS_TABSTOP: WINDOW_STYLE = WINDOW_STYLE(65536u32);
                pub const WS_MINIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(131072u32);
                pub const WS_MAXIMIZEBOX: WINDOW_STYLE = WINDOW_STYLE(65536u32);
                pub const WS_TILED: WINDOW_STYLE = WINDOW_STYLE(0u32);
                pub const WS_ICONIC: WINDOW_STYLE = WINDOW_STYLE(536870912u32);
                pub const WS_SIZEBOX: WINDOW_STYLE = WINDOW_STYLE(262144u32);
                pub const WS_TILEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
                pub const WS_OVERLAPPEDWINDOW: WINDOW_STYLE = WINDOW_STYLE(13565952u32);
                pub const WS_POPUPWINDOW: WINDOW_STYLE = WINDOW_STYLE(2156396544u32);
                pub const WS_CHILDWINDOW: WINDOW_STYLE = WINDOW_STYLE(1073741824u32);
                pub const WS_ACTIVECAPTION: WINDOW_STYLE = WINDOW_STYLE(1u32);
                impl ::std::convert::From<u32> for WINDOW_STYLE {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WINDOW_STYLE {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WINDOW_STYLE {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WINDOW_STYLE {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WINDOW_STYLE {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WINDOW_STYLE {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub const WM_ACTIVATE: u32 = 6u32;
                pub const WM_DESTROY: u32 = 2u32;
                pub const WM_DISPLAYCHANGE: u32 = 126u32;
                pub const WM_NCCREATE: u32 = 129u32;
                pub const WM_PAINT: u32 = 15u32;
                pub const WM_QUIT: u32 = 18u32;
                pub const WM_SIZE: u32 = 5u32;
                pub const WM_USER: u32 = 1024u32;
                #[derive(:: std :: clone :: Clone)]
                #[repr(C)]
                pub struct WNDCLASSA {
                    pub style: WNDCLASS_STYLES,
                    pub lpfnWndProc: ::std::option::Option<WNDPROC>,
                    pub cbClsExtra: i32,
                    pub cbWndExtra: i32,
                    pub hInstance: super::super::Foundation::HINSTANCE,
                    pub hIcon: HICON,
                    pub hCursor: HCURSOR,
                    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
                    pub lpszMenuName: super::super::Foundation::PSTR,
                    pub lpszClassName: super::super::Foundation::PSTR,
                }
                impl WNDCLASSA {}
                impl ::std::default::Default for WNDCLASSA {
                    fn default() -> Self {
                        Self {
                            style: ::std::default::Default::default(),
                            lpfnWndProc: ::std::default::Default::default(),
                            cbClsExtra: 0,
                            cbWndExtra: 0,
                            hInstance: ::std::default::Default::default(),
                            hIcon: ::std::default::Default::default(),
                            hCursor: ::std::default::Default::default(),
                            hbrBackground: ::std::default::Default::default(),
                            lpszMenuName: ::std::default::Default::default(),
                            lpszClassName: ::std::default::Default::default(),
                        }
                    }
                }
                impl ::std::fmt::Debug for WNDCLASSA {
                    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                        fmt.debug_struct("WNDCLASSA")
                            .field("style", &self.style)
                            .field("cbClsExtra", &self.cbClsExtra)
                            .field("cbWndExtra", &self.cbWndExtra)
                            .field("hInstance", &self.hInstance)
                            .field("hIcon", &self.hIcon)
                            .field("hCursor", &self.hCursor)
                            .field("hbrBackground", &self.hbrBackground)
                            .field("lpszMenuName", &self.lpszMenuName)
                            .field("lpszClassName", &self.lpszClassName)
                            .finish()
                    }
                }
                impl ::std::cmp::PartialEq for WNDCLASSA {
                    fn eq(&self, other: &Self) -> bool {
                        self.style == other.style
                            && self.lpfnWndProc.map(|f| f as usize)
                                == other.lpfnWndProc.map(|f| f as usize)
                            && self.cbClsExtra == other.cbClsExtra
                            && self.cbWndExtra == other.cbWndExtra
                            && self.hInstance == other.hInstance
                            && self.hIcon == other.hIcon
                            && self.hCursor == other.hCursor
                            && self.hbrBackground == other.hbrBackground
                            && self.lpszMenuName == other.lpszMenuName
                            && self.lpszClassName == other.lpszClassName
                    }
                }
                impl ::std::cmp::Eq for WNDCLASSA {}
                #[repr(C)]
                #[doc(hidden)]
                #[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
                pub struct WNDCLASSA_abi {
                    pub style: WNDCLASS_STYLES,
                    pub lpfnWndProc: ::windows::RawPtr,
                    pub cbClsExtra: i32,
                    pub cbWndExtra: i32,
                    pub hInstance: super::super::Foundation::HINSTANCE,
                    pub hIcon: HICON,
                    pub hCursor: HCURSOR,
                    pub hbrBackground: super::super::Graphics::Gdi::HBRUSH,
                    pub lpszMenuName: super::super::Foundation::PSTR,
                    pub lpszClassName: super::super::Foundation::PSTR,
                }
                unsafe impl ::windows::Abi for WNDCLASSA {
                    type Abi = WNDCLASSA_abi;
                    type DefaultType = Self;
                }
                #[derive(
                    :: std :: cmp :: PartialEq,
                    :: std :: cmp :: Eq,
                    :: std :: marker :: Copy,
                    :: std :: clone :: Clone,
                    :: std :: default :: Default,
                    :: std :: fmt :: Debug,
                )]
                #[repr(transparent)]
                pub struct WNDCLASS_STYLES(pub u32);
                pub const CS_VREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(1u32);
                pub const CS_HREDRAW: WNDCLASS_STYLES = WNDCLASS_STYLES(2u32);
                pub const CS_DBLCLKS: WNDCLASS_STYLES = WNDCLASS_STYLES(8u32);
                pub const CS_OWNDC: WNDCLASS_STYLES = WNDCLASS_STYLES(32u32);
                pub const CS_CLASSDC: WNDCLASS_STYLES = WNDCLASS_STYLES(64u32);
                pub const CS_PARENTDC: WNDCLASS_STYLES = WNDCLASS_STYLES(128u32);
                pub const CS_NOCLOSE: WNDCLASS_STYLES = WNDCLASS_STYLES(512u32);
                pub const CS_SAVEBITS: WNDCLASS_STYLES = WNDCLASS_STYLES(2048u32);
                pub const CS_BYTEALIGNCLIENT: WNDCLASS_STYLES = WNDCLASS_STYLES(4096u32);
                pub const CS_BYTEALIGNWINDOW: WNDCLASS_STYLES = WNDCLASS_STYLES(8192u32);
                pub const CS_GLOBALCLASS: WNDCLASS_STYLES = WNDCLASS_STYLES(16384u32);
                pub const CS_IME: WNDCLASS_STYLES = WNDCLASS_STYLES(65536u32);
                pub const CS_DROPSHADOW: WNDCLASS_STYLES = WNDCLASS_STYLES(131072u32);
                impl ::std::convert::From<u32> for WNDCLASS_STYLES {
                    fn from(value: u32) -> Self {
                        Self(value)
                    }
                }
                unsafe impl ::windows::Abi for WNDCLASS_STYLES {
                    type Abi = Self;
                    type DefaultType = Self;
                }
                impl ::std::ops::BitOr for WNDCLASS_STYLES {
                    type Output = Self;
                    fn bitor(self, rhs: Self) -> Self {
                        Self(self.0 | rhs.0)
                    }
                }
                impl ::std::ops::BitAnd for WNDCLASS_STYLES {
                    type Output = Self;
                    fn bitand(self, rhs: Self) -> Self {
                        Self(self.0 & rhs.0)
                    }
                }
                impl ::std::ops::BitOrAssign for WNDCLASS_STYLES {
                    fn bitor_assign(&mut self, rhs: Self) {
                        self.0.bitor_assign(rhs.0)
                    }
                }
                impl ::std::ops::BitAndAssign for WNDCLASS_STYLES {
                    fn bitand_assign(&mut self, rhs: Self) {
                        self.0.bitand_assign(rhs.0)
                    }
                }
                pub type WNDPROC = unsafe extern "system" fn(
                    param0: super::super::Foundation::HWND,
                    param1: u32,
                    param2: super::super::Foundation::WPARAM,
                    param3: super::super::Foundation::LPARAM,
                )
                    -> super::super::Foundation::LRESULT;
            }
        }
    }
}
