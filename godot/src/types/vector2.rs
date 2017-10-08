use super::*;

#[derive(Clone, Copy)]
pub struct Vector2(pub(super) sys::godot_vector2);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        unsafe {
            let mut dest = sys::godot_vector2::default();
            (get_api().godot_vector2_new)(&mut dest, x, y);
            Vector2(dest)
        }
    }

    pub fn x(&self) -> f32 {
        unsafe {
            (get_api().godot_vector2_get_x)(&self.0)
        }
    }

    pub fn y(&self) -> f32 {
        unsafe {
            (get_api().godot_vector2_get_y)(&self.0)
        }
    }

    pub fn set_x(&mut self, val: f32) {
        unsafe {
            (get_api().godot_vector2_set_x)(&mut self.0, val)
        }
    }

    pub fn set_y(&mut self, val: f32) {
        unsafe {
            (get_api().godot_vector2_set_y)(&mut self.0, val)
        }
    }

    pub fn length(&self) -> f32 {
        unsafe {
            (get_api().godot_vector2_length)(&self.0)
        }
    }

    pub fn length_squared(&self) -> f32 {
        unsafe {
            (get_api().godot_vector2_length_squared)(&self.0)
        }
    }

    pub fn is_normalized(&self) -> bool {
        unsafe {
            (get_api().godot_vector2_is_normalized)(&self.0)
        }
    }

    pub fn normalized(&self) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_normalized)(&self.0))
        }
    }

    pub fn snapped(&self, by: &Vector2) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_snapped)(&self.0, &by.0))
        }
    }

    pub fn rotated(&self, phi: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_rotated)(&self.0, phi))
        }
    }

    pub fn linear_interpolate(&self, b: &Vector2, t: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_linear_interpolate)(&self.0, &b.0, t))
        }
    }

    pub fn cubic_interpolate(&self, b: &Vector2, pre_a: &Vector2, post_b: &Vector2, t: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_cubic_interpolate)(&self.0, &b.0, &pre_a.0, &post_b.0, t))
        }
    }

    pub fn dot(&self, b: &Vector2) -> f32 {
        unsafe {
            (get_api().godot_vector2_dot)(&self.0, &b.0)
        }
    }

    pub fn abs(&self) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_abs)(&self.0))
        }
    }

    pub fn floor(&self) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_floor)(&self.0))
        }
    }

    pub fn distance_to(&self, b: &Vector2) -> f32 {
        unsafe {
            (get_api().godot_vector2_distance_to)(&self.0, &b.0)
        }
    }

    pub fn distance_squared_to(&self, b: &Vector2) -> f32 {
        unsafe {
            (get_api().godot_vector2_distance_squared_to)(&self.0, &b.0)
        }
    }

    pub fn angle_to(&self, to: &Vector2) -> f32 {
        unsafe {
            (get_api().godot_vector2_angle_to)(&self.0, &to.0)
        }
    }

    pub fn slide(&self, n: &Vector2) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_slide)(&self.0, &n.0))
        }
    }

    pub fn bounce(&self, n: &Vector2) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_bounce)(&self.0, &n.0))
        }
    }

    pub fn reflect(&self, n: &Vector2) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_reflect)(&self.0, &n.0))
        }
    }
}

macro_rules! impl_godot_op {
    ($op:ty, $opfunc:ident, $a:ident, $func:ident) => (

impl $op for $a {
    type Output = $a;
    fn $opfunc(self, other: $a) -> $a {
        unsafe {
            $a((get_api().godot_vector2_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}

impl <'a> $op for &'a Vector2 {
    type Output = Vector2;
    fn $opfunc(self, other: &$a) -> Vector2 {
        unsafe {
            $a((get_api().godot_vector2_operator_add)(
                &self.0,
                &other.0,
            ))
        }
    }
}
    )
}

impl_godot_op!(Add, add, Vector2, godot_vector2_operator_add);
impl_godot_op!(Sub, sub, Vector2, godot_vector2_operator_sub);
impl_godot_op!(Mul, mul, Vector2, godot_vector2_operator_multiply_vector);
impl_godot_op!(Div, div, Vector2, godot_vector2_operator_divide_vector);

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_operator_multiply_scalar)(
                &self.0,
                other,
            ))
        }
    }
}

impl <'a> Mul<f32> for &'a Vector2 {
    type Output = Vector2;
    fn mul(self, other: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_operator_multiply_scalar)(
                &self.0,
                other
            ))
        }
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, other: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_operator_divide_scalar)(
                &self.0,
                other,
            ))
        }
    }
}

impl <'a> Div<f32> for &'a Vector2 {
    type Output = Vector2;
    fn div(self, other: f32) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_operator_divide_scalar)(
                &self.0,
                other
            ))
        }
    }
}

impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        unsafe {
            Vector2((get_api().godot_vector2_operator_neg)(&self.0))
        }
    }
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Vector2) -> bool {
        unsafe {
            (get_api().godot_vector2_operator_equal)(&self.0, &other.0)
        }
    }
}

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Vector2) -> Option<Ordering> {
        Some(unsafe {
            let api = get_api();
            if (api.godot_vector2_operator_equal)(&self.0, &other.0) {
                Ordering::Equal
            } else if (api.godot_vector2_operator_less)(&self.0, &other.0) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

impl Debug for Vector2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Vector2")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}

unsafe impl GodotType for Vector2 {
    fn as_variant(&self) -> sys::godot_variant {
        unsafe {
            let mut ret = sys::godot_variant::default();
            (get_api().godot_variant_new_vector2)(&mut ret, &self.0);
            ret
        }
    }

    fn from_variant(variant: &mut sys::godot_variant) -> Option<Self> {
        unsafe {
            let api = get_api();
            if (api.godot_variant_get_type)(variant) == sys::godot_variant_type::GODOT_VARIANT_TYPE_VECTOR2 {
                Some(Vector2((api.godot_variant_as_vector2)(variant)))
            } else {
                None
            }
        }
    }
}