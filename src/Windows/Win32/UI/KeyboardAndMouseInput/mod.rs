#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct ACTIVATE_KEYBOARD_LAYOUT_FLAGS(pub u32);
pub const KLF_REORDER: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(8u32);
pub const KLF_RESET: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(1073741824u32);
pub const KLF_SETFORPROCESS: ACTIVATE_KEYBOARD_LAYOUT_FLAGS =
    ACTIVATE_KEYBOARD_LAYOUT_FLAGS(256u32);
pub const KLF_SHIFTLOCK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(65536u32);
pub const KLF_ACTIVATE: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(1u32);
pub const KLF_NOTELLSHELL: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(128u32);
pub const KLF_REPLACELANG: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(16u32);
pub const KLF_SUBSTITUTE_OK: ACTIVATE_KEYBOARD_LAYOUT_FLAGS = ACTIVATE_KEYBOARD_LAYOUT_FLAGS(2u32);
impl ::std::convert::From<u32> for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ACTIVATE_KEYBOARD_LAYOUT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const ACUTE: u32 = 769u32;
pub const AX_KBD_DESKTOP_TYPE: u32 = 1u32;
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ActivateKeyboardLayout<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    hkl: Param0,
    flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
) -> super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ActivateKeyboardLayout(
                hkl: super::TextServices::HKL,
                flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
            ) -> super::TextServices::HKL;
        }
        ::std::mem::transmute(ActivateKeyboardLayout(
            hkl.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const BREVE: u32 = 774u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn BlockInput<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fblockit: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn BlockInput(
                fblockit: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(BlockInput(fblockit.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const CAPLOK: u32 = 1u32;
pub const CAPLOKALTGR: u32 = 4u32;
pub const CEDILLA: u32 = 807u32;
pub const CIRCUMFLEX: u32 = 770u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DEADKEY {
    pub dwBoth: u32,
    pub wchComposed: u16,
    pub uFlags: u16,
}
impl DEADKEY {}
impl ::std::default::Default for DEADKEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DEADKEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEADKEY")
            .field("dwBoth", &self.dwBoth)
            .field("wchComposed", &self.wchComposed)
            .field("uFlags", &self.uFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DEADKEY {
    fn eq(&self, other: &Self) -> bool {
        self.dwBoth == other.dwBoth
            && self.wchComposed == other.wchComposed
            && self.uFlags == other.uFlags
    }
}
impl ::std::cmp::Eq for DEADKEY {}
unsafe impl ::windows::runtime::Abi for DEADKEY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DEC_KBD_ANSI_LAYOUT_TYPE: u32 = 1u32;
pub const DEC_KBD_JIS_LAYOUT_TYPE: u32 = 2u32;
pub const DIARESIS: u32 = 776u32;
pub const DIARESIS_TONOS: u32 = 901u32;
pub const DKF_DEAD: u32 = 1u32;
pub const DONTCARE_BIT: u32 = 33554432u32;
pub const DOT_ABOVE: u32 = 775u32;
pub const DOUBLE_ACUTE: u32 = 779u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DefRawInputProc(
    parawinput: *const *const RAWINPUT,
    ninput: i32,
    cbsizeheader: u32,
) -> super::super::Foundation::LRESULT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DefRawInputProc(
                parawinput: *const *const RAWINPUT,
                ninput: i32,
                cbsizeheader: u32,
            ) -> super::super::Foundation::LRESULT;
        }
        ::std::mem::transmute(DefRawInputProc(
            ::std::mem::transmute(parawinput),
            ::std::mem::transmute(ninput),
            ::std::mem::transmute(cbsizeheader),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DragDetect<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>,
>(
    hwnd: Param0,
    pt: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DragDetect(
                hwnd: super::super::Foundation::HWND,
                pt: super::super::Foundation::POINT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DragDetect(hwnd.into_param().abi(), pt.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const EXTENDED_BIT: u32 = 16777216u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnableWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    hwnd: Param0,
    benable: Param1,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnableWindow(
                hwnd: super::super::Foundation::HWND,
                benable: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnableWindow(
            hwnd.into_param().abi(),
            benable.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FAKE_KEYSTROKE: u32 = 33554432u32;
pub const FMR_KBD_JIS_TYPE: u32 = 0u32;
pub const FMR_KBD_OASYS_TYPE: u32 = 1u32;
pub const FMV_KBD_OASYS_TYPE: u32 = 2u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(pub u32);
pub const GMMP_USE_DISPLAY_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION =
    GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(1u32);
pub const GMMP_USE_HIGH_RESOLUTION_POINTS: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION =
    GET_MOUSE_MOVE_POINTS_EX_RESOLUTION(2u32);
impl ::std::convert::From<u32> for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GET_MOUSE_MOVE_POINTS_EX_RESOLUTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GRAVE: u32 = 768u32;
pub const GRPSELTAP: u32 = 128u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetActiveWindow() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveWindow() -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(GetActiveWindow())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetAsyncKeyState(vkey: i32) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAsyncKeyState(vkey: i32) -> i16;
        }
        ::std::mem::transmute(GetAsyncKeyState(::std::mem::transmute(vkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCapture() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCapture() -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(GetCapture())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetDoubleClickTime() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetDoubleClickTime() -> u32;
        }
        ::std::mem::transmute(GetDoubleClickTime())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetFocus() -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetFocus() -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(GetFocus())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetKBCodePage() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKBCodePage() -> u32;
        }
        ::std::mem::transmute(GetKBCodePage())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKeyNameTextA(
    lparam: i32,
    lpstring: super::super::Foundation::PSTR,
    cchsize: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyNameTextA(
                lparam: i32,
                lpstring: super::super::Foundation::PSTR,
                cchsize: i32,
            ) -> i32;
        }
        ::std::mem::transmute(GetKeyNameTextA(
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(lpstring),
            ::std::mem::transmute(cchsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKeyNameTextW(
    lparam: i32,
    lpstring: super::super::Foundation::PWSTR,
    cchsize: i32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyNameTextW(
                lparam: i32,
                lpstring: super::super::Foundation::PWSTR,
                cchsize: i32,
            ) -> i32;
        }
        ::std::mem::transmute(GetKeyNameTextW(
            ::std::mem::transmute(lparam),
            ::std::mem::transmute(lpstring),
            ::std::mem::transmute(cchsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetKeyState(nvirtkey: i32) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyState(nvirtkey: i32) -> i16;
        }
        ::std::mem::transmute(GetKeyState(::std::mem::transmute(nvirtkey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn GetKeyboardLayout(idthread: u32) -> super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardLayout(idthread: u32) -> super::TextServices::HKL;
        }
        ::std::mem::transmute(GetKeyboardLayout(::std::mem::transmute(idthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn GetKeyboardLayoutList(nbuff: i32, lplist: *mut super::TextServices::HKL) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardLayoutList(nbuff: i32, lplist: *mut super::TextServices::HKL) -> i32;
        }
        ::std::mem::transmute(GetKeyboardLayoutList(
            ::std::mem::transmute(nbuff),
            ::std::mem::transmute(lplist),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKeyboardLayoutNameA(
    pwszklid: super::super::Foundation::PSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardLayoutNameA(
                pwszklid: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetKeyboardLayoutNameA(::std::mem::transmute(pwszklid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKeyboardLayoutNameW(
    pwszklid: super::super::Foundation::PWSTR,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardLayoutNameW(
                pwszklid: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetKeyboardLayoutNameW(::std::mem::transmute(pwszklid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetKeyboardState(lpkeystate: *mut u8) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardState(lpkeystate: *mut u8) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetKeyboardState(::std::mem::transmute(lpkeystate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetKeyboardType(ntypeflag: i32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetKeyboardType(ntypeflag: i32) -> i32;
        }
        ::std::mem::transmute(GetKeyboardType(::std::mem::transmute(ntypeflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLastInputInfo(plii: *mut LASTINPUTINFO) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetLastInputInfo(::std::mem::transmute(plii)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetMouseMovePointsEx(
    cbsize: u32,
    lppt: *const MOUSEMOVEPOINT,
    lpptbuf: *mut MOUSEMOVEPOINT,
    nbufpoints: i32,
    resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMouseMovePointsEx(
                cbsize: u32,
                lppt: *const MOUSEMOVEPOINT,
                lpptbuf: *mut MOUSEMOVEPOINT,
                nbufpoints: i32,
                resolution: GET_MOUSE_MOVE_POINTS_EX_RESOLUTION,
            ) -> i32;
        }
        ::std::mem::transmute(GetMouseMovePointsEx(
            ::std::mem::transmute(cbsize),
            ::std::mem::transmute(lppt),
            ::std::mem::transmute(lpptbuf),
            ::std::mem::transmute(nbufpoints),
            ::std::mem::transmute(resolution),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputBuffer(pdata: *mut RAWINPUT, pcbsize: *mut u32, cbsizeheader: u32)
                -> u32;
        }
        ::std::mem::transmute(GetRawInputBuffer(
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(pcbsize),
            ::std::mem::transmute(cbsizeheader),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn GetRawInputData<'a, Param0: ::windows::runtime::IntoParam<'a, HRAWINPUT>>(
    hrawinput: Param0,
    uicommand: RAW_INPUT_DATA_COMMAND_FLAGS,
    pdata: *mut ::std::ffi::c_void,
    pcbsize: *mut u32,
    cbsizeheader: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputData(
                hrawinput: HRAWINPUT,
                uicommand: RAW_INPUT_DATA_COMMAND_FLAGS,
                pdata: *mut ::std::ffi::c_void,
                pcbsize: *mut u32,
                cbsizeheader: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetRawInputData(
            hrawinput.into_param().abi(),
            ::std::mem::transmute(uicommand),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(pcbsize),
            ::std::mem::transmute(cbsizeheader),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    uicommand: RAW_INPUT_DEVICE_INFO_COMMAND,
    pdata: *mut ::std::ffi::c_void,
    pcbsize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoA(
                hdevice: super::super::Foundation::HANDLE,
                uicommand: RAW_INPUT_DEVICE_INFO_COMMAND,
                pdata: *mut ::std::ffi::c_void,
                pcbsize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceInfoA(
            hdevice.into_param().abi(),
            ::std::mem::transmute(uicommand),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(pcbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceInfoW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    hdevice: Param0,
    uicommand: RAW_INPUT_DEVICE_INFO_COMMAND,
    pdata: *mut ::std::ffi::c_void,
    pcbsize: *mut u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceInfoW(
                hdevice: super::super::Foundation::HANDLE,
                uicommand: RAW_INPUT_DEVICE_INFO_COMMAND,
                pdata: *mut ::std::ffi::c_void,
                pcbsize: *mut u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceInfoW(
            hdevice.into_param().abi(),
            ::std::mem::transmute(uicommand),
            ::std::mem::transmute(pdata),
            ::std::mem::transmute(pcbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRawInputDeviceList(
    prawinputdevicelist: *mut RAWINPUTDEVICELIST,
    puinumdevices: *mut u32,
    cbsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRawInputDeviceList(
                prawinputdevicelist: *mut RAWINPUTDEVICELIST,
                puinumdevices: *mut u32,
                cbsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetRawInputDeviceList(
            ::std::mem::transmute(prawinputdevicelist),
            ::std::mem::transmute(puinumdevices),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetRegisteredRawInputDevices(
    prawinputdevices: *mut RAWINPUTDEVICE,
    puinumdevices: *mut u32,
    cbsize: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRegisteredRawInputDevices(
                prawinputdevices: *mut RAWINPUTDEVICE,
                puinumdevices: *mut u32,
                cbsize: u32,
            ) -> u32;
        }
        ::std::mem::transmute(GetRegisteredRawInputDevices(
            ::std::mem::transmute(prawinputdevices),
            ::std::mem::transmute(puinumdevices),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const HACEK: u32 = 780u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HARDWAREINPUT {
    pub uMsg: u32,
    pub wParamL: u16,
    pub wParamH: u16,
}
impl HARDWAREINPUT {}
impl ::std::default::Default for HARDWAREINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HARDWAREINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HARDWAREINPUT")
            .field("uMsg", &self.uMsg)
            .field("wParamL", &self.wParamL)
            .field("wParamH", &self.wParamH)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HARDWAREINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.uMsg == other.uMsg && self.wParamL == other.wParamL && self.wParamH == other.wParamH
    }
}
impl ::std::cmp::Eq for HARDWAREINPUT {}
unsafe impl ::windows::runtime::Abi for HARDWAREINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const HOOK_ABOVE: u32 = 777u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct HOT_KEY_MODIFIERS(pub u32);
pub const MOD_ALT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(1u32);
pub const MOD_CONTROL: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(2u32);
pub const MOD_NOREPEAT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(16384u32);
pub const MOD_SHIFT: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(4u32);
pub const MOD_WIN: HOT_KEY_MODIFIERS = HOT_KEY_MODIFIERS(8u32);
impl ::std::convert::From<u32> for HOT_KEY_MODIFIERS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HOT_KEY_MODIFIERS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HOT_KEY_MODIFIERS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HOT_KEY_MODIFIERS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HOT_KEY_MODIFIERS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(
    :: std :: clone :: Clone,
    :: std :: marker :: Copy,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
)]
#[repr(transparent)]
pub struct HRAWINPUT(pub isize);
impl ::std::default::Default for HRAWINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRAWINPUT {}
unsafe impl ::windows::runtime::Abi for HRAWINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct INPUT {
    pub r#type: INPUT_TYPE,
    pub Anonymous: INPUT_0,
}
impl INPUT {}
impl ::std::default::Default for INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INPUT {}
unsafe impl ::windows::runtime::Abi for INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union INPUT_0 {
    pub mi: MOUSEINPUT,
    pub ki: KEYBDINPUT,
    pub hi: HARDWAREINPUT,
}
impl INPUT_0 {}
impl ::std::default::Default for INPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for INPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for INPUT_0 {}
unsafe impl ::windows::runtime::Abi for INPUT_0 {
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
pub struct INPUT_TYPE(pub u32);
pub const INPUT_MOUSE: INPUT_TYPE = INPUT_TYPE(0u32);
pub const INPUT_KEYBOARD: INPUT_TYPE = INPUT_TYPE(1u32);
pub const INPUT_HARDWARE: INPUT_TYPE = INPUT_TYPE(2u32);
impl ::std::convert::From<u32> for INPUT_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for INPUT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for INPUT_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for INPUT_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for INPUT_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for INPUT_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for INPUT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWindowEnabled<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWindowEnabled(
                hwnd: super::super::Foundation::HWND,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsWindowEnabled(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const KANALOK: u32 = 8u32;
pub const KBDALT: u32 = 4u32;
pub const KBDBASE: u32 = 0u32;
pub const KBDCTRL: u32 = 2u32;
pub const KBDGRPSELTAP: u32 = 128u32;
pub const KBDKANA: u32 = 8u32;
pub const KBDLOYA: u32 = 32u32;
pub const KBDNLS_ALPHANUM: u32 = 5u32;
pub const KBDNLS_CODEINPUT: u32 = 10u32;
pub const KBDNLS_CONV_OR_NONCONV: u32 = 15u32;
pub const KBDNLS_HELP_OR_END: u32 = 11u32;
pub const KBDNLS_HIRAGANA: u32 = 6u32;
pub const KBDNLS_HOME_OR_CLEAR: u32 = 12u32;
pub const KBDNLS_INDEX_ALT: u32 = 2u32;
pub const KBDNLS_INDEX_NORMAL: u32 = 1u32;
pub const KBDNLS_KANAEVENT: u32 = 14u32;
pub const KBDNLS_KANALOCK: u32 = 4u32;
pub const KBDNLS_KATAKANA: u32 = 7u32;
pub const KBDNLS_NOEVENT: u32 = 1u32;
pub const KBDNLS_NULL: u32 = 0u32;
pub const KBDNLS_NUMPAD: u32 = 13u32;
pub const KBDNLS_ROMAN: u32 = 9u32;
pub const KBDNLS_SBCSDBCS: u32 = 8u32;
pub const KBDNLS_SEND_BASE_VK: u32 = 2u32;
pub const KBDNLS_SEND_PARAM_VK: u32 = 3u32;
pub const KBDNLS_TYPE_NORMAL: u32 = 1u32;
pub const KBDNLS_TYPE_NULL: u32 = 0u32;
pub const KBDNLS_TYPE_TOGGLE: u32 = 2u32;
pub const KBDROYA: u32 = 16u32;
pub const KBDSHIFT: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KBDTABLE_DESC {
    pub wszDllName: [u16; 32],
    pub dwType: u32,
    pub dwSubType: u32,
}
impl KBDTABLE_DESC {}
impl ::std::default::Default for KBDTABLE_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KBDTABLE_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KBDTABLE_DESC")
            .field("wszDllName", &self.wszDllName)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KBDTABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.wszDllName == other.wszDllName
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
impl ::std::cmp::Eq for KBDTABLE_DESC {}
unsafe impl ::windows::runtime::Abi for KBDTABLE_DESC {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KBDTABLE_MULTI {
    pub nTables: u32,
    pub aKbdTables: [KBDTABLE_DESC; 8],
}
impl KBDTABLE_MULTI {}
impl ::std::default::Default for KBDTABLE_MULTI {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KBDTABLE_MULTI {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KBDTABLE_MULTI")
            .field("nTables", &self.nTables)
            .field("aKbdTables", &self.aKbdTables)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KBDTABLE_MULTI {
    fn eq(&self, other: &Self) -> bool {
        self.nTables == other.nTables && self.aKbdTables == other.aKbdTables
    }
}
impl ::std::cmp::Eq for KBDTABLE_MULTI {}
unsafe impl ::windows::runtime::Abi for KBDTABLE_MULTI {
    type Abi = Self;
    type DefaultType = Self;
}
pub const KBDTABLE_MULTI_MAX: u32 = 8u32;
pub const KBD_TYPE: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KBD_TYPE_INFO {
    pub dwVersion: u32,
    pub dwType: u32,
    pub dwSubType: u32,
}
impl KBD_TYPE_INFO {}
impl ::std::default::Default for KBD_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KBD_TYPE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KBD_TYPE_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KBD_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
impl ::std::cmp::Eq for KBD_TYPE_INFO {}
unsafe impl ::windows::runtime::Abi for KBD_TYPE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const KBD_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct KEYBDINPUT {
    pub wVk: VIRTUAL_KEY,
    pub wScan: u16,
    pub dwFlags: KEYBD_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl KEYBDINPUT {}
impl ::std::default::Default for KEYBDINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for KEYBDINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("KEYBDINPUT")
            .field("wVk", &self.wVk)
            .field("wScan", &self.wScan)
            .field("dwFlags", &self.dwFlags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for KEYBDINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.wVk == other.wVk
            && self.wScan == other.wScan
            && self.dwFlags == other.dwFlags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::std::cmp::Eq for KEYBDINPUT {}
unsafe impl ::windows::runtime::Abi for KEYBDINPUT {
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
pub struct KEYBD_EVENT_FLAGS(pub u32);
pub const KEYEVENTF_EXTENDEDKEY: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(1u32);
pub const KEYEVENTF_KEYUP: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(2u32);
pub const KEYEVENTF_SCANCODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(8u32);
pub const KEYEVENTF_UNICODE: KEYBD_EVENT_FLAGS = KEYBD_EVENT_FLAGS(4u32);
impl ::std::convert::From<u32> for KEYBD_EVENT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KEYBD_EVENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for KEYBD_EVENT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for KEYBD_EVENT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for KEYBD_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const KEYBOARD_TYPE_GENERIC_101: u32 = 4u32;
pub const KEYBOARD_TYPE_JAPAN: u32 = 7u32;
pub const KEYBOARD_TYPE_KOREA: u32 = 8u32;
pub const KEYBOARD_TYPE_UNKNOWN: u32 = 81u32;
pub const KLLF_ALTGR: u32 = 1u32;
pub const KLLF_GLOBAL_ATTRS: u32 = 2u32;
pub const KLLF_LRM_RLM: u32 = 4u32;
pub const KLLF_SHIFTLOCK: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LASTINPUTINFO {
    pub cbSize: u32,
    pub dwTime: u32,
}
impl LASTINPUTINFO {}
impl ::std::default::Default for LASTINPUTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LASTINPUTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LASTINPUTINFO")
            .field("cbSize", &self.cbSize)
            .field("dwTime", &self.dwTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LASTINPUTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwTime == other.dwTime
    }
}
impl ::std::cmp::Eq for LASTINPUTINFO {}
unsafe impl ::windows::runtime::Abi for LASTINPUTINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LIGATURE1 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 1],
}
impl LIGATURE1 {}
impl ::std::default::Default for LIGATURE1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LIGATURE1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LIGATURE1")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LIGATURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for LIGATURE1 {}
unsafe impl ::windows::runtime::Abi for LIGATURE1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LIGATURE2 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 2],
}
impl LIGATURE2 {}
impl ::std::default::Default for LIGATURE2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LIGATURE2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LIGATURE2")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LIGATURE2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for LIGATURE2 {}
unsafe impl ::windows::runtime::Abi for LIGATURE2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LIGATURE3 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 3],
}
impl LIGATURE3 {}
impl ::std::default::Default for LIGATURE3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LIGATURE3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LIGATURE3")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LIGATURE3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for LIGATURE3 {}
unsafe impl ::windows::runtime::Abi for LIGATURE3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LIGATURE4 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 4],
}
impl LIGATURE4 {}
impl ::std::default::Default for LIGATURE4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LIGATURE4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LIGATURE4")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LIGATURE4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for LIGATURE4 {}
unsafe impl ::windows::runtime::Abi for LIGATURE4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct LIGATURE5 {
    pub VirtualKey: u8,
    pub ModificationNumber: u16,
    pub wch: [u16; 5],
}
impl LIGATURE5 {}
impl ::std::default::Default for LIGATURE5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for LIGATURE5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LIGATURE5")
            .field("VirtualKey", &self.VirtualKey)
            .field("ModificationNumber", &self.ModificationNumber)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for LIGATURE5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.ModificationNumber == other.ModificationNumber
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for LIGATURE5 {}
unsafe impl ::windows::runtime::Abi for LIGATURE5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn LoadKeyboardLayoutA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    pwszklid: Param0,
    flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
) -> super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadKeyboardLayoutA(
                pwszklid: super::super::Foundation::PSTR,
                flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
            ) -> super::TextServices::HKL;
        }
        ::std::mem::transmute(LoadKeyboardLayoutA(
            pwszklid.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn LoadKeyboardLayoutW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pwszklid: Param0,
    flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
) -> super::TextServices::HKL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadKeyboardLayoutW(
                pwszklid: super::super::Foundation::PWSTR,
                flags: ACTIVATE_KEYBOARD_LAYOUT_FLAGS,
            ) -> super::TextServices::HKL;
        }
        ::std::mem::transmute(LoadKeyboardLayoutW(
            pwszklid.into_param().abi(),
            ::std::mem::transmute(flags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MACRON: u32 = 772u32;
pub const MICROSOFT_KBD_001_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_002_TYPE: u32 = 3u32;
pub const MICROSOFT_KBD_101A_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_101B_TYPE: u32 = 4u32;
pub const MICROSOFT_KBD_101C_TYPE: u32 = 5u32;
pub const MICROSOFT_KBD_101_TYPE: u32 = 0u32;
pub const MICROSOFT_KBD_103_TYPE: u32 = 6u32;
pub const MICROSOFT_KBD_106_TYPE: u32 = 2u32;
pub const MICROSOFT_KBD_AX_TYPE: u32 = 1u32;
pub const MICROSOFT_KBD_FUNC: u32 = 12u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MODIFIERS {
    pub pVkToBit: *mut VK_TO_BIT,
    pub wMaxModBits: u16,
    pub ModNumber: [u8; 1],
}
impl MODIFIERS {}
impl ::std::default::Default for MODIFIERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MODIFIERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MODIFIERS")
            .field("pVkToBit", &self.pVkToBit)
            .field("wMaxModBits", &self.wMaxModBits)
            .field("ModNumber", &self.ModNumber)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MODIFIERS {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToBit == other.pVkToBit
            && self.wMaxModBits == other.wMaxModBits
            && self.ModNumber == other.ModNumber
    }
}
impl ::std::cmp::Eq for MODIFIERS {}
unsafe impl ::windows::runtime::Abi for MODIFIERS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MOUSEINPUT {
    pub dx: i32,
    pub dy: i32,
    pub mouseData: u32,
    pub dwFlags: MOUSE_EVENT_FLAGS,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl MOUSEINPUT {}
impl ::std::default::Default for MOUSEINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MOUSEINPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MOUSEINPUT")
            .field("dx", &self.dx)
            .field("dy", &self.dy)
            .field("mouseData", &self.mouseData)
            .field("dwFlags", &self.dwFlags)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MOUSEINPUT {
    fn eq(&self, other: &Self) -> bool {
        self.dx == other.dx
            && self.dy == other.dy
            && self.mouseData == other.mouseData
            && self.dwFlags == other.dwFlags
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::std::cmp::Eq for MOUSEINPUT {}
unsafe impl ::windows::runtime::Abi for MOUSEINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MOUSEMOVEPOINT {
    pub x: i32,
    pub y: i32,
    pub time: u32,
    pub dwExtraInfo: usize,
}
impl MOUSEMOVEPOINT {}
impl ::std::default::Default for MOUSEMOVEPOINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MOUSEMOVEPOINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MOUSEMOVEPOINT")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("time", &self.time)
            .field("dwExtraInfo", &self.dwExtraInfo)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MOUSEMOVEPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.time == other.time
            && self.dwExtraInfo == other.dwExtraInfo
    }
}
impl ::std::cmp::Eq for MOUSEMOVEPOINT {}
unsafe impl ::windows::runtime::Abi for MOUSEMOVEPOINT {
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
pub struct MOUSE_EVENT_FLAGS(pub u32);
pub const MOUSEEVENTF_ABSOLUTE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32768u32);
pub const MOUSEEVENTF_LEFTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2u32);
pub const MOUSEEVENTF_LEFTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4u32);
pub const MOUSEEVENTF_MIDDLEDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(32u32);
pub const MOUSEEVENTF_MIDDLEUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(64u32);
pub const MOUSEEVENTF_MOVE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(1u32);
pub const MOUSEEVENTF_RIGHTDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(8u32);
pub const MOUSEEVENTF_RIGHTUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16u32);
pub const MOUSEEVENTF_WHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(2048u32);
pub const MOUSEEVENTF_XDOWN: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(128u32);
pub const MOUSEEVENTF_XUP: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(256u32);
pub const MOUSEEVENTF_HWHEEL: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(4096u32);
pub const MOUSEEVENTF_MOVE_NOCOALESCE: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(8192u32);
pub const MOUSEEVENTF_VIRTUALDESK: MOUSE_EVENT_FLAGS = MOUSE_EVENT_FLAGS(16384u32);
impl ::std::convert::From<u32> for MOUSE_EVENT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MOUSE_EVENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MOUSE_EVENT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MOUSE_EVENT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MOUSE_EVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapVirtualKeyA(ucode: u32, umaptype: u32) -> u32;
        }
        ::std::mem::transmute(MapVirtualKeyA(
            ::std::mem::transmute(ucode),
            ::std::mem::transmute(umaptype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn MapVirtualKeyExA<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    ucode: u32,
    umaptype: u32,
    dwhkl: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapVirtualKeyExA(ucode: u32, umaptype: u32, dwhkl: super::TextServices::HKL) -> u32;
        }
        ::std::mem::transmute(MapVirtualKeyExA(
            ::std::mem::transmute(ucode),
            ::std::mem::transmute(umaptype),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn MapVirtualKeyExW<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    ucode: u32,
    umaptype: u32,
    dwhkl: Param2,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapVirtualKeyExW(ucode: u32, umaptype: u32, dwhkl: super::TextServices::HKL) -> u32;
        }
        ::std::mem::transmute(MapVirtualKeyExW(
            ::std::mem::transmute(ucode),
            ::std::mem::transmute(umaptype),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MapVirtualKeyW(ucode: u32, umaptype: u32) -> u32;
        }
        ::std::mem::transmute(MapVirtualKeyW(
            ::std::mem::transmute(ucode),
            ::std::mem::transmute(umaptype),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const NEC_KBD_106_TYPE: u32 = 5u32;
pub const NEC_KBD_H_MODE_TYPE: u32 = 3u32;
pub const NEC_KBD_LAPTOP_TYPE: u32 = 4u32;
pub const NEC_KBD_NORMAL_TYPE: u32 = 1u32;
pub const NEC_KBD_N_MODE_TYPE: u32 = 2u32;
pub const NLSKBD_INFO_ACCESSIBILITY_KEYMAP: u32 = 2u32;
pub const NLSKBD_INFO_EMURATE_101_KEYBOARD: u32 = 16u32;
pub const NLSKBD_INFO_EMURATE_106_KEYBOARD: u32 = 32u32;
pub const NLSKBD_INFO_SEND_IME_NOTIFICATION: u32 = 1u32;
pub const NLSKBD_OEM_AX: u32 = 1u32;
pub const NLSKBD_OEM_DEC: u32 = 24u32;
pub const NLSKBD_OEM_EPSON: u32 = 4u32;
pub const NLSKBD_OEM_FUJITSU: u32 = 5u32;
pub const NLSKBD_OEM_IBM: u32 = 7u32;
pub const NLSKBD_OEM_MATSUSHITA: u32 = 10u32;
pub const NLSKBD_OEM_MICROSOFT: u32 = 0u32;
pub const NLSKBD_OEM_NEC: u32 = 13u32;
pub const NLSKBD_OEM_TOSHIBA: u32 = 18u32;
pub const OGONEK: u32 = 808u32;
pub const OVERSCORE: u32 = 773u32;
#[inline]
pub unsafe fn OemKeyScan(woemchar: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OemKeyScan(woemchar: u16) -> u32;
        }
        ::std::mem::transmute(OemKeyScan(::std::mem::transmute(woemchar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAWHID {
    pub dwSizeHid: u32,
    pub dwCount: u32,
    pub bRawData: [u8; 1],
}
impl RAWHID {}
impl ::std::default::Default for RAWHID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWHID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWHID")
            .field("dwSizeHid", &self.dwSizeHid)
            .field("dwCount", &self.dwCount)
            .field("bRawData", &self.bRawData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RAWHID {
    fn eq(&self, other: &Self) -> bool {
        self.dwSizeHid == other.dwSizeHid
            && self.dwCount == other.dwCount
            && self.bRawData == other.bRawData
    }
}
impl ::std::cmp::Eq for RAWHID {}
unsafe impl ::windows::runtime::Abi for RAWHID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUT {
    pub header: RAWINPUTHEADER,
    pub data: RAWINPUT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union RAWINPUT_0 {
    pub mouse: RAWMOUSE,
    pub keyboard: RAWKEYBOARD,
    pub hid: RAWHID,
}
impl RAWINPUT_0 {}
impl ::std::default::Default for RAWINPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAWINPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAWINPUT_0 {}
unsafe impl ::windows::runtime::Abi for RAWINPUT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICE {
    pub usUsagePage: u16,
    pub usUsage: u16,
    pub dwFlags: RAWINPUTDEVICE_FLAGS,
    pub hwndTarget: super::super::Foundation::HWND,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTDEVICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTDEVICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTDEVICE")
            .field("usUsagePage", &self.usUsagePage)
            .field("usUsage", &self.usUsage)
            .field("dwFlags", &self.dwFlags)
            .field("hwndTarget", &self.hwndTarget)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTDEVICE {
    fn eq(&self, other: &Self) -> bool {
        self.usUsagePage == other.usUsagePage
            && self.usUsage == other.usUsage
            && self.dwFlags == other.dwFlags
            && self.hwndTarget == other.hwndTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTDEVICE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTDEVICELIST {
    pub hDevice: super::super::Foundation::HANDLE,
    pub dwType: RID_DEVICE_INFO_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTDEVICELIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTDEVICELIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTDEVICELIST")
            .field("hDevice", &self.hDevice)
            .field("dwType", &self.dwType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTDEVICELIST {
    fn eq(&self, other: &Self) -> bool {
        self.hDevice == other.hDevice && self.dwType == other.dwType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTDEVICELIST {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICELIST {
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
pub struct RAWINPUTDEVICE_FLAGS(pub u32);
pub const RIDEV_REMOVE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1u32);
pub const RIDEV_EXCLUDE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(16u32);
pub const RIDEV_PAGEONLY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(32u32);
pub const RIDEV_NOLEGACY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(48u32);
pub const RIDEV_INPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(256u32);
pub const RIDEV_CAPTUREMOUSE: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_NOHOTKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(512u32);
pub const RIDEV_APPKEYS: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(1024u32);
pub const RIDEV_EXINPUTSINK: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(4096u32);
pub const RIDEV_DEVNOTIFY: RAWINPUTDEVICE_FLAGS = RAWINPUTDEVICE_FLAGS(8192u32);
impl ::std::convert::From<u32> for RAWINPUTDEVICE_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAWINPUTDEVICE_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAWINPUTDEVICE_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAWINPUTDEVICE_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAWINPUTDEVICE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAWINPUTHEADER {
    pub dwType: u32,
    pub dwSize: u32,
    pub hDevice: super::super::Foundation::HANDLE,
    pub wParam: super::super::Foundation::WPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAWINPUTHEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAWINPUTHEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWINPUTHEADER")
            .field("dwType", &self.dwType)
            .field("dwSize", &self.dwSize)
            .field("hDevice", &self.hDevice)
            .field("wParam", &self.wParam)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAWINPUTHEADER {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwSize == other.dwSize
            && self.hDevice == other.hDevice
            && self.wParam == other.wParam
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAWINPUTHEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAWINPUTHEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAWKEYBOARD {
    pub MakeCode: u16,
    pub Flags: u16,
    pub Reserved: u16,
    pub VKey: u16,
    pub Message: u32,
    pub ExtraInformation: u32,
}
impl RAWKEYBOARD {}
impl ::std::default::Default for RAWKEYBOARD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWKEYBOARD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAWKEYBOARD")
            .field("MakeCode", &self.MakeCode)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("VKey", &self.VKey)
            .field("Message", &self.Message)
            .field("ExtraInformation", &self.ExtraInformation)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RAWKEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.MakeCode == other.MakeCode
            && self.Flags == other.Flags
            && self.Reserved == other.Reserved
            && self.VKey == other.VKey
            && self.Message == other.Message
            && self.ExtraInformation == other.ExtraInformation
    }
}
impl ::std::cmp::Eq for RAWKEYBOARD {}
unsafe impl ::windows::runtime::Abi for RAWKEYBOARD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAWMOUSE {
    pub usFlags: u16,
    pub Anonymous: RAWMOUSE_0,
    pub ulRawButtons: u32,
    pub lLastX: i32,
    pub lLastY: i32,
    pub ulExtraInformation: u32,
}
impl RAWMOUSE {}
impl ::std::default::Default for RAWMOUSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAWMOUSE {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union RAWMOUSE_0 {
    pub ulButtons: u32,
    pub Anonymous: RAWMOUSE_0_0,
}
impl RAWMOUSE_0 {}
impl ::std::default::Default for RAWMOUSE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RAWMOUSE_0 {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAWMOUSE_0_0 {
    pub usButtonFlags: u16,
    pub usButtonData: u16,
}
impl RAWMOUSE_0_0 {}
impl ::std::default::Default for RAWMOUSE_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAWMOUSE_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct")
            .field("usButtonFlags", &self.usButtonFlags)
            .field("usButtonData", &self.usButtonData)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RAWMOUSE_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.usButtonFlags == other.usButtonFlags && self.usButtonData == other.usButtonData
    }
}
impl ::std::cmp::Eq for RAWMOUSE_0_0 {}
unsafe impl ::windows::runtime::Abi for RAWMOUSE_0_0 {
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
pub struct RAW_INPUT_DATA_COMMAND_FLAGS(pub u32);
pub const RID_HEADER: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435461u32);
pub const RID_INPUT: RAW_INPUT_DATA_COMMAND_FLAGS = RAW_INPUT_DATA_COMMAND_FLAGS(268435459u32);
impl ::std::convert::From<u32> for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAW_INPUT_DATA_COMMAND_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAW_INPUT_DATA_COMMAND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
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
pub struct RAW_INPUT_DEVICE_INFO_COMMAND(pub u32);
pub const RIDI_PREPARSEDDATA: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870917u32);
pub const RIDI_DEVICENAME: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870919u32);
pub const RIDI_DEVICEINFO: RAW_INPUT_DEVICE_INFO_COMMAND =
    RAW_INPUT_DEVICE_INFO_COMMAND(536870923u32);
impl ::std::convert::From<u32> for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAW_INPUT_DEVICE_INFO_COMMAND {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAW_INPUT_DEVICE_INFO_COMMAND {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO {
    pub cbSize: u32,
    pub dwType: RID_DEVICE_INFO_TYPE,
    pub Anonymous: RID_DEVICE_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RID_DEVICE_INFO_0 {
    pub mouse: RID_DEVICE_INFO_MOUSE,
    pub keyboard: RID_DEVICE_INFO_KEYBOARD,
    pub hid: RID_DEVICE_INFO_HID,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RID_DEVICE_INFO_HID {
    pub dwVendorId: u32,
    pub dwProductId: u32,
    pub dwVersionNumber: u32,
    pub usUsagePage: u16,
    pub usUsage: u16,
}
impl RID_DEVICE_INFO_HID {}
impl ::std::default::Default for RID_DEVICE_INFO_HID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RID_DEVICE_INFO_HID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_HID")
            .field("dwVendorId", &self.dwVendorId)
            .field("dwProductId", &self.dwProductId)
            .field("dwVersionNumber", &self.dwVersionNumber)
            .field("usUsagePage", &self.usUsagePage)
            .field("usUsage", &self.usUsage)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_HID {
    fn eq(&self, other: &Self) -> bool {
        self.dwVendorId == other.dwVendorId
            && self.dwProductId == other.dwProductId
            && self.dwVersionNumber == other.dwVersionNumber
            && self.usUsagePage == other.usUsagePage
            && self.usUsage == other.usUsage
    }
}
impl ::std::cmp::Eq for RID_DEVICE_INFO_HID {}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_HID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RID_DEVICE_INFO_KEYBOARD {
    pub dwType: u32,
    pub dwSubType: u32,
    pub dwKeyboardMode: u32,
    pub dwNumberOfFunctionKeys: u32,
    pub dwNumberOfIndicators: u32,
    pub dwNumberOfKeysTotal: u32,
}
impl RID_DEVICE_INFO_KEYBOARD {}
impl ::std::default::Default for RID_DEVICE_INFO_KEYBOARD {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RID_DEVICE_INFO_KEYBOARD {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_KEYBOARD")
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .field("dwKeyboardMode", &self.dwKeyboardMode)
            .field("dwNumberOfFunctionKeys", &self.dwNumberOfFunctionKeys)
            .field("dwNumberOfIndicators", &self.dwNumberOfIndicators)
            .field("dwNumberOfKeysTotal", &self.dwNumberOfKeysTotal)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_KEYBOARD {
    fn eq(&self, other: &Self) -> bool {
        self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
            && self.dwKeyboardMode == other.dwKeyboardMode
            && self.dwNumberOfFunctionKeys == other.dwNumberOfFunctionKeys
            && self.dwNumberOfIndicators == other.dwNumberOfIndicators
            && self.dwNumberOfKeysTotal == other.dwNumberOfKeysTotal
    }
}
impl ::std::cmp::Eq for RID_DEVICE_INFO_KEYBOARD {}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_KEYBOARD {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RID_DEVICE_INFO_MOUSE {
    pub dwId: u32,
    pub dwNumberOfButtons: u32,
    pub dwSampleRate: u32,
    pub fHasHorizontalWheel: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RID_DEVICE_INFO_MOUSE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RID_DEVICE_INFO_MOUSE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RID_DEVICE_INFO_MOUSE")
            .field("dwId", &self.dwId)
            .field("dwNumberOfButtons", &self.dwNumberOfButtons)
            .field("dwSampleRate", &self.dwSampleRate)
            .field("fHasHorizontalWheel", &self.fHasHorizontalWheel)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RID_DEVICE_INFO_MOUSE {
    fn eq(&self, other: &Self) -> bool {
        self.dwId == other.dwId
            && self.dwNumberOfButtons == other.dwNumberOfButtons
            && self.dwSampleRate == other.dwSampleRate
            && self.fHasHorizontalWheel == other.fHasHorizontalWheel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RID_DEVICE_INFO_MOUSE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_MOUSE {
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
pub struct RID_DEVICE_INFO_TYPE(pub u32);
pub const RIM_TYPEMOUSE: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(0u32);
pub const RIM_TYPEKEYBOARD: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(1u32);
pub const RIM_TYPEHID: RID_DEVICE_INFO_TYPE = RID_DEVICE_INFO_TYPE(2u32);
impl ::std::convert::From<u32> for RID_DEVICE_INFO_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RID_DEVICE_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RID_DEVICE_INFO_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RID_DEVICE_INFO_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RID_DEVICE_INFO_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RING: u32 = 778u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterHotKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    id: i32,
    fsmodifiers: HOT_KEY_MODIFIERS,
    vk: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterHotKey(
                hwnd: super::super::Foundation::HWND,
                id: i32,
                fsmodifiers: HOT_KEY_MODIFIERS,
                vk: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterHotKey(
            hwnd.into_param().abi(),
            ::std::mem::transmute(id),
            ::std::mem::transmute(fsmodifiers),
            ::std::mem::transmute(vk),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterRawInputDevices(
    prawinputdevices: *const RAWINPUTDEVICE,
    uinumdevices: u32,
    cbsize: u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterRawInputDevices(
                prawinputdevices: *const RAWINPUTDEVICE,
                uinumdevices: u32,
                cbsize: u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterRawInputDevices(
            ::std::mem::transmute(prawinputdevices),
            ::std::mem::transmute(uinumdevices),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseCapture() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseCapture() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ReleaseCapture())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SCANCODE_ALT: u32 = 56u32;
pub const SCANCODE_CTRL: u32 = 29u32;
pub const SCANCODE_LSHIFT: u32 = 42u32;
pub const SCANCODE_LWIN: u32 = 91u32;
pub const SCANCODE_NUMPAD_FIRST: u32 = 71u32;
pub const SCANCODE_NUMPAD_LAST: u32 = 82u32;
pub const SCANCODE_RSHIFT: u32 = 54u32;
pub const SCANCODE_RWIN: u32 = 92u32;
pub const SCANCODE_THAI_LAYOUT_TOGGLE: u32 = 41u32;
pub const SGCAPS: u32 = 2u32;
pub const SHFT_INVALID: u32 = 15u32;
#[inline]
pub unsafe fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SendInput(cinputs: u32, pinputs: *const INPUT, cbsize: i32) -> u32;
        }
        ::std::mem::transmute(SendInput(
            ::std::mem::transmute(cinputs),
            ::std::mem::transmute(pinputs),
            ::std::mem::transmute(cbsize),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetActiveWindow<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetActiveWindow(
                hwnd: super::super::Foundation::HWND,
            ) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(SetActiveWindow(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetCapture<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCapture(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(SetCapture(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetDoubleClickTime(param0: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetDoubleClickTime(param0: u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetDoubleClickTime(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetFocus<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
) -> super::super::Foundation::HWND {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFocus(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::HWND;
        }
        ::std::mem::transmute(SetFocus(hwnd.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetKeyboardState(lpkeystate: *const u8) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetKeyboardState(lpkeystate: *const u8) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetKeyboardState(::std::mem::transmute(lpkeystate)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwapMouseButton<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    fswap: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwapMouseButton(
                fswap: super::super::Foundation::BOOL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SwapMouseButton(fswap.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const TILDE: u32 = 771u32;
pub const TONOS: u32 = 900u32;
pub const TOSHIBA_KBD_DESKTOP_TYPE: u32 = 13u32;
pub const TOSHIBA_KBD_LAPTOP_TYPE: u32 = 15u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct TRACKMOUSEEVENT {
    pub cbSize: u32,
    pub dwFlags: TRACKMOUSEEVENT_FLAGS,
    pub hwndTrack: super::super::Foundation::HWND,
    pub dwHoverTime: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl TRACKMOUSEEVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for TRACKMOUSEEVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for TRACKMOUSEEVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TRACKMOUSEEVENT")
            .field("cbSize", &self.cbSize)
            .field("dwFlags", &self.dwFlags)
            .field("hwndTrack", &self.hwndTrack)
            .field("dwHoverTime", &self.dwHoverTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for TRACKMOUSEEVENT {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize
            && self.dwFlags == other.dwFlags
            && self.hwndTrack == other.hwndTrack
            && self.dwHoverTime == other.dwHoverTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for TRACKMOUSEEVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for TRACKMOUSEEVENT {
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
pub struct TRACKMOUSEEVENT_FLAGS(pub u32);
pub const TME_CANCEL: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(2147483648u32);
pub const TME_HOVER: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(1u32);
pub const TME_LEAVE: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(2u32);
pub const TME_NONCLIENT: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(16u32);
pub const TME_QUERY: TRACKMOUSEEVENT_FLAGS = TRACKMOUSEEVENT_FLAGS(1073741824u32);
impl ::std::convert::From<u32> for TRACKMOUSEEVENT_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TRACKMOUSEEVENT_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for TRACKMOUSEEVENT_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for TRACKMOUSEEVENT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[inline]
pub unsafe fn ToAscii(
    uvirtkey: u32,
    uscancode: u32,
    lpkeystate: *const u8,
    lpchar: *mut u16,
    uflags: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ToAscii(
                uvirtkey: u32,
                uscancode: u32,
                lpkeystate: *const u8,
                lpchar: *mut u16,
                uflags: u32,
            ) -> i32;
        }
        ::std::mem::transmute(ToAscii(
            ::std::mem::transmute(uvirtkey),
            ::std::mem::transmute(uscancode),
            ::std::mem::transmute(lpkeystate),
            ::std::mem::transmute(lpchar),
            ::std::mem::transmute(uflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn ToAsciiEx<'a, Param5: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>>(
    uvirtkey: u32,
    uscancode: u32,
    lpkeystate: *const u8,
    lpchar: *mut u16,
    uflags: u32,
    dwhkl: Param5,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ToAsciiEx(
                uvirtkey: u32,
                uscancode: u32,
                lpkeystate: *const u8,
                lpchar: *mut u16,
                uflags: u32,
                dwhkl: super::TextServices::HKL,
            ) -> i32;
        }
        ::std::mem::transmute(ToAsciiEx(
            ::std::mem::transmute(uvirtkey),
            ::std::mem::transmute(uscancode),
            ::std::mem::transmute(lpkeystate),
            ::std::mem::transmute(lpchar),
            ::std::mem::transmute(uflags),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ToUnicode(
    wvirtkey: u32,
    wscancode: u32,
    lpkeystate: *const u8,
    pwszbuff: super::super::Foundation::PWSTR,
    cchbuff: i32,
    wflags: u32,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ToUnicode(
                wvirtkey: u32,
                wscancode: u32,
                lpkeystate: *const u8,
                pwszbuff: super::super::Foundation::PWSTR,
                cchbuff: i32,
                wflags: u32,
            ) -> i32;
        }
        ::std::mem::transmute(ToUnicode(
            ::std::mem::transmute(wvirtkey),
            ::std::mem::transmute(wscancode),
            ::std::mem::transmute(lpkeystate),
            ::std::mem::transmute(pwszbuff),
            ::std::mem::transmute(cchbuff),
            ::std::mem::transmute(wflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn ToUnicodeEx<
    'a,
    Param6: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    wvirtkey: u32,
    wscancode: u32,
    lpkeystate: *const u8,
    pwszbuff: super::super::Foundation::PWSTR,
    cchbuff: i32,
    wflags: u32,
    dwhkl: Param6,
) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ToUnicodeEx(
                wvirtkey: u32,
                wscancode: u32,
                lpkeystate: *const u8,
                pwszbuff: super::super::Foundation::PWSTR,
                cchbuff: i32,
                wflags: u32,
                dwhkl: super::TextServices::HKL,
            ) -> i32;
        }
        ::std::mem::transmute(ToUnicodeEx(
            ::std::mem::transmute(wvirtkey),
            ::std::mem::transmute(wscancode),
            ::std::mem::transmute(lpkeystate),
            ::std::mem::transmute(pwszbuff),
            ::std::mem::transmute(cchbuff),
            ::std::mem::transmute(wflags),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TrackMouseEvent(
    lpeventtrack: *mut TRACKMOUSEEVENT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TrackMouseEvent(
                lpeventtrack: *mut TRACKMOUSEEVENT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(TrackMouseEvent(::std::mem::transmute(lpeventtrack)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const UMLAUT: u32 = 776u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_TextServices"))]
#[inline]
pub unsafe fn UnloadKeyboardLayout<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    hkl: Param0,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnloadKeyboardLayout(
                hkl: super::TextServices::HKL,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnloadKeyboardLayout(hkl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterHotKey<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
>(
    hwnd: Param0,
    id: i32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterHotKey(
                hwnd: super::super::Foundation::HWND,
                id: i32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterHotKey(
            hwnd.into_param().abi(),
            ::std::mem::transmute(id),
        ))
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
pub struct VIRTUAL_KEY(pub u16);
pub const VK_0: VIRTUAL_KEY = VIRTUAL_KEY(48u16);
pub const VK_1: VIRTUAL_KEY = VIRTUAL_KEY(49u16);
pub const VK_2: VIRTUAL_KEY = VIRTUAL_KEY(50u16);
pub const VK_3: VIRTUAL_KEY = VIRTUAL_KEY(51u16);
pub const VK_4: VIRTUAL_KEY = VIRTUAL_KEY(52u16);
pub const VK_5: VIRTUAL_KEY = VIRTUAL_KEY(53u16);
pub const VK_6: VIRTUAL_KEY = VIRTUAL_KEY(54u16);
pub const VK_7: VIRTUAL_KEY = VIRTUAL_KEY(55u16);
pub const VK_8: VIRTUAL_KEY = VIRTUAL_KEY(56u16);
pub const VK_9: VIRTUAL_KEY = VIRTUAL_KEY(57u16);
pub const VK_A: VIRTUAL_KEY = VIRTUAL_KEY(65u16);
pub const VK_B: VIRTUAL_KEY = VIRTUAL_KEY(66u16);
pub const VK_C: VIRTUAL_KEY = VIRTUAL_KEY(67u16);
pub const VK_D: VIRTUAL_KEY = VIRTUAL_KEY(68u16);
pub const VK_E: VIRTUAL_KEY = VIRTUAL_KEY(69u16);
pub const VK_F: VIRTUAL_KEY = VIRTUAL_KEY(70u16);
pub const VK_G: VIRTUAL_KEY = VIRTUAL_KEY(71u16);
pub const VK_H: VIRTUAL_KEY = VIRTUAL_KEY(72u16);
pub const VK_I: VIRTUAL_KEY = VIRTUAL_KEY(73u16);
pub const VK_J: VIRTUAL_KEY = VIRTUAL_KEY(74u16);
pub const VK_K: VIRTUAL_KEY = VIRTUAL_KEY(75u16);
pub const VK_L: VIRTUAL_KEY = VIRTUAL_KEY(76u16);
pub const VK_M: VIRTUAL_KEY = VIRTUAL_KEY(77u16);
pub const VK_N: VIRTUAL_KEY = VIRTUAL_KEY(78u16);
pub const VK_O: VIRTUAL_KEY = VIRTUAL_KEY(79u16);
pub const VK_P: VIRTUAL_KEY = VIRTUAL_KEY(80u16);
pub const VK_Q: VIRTUAL_KEY = VIRTUAL_KEY(81u16);
pub const VK_R: VIRTUAL_KEY = VIRTUAL_KEY(82u16);
pub const VK_S: VIRTUAL_KEY = VIRTUAL_KEY(83u16);
pub const VK_T: VIRTUAL_KEY = VIRTUAL_KEY(84u16);
pub const VK_U: VIRTUAL_KEY = VIRTUAL_KEY(85u16);
pub const VK_V: VIRTUAL_KEY = VIRTUAL_KEY(86u16);
pub const VK_W: VIRTUAL_KEY = VIRTUAL_KEY(87u16);
pub const VK_X: VIRTUAL_KEY = VIRTUAL_KEY(88u16);
pub const VK_Y: VIRTUAL_KEY = VIRTUAL_KEY(89u16);
pub const VK_Z: VIRTUAL_KEY = VIRTUAL_KEY(90u16);
pub const VK_LBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(1u16);
pub const VK_RBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(2u16);
pub const VK_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(3u16);
pub const VK_MBUTTON: VIRTUAL_KEY = VIRTUAL_KEY(4u16);
pub const VK_XBUTTON1: VIRTUAL_KEY = VIRTUAL_KEY(5u16);
pub const VK_XBUTTON2: VIRTUAL_KEY = VIRTUAL_KEY(6u16);
pub const VK_BACK: VIRTUAL_KEY = VIRTUAL_KEY(8u16);
pub const VK_TAB: VIRTUAL_KEY = VIRTUAL_KEY(9u16);
pub const VK_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(12u16);
pub const VK_RETURN: VIRTUAL_KEY = VIRTUAL_KEY(13u16);
pub const VK_SHIFT: VIRTUAL_KEY = VIRTUAL_KEY(16u16);
pub const VK_CONTROL: VIRTUAL_KEY = VIRTUAL_KEY(17u16);
pub const VK_MENU: VIRTUAL_KEY = VIRTUAL_KEY(18u16);
pub const VK_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(19u16);
pub const VK_CAPITAL: VIRTUAL_KEY = VIRTUAL_KEY(20u16);
pub const VK_KANA: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANGEUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_HANGUL: VIRTUAL_KEY = VIRTUAL_KEY(21u16);
pub const VK_IME_ON: VIRTUAL_KEY = VIRTUAL_KEY(22u16);
pub const VK_JUNJA: VIRTUAL_KEY = VIRTUAL_KEY(23u16);
pub const VK_FINAL: VIRTUAL_KEY = VIRTUAL_KEY(24u16);
pub const VK_HANJA: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_KANJI: VIRTUAL_KEY = VIRTUAL_KEY(25u16);
pub const VK_IME_OFF: VIRTUAL_KEY = VIRTUAL_KEY(26u16);
pub const VK_ESCAPE: VIRTUAL_KEY = VIRTUAL_KEY(27u16);
pub const VK_CONVERT: VIRTUAL_KEY = VIRTUAL_KEY(28u16);
pub const VK_NONCONVERT: VIRTUAL_KEY = VIRTUAL_KEY(29u16);
pub const VK_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(30u16);
pub const VK_MODECHANGE: VIRTUAL_KEY = VIRTUAL_KEY(31u16);
pub const VK_SPACE: VIRTUAL_KEY = VIRTUAL_KEY(32u16);
pub const VK_PRIOR: VIRTUAL_KEY = VIRTUAL_KEY(33u16);
pub const VK_NEXT: VIRTUAL_KEY = VIRTUAL_KEY(34u16);
pub const VK_END: VIRTUAL_KEY = VIRTUAL_KEY(35u16);
pub const VK_HOME: VIRTUAL_KEY = VIRTUAL_KEY(36u16);
pub const VK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(37u16);
pub const VK_UP: VIRTUAL_KEY = VIRTUAL_KEY(38u16);
pub const VK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(39u16);
pub const VK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(40u16);
pub const VK_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(41u16);
pub const VK_PRINT: VIRTUAL_KEY = VIRTUAL_KEY(42u16);
pub const VK_EXECUTE: VIRTUAL_KEY = VIRTUAL_KEY(43u16);
pub const VK_SNAPSHOT: VIRTUAL_KEY = VIRTUAL_KEY(44u16);
pub const VK_INSERT: VIRTUAL_KEY = VIRTUAL_KEY(45u16);
pub const VK_DELETE: VIRTUAL_KEY = VIRTUAL_KEY(46u16);
pub const VK_HELP: VIRTUAL_KEY = VIRTUAL_KEY(47u16);
pub const VK_LWIN: VIRTUAL_KEY = VIRTUAL_KEY(91u16);
pub const VK_RWIN: VIRTUAL_KEY = VIRTUAL_KEY(92u16);
pub const VK_APPS: VIRTUAL_KEY = VIRTUAL_KEY(93u16);
pub const VK_SLEEP: VIRTUAL_KEY = VIRTUAL_KEY(95u16);
pub const VK_NUMPAD0: VIRTUAL_KEY = VIRTUAL_KEY(96u16);
pub const VK_NUMPAD1: VIRTUAL_KEY = VIRTUAL_KEY(97u16);
pub const VK_NUMPAD2: VIRTUAL_KEY = VIRTUAL_KEY(98u16);
pub const VK_NUMPAD3: VIRTUAL_KEY = VIRTUAL_KEY(99u16);
pub const VK_NUMPAD4: VIRTUAL_KEY = VIRTUAL_KEY(100u16);
pub const VK_NUMPAD5: VIRTUAL_KEY = VIRTUAL_KEY(101u16);
pub const VK_NUMPAD6: VIRTUAL_KEY = VIRTUAL_KEY(102u16);
pub const VK_NUMPAD7: VIRTUAL_KEY = VIRTUAL_KEY(103u16);
pub const VK_NUMPAD8: VIRTUAL_KEY = VIRTUAL_KEY(104u16);
pub const VK_NUMPAD9: VIRTUAL_KEY = VIRTUAL_KEY(105u16);
pub const VK_MULTIPLY: VIRTUAL_KEY = VIRTUAL_KEY(106u16);
pub const VK_ADD: VIRTUAL_KEY = VIRTUAL_KEY(107u16);
pub const VK_SEPARATOR: VIRTUAL_KEY = VIRTUAL_KEY(108u16);
pub const VK_SUBTRACT: VIRTUAL_KEY = VIRTUAL_KEY(109u16);
pub const VK_DECIMAL: VIRTUAL_KEY = VIRTUAL_KEY(110u16);
pub const VK_DIVIDE: VIRTUAL_KEY = VIRTUAL_KEY(111u16);
pub const VK_F1: VIRTUAL_KEY = VIRTUAL_KEY(112u16);
pub const VK_F2: VIRTUAL_KEY = VIRTUAL_KEY(113u16);
pub const VK_F3: VIRTUAL_KEY = VIRTUAL_KEY(114u16);
pub const VK_F4: VIRTUAL_KEY = VIRTUAL_KEY(115u16);
pub const VK_F5: VIRTUAL_KEY = VIRTUAL_KEY(116u16);
pub const VK_F6: VIRTUAL_KEY = VIRTUAL_KEY(117u16);
pub const VK_F7: VIRTUAL_KEY = VIRTUAL_KEY(118u16);
pub const VK_F8: VIRTUAL_KEY = VIRTUAL_KEY(119u16);
pub const VK_F9: VIRTUAL_KEY = VIRTUAL_KEY(120u16);
pub const VK_F10: VIRTUAL_KEY = VIRTUAL_KEY(121u16);
pub const VK_F11: VIRTUAL_KEY = VIRTUAL_KEY(122u16);
pub const VK_F12: VIRTUAL_KEY = VIRTUAL_KEY(123u16);
pub const VK_F13: VIRTUAL_KEY = VIRTUAL_KEY(124u16);
pub const VK_F14: VIRTUAL_KEY = VIRTUAL_KEY(125u16);
pub const VK_F15: VIRTUAL_KEY = VIRTUAL_KEY(126u16);
pub const VK_F16: VIRTUAL_KEY = VIRTUAL_KEY(127u16);
pub const VK_F17: VIRTUAL_KEY = VIRTUAL_KEY(128u16);
pub const VK_F18: VIRTUAL_KEY = VIRTUAL_KEY(129u16);
pub const VK_F19: VIRTUAL_KEY = VIRTUAL_KEY(130u16);
pub const VK_F20: VIRTUAL_KEY = VIRTUAL_KEY(131u16);
pub const VK_F21: VIRTUAL_KEY = VIRTUAL_KEY(132u16);
pub const VK_F22: VIRTUAL_KEY = VIRTUAL_KEY(133u16);
pub const VK_F23: VIRTUAL_KEY = VIRTUAL_KEY(134u16);
pub const VK_F24: VIRTUAL_KEY = VIRTUAL_KEY(135u16);
pub const VK_NAVIGATION_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(136u16);
pub const VK_NAVIGATION_MENU: VIRTUAL_KEY = VIRTUAL_KEY(137u16);
pub const VK_NAVIGATION_UP: VIRTUAL_KEY = VIRTUAL_KEY(138u16);
pub const VK_NAVIGATION_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(139u16);
pub const VK_NAVIGATION_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(140u16);
pub const VK_NAVIGATION_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(141u16);
pub const VK_NAVIGATION_ACCEPT: VIRTUAL_KEY = VIRTUAL_KEY(142u16);
pub const VK_NAVIGATION_CANCEL: VIRTUAL_KEY = VIRTUAL_KEY(143u16);
pub const VK_NUMLOCK: VIRTUAL_KEY = VIRTUAL_KEY(144u16);
pub const VK_SCROLL: VIRTUAL_KEY = VIRTUAL_KEY(145u16);
pub const VK_OEM_NEC_EQUAL: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_FJ_JISHO: VIRTUAL_KEY = VIRTUAL_KEY(146u16);
pub const VK_OEM_FJ_MASSHOU: VIRTUAL_KEY = VIRTUAL_KEY(147u16);
pub const VK_OEM_FJ_TOUROKU: VIRTUAL_KEY = VIRTUAL_KEY(148u16);
pub const VK_OEM_FJ_LOYA: VIRTUAL_KEY = VIRTUAL_KEY(149u16);
pub const VK_OEM_FJ_ROYA: VIRTUAL_KEY = VIRTUAL_KEY(150u16);
pub const VK_LSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(160u16);
pub const VK_RSHIFT: VIRTUAL_KEY = VIRTUAL_KEY(161u16);
pub const VK_LCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(162u16);
pub const VK_RCONTROL: VIRTUAL_KEY = VIRTUAL_KEY(163u16);
pub const VK_LMENU: VIRTUAL_KEY = VIRTUAL_KEY(164u16);
pub const VK_RMENU: VIRTUAL_KEY = VIRTUAL_KEY(165u16);
pub const VK_BROWSER_BACK: VIRTUAL_KEY = VIRTUAL_KEY(166u16);
pub const VK_BROWSER_FORWARD: VIRTUAL_KEY = VIRTUAL_KEY(167u16);
pub const VK_BROWSER_REFRESH: VIRTUAL_KEY = VIRTUAL_KEY(168u16);
pub const VK_BROWSER_STOP: VIRTUAL_KEY = VIRTUAL_KEY(169u16);
pub const VK_BROWSER_SEARCH: VIRTUAL_KEY = VIRTUAL_KEY(170u16);
pub const VK_BROWSER_FAVORITES: VIRTUAL_KEY = VIRTUAL_KEY(171u16);
pub const VK_BROWSER_HOME: VIRTUAL_KEY = VIRTUAL_KEY(172u16);
pub const VK_VOLUME_MUTE: VIRTUAL_KEY = VIRTUAL_KEY(173u16);
pub const VK_VOLUME_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(174u16);
pub const VK_VOLUME_UP: VIRTUAL_KEY = VIRTUAL_KEY(175u16);
pub const VK_MEDIA_NEXT_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(176u16);
pub const VK_MEDIA_PREV_TRACK: VIRTUAL_KEY = VIRTUAL_KEY(177u16);
pub const VK_MEDIA_STOP: VIRTUAL_KEY = VIRTUAL_KEY(178u16);
pub const VK_MEDIA_PLAY_PAUSE: VIRTUAL_KEY = VIRTUAL_KEY(179u16);
pub const VK_LAUNCH_MAIL: VIRTUAL_KEY = VIRTUAL_KEY(180u16);
pub const VK_LAUNCH_MEDIA_SELECT: VIRTUAL_KEY = VIRTUAL_KEY(181u16);
pub const VK_LAUNCH_APP1: VIRTUAL_KEY = VIRTUAL_KEY(182u16);
pub const VK_LAUNCH_APP2: VIRTUAL_KEY = VIRTUAL_KEY(183u16);
pub const VK_OEM_1: VIRTUAL_KEY = VIRTUAL_KEY(186u16);
pub const VK_OEM_PLUS: VIRTUAL_KEY = VIRTUAL_KEY(187u16);
pub const VK_OEM_COMMA: VIRTUAL_KEY = VIRTUAL_KEY(188u16);
pub const VK_OEM_MINUS: VIRTUAL_KEY = VIRTUAL_KEY(189u16);
pub const VK_OEM_PERIOD: VIRTUAL_KEY = VIRTUAL_KEY(190u16);
pub const VK_OEM_2: VIRTUAL_KEY = VIRTUAL_KEY(191u16);
pub const VK_OEM_3: VIRTUAL_KEY = VIRTUAL_KEY(192u16);
pub const VK_GAMEPAD_A: VIRTUAL_KEY = VIRTUAL_KEY(195u16);
pub const VK_GAMEPAD_B: VIRTUAL_KEY = VIRTUAL_KEY(196u16);
pub const VK_GAMEPAD_X: VIRTUAL_KEY = VIRTUAL_KEY(197u16);
pub const VK_GAMEPAD_Y: VIRTUAL_KEY = VIRTUAL_KEY(198u16);
pub const VK_GAMEPAD_RIGHT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(199u16);
pub const VK_GAMEPAD_LEFT_SHOULDER: VIRTUAL_KEY = VIRTUAL_KEY(200u16);
pub const VK_GAMEPAD_LEFT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(201u16);
pub const VK_GAMEPAD_RIGHT_TRIGGER: VIRTUAL_KEY = VIRTUAL_KEY(202u16);
pub const VK_GAMEPAD_DPAD_UP: VIRTUAL_KEY = VIRTUAL_KEY(203u16);
pub const VK_GAMEPAD_DPAD_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(204u16);
pub const VK_GAMEPAD_DPAD_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(205u16);
pub const VK_GAMEPAD_DPAD_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(206u16);
pub const VK_GAMEPAD_MENU: VIRTUAL_KEY = VIRTUAL_KEY(207u16);
pub const VK_GAMEPAD_VIEW: VIRTUAL_KEY = VIRTUAL_KEY(208u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(209u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_BUTTON: VIRTUAL_KEY = VIRTUAL_KEY(210u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(211u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(212u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(213u16);
pub const VK_GAMEPAD_LEFT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(214u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_UP: VIRTUAL_KEY = VIRTUAL_KEY(215u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_DOWN: VIRTUAL_KEY = VIRTUAL_KEY(216u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_RIGHT: VIRTUAL_KEY = VIRTUAL_KEY(217u16);
pub const VK_GAMEPAD_RIGHT_THUMBSTICK_LEFT: VIRTUAL_KEY = VIRTUAL_KEY(218u16);
pub const VK_OEM_4: VIRTUAL_KEY = VIRTUAL_KEY(219u16);
pub const VK_OEM_5: VIRTUAL_KEY = VIRTUAL_KEY(220u16);
pub const VK_OEM_6: VIRTUAL_KEY = VIRTUAL_KEY(221u16);
pub const VK_OEM_7: VIRTUAL_KEY = VIRTUAL_KEY(222u16);
pub const VK_OEM_8: VIRTUAL_KEY = VIRTUAL_KEY(223u16);
pub const VK_OEM_AX: VIRTUAL_KEY = VIRTUAL_KEY(225u16);
pub const VK_OEM_102: VIRTUAL_KEY = VIRTUAL_KEY(226u16);
pub const VK_ICO_HELP: VIRTUAL_KEY = VIRTUAL_KEY(227u16);
pub const VK_ICO_00: VIRTUAL_KEY = VIRTUAL_KEY(228u16);
pub const VK_PROCESSKEY: VIRTUAL_KEY = VIRTUAL_KEY(229u16);
pub const VK_ICO_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(230u16);
pub const VK_PACKET: VIRTUAL_KEY = VIRTUAL_KEY(231u16);
pub const VK_OEM_RESET: VIRTUAL_KEY = VIRTUAL_KEY(233u16);
pub const VK_OEM_JUMP: VIRTUAL_KEY = VIRTUAL_KEY(234u16);
pub const VK_OEM_PA1: VIRTUAL_KEY = VIRTUAL_KEY(235u16);
pub const VK_OEM_PA2: VIRTUAL_KEY = VIRTUAL_KEY(236u16);
pub const VK_OEM_PA3: VIRTUAL_KEY = VIRTUAL_KEY(237u16);
pub const VK_OEM_WSCTRL: VIRTUAL_KEY = VIRTUAL_KEY(238u16);
pub const VK_OEM_CUSEL: VIRTUAL_KEY = VIRTUAL_KEY(239u16);
pub const VK_OEM_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(240u16);
pub const VK_OEM_FINISH: VIRTUAL_KEY = VIRTUAL_KEY(241u16);
pub const VK_OEM_COPY: VIRTUAL_KEY = VIRTUAL_KEY(242u16);
pub const VK_OEM_AUTO: VIRTUAL_KEY = VIRTUAL_KEY(243u16);
pub const VK_OEM_ENLW: VIRTUAL_KEY = VIRTUAL_KEY(244u16);
pub const VK_OEM_BACKTAB: VIRTUAL_KEY = VIRTUAL_KEY(245u16);
pub const VK_ATTN: VIRTUAL_KEY = VIRTUAL_KEY(246u16);
pub const VK_CRSEL: VIRTUAL_KEY = VIRTUAL_KEY(247u16);
pub const VK_EXSEL: VIRTUAL_KEY = VIRTUAL_KEY(248u16);
pub const VK_EREOF: VIRTUAL_KEY = VIRTUAL_KEY(249u16);
pub const VK_PLAY: VIRTUAL_KEY = VIRTUAL_KEY(250u16);
pub const VK_ZOOM: VIRTUAL_KEY = VIRTUAL_KEY(251u16);
pub const VK_NONAME: VIRTUAL_KEY = VIRTUAL_KEY(252u16);
pub const VK_PA1: VIRTUAL_KEY = VIRTUAL_KEY(253u16);
pub const VK_OEM_CLEAR: VIRTUAL_KEY = VIRTUAL_KEY(254u16);
impl ::std::convert::From<u16> for VIRTUAL_KEY {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VIRTUAL_KEY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VK_ABNT_C1: u32 = 193u32;
pub const VK_ABNT_C2: u32 = 194u32;
pub const VK_DBE_ALPHANUMERIC: u32 = 240u32;
pub const VK_DBE_CODEINPUT: u32 = 250u32;
pub const VK_DBE_DBCSCHAR: u32 = 244u32;
pub const VK_DBE_DETERMINESTRING: u32 = 252u32;
pub const VK_DBE_ENTERDLGCONVERSIONMODE: u32 = 253u32;
pub const VK_DBE_ENTERIMECONFIGMODE: u32 = 248u32;
pub const VK_DBE_ENTERWORDREGISTERMODE: u32 = 247u32;
pub const VK_DBE_FLUSHSTRING: u32 = 249u32;
pub const VK_DBE_HIRAGANA: u32 = 242u32;
pub const VK_DBE_KATAKANA: u32 = 241u32;
pub const VK_DBE_NOCODEINPUT: u32 = 251u32;
pub const VK_DBE_NOROMAN: u32 = 246u32;
pub const VK_DBE_ROMAN: u32 = 245u32;
pub const VK_DBE_SBCSCHAR: u32 = 243u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_BIT {
    pub Vk: u8,
    pub ModBits: u8,
}
impl VK_TO_BIT {}
impl ::std::default::Default for VK_TO_BIT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_BIT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_BIT")
            .field("Vk", &self.Vk)
            .field("ModBits", &self.ModBits)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_BIT {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.ModBits == other.ModBits
    }
}
impl ::std::cmp::Eq for VK_TO_BIT {}
unsafe impl ::windows::runtime::Abi for VK_TO_BIT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS1 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 1],
}
impl VK_TO_WCHARS1 {}
impl ::std::default::Default for VK_TO_WCHARS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS1")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS1 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS1 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS10 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 10],
}
impl VK_TO_WCHARS10 {}
impl ::std::default::Default for VK_TO_WCHARS10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS10 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS10")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS10 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS10 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS10 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS2 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 2],
}
impl VK_TO_WCHARS2 {}
impl ::std::default::Default for VK_TO_WCHARS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS2")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS2 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS2 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS3 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 3],
}
impl VK_TO_WCHARS3 {}
impl ::std::default::Default for VK_TO_WCHARS3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS3")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS3 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS3 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS4 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 4],
}
impl VK_TO_WCHARS4 {}
impl ::std::default::Default for VK_TO_WCHARS4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS4")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS4 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS4 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS5 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 5],
}
impl VK_TO_WCHARS5 {}
impl ::std::default::Default for VK_TO_WCHARS5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS5")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS5 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS5 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS5 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS6 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 6],
}
impl VK_TO_WCHARS6 {}
impl ::std::default::Default for VK_TO_WCHARS6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS6")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS6 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS6 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS7 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 7],
}
impl VK_TO_WCHARS7 {}
impl ::std::default::Default for VK_TO_WCHARS7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS7")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS7 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS7 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS8 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 8],
}
impl VK_TO_WCHARS8 {}
impl ::std::default::Default for VK_TO_WCHARS8 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS8 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS8")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS8 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS8 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS8 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHARS9 {
    pub VirtualKey: u8,
    pub Attributes: u8,
    pub wch: [u16; 9],
}
impl VK_TO_WCHARS9 {}
impl ::std::default::Default for VK_TO_WCHARS9 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHARS9 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHARS9")
            .field("VirtualKey", &self.VirtualKey)
            .field("Attributes", &self.Attributes)
            .field("wch", &self.wch)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHARS9 {
    fn eq(&self, other: &Self) -> bool {
        self.VirtualKey == other.VirtualKey
            && self.Attributes == other.Attributes
            && self.wch == other.wch
    }
}
impl ::std::cmp::Eq for VK_TO_WCHARS9 {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHARS9 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_TO_WCHAR_TABLE {
    pub pVkToWchars: *mut VK_TO_WCHARS1,
    pub nModifications: u8,
    pub cbSize: u8,
}
impl VK_TO_WCHAR_TABLE {}
impl ::std::default::Default for VK_TO_WCHAR_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_TO_WCHAR_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_TO_WCHAR_TABLE")
            .field("pVkToWchars", &self.pVkToWchars)
            .field("nModifications", &self.nModifications)
            .field("cbSize", &self.cbSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_TO_WCHAR_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.pVkToWchars == other.pVkToWchars
            && self.nModifications == other.nModifications
            && self.cbSize == other.cbSize
    }
}
impl ::std::cmp::Eq for VK_TO_WCHAR_TABLE {}
unsafe impl ::windows::runtime::Abi for VK_TO_WCHAR_TABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VK_VSC {
    pub Vk: u8,
    pub Vsc: u8,
}
impl VK_VSC {}
impl ::std::default::Default for VK_VSC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VK_VSC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VK_VSC")
            .field("Vk", &self.Vk)
            .field("Vsc", &self.Vsc)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VK_VSC {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk && self.Vsc == other.Vsc
    }
}
impl ::std::cmp::Eq for VK_VSC {}
unsafe impl ::windows::runtime::Abi for VK_VSC {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VK__none_: u32 = 255u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VSC_LPWSTR {
    pub vsc: u8,
    pub pwsz: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl VSC_LPWSTR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VSC_LPWSTR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VSC_LPWSTR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSC_LPWSTR")
            .field("vsc", &self.vsc)
            .field("pwsz", &self.pwsz)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VSC_LPWSTR {
    fn eq(&self, other: &Self) -> bool {
        self.vsc == other.vsc && self.pwsz == other.pwsz
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VSC_LPWSTR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VSC_LPWSTR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VSC_VK {
    pub Vsc: u8,
    pub Vk: u16,
}
impl VSC_VK {}
impl ::std::default::Default for VSC_VK {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VSC_VK {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VSC_VK")
            .field("Vsc", &self.Vsc)
            .field("Vk", &self.Vk)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VSC_VK {
    fn eq(&self, other: &Self) -> bool {
        self.Vsc == other.Vsc && self.Vk == other.Vk
    }
}
impl ::std::cmp::Eq for VSC_VK {}
unsafe impl ::windows::runtime::Abi for VSC_VK {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
#[inline]
pub unsafe fn VkKeyScanA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::CHAR>,
>(
    ch: Param0,
) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VkKeyScanA(ch: super::super::System::SystemServices::CHAR) -> i16;
        }
        ::std::mem::transmute(VkKeyScanA(ch.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(
    feature = "Win32_System_SystemServices",
    feature = "Win32_UI_TextServices"
))]
#[inline]
pub unsafe fn VkKeyScanExA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::System::SystemServices::CHAR>,
    Param1: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    ch: Param0,
    dwhkl: Param1,
) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VkKeyScanExA(
                ch: super::super::System::SystemServices::CHAR,
                dwhkl: super::TextServices::HKL,
            ) -> i16;
        }
        ::std::mem::transmute(VkKeyScanExA(
            ch.into_param().abi(),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_UI_TextServices")]
#[inline]
pub unsafe fn VkKeyScanExW<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::TextServices::HKL>,
>(
    ch: u16,
    dwhkl: Param1,
) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VkKeyScanExW(ch: u16, dwhkl: super::TextServices::HKL) -> i16;
        }
        ::std::mem::transmute(VkKeyScanExW(
            ::std::mem::transmute(ch),
            dwhkl.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn VkKeyScanW(ch: u16) -> i16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn VkKeyScanW(ch: u16) -> i16;
        }
        ::std::mem::transmute(VkKeyScanW(::std::mem::transmute(ch)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WCH_DEAD: u32 = 61441u32;
pub const WCH_LGTR: u32 = 61442u32;
pub const WCH_NONE: u32 = 61440u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn _TrackMouseEvent(
    lpeventtrack: *mut TRACKMOUSEEVENT,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn _TrackMouseEvent(
                lpeventtrack: *mut TRACKMOUSEEVENT,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(_TrackMouseEvent(::std::mem::transmute(lpeventtrack)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _VK_FUNCTION_PARAM {
    pub NLSFEProcIndex: u8,
    pub NLSFEProcParam: u32,
}
impl _VK_FUNCTION_PARAM {}
impl ::std::default::Default for _VK_FUNCTION_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _VK_FUNCTION_PARAM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VK_FUNCTION_PARAM")
            .field("NLSFEProcIndex", &self.NLSFEProcIndex)
            .field("NLSFEProcParam", &self.NLSFEProcParam)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _VK_FUNCTION_PARAM {
    fn eq(&self, other: &Self) -> bool {
        self.NLSFEProcIndex == other.NLSFEProcIndex && self.NLSFEProcParam == other.NLSFEProcParam
    }
}
impl ::std::cmp::Eq for _VK_FUNCTION_PARAM {}
unsafe impl ::windows::runtime::Abi for _VK_FUNCTION_PARAM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct _VK_TO_FUNCTION_TABLE {
    pub Vk: u8,
    pub NLSFEProcType: u8,
    pub NLSFEProcCurrent: u8,
    pub NLSFEProcSwitch: u8,
    pub NLSFEProc: [_VK_FUNCTION_PARAM; 8],
    pub NLSFEProcAlt: [_VK_FUNCTION_PARAM; 8],
}
impl _VK_TO_FUNCTION_TABLE {}
impl ::std::default::Default for _VK_TO_FUNCTION_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _VK_TO_FUNCTION_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_VK_TO_FUNCTION_TABLE")
            .field("Vk", &self.Vk)
            .field("NLSFEProcType", &self.NLSFEProcType)
            .field("NLSFEProcCurrent", &self.NLSFEProcCurrent)
            .field("NLSFEProcSwitch", &self.NLSFEProcSwitch)
            .field("NLSFEProc", &self.NLSFEProc)
            .field("NLSFEProcAlt", &self.NLSFEProcAlt)
            .finish()
    }
}
impl ::std::cmp::PartialEq for _VK_TO_FUNCTION_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Vk == other.Vk
            && self.NLSFEProcType == other.NLSFEProcType
            && self.NLSFEProcCurrent == other.NLSFEProcCurrent
            && self.NLSFEProcSwitch == other.NLSFEProcSwitch
            && self.NLSFEProc == other.NLSFEProc
            && self.NLSFEProcAlt == other.NLSFEProcAlt
    }
}
impl ::std::cmp::Eq for _VK_TO_FUNCTION_TABLE {}
unsafe impl ::windows::runtime::Abi for _VK_TO_FUNCTION_TABLE {
    type Abi = Self;
    type DefaultType = Self;
}
#[inline]
pub unsafe fn keybd_event(bvk: u8, bscan: u8, dwflags: KEYBD_EVENT_FLAGS, dwextrainfo: usize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn keybd_event(bvk: u8, bscan: u8, dwflags: KEYBD_EVENT_FLAGS, dwextrainfo: usize);
        }
        ::std::mem::transmute(keybd_event(
            ::std::mem::transmute(bvk),
            ::std::mem::transmute(bscan),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwextrainfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn mouse_event(
    dwflags: MOUSE_EVENT_FLAGS,
    dx: i32,
    dy: i32,
    dwdata: u32,
    dwextrainfo: usize,
) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn mouse_event(
                dwflags: MOUSE_EVENT_FLAGS,
                dx: i32,
                dy: i32,
                dwdata: u32,
                dwextrainfo: usize,
            );
        }
        ::std::mem::transmute(mouse_event(
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dx),
            ::std::mem::transmute(dy),
            ::std::mem::transmute(dwdata),
            ::std::mem::transmute(dwextrainfo),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct tagKbdLayer {
    pub pCharModifiers: *mut MODIFIERS,
    pub pVkToWcharTable: *mut VK_TO_WCHAR_TABLE,
    pub pDeadKey: *mut DEADKEY,
    pub pKeyNames: *mut VSC_LPWSTR,
    pub pKeyNamesExt: *mut VSC_LPWSTR,
    pub pKeyNamesDead: *mut *mut u16,
    pub pusVSCtoVK: *mut u16,
    pub bMaxVSCtoVK: u8,
    pub pVSCtoVK_E0: *mut VSC_VK,
    pub pVSCtoVK_E1: *mut VSC_VK,
    pub fLocaleFlags: u32,
    pub nLgMax: u8,
    pub cbLgEntry: u8,
    pub pLigature: *mut LIGATURE1,
    pub dwType: u32,
    pub dwSubType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl tagKbdLayer {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for tagKbdLayer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for tagKbdLayer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tagKbdLayer")
            .field("pCharModifiers", &self.pCharModifiers)
            .field("pVkToWcharTable", &self.pVkToWcharTable)
            .field("pDeadKey", &self.pDeadKey)
            .field("pKeyNames", &self.pKeyNames)
            .field("pKeyNamesExt", &self.pKeyNamesExt)
            .field("pKeyNamesDead", &self.pKeyNamesDead)
            .field("pusVSCtoVK", &self.pusVSCtoVK)
            .field("bMaxVSCtoVK", &self.bMaxVSCtoVK)
            .field("pVSCtoVK_E0", &self.pVSCtoVK_E0)
            .field("pVSCtoVK_E1", &self.pVSCtoVK_E1)
            .field("fLocaleFlags", &self.fLocaleFlags)
            .field("nLgMax", &self.nLgMax)
            .field("cbLgEntry", &self.cbLgEntry)
            .field("pLigature", &self.pLigature)
            .field("dwType", &self.dwType)
            .field("dwSubType", &self.dwSubType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for tagKbdLayer {
    fn eq(&self, other: &Self) -> bool {
        self.pCharModifiers == other.pCharModifiers
            && self.pVkToWcharTable == other.pVkToWcharTable
            && self.pDeadKey == other.pDeadKey
            && self.pKeyNames == other.pKeyNames
            && self.pKeyNamesExt == other.pKeyNamesExt
            && self.pKeyNamesDead == other.pKeyNamesDead
            && self.pusVSCtoVK == other.pusVSCtoVK
            && self.bMaxVSCtoVK == other.bMaxVSCtoVK
            && self.pVSCtoVK_E0 == other.pVSCtoVK_E0
            && self.pVSCtoVK_E1 == other.pVSCtoVK_E1
            && self.fLocaleFlags == other.fLocaleFlags
            && self.nLgMax == other.nLgMax
            && self.cbLgEntry == other.cbLgEntry
            && self.pLigature == other.pLigature
            && self.dwType == other.dwType
            && self.dwSubType == other.dwSubType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for tagKbdLayer {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for tagKbdLayer {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct tagKbdNlsLayer {
    pub OEMIdentifier: u16,
    pub LayoutInformation: u16,
    pub NumOfVkToF: u32,
    pub pVkToF: *mut _VK_TO_FUNCTION_TABLE,
    pub NumOfMouseVKey: i32,
    pub pusMouseVKey: *mut u16,
}
impl tagKbdNlsLayer {}
impl ::std::default::Default for tagKbdNlsLayer {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for tagKbdNlsLayer {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("tagKbdNlsLayer")
            .field("OEMIdentifier", &self.OEMIdentifier)
            .field("LayoutInformation", &self.LayoutInformation)
            .field("NumOfVkToF", &self.NumOfVkToF)
            .field("pVkToF", &self.pVkToF)
            .field("NumOfMouseVKey", &self.NumOfMouseVKey)
            .field("pusMouseVKey", &self.pusMouseVKey)
            .finish()
    }
}
impl ::std::cmp::PartialEq for tagKbdNlsLayer {
    fn eq(&self, other: &Self) -> bool {
        self.OEMIdentifier == other.OEMIdentifier
            && self.LayoutInformation == other.LayoutInformation
            && self.NumOfVkToF == other.NumOfVkToF
            && self.pVkToF == other.pVkToF
            && self.NumOfMouseVKey == other.NumOfMouseVKey
            && self.pusMouseVKey == other.pusMouseVKey
    }
}
impl ::std::cmp::Eq for tagKbdNlsLayer {}
unsafe impl ::windows::runtime::Abi for tagKbdNlsLayer {
    type Abi = Self;
    type DefaultType = Self;
}