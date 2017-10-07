
use super::*;
use std::sync::{Once, ONCE_INIT};
use std::ops::*;
use std::cmp::Ordering;
use std::fmt::{self, Formatter, Debug};

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
pub struct Color(sys::godot_color);

impl Color {
    pub fn new_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        unsafe {
            let mut dest = sys::godot_color::default();
            (get_api().godot_color_new_rgba)(&mut dest, r, g, b, a);
            Color(dest)
        }
    }

    pub fn new_rgb(r: f32, g: f32, b: f32) -> Color {
        unsafe {
            let mut dest = sys::godot_color::default();
            (get_api().godot_color_new_rgb)(&mut dest, r, g, b);
            Color(dest)
        }
    }

    pub fn r(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_r)(&self.0)
        }
    }

    pub fn set_r(&mut self, v: f32) {
        unsafe {
            (get_api().godot_color_set_r)(&mut self.0, v)
        }
    }

    pub fn g(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_g)(&self.0)
        }
    }

    pub fn set_g(&mut self, v: f32) {
        unsafe {
            (get_api().godot_color_set_g)(&mut self.0, v)
        }
    }

    pub fn b(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_b)(&self.0)
        }
    }

    pub fn set_b(&mut self, v: f32) {
        unsafe {
            (get_api().godot_color_set_b)(&mut self.0, v)
        }
    }

    pub fn a(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_a)(&self.0)
        }
    }

    pub fn set_a(&mut self, v: f32) {
        unsafe {
            (get_api().godot_color_set_a)(&mut self.0, v)
        }
    }

    pub fn h(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_h)(&self.0)
        }
    }

    pub fn s(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_s)(&self.0)
        }
    }

    pub fn l(&self) -> f32 {
        unsafe {
            (get_api().godot_color_get_l)(&self.0)
        }
    }
}

#[derive(Clone, Copy)]
pub struct Basis(sys::godot_basis);

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

    pub fn x(&self) -> f32 {
        unsafe {
            (get_api().godot_vector3_get_axis)(&self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_X)
        }
    }

    pub fn y(&self) -> f32 {
        unsafe {
            (get_api().godot_vector3_get_axis)(&self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_Y)
        }
    }

    pub fn z(&self) -> f32 {
        unsafe {
            (get_api().godot_vector3_get_axis)(&self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_Z)
        }
    }

    pub fn set_x(&mut self, val: f32) {
        unsafe {
            (get_api().godot_vector3_set_axis)(&mut self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_X, val)
        }
    }

    pub fn set_y(&mut self, val: f32) {
        unsafe {
            (get_api().godot_vector3_set_axis)(&mut self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_Y, val)
        }
    }

    pub fn set_z(&mut self, val: f32) {
        unsafe {
            (get_api().godot_vector3_set_axis)(&mut self.0, sys::godot_vector3_axis::GODOT_VECTOR3_AXIS_Z, val)
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            (get_api().godot_vector3_length)(&self.0)
        }
    }

    pub fn length_squared(&self) -> f32 {
        unsafe {
            (get_api().godot_vector3_length_squared)(&self.0)
        }
    }

    pub fn is_normalized(&self) -> bool {
        unsafe {
            (get_api().godot_vector3_is_normalized)(&self.0)
        }
    }

    pub fn normalized(&self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_normalized)(&self.0))
        }
    }

    pub fn inverse(&self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_inverse)(&self.0))
        }
    }

    pub fn snapped(&self, by: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_snapped)(&self.0, &by.0))
        }
    }

    pub fn rotated(&self, axis: &Vector3, phi: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_rotated)(&self.0, &axis.0, phi))
        }
    }

    pub fn linear_interpolate(&self, b: &Vector3, t: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_linear_interpolate)(&self.0, &b.0, t))
        }
    }

    pub fn cubic_interpolate(&self, b: &Vector3, pre_a: &Vector3, post_b: &Vector3, t: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_cubic_interpolate)(&self.0, &b.0, &pre_a.0, &post_b.0, t))
        }
    }

    pub fn dot(&self, b: &Vector3) -> f32 {
        unsafe {
            (get_api().godot_vector3_dot)(&self.0, &b.0)
        }
    }

    pub fn cross(&self, b: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_cross)(&self.0, &b.0))
        }
    }

    pub fn outer(&self, b: &Vector3) -> Basis {
        unsafe {
            Basis((get_api().godot_vector3_outer)(&self.0, &b.0))
        }
    }

    pub fn to_diagonal_matrix(&self) -> Basis {
        unsafe {
            Basis((get_api().godot_vector3_to_diagonal_matrix)(&self.0))
        }
    }

    pub fn abs(&self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_abs)(&self.0))
        }
    }

    pub fn floor(&self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_floor)(&self.0))
        }
    }

    pub fn ceil(&self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_ceil)(&self.0))
        }
    }

    pub fn distance_to(&self, b: &Vector3) -> f32 {
        unsafe {
            (get_api().godot_vector3_distance_to)(&self.0, &b.0)
        }
    }

    pub fn distance_squared_to(&self, b: &Vector3) -> f32 {
        unsafe {
            (get_api().godot_vector3_distance_squared_to)(&self.0, &b.0)
        }
    }

    pub fn angle_to(&self, to: &Vector3) -> f32 {
        unsafe {
            (get_api().godot_vector3_angle_to)(&self.0, &to.0)
        }
    }

    pub fn slide(&self, n: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_slide)(&self.0, &n.0))
        }
    }

    pub fn bounce(&self, n: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_bounce)(&self.0, &n.0))
        }
    }

    pub fn reflect(&self, n: &Vector3) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_reflect)(&self.0, &n.0))
        }
    }
}

