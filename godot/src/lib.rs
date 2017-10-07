#[doc(hidden)]
pub extern crate libc;#[doc(hidden)]
pub extern crate godot_sys as sys;

#[macro_export]
macro_rules! gprint_warn {
    ($($args:tt)*) => ({
        let msg = format!($($args)*);
        let line = line!();
        let file = file!();
        #[allow(unused_unsafe)]
        unsafe {
            let msg = ::std::ffi::CString::new(msg).unwrap();
            let file = ::std::ffi::CString::new(file).unwrap();
            let func = b"<native>\0";
            ($crate::get_api().godot_print_warning)(
                msg.as_ptr() as *const _,
                func.as_ptr() as *const _,
                file.as_ptr() as *const _,
                line as _,
            );
        }
    })
}
#[macro_export]
macro_rules! gprint_error {
    ($($args:tt)*) => ({
        let msg = format!($($args)*);
        let line = line!();
        let file = file!();
        unsafe {
            let msg = ::std::ffi::CString::new(msg).unwrap();
            let file = ::std::ffi::CString::new(file).unwrap();
            let func = b"<native>\0";
            ($crate::get_api().godot_print_error)(
                msg.as_ptr() as *const _,
                func.as_ptr() as *const _,
                file.as_ptr() as *const _,
                line as _,
            );
        }
    })
}

mod internal;
pub use self::internal::*;
#[macro_use]
mod class;
pub use self::class::*;
pub mod types;


#[doc(hidden)]
pub static mut GODOT_API: Option<GodotApi> = None;
#[inline]
#[doc(hidden)]
pub fn get_api() -> &'static GodotApi {
    unsafe {
        &GODOT_API.as_ref()
            .expect("API not bound")
    }
}

#[macro_export]
macro_rules! gd_init {
    (
        $(
            $class:ty
        ),*
    ) => (
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn godot_gdnative_init(options: *mut $crate::sys::godot_gdnative_init_options) {
            unsafe {
                $crate::GODOT_API = Some($crate::GodotApi::from_raw((*options).api_struct));
            }
        }

        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn godot_gdnative_terminate(_options: *mut $crate::sys::godot_gdnative_terminate_options) {
            unsafe {
                $crate::GODOT_API = None;
            }
        }

        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn godot_nativescript_init(desc: *mut $crate::libc::c_void) {
            gprint_warn!("Init nativescript called");
            unsafe {
                $(
                    <$class as $crate::GodotClass>::register_class(desc);
                )*
            }
        }
    )
}