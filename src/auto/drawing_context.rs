// This file was generated by gir (4b09025) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_22")]
use Window;
#[cfg(feature = "v3_22")]
use cairo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct DrawingContext(Object<ffi::GdkDrawingContext>);

    match fn {
        get_type => || ffi::gdk_drawing_context_get_type(),
    }
}

pub trait DrawingContextExt {
    #[cfg(feature = "v3_22")]
    fn get_cairo_context(&self) -> Option<cairo::Context>;

    #[cfg(feature = "v3_22")]
    fn get_clip(&self) -> Option<cairo::Region>;

    #[cfg(feature = "v3_22")]
    fn get_window(&self) -> Option<Window>;

    #[cfg(feature = "v3_22")]
    fn is_valid(&self) -> bool;
}

impl<O: IsA<DrawingContext>> DrawingContextExt for O {
    #[cfg(feature = "v3_22")]
    fn get_cairo_context(&self) -> Option<cairo::Context> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_cairo_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_clip(&self) -> Option<cairo::Region> {
        unsafe {
            from_glib_full(ffi::gdk_drawing_context_get_clip(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_drawing_context_get_window(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_22")]
    fn is_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gdk_drawing_context_is_valid(self.to_glib_none().0))
        }
    }
}
