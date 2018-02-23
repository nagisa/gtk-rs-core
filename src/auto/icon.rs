// This file was generated by gir (https://github.com/gtk-rs/gir @ bd67955)
// from gir-files (https://github.com/gtk-rs/gir-files @ db49619)
// DO NOT EDIT

use Error;
use ffi;
#[cfg(any(feature = "v2_38", feature = "dox"))]
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Icon(Object<ffi::GIcon, ffi::GIconIface>);

    match fn {
        get_type => || ffi::g_icon_get_type(),
    }
}

impl Icon {
    #[cfg(any(feature = "v2_38", feature = "dox"))]
    pub fn deserialize(value: &glib::Variant) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_icon_deserialize(value.to_glib_none().0))
        }
    }

    //pub fn hash(icon: /*Unimplemented*/Fundamental: Pointer) -> u32 {
    //    unsafe { TODO: call ffi::g_icon_hash() }
    //}

    pub fn new_for_string(str: &str) -> Result<Icon, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_icon_new_for_string(str.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait IconExt {
    fn equal<'a, P: IsA<Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon2: Q) -> bool;

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn serialize(&self) -> Option<glib::Variant>;

    fn to_string(&self) -> Option<String>;
}

impl<O: IsA<Icon>> IconExt for O {
    fn equal<'a, P: IsA<Icon> + 'a, Q: Into<Option<&'a P>>>(&self, icon2: Q) -> bool {
        let icon2 = icon2.into();
        let icon2 = icon2.to_glib_none();
        unsafe {
            from_glib(ffi::g_icon_equal(self.to_glib_none().0, icon2.0))
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn serialize(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_icon_serialize(self.to_glib_none().0))
        }
    }

    fn to_string(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_icon_to_string(self.to_glib_none().0))
        }
    }
}