macro_rules! impl_godot_op {
    ($op:ty, $opfunc:ident, $a:ident, $func:ident) => (

impl $op for $a {
    type Output = $a;
    fn $opfunc(self, other: $a) -> $a {
        unsafe {
            $a((get_api().godot_vector3_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}

impl <'a> $op for &'a Vector3 {
    type Output = Vector3;
    fn $opfunc(self, other: &$a) -> Vector3 {
        unsafe {
            $a((get_api().godot_vector3_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}
    )
}

impl_godot_op!(Add, add, Vector3, godot_vector3_operator_add);
impl_godot_op!(Sub, sub, Vector3, godot_vector3_operator_sub);
impl_godot_op!(Mul, mul, Vector3, godot_vector3_operator_multiply_vector);
impl_godot_op!(Div, div, Vector3, godot_vector3_operator_divide_vector);

impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, other: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_multiply_scalar)(
                &self.0,
                other,
            ))
        }
    }
}

impl <'a> Mul<f32> for &'a Vector3 {
    type Output = Vector3;
    fn mul(self, other: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_multiply_scalar)(
                &self.0,
                other
            ))
        }
    }
}

impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, other: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_divide_scalar)(
                &self.0,
                other,
            ))
        }
    }
}

impl <'a> Div<f32> for &'a Vector3 {
    type Output = Vector3;
    fn div(self, other: f32) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_divide_scalar)(
                &self.0,
                other
            ))
        }
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        unsafe {
            Vector3((get_api().godot_vector3_operator_neg)(&self.0))
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Vector3) -> bool {
        unsafe {
            (get_api().godot_vector3_operator_equal)(&self.0, &other.0)
        }
    }
}

impl PartialOrd for Vector3 {
    fn partial_cmp(&self, other: &Vector3) -> Option<Ordering> {
        Some(unsafe {
            let api = get_api();
            if (api.godot_vector3_operator_equal)(&self.0, &other.0) {
                Ordering::Equal
            } else if (api.godot_vector3_operator_less)(&self.0, &other.0) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

impl Debug for Vector3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
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