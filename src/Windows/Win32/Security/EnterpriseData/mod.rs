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
pub struct ENTERPRISE_DATA_POLICIES(pub u32);
pub const ENTERPRISE_POLICY_NONE: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(0u32);
pub const ENTERPRISE_POLICY_ALLOWED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(1u32);
pub const ENTERPRISE_POLICY_ENLIGHTENED: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(2u32);
pub const ENTERPRISE_POLICY_EXEMPT: ENTERPRISE_DATA_POLICIES = ENTERPRISE_DATA_POLICIES(4u32);
impl ::std::convert::From<u32> for ENTERPRISE_DATA_POLICIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENTERPRISE_DATA_POLICIES {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENTERPRISE_DATA_POLICIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENTERPRISE_DATA_POLICIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENTERPRISE_DATA_POLICIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct FILE_UNPROTECT_OPTIONS {
    pub audit: bool,
}
impl FILE_UNPROTECT_OPTIONS {}
impl ::std::default::Default for FILE_UNPROTECT_OPTIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FILE_UNPROTECT_OPTIONS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FILE_UNPROTECT_OPTIONS")
            .field("audit", &self.audit)
            .finish()
    }
}
impl ::std::cmp::PartialEq for FILE_UNPROTECT_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.audit == other.audit
    }
}
impl ::std::cmp::Eq for FILE_UNPROTECT_OPTIONS {}
unsafe impl ::windows::runtime::Abi for FILE_UNPROTECT_OPTIONS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HTHREAD_NETWORK_CONTEXT {
    pub ThreadId: u32,
    pub ThreadContext: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HTHREAD_NETWORK_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HTHREAD_NETWORK_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HTHREAD_NETWORK_CONTEXT")
            .field("ThreadId", &self.ThreadId)
            .field("ThreadContext", &self.ThreadContext)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HTHREAD_NETWORK_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadId == other.ThreadId && self.ThreadContext == other.ThreadContext
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HTHREAD_NETWORK_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HTHREAD_NETWORK_CONTEXT {
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
pub struct IProtectionPolicyManagerInterop(::windows::runtime::IUnknown);
impl IProtectionPolicyManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        targetidentity: Param2,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            targetidentity.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetForWindow<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerInterop {
    type Vtable = IProtectionPolicyManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1179804957,
        49662,
        19361,
        [159, 10, 192, 245, 101, 150, 247, 33],
    );
}
impl ::std::convert::From<IProtectionPolicyManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProtectionPolicyManagerInterop> for ::windows::runtime::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProtectionPolicyManagerInterop
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProtectionPolicyManagerInterop
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        riid: *const ::windows::runtime::GUID,
        result: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProtectionPolicyManagerInterop2(::windows::runtime::IUnknown);
impl IProtectionPolicyManagerInterop2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        apppackagefamilyname: Param2,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithAuditingInfoForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        targetidentity: Param2,
        auditinfounk: Param3,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            targetidentity.into_param().abi(),
            auditinfounk.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithMessageForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        targetidentity: Param2,
        auditinfounk: Param3,
        messagefromapp: Param4,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            targetidentity.into_param().abi(),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithAuditingInfoForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        apppackagefamilyname: Param2,
        auditinfounk: Param3,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            auditinfounk.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithMessageForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        apppackagefamilyname: Param2,
        auditinfounk: Param3,
        messagefromapp: Param4,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerInterop2 {
    type Vtable = IProtectionPolicyManagerInterop2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        360512484,
        42893,
        16726,
        [179, 132, 97, 253, 172, 65, 230, 134],
    );
}
impl ::std::convert::From<IProtectionPolicyManagerInterop2> for ::windows::runtime::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProtectionPolicyManagerInterop2> for ::windows::runtime::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProtectionPolicyManagerInterop2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProtectionPolicyManagerInterop2
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IProtectionPolicyManagerInterop3(::windows::runtime::IUnknown);
impl IProtectionPolicyManagerInterop3 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessWithBehaviorForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        targetidentity: Param2,
        auditinfounk: Param3,
        messagefromapp: Param4,
        behavior: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            targetidentity.into_param().abi(),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            ::std::mem::transmute(behavior),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessForAppWithBehaviorForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceidentity: Param1,
        apppackagefamilyname: Param2,
        auditinfounk: Param3,
        messagefromapp: Param4,
        behavior: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceidentity.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            ::std::mem::transmute(behavior),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceitemlistunk: Param1,
        apppackagefamilyname: Param2,
        auditinfounk: Param3,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceitemlistunk.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            auditinfounk.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForAppWithMessageAndBehaviorForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceitemlistunk: Param1,
        apppackagefamilyname: Param2,
        auditinfounk: Param3,
        messagefromapp: Param4,
        behavior: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceitemlistunk.into_param().abi(),
            apppackagefamilyname.into_param().abi(),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            ::std::mem::transmute(behavior),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceitemlistunk: Param1,
        processid: u32,
        auditinfounk: Param3,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceitemlistunk.into_param().abi(),
            ::std::mem::transmute(processid),
            auditinfounk.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RequestAccessToFilesForProcessWithMessageAndBehaviorForWindowAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        T: ::windows::runtime::Interface,
    >(
        &self,
        appwindow: Param0,
        sourceitemlistunk: Param1,
        processid: u32,
        auditinfounk: Param3,
        messagefromapp: Param4,
        behavior: u32,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            appwindow.into_param().abi(),
            sourceitemlistunk.into_param().abi(),
            ::std::mem::transmute(processid),
            auditinfounk.into_param().abi(),
            messagefromapp.into_param().abi(),
            ::std::mem::transmute(behavior),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IProtectionPolicyManagerInterop3 {
    type Vtable = IProtectionPolicyManagerInterop3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3250600243,
        45976,
        19859,
        [176, 253, 41, 114, 173, 248, 2, 194],
    );
}
impl ::std::convert::From<IProtectionPolicyManagerInterop3> for ::windows::runtime::IUnknown {
    fn from(value: IProtectionPolicyManagerInterop3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IProtectionPolicyManagerInterop3> for ::windows::runtime::IUnknown {
    fn from(value: &IProtectionPolicyManagerInterop3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IProtectionPolicyManagerInterop3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IProtectionPolicyManagerInterop3
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProtectionPolicyManagerInterop3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        targetidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: u32,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceidentity: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: u32,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceitemlistunk: ::windows::runtime::RawPtr,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceitemlistunk: ::windows::runtime::RawPtr,
        apppackagefamilyname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: u32,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceitemlistunk: ::windows::runtime::RawPtr,
        processid: u32,
        auditinfounk: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appwindow: super::super::Foundation::HWND,
        sourceitemlistunk: ::windows::runtime::RawPtr,
        processid: u32,
        auditinfounk: ::windows::runtime::RawPtr,
        messagefromapp: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        behavior: u32,
        riid: *const ::windows::runtime::GUID,
        asyncoperation: *mut *mut ::std::ffi::c_void,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn ProtectFileToEnterpriseIdentity<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    fileorfolderpath: Param0,
    identity: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ProtectFileToEnterpriseIdentity(
                fileorfolderpath: super::super::Foundation::PWSTR,
                identity: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        ProtectFileToEnterpriseIdentity(
            fileorfolderpath.into_param().abi(),
            identity.into_param().abi(),
        )
        .ok()
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
pub struct SRPHOSTING_TYPE(pub i32);
pub const SRPHOSTING_TYPE_NONE: SRPHOSTING_TYPE = SRPHOSTING_TYPE(0i32);
pub const SRPHOSTING_TYPE_WINHTTP: SRPHOSTING_TYPE = SRPHOSTING_TYPE(1i32);
pub const SRPHOSTING_TYPE_WININET: SRPHOSTING_TYPE = SRPHOSTING_TYPE(2i32);
impl ::std::convert::From<i32> for SRPHOSTING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SRPHOSTING_TYPE {
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
pub struct SRPHOSTING_VERSION(pub i32);
pub const SRPHOSTING_VERSION1: SRPHOSTING_VERSION = SRPHOSTING_VERSION(1i32);
impl ::std::convert::From<i32> for SRPHOSTING_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SRPHOSTING_VERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpCloseThreadNetworkContext(
    threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCloseThreadNetworkContext(
                threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT,
            ) -> ::windows::runtime::HRESULT;
        }
        SrpCloseThreadNetworkContext(::std::mem::transmute(threadnetworkcontext)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpCreateThreadNetworkContext<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    enterpriseid: Param0,
) -> ::windows::runtime::Result<HTHREAD_NETWORK_CONTEXT> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpCreateThreadNetworkContext(
                enterpriseid: super::super::Foundation::PWSTR,
                threadnetworkcontext: *mut HTHREAD_NETWORK_CONTEXT,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HTHREAD_NETWORK_CONTEXT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        SrpCreateThreadNetworkContext(enterpriseid.into_param().abi(), &mut result__)
            .from_abi::<HTHREAD_NETWORK_CONTEXT>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SrpDisablePermissiveModeFileEncryption() -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDisablePermissiveModeFileEncryption() -> ::windows::runtime::HRESULT;
        }
        SrpDisablePermissiveModeFileEncryption().ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Appx"))]
pub unsafe fn SrpDoesPolicyAllowAppExecution(
    packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID,
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpDoesPolicyAllowAppExecution(
                packageid: *const super::super::Storage::Packaging::Appx::PACKAGE_ID,
                isallowed: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        SrpDoesPolicyAllowAppExecution(::std::mem::transmute(packageid), &mut result__)
            .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpEnablePermissiveModeFileEncryption<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    enterpriseid: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpEnablePermissiveModeFileEncryption(
                enterpriseid: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        SrpEnablePermissiveModeFileEncryption(enterpriseid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpGetEnterpriseIds<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
    numberofbytes: *mut u32,
    enterpriseids: *mut super::super::Foundation::PWSTR,
    enterpriseidcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterpriseIds(
                tokenhandle: super::super::Foundation::HANDLE,
                numberofbytes: *mut u32,
                enterpriseids: *mut super::super::Foundation::PWSTR,
                enterpriseidcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        SrpGetEnterpriseIds(
            tokenhandle.into_param().abi(),
            ::std::mem::transmute(numberofbytes),
            ::std::mem::transmute(enterpriseids),
            ::std::mem::transmute(enterpriseidcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpGetEnterprisePolicy<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
) -> ::windows::runtime::Result<ENTERPRISE_DATA_POLICIES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpGetEnterprisePolicy(
                tokenhandle: super::super::Foundation::HANDLE,
                policyflags: *mut ENTERPRISE_DATA_POLICIES,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ENTERPRISE_DATA_POLICIES as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        SrpGetEnterprisePolicy(tokenhandle.into_param().abi(), &mut result__)
            .from_abi::<ENTERPRISE_DATA_POLICIES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn SrpHostingInitialize(
    version: SRPHOSTING_VERSION,
    r#type: SRPHOSTING_TYPE,
    pvdata: *const ::std::ffi::c_void,
    cbdata: u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingInitialize(
                version: SRPHOSTING_VERSION,
                r#type: SRPHOSTING_TYPE,
                pvdata: *const ::std::ffi::c_void,
                cbdata: u32,
            ) -> ::windows::runtime::HRESULT;
        }
        SrpHostingInitialize(
            ::std::mem::transmute(version),
            ::std::mem::transmute(r#type),
            ::std::mem::transmute(pvdata),
            ::std::mem::transmute(cbdata),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpHostingTerminate(r#type: SRPHOSTING_TYPE);
        }
        ::std::mem::transmute(SrpHostingTerminate(::std::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpIsTokenService<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
>(
    tokenhandle: Param0,
    istokenservice: *mut u8,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpIsTokenService(
                tokenhandle: super::super::Foundation::HANDLE,
                istokenservice: *mut u8,
            ) -> super::super::Foundation::NTSTATUS;
        }
        SrpIsTokenService(
            tokenhandle.into_param().abi(),
            ::std::mem::transmute(istokenservice),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn SrpSetTokenEnterpriseId<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    tokenhandle: Param0,
    enterpriseid: Param1,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SrpSetTokenEnterpriseId(
                tokenhandle: super::super::Foundation::HANDLE,
                enterpriseid: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        SrpSetTokenEnterpriseId(
            tokenhandle.into_param().abi(),
            enterpriseid.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn UnprotectFile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    fileorfolderpath: Param0,
    options: *const FILE_UNPROTECT_OPTIONS,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnprotectFile(
                fileorfolderpath: super::super::Foundation::PWSTR,
                options: *const FILE_UNPROTECT_OPTIONS,
            ) -> ::windows::runtime::HRESULT;
        }
        UnprotectFile(
            fileorfolderpath.into_param().abi(),
            ::std::mem::transmute(options),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}