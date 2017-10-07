
use super::*;
use std::sync::{Once, ONCE_INIT};
use std::ops::*;

pub struct Nothing {
    info: GodotClassInfo,
}

unsafe impl GodotClass for Nothing {
    type ClassData = Nothing;

    fn godot_name() -> &'static str {
        ""
    }

    unsafe fn register_class(_desc: *mut libc::c_void) {
        panic!("Can't register");
    }

    fn godot_info(&self) -> &GodotClassInfo {
        &self.info
    }
}
unsafe impl GodotNativeClass for Nothing {
    unsafe fn from_object(obj: *mut sys::godot_object) -> Self {
        Nothing {
            info: GodotClassInfo {
                this: obj,
            },
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/types.rs"));

// TODO: Generate this
// START

/*

pub struct Node {
    info: GodotClassInfo,
}

impl Node {
    pub fn get_name(&self) -> String {
        use std::ptr;
        unsafe {
            let api = ::get_api();
            static mut METHOD_BIND: *mut sys::godot_method_bind = 0 as _;
            static INIT: Once = ONCE_INIT;
            INIT.call_once(|| {
                let class = ::std::ffi::CString::new("Node").unwrap();
                let method = ::std::ffi::CString::new("get_name").unwrap();
                METHOD_BIND = (api.godot_method_bind_get_method)(
                    class.as_ptr() as *const _,
                    method.as_ptr() as *const _
                );
                gprint_warn!("Method bind init");
            });

            gprint_warn!("Getting get_name({:?})", METHOD_BIND);

            let mut ret = sys::godot_string::default();
            (api.godot_method_bind_ptrcall)(METHOD_BIND, self.info.this, ptr::null_mut(), &mut ret as *mut _ as *mut _);
            gprint_warn!("Called get_name({:?}) and got: {:?}", METHOD_BIND, ret);

            // gprint_warn!("{:?}", (api.godot_variant_get_type)(&mut ret));
            // let rets = (api.godot_variant_as_string)(&mut ret);
            ::std::ffi::CStr::from_ptr((api.godot_string_c_str)(&ret) as *const _)
                .to_string_lossy()
                .into_owned()
        }
    }
}

unsafe impl GodotClass for Node {
    type ClassData = Node;

    fn godot_name() -> &'static str {
        "Node"
    }

    unsafe fn register_class(_desc: *mut libc::c_void) {
        panic!("Can't register");
    }

    fn godot_info(&self) -> &GodotClassInfo {
        &self.info
    }
}
unsafe impl GodotNativeClass for Node {
    unsafe fn from_object(obj: *mut sys::godot_object) -> Self {
        Node {
            info: GodotClassInfo {
                this: obj,
            },
        }
    }
}

*/

// END

pub unsafe trait GodotType: Sized {
    fn as_variant(&self) -> sys::godot_variant;
    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self>;
}

unsafe impl GodotType for () {
    fn as_variant(&self) -> sys::godot_variant {
        unsafe {
            let mut ret = sys::godot_variant::default();
            (get_api().godot_variant_new_nil)(&mut ret);
            ret
        }
    }

    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
        unsafe {
            if (get_api().godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_NIL {
                Some(())
            } else {
                None
            }
        }
    }
}

macro_rules! godot_int_impl {
    ($ty:ty) => (
        unsafe impl GodotType for $ty {
            fn as_variant(&self) -> sys::godot_variant {
                unsafe {
                    let mut ret = sys::godot_variant::default();
                    (get_api().godot_variant_new_int)(&mut ret, *self as i64);
                    ret
                }
            }

            fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
                unsafe {
                    let api = get_api();
                    if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_INT {
                        Some((api.godot_variant_as_int)(variant) as Self)
                    } else {
                        None
                    }
                }
            }
        }
            )
}

godot_int_impl!(i8);
godot_int_impl!(i16);
godot_int_impl!(i32);
godot_int_impl!(i64);

macro_rules! godot_uint_impl {
    ($ty:ty) => (
        unsafe impl GodotType for $ty {
            fn as_variant(&self) -> sys::godot_variant {
                unsafe {
                    let mut ret = sys::godot_variant::default();
                    (get_api().godot_variant_new_uint)(&mut ret, *self as u64);
                    ret
                }
            }

            fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
                unsafe {
                    let api = get_api();
                    if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_INT {
                        Some((api.godot_variant_as_uint)(variant) as Self)
                    } else {
                        None
                    }
                }
            }
        }
            )
}

godot_uint_impl!(u8);
godot_uint_impl!(u16);
godot_uint_impl!(u32);
godot_uint_impl!(u64);


unsafe impl GodotType for f32 {
    fn as_variant(&self) -> sys::godot_variant {
        unsafe {
            let mut ret = sys::godot_variant::default();
            (get_api().godot_variant_new_real)(&mut ret, *self as f64);
            ret
        }
    }

    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
        unsafe {
            let api = get_api();
            if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_REAL {
                Some((api.godot_variant_as_real)(variant) as Self)
            } else {
                None
            }
        }
    }
}

unsafe impl GodotType for f64 {
    fn as_variant(&self) -> sys::godot_variant {
        unsafe {
            let mut ret = sys::godot_variant::default();
            (get_api().godot_variant_new_real)(&mut ret, *self);
            ret
        }
    }

    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
        unsafe {
            let api = get_api();
            if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_REAL {
                Some((api.godot_variant_as_real)(variant) as Self)
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vector3(sys::godot_vector3);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        unsafe {
            let mut dest = sys::godot_vector3::default();
            (get_api().godot_vector3_new)(&mut dest, x, y, z);
            Vector3(dest)
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;
    fn add(self, other: Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}

impl <'a> Add for &'a Vector3 {
    type Output = Vector3;
    fn add(self, other: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}

unsafe impl GodotType for Vector3 {
    fn as_variant(&self) -> sys::godot_variant {
        unsafe {
            let mut ret = sys::godot_variant::default();
            (get_api().godot_variant_new_vector3)(&mut ret, &self.0);
            ret
        }
    }

    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
        unsafe {
            let api = get_api();
            if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_VECTOR3 {
                Some(Vector3((api.godot_variant_as_vector3)(variant)))
            } else {
                None
            }
        }
    }
}