use libc;
use sys::*;

macro_rules! def_api {
    (
struct GodotApi {
    $(
        pub $name:ident : $ty:ty,
    )*
}
    ) => (
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct GodotApi {
    $(
        pub $name: $ty,
    )*
}

impl GodotApi {
    pub unsafe fn from_raw(api: *const godot_gdnative_api_struct) -> GodotApi {
        let api = &*api;
        GodotApi {
            $(
                $name: api.$name.expect(concat!("Missing function: ", stringify!($name))),
            )*
        }
    }
}
    )
}

def_api! {
struct GodotApi {
    pub godot_color_new_rgba: unsafe extern "C" fn(r_dest:
                                                                             *mut godot_color,
                                                                         p_r:
                                                                             godot_real,
                                                                         p_g:
                                                                             godot_real,
                                                                         p_b:
                                                                             godot_real,
                                                                         p_a:
                                                                             godot_real),
    pub godot_color_new_rgb: unsafe extern "C" fn(r_dest:
                                                                            *mut godot_color,
                                                                        p_r:
                                                                            godot_real,
                                                                        p_g:
                                                                            godot_real,
                                                                        p_b:
                                                                            godot_real),
    pub godot_color_get_r: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_set_r: unsafe extern "C" fn(p_self:
                                                                          *mut godot_color,
                                                                      r:
                                                                          godot_real),
    pub godot_color_get_g: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_set_g: unsafe extern "C" fn(p_self:
                                                                          *mut godot_color,
                                                                      g:
                                                                          godot_real),
    pub godot_color_get_b: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_set_b: unsafe extern "C" fn(p_self:
                                                                          *mut godot_color,
                                                                      b:
                                                                          godot_real),
    pub godot_color_get_a: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_set_a: unsafe extern "C" fn(p_self:
                                                                          *mut godot_color,
                                                                      a:
                                                                          godot_real),
    pub godot_color_get_h: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_get_s: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_get_v: unsafe extern "C" fn(p_self:
                                                                          *const godot_color)
                                                     -> godot_real,
    pub godot_color_as_string: unsafe extern "C" fn(p_self:
                                                                              *const godot_color)
                                                         -> godot_string,
    pub godot_color_to_rgba32: unsafe extern "C" fn(p_self:
                                                                              *const godot_color)
                                                         -> godot_int,
    pub godot_color_to_argb32: unsafe extern "C" fn(p_self:
                                                                              *const godot_color)
                                                         -> godot_int,
    pub godot_color_gray: unsafe extern "C" fn(p_self:
                                                                         *const godot_color)
                                                    -> godot_real,
    pub godot_color_inverted: unsafe extern "C" fn(p_self:
                                                                             *const godot_color)
                                                        -> godot_color,
    pub godot_color_contrasted: unsafe extern "C" fn(p_self:
                                                                               *const godot_color)
                                                          -> godot_color,
    pub godot_color_linear_interpolate: unsafe extern "C" fn(p_self:
                                                                                       *const godot_color,
                                                                                   p_b:
                                                                                       *const godot_color,
                                                                                   p_t:
                                                                                       godot_real)
                                                                  ->
                                                                      godot_color,
    pub godot_color_blend: unsafe extern "C" fn(p_self:
                                                                          *const godot_color,
                                                                      p_over:
                                                                          *const godot_color)
                                                     -> godot_color,
    pub godot_color_to_html: unsafe extern "C" fn(p_self:
                                                                            *const godot_color,
                                                                        p_with_alpha:
                                                                            godot_bool)
                                                       -> godot_string,
    pub godot_color_operator_equal: unsafe extern "C" fn(p_self:
                                                                                   *const godot_color,
                                                                               p_b:
                                                                                   *const godot_color)
                                                              -> godot_bool,
    pub godot_color_operator_less: unsafe extern "C" fn(p_self:
                                                                                  *const godot_color,
                                                                              p_b:
                                                                                  *const godot_color)
                                                             -> godot_bool,
    pub godot_vector2_new: unsafe extern "C" fn(r_dest:
                                                                          *mut godot_vector2,
                                                                      p_x:
                                                                          godot_real,
                                                                      p_y:
                                                                          godot_real),
    pub godot_vector2_as_string: unsafe extern "C" fn(p_self:
                                                                                *const godot_vector2)
                                                           -> godot_string,
    pub godot_vector2_normalized: unsafe extern "C" fn(p_self:
                                                                                 *const godot_vector2)
                                                            -> godot_vector2,
    pub godot_vector2_length: unsafe extern "C" fn(p_self:
                                                                             *const godot_vector2)
                                                        -> godot_real,
    pub godot_vector2_angle: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector2)
                                                       -> godot_real,
    pub godot_vector2_length_squared: unsafe extern "C" fn(p_self:
                                                                                     *const godot_vector2)
                                                                ->
                                                                    godot_real,
    pub godot_vector2_is_normalized: unsafe extern "C" fn(p_self:
                                                                                    *const godot_vector2)
                                                               -> godot_bool,
    pub godot_vector2_distance_to: unsafe extern "C" fn(p_self:
                                                                                  *const godot_vector2,
                                                                              p_to:
                                                                                  *const godot_vector2)
                                                             -> godot_real,
    pub godot_vector2_distance_squared_to: unsafe extern "C" fn(p_self:
                                                                                          *const godot_vector2,
                                                                                      p_to:
                                                                                          *const godot_vector2)
                                                                     ->
                                                                         godot_real,
    pub godot_vector2_angle_to: unsafe extern "C" fn(p_self:
                                                                               *const godot_vector2,
                                                                           p_to:
                                                                               *const godot_vector2)
                                                          -> godot_real,
    pub godot_vector2_angle_to_point: unsafe extern "C" fn(p_self:
                                                                                     *const godot_vector2,
                                                                                 p_to:
                                                                                     *const godot_vector2)
                                                                ->
                                                                    godot_real,
    pub godot_vector2_linear_interpolate: unsafe extern "C" fn(p_self:
                                                                                         *const godot_vector2,
                                                                                     p_b:
                                                                                         *const godot_vector2,
                                                                                     p_t:
                                                                                         godot_real)
                                                                    ->
                                                                        godot_vector2,
    pub godot_vector2_cubic_interpolate: unsafe extern "C" fn(p_self:
                                                                                        *const godot_vector2,
                                                                                    p_b:
                                                                                        *const godot_vector2,
                                                                                    p_pre_a:
                                                                                        *const godot_vector2,
                                                                                    p_post_b:
                                                                                        *const godot_vector2,
                                                                                    p_t:
                                                                                        godot_real)
                                                                   ->
                                                                       godot_vector2,
    pub godot_vector2_rotated: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector2,
                                                                          p_phi:
                                                                              godot_real)
                                                         -> godot_vector2,
    pub godot_vector2_tangent: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector2)
                                                         -> godot_vector2,
    pub godot_vector2_floor: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector2)
                                                       -> godot_vector2,
    pub godot_vector2_snapped: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector2,
                                                                          p_by:
                                                                              *const godot_vector2)
                                                         -> godot_vector2,
    pub godot_vector2_aspect: unsafe extern "C" fn(p_self:
                                                                             *const godot_vector2)
                                                        -> godot_real,
    pub godot_vector2_dot: unsafe extern "C" fn(p_self:
                                                                          *const godot_vector2,
                                                                      p_with:
                                                                          *const godot_vector2)
                                                     -> godot_real,
    pub godot_vector2_slide: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector2,
                                                                        p_n:
                                                                            *const godot_vector2)
                                                       -> godot_vector2,
    pub godot_vector2_bounce: unsafe extern "C" fn(p_self:
                                                                             *const godot_vector2,
                                                                         p_n:
                                                                             *const godot_vector2)
                                                        -> godot_vector2,
    pub godot_vector2_reflect: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector2,
                                                                          p_n:
                                                                              *const godot_vector2)
                                                         -> godot_vector2,
    pub godot_vector2_abs: unsafe extern "C" fn(p_self:
                                                                          *const godot_vector2)
                                                     -> godot_vector2,
    pub godot_vector2_clamped: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector2,
                                                                          p_length:
                                                                              godot_real)
                                                         -> godot_vector2,
    pub godot_vector2_operator_add: unsafe extern "C" fn(p_self:
                                                                                   *const godot_vector2,
                                                                               p_b:
                                                                                   *const godot_vector2)
                                                              ->
                                                                  godot_vector2,
    pub godot_vector2_operator_substract: unsafe extern "C" fn(p_self:
                                                                                         *const godot_vector2,
                                                                                     p_b:
                                                                                         *const godot_vector2)
                                                                    ->
                                                                        godot_vector2,
    pub godot_vector2_operator_multiply_vector: unsafe extern "C" fn(p_self:
                                                                                               *const godot_vector2,
                                                                                           p_b:
                                                                                               *const godot_vector2)
                                                                          ->
                                                                              godot_vector2,
    pub godot_vector2_operator_multiply_scalar: unsafe extern "C" fn(p_self:
                                                                                               *const godot_vector2,
                                                                                           p_b:
                                                                                               godot_real)
                                                                          ->
                                                                              godot_vector2,
    pub godot_vector2_operator_divide_vector: unsafe extern "C" fn(p_self:
                                                                                             *const godot_vector2,
                                                                                         p_b:
                                                                                             *const godot_vector2)
                                                                        ->
                                                                            godot_vector2,
    pub godot_vector2_operator_divide_scalar: unsafe extern "C" fn(p_self:
                                                                                             *const godot_vector2,
                                                                                         p_b:
                                                                                             godot_real)
                                                                        ->
                                                                            godot_vector2,
    pub godot_vector2_operator_equal: unsafe extern "C" fn(p_self:
                                                                                     *const godot_vector2,
                                                                                 p_b:
                                                                                     *const godot_vector2)
                                                                ->
                                                                    godot_bool,
    pub godot_vector2_operator_less: unsafe extern "C" fn(p_self:
                                                                                    *const godot_vector2,
                                                                                p_b:
                                                                                    *const godot_vector2)
                                                               -> godot_bool,
    pub godot_vector2_operator_neg: unsafe extern "C" fn(p_self:
                                                                                   *const godot_vector2)
                                                              ->
                                                                  godot_vector2,
    pub godot_vector2_set_x: unsafe extern "C" fn(p_self:
                                                                            *mut godot_vector2,
                                                                        p_x:
                                                                            godot_real),
    pub godot_vector2_set_y: unsafe extern "C" fn(p_self:
                                                                            *mut godot_vector2,
                                                                        p_y:
                                                                            godot_real),
    pub godot_vector2_get_x: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector2)
                                                       -> godot_real,
    pub godot_vector2_get_y: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector2)
                                                       -> godot_real,
    pub godot_quat_new: unsafe extern "C" fn(r_dest:
                                                                       *mut godot_quat,
                                                                   p_x:
                                                                       godot_real,
                                                                   p_y:
                                                                       godot_real,
                                                                   p_z:
                                                                       godot_real,
                                                                   p_w:
                                                                       godot_real),
    pub godot_quat_new_with_axis_angle: unsafe extern "C" fn(r_dest:
                                                                                       *mut godot_quat,
                                                                                   p_axis:
                                                                                       *const godot_vector3,
                                                                                   p_angle:
                                                                                       godot_real),
    pub godot_quat_get_x: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat)
                                                    -> godot_real,
    pub godot_quat_set_x: unsafe extern "C" fn(p_self:
                                                                         *mut godot_quat,
                                                                     val:
                                                                         godot_real),
    pub godot_quat_get_y: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat)
                                                    -> godot_real,
    pub godot_quat_set_y: unsafe extern "C" fn(p_self:
                                                                         *mut godot_quat,
                                                                     val:
                                                                         godot_real),
    pub godot_quat_get_z: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat)
                                                    -> godot_real,
    pub godot_quat_set_z: unsafe extern "C" fn(p_self:
                                                                         *mut godot_quat,
                                                                     val:
                                                                         godot_real),
    pub godot_quat_get_w: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat)
                                                    -> godot_real,
    pub godot_quat_set_w: unsafe extern "C" fn(p_self:
                                                                         *mut godot_quat,
                                                                     val:
                                                                         godot_real),
    pub godot_quat_as_string: unsafe extern "C" fn(p_self:
                                                                             *const godot_quat)
                                                        -> godot_string,
    pub godot_quat_length: unsafe extern "C" fn(p_self:
                                                                          *const godot_quat)
                                                     -> godot_real,
    pub godot_quat_length_squared: unsafe extern "C" fn(p_self:
                                                                                  *const godot_quat)
                                                             -> godot_real,
    pub godot_quat_normalized: unsafe extern "C" fn(p_self:
                                                                              *const godot_quat)
                                                         -> godot_quat,
    pub godot_quat_is_normalized: unsafe extern "C" fn(p_self:
                                                                                 *const godot_quat)
                                                            -> godot_bool,
    pub godot_quat_inverse: unsafe extern "C" fn(p_self:
                                                                           *const godot_quat)
                                                      -> godot_quat,
    pub godot_quat_dot: unsafe extern "C" fn(p_self:
                                                                       *const godot_quat,
                                                                   p_b:
                                                                       *const godot_quat)
                                                  -> godot_real,
    pub godot_quat_xform: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat,
                                                                     p_v:
                                                                         *const godot_vector3)
                                                    -> godot_vector3,
    pub godot_quat_slerp: unsafe extern "C" fn(p_self:
                                                                         *const godot_quat,
                                                                     p_b:
                                                                         *const godot_quat,
                                                                     p_t:
                                                                         godot_real)
                                                    -> godot_quat,
    pub godot_quat_slerpni: unsafe extern "C" fn(p_self:
                                                                           *const godot_quat,
                                                                       p_b:
                                                                           *const godot_quat,
                                                                       p_t:
                                                                           godot_real)
                                                      -> godot_quat,
    pub godot_quat_cubic_slerp: unsafe extern "C" fn(p_self:
                                                                               *const godot_quat,
                                                                           p_b:
                                                                               *const godot_quat,
                                                                           p_pre_a:
                                                                               *const godot_quat,
                                                                           p_post_b:
                                                                               *const godot_quat,
                                                                           p_t:
                                                                               godot_real)
                                                          -> godot_quat,
    pub godot_quat_operator_multiply: unsafe extern "C" fn(p_self:
                                                                                     *const godot_quat,
                                                                                 p_b:
                                                                                     godot_real)
                                                                ->
                                                                    godot_quat,
    pub godot_quat_operator_add: unsafe extern "C" fn(p_self:
                                                                                *const godot_quat,
                                                                            p_b:
                                                                                *const godot_quat)
                                                           -> godot_quat,
    pub godot_quat_operator_substract: unsafe extern "C" fn(p_self:
                                                                                      *const godot_quat,
                                                                                  p_b:
                                                                                      *const godot_quat)
                                                                 ->
                                                                     godot_quat,
    pub godot_quat_operator_divide: unsafe extern "C" fn(p_self:
                                                                                   *const godot_quat,
                                                                               p_b:
                                                                                   godot_real)
                                                              -> godot_quat,
    pub godot_quat_operator_equal: unsafe extern "C" fn(p_self:
                                                                                  *const godot_quat,
                                                                              p_b:
                                                                                  *const godot_quat)
                                                             -> godot_bool,
    pub godot_quat_operator_neg: unsafe extern "C" fn(p_self:
                                                                                *const godot_quat)
                                                           -> godot_quat,
    pub godot_basis_new_with_rows: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_basis,
                                                                              p_x_axis:
                                                                                  *const godot_vector3,
                                                                              p_y_axis:
                                                                                  *const godot_vector3,
                                                                              p_z_axis:
                                                                                  *const godot_vector3),
    pub godot_basis_new_with_axis_and_angle: unsafe extern "C" fn(r_dest:
                                                                                            *mut godot_basis,
                                                                                        p_axis:
                                                                                            *const godot_vector3,
                                                                                        p_phi:
                                                                                            godot_real),
    pub godot_basis_new_with_euler: unsafe extern "C" fn(r_dest:
                                                                                   *mut godot_basis,
                                                                               p_euler:
                                                                                   *const godot_vector3),
    pub godot_basis_as_string: unsafe extern "C" fn(p_self:
                                                                              *const godot_basis)
                                                         -> godot_string,
    pub godot_basis_inverse: unsafe extern "C" fn(p_self:
                                                                            *const godot_basis)
                                                       -> godot_basis,
    pub godot_basis_transposed: unsafe extern "C" fn(p_self:
                                                                               *const godot_basis)
                                                          -> godot_basis,
    pub godot_basis_orthonormalized: unsafe extern "C" fn(p_self:
                                                                                    *const godot_basis)
                                                               ->
                                                                   godot_basis,
    pub godot_basis_determinant: unsafe extern "C" fn(p_self:
                                                                                *const godot_basis)
                                                           -> godot_real,
    pub godot_basis_rotated: unsafe extern "C" fn(p_self:
                                                                            *const godot_basis,
                                                                        p_axis:
                                                                            *const godot_vector3,
                                                                        p_phi:
                                                                            godot_real)
                                                       -> godot_basis,
    pub godot_basis_scaled: unsafe extern "C" fn(p_self:
                                                                           *const godot_basis,
                                                                       p_scale:
                                                                           *const godot_vector3)
                                                      -> godot_basis,
    pub godot_basis_get_scale: unsafe extern "C" fn(p_self:
                                                                              *const godot_basis)
                                                         -> godot_vector3,
    pub godot_basis_get_euler: unsafe extern "C" fn(p_self:
                                                                              *const godot_basis)
                                                         -> godot_vector3,
    pub godot_basis_tdotx: unsafe extern "C" fn(p_self:
                                                                          *const godot_basis,
                                                                      p_with:
                                                                          *const godot_vector3)
                                                     -> godot_real,
    pub godot_basis_tdoty: unsafe extern "C" fn(p_self:
                                                                          *const godot_basis,
                                                                      p_with:
                                                                          *const godot_vector3)
                                                     -> godot_real,
    pub godot_basis_tdotz: unsafe extern "C" fn(p_self:
                                                                          *const godot_basis,
                                                                      p_with:
                                                                          *const godot_vector3)
                                                     -> godot_real,
    pub godot_basis_xform: unsafe extern "C" fn(p_self:
                                                                          *const godot_basis,
                                                                      p_v:
                                                                          *const godot_vector3)
                                                     -> godot_vector3,
    pub godot_basis_xform_inv: unsafe extern "C" fn(p_self:
                                                                              *const godot_basis,
                                                                          p_v:
                                                                              *const godot_vector3)
                                                         -> godot_vector3,
    pub godot_basis_get_orthogonal_index: unsafe extern "C" fn(p_self:
                                                                                         *const godot_basis)
                                                                    ->
                                                                        godot_int,
    pub godot_basis_new: unsafe extern "C" fn(r_dest:
                                                                        *mut godot_basis),
    pub godot_basis_new_with_euler_quat: unsafe extern "C" fn(r_dest:
                                                                                        *mut godot_basis,
                                                                                    p_euler:
                                                                                        *const godot_quat),
    pub godot_basis_get_elements: unsafe extern "C" fn(p_self:
                                                                                 *mut godot_basis,
                                                                             p_elements:
                                                                                 *mut godot_vector3),
    pub godot_basis_get_axis: unsafe extern "C" fn(p_self:
                                                                             *const godot_basis,
                                                                         p_axis:
                                                                             godot_int)
                                                        -> godot_vector3,
    pub godot_basis_set_axis: unsafe extern "C" fn(p_self:
                                                                             *mut godot_basis,
                                                                         p_axis:
                                                                             godot_int,
                                                                         p_value:
                                                                             *const godot_vector3),
    pub godot_basis_get_row: unsafe extern "C" fn(p_self:
                                                                            *const godot_basis,
                                                                        p_row:
                                                                            godot_int)
                                                       -> godot_vector3,
    pub godot_basis_set_row: unsafe extern "C" fn(p_self:
                                                                            *mut godot_basis,
                                                                        p_row:
                                                                            godot_int,
                                                                        p_value:
                                                                            *const godot_vector3),
    pub godot_basis_operator_equal: unsafe extern "C" fn(p_self:
                                                                                   *const godot_basis,
                                                                               p_b:
                                                                                   *const godot_basis)
                                                              -> godot_bool,
    pub godot_basis_operator_add: unsafe extern "C" fn(p_self:
                                                                                 *const godot_basis,
                                                                             p_b:
                                                                                 *const godot_basis)
                                                            -> godot_basis,
    pub godot_basis_operator_substract: unsafe extern "C" fn(p_self:
                                                                                       *const godot_basis,
                                                                                   p_b:
                                                                                       *const godot_basis)
                                                                  ->
                                                                      godot_basis,
    pub godot_basis_operator_multiply_vector: unsafe extern "C" fn(p_self:
                                                                                             *const godot_basis,
                                                                                         p_b:
                                                                                             *const godot_basis)
                                                                        ->
                                                                            godot_basis,
    pub godot_basis_operator_multiply_scalar: unsafe extern "C" fn(p_self:
                                                                                             *const godot_basis,
                                                                                         p_b:
                                                                                             godot_real)
                                                                        ->
                                                                            godot_basis,
    pub godot_vector3_new: unsafe extern "C" fn(r_dest:
                                                                          *mut godot_vector3,
                                                                      p_x:
                                                                          godot_real,
                                                                      p_y:
                                                                          godot_real,
                                                                      p_z:
                                                                          godot_real),
    pub godot_vector3_as_string: unsafe extern "C" fn(p_self:
                                                                                *const godot_vector3)
                                                           -> godot_string,
    pub godot_vector3_min_axis: unsafe extern "C" fn(p_self:
                                                                               *const godot_vector3)
                                                          -> godot_int,
    pub godot_vector3_max_axis: unsafe extern "C" fn(p_self:
                                                                               *const godot_vector3)
                                                          -> godot_int,
    pub godot_vector3_length: unsafe extern "C" fn(p_self:
                                                                             *const godot_vector3)
                                                        -> godot_real,
    pub godot_vector3_length_squared: unsafe extern "C" fn(p_self:
                                                                                     *const godot_vector3)
                                                                ->
                                                                    godot_real,
    pub godot_vector3_is_normalized: unsafe extern "C" fn(p_self:
                                                                                    *const godot_vector3)
                                                               -> godot_bool,
    pub godot_vector3_normalized: unsafe extern "C" fn(p_self:
                                                                                 *const godot_vector3)
                                                            -> godot_vector3,
    pub godot_vector3_inverse: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector3)
                                                         -> godot_vector3,
    pub godot_vector3_snapped: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector3,
                                                                          p_by:
                                                                              *const godot_vector3)
                                                         -> godot_vector3,
    pub godot_vector3_rotated: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector3,
                                                                          p_axis:
                                                                              *const godot_vector3,
                                                                          p_phi:
                                                                              godot_real)
                                                         -> godot_vector3,
    pub godot_vector3_linear_interpolate: unsafe extern "C" fn(p_self:
                                                                                         *const godot_vector3,
                                                                                     p_b:
                                                                                         *const godot_vector3,
                                                                                     p_t:
                                                                                         godot_real)
                                                                    ->
                                                                        godot_vector3,
    pub godot_vector3_cubic_interpolate: unsafe extern "C" fn(p_self:
                                                                                        *const godot_vector3,
                                                                                    p_b:
                                                                                        *const godot_vector3,
                                                                                    p_pre_a:
                                                                                        *const godot_vector3,
                                                                                    p_post_b:
                                                                                        *const godot_vector3,
                                                                                    p_t:
                                                                                        godot_real)
                                                                   ->
                                                                       godot_vector3,
    pub godot_vector3_dot: unsafe extern "C" fn(p_self:
                                                                          *const godot_vector3,
                                                                      p_b:
                                                                          *const godot_vector3)
                                                     -> godot_real,
    pub godot_vector3_cross: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector3,
                                                                        p_b:
                                                                            *const godot_vector3)
                                                       -> godot_vector3,
    pub godot_vector3_outer: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector3,
                                                                        p_b:
                                                                            *const godot_vector3)
                                                       -> godot_basis,
    pub godot_vector3_to_diagonal_matrix: unsafe extern "C" fn(p_self:
                                                                                         *const godot_vector3)
                                                                    ->
                                                                        godot_basis,
    pub godot_vector3_abs: unsafe extern "C" fn(p_self:
                                                                          *const godot_vector3)
                                                     -> godot_vector3,
    pub godot_vector3_floor: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector3)
                                                       -> godot_vector3,
    pub godot_vector3_ceil: unsafe extern "C" fn(p_self:
                                                                           *const godot_vector3)
                                                      -> godot_vector3,
    pub godot_vector3_distance_to: unsafe extern "C" fn(p_self:
                                                                                  *const godot_vector3,
                                                                              p_b:
                                                                                  *const godot_vector3)
                                                             -> godot_real,
    pub godot_vector3_distance_squared_to: unsafe extern "C" fn(p_self:
                                                                                          *const godot_vector3,
                                                                                      p_b:
                                                                                          *const godot_vector3)
                                                                     ->
                                                                         godot_real,
    pub godot_vector3_angle_to: unsafe extern "C" fn(p_self:
                                                                               *const godot_vector3,
                                                                           p_to:
                                                                               *const godot_vector3)
                                                          -> godot_real,
    pub godot_vector3_slide: unsafe extern "C" fn(p_self:
                                                                            *const godot_vector3,
                                                                        p_n:
                                                                            *const godot_vector3)
                                                       -> godot_vector3,
    pub godot_vector3_bounce: unsafe extern "C" fn(p_self:
                                                                             *const godot_vector3,
                                                                         p_n:
                                                                             *const godot_vector3)
                                                        -> godot_vector3,
    pub godot_vector3_reflect: unsafe extern "C" fn(p_self:
                                                                              *const godot_vector3,
                                                                          p_n:
                                                                              *const godot_vector3)
                                                         -> godot_vector3,
    pub godot_vector3_operator_add: unsafe extern "C" fn(p_self:
                                                                                   *const godot_vector3,
                                                                               p_b:
                                                                                   *const godot_vector3)
                                                              ->
                                                                  godot_vector3,
    pub godot_vector3_operator_substract: unsafe extern "C" fn(p_self:
                                                                                         *const godot_vector3,
                                                                                     p_b:
                                                                                         *const godot_vector3)
                                                                    ->
                                                                        godot_vector3,
    pub godot_vector3_operator_multiply_vector: unsafe extern "C" fn(p_self:
                                                                                               *const godot_vector3,
                                                                                           p_b:
                                                                                               *const godot_vector3)
                                                                          ->
                                                                              godot_vector3,
    pub godot_vector3_operator_multiply_scalar: unsafe extern "C" fn(p_self:
                                                                                               *const godot_vector3,
                                                                                           p_b:
                                                                                               godot_real)
                                                                          ->
                                                                              godot_vector3,
    pub godot_vector3_operator_divide_vector: unsafe extern "C" fn(p_self:
                                                                                             *const godot_vector3,
                                                                                         p_b:
                                                                                             *const godot_vector3)
                                                                        ->
                                                                            godot_vector3,
    pub godot_vector3_operator_divide_scalar: unsafe extern "C" fn(p_self:
                                                                                             *const godot_vector3,
                                                                                         p_b:
                                                                                             godot_real)
                                                                        ->
                                                                            godot_vector3,
    pub godot_vector3_operator_equal: unsafe extern "C" fn(p_self:
                                                                                     *const godot_vector3,
                                                                                 p_b:
                                                                                     *const godot_vector3)
                                                                ->
                                                                    godot_bool,
    pub godot_vector3_operator_less: unsafe extern "C" fn(p_self:
                                                                                    *const godot_vector3,
                                                                                p_b:
                                                                                    *const godot_vector3)
                                                               -> godot_bool,
    pub godot_vector3_operator_neg: unsafe extern "C" fn(p_self:
                                                                                   *const godot_vector3)
                                                              ->
                                                                  godot_vector3,
    pub godot_vector3_set_axis: unsafe extern "C" fn(p_self:
                                                                               *mut godot_vector3,
                                                                           p_axis:
                                                                               godot_vector3_axis,
                                                                           p_val:
                                                                               godot_real),
    pub godot_vector3_get_axis: unsafe extern "C" fn(p_self:
                                                                               *const godot_vector3,
                                                                           p_axis:
                                                                               godot_vector3_axis)
                                                          -> godot_real,
    pub godot_pool_byte_array_new: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_pool_byte_array),
    pub godot_pool_byte_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                       *mut godot_pool_byte_array,
                                                                                   p_src:
                                                                                       *const godot_pool_byte_array),
    pub godot_pool_byte_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                             *mut godot_pool_byte_array,
                                                                                         p_a:
                                                                                             *const godot_array),
    pub godot_pool_byte_array_append: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_byte_array,
                                                                                 p_data:
                                                                                     u8),
    pub godot_pool_byte_array_append_array: unsafe extern "C" fn(p_self:
                                                                                           *mut godot_pool_byte_array,
                                                                                       p_array:
                                                                                           *const godot_pool_byte_array),
    pub godot_pool_byte_array_insert: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_byte_array,
                                                                                 p_idx:
                                                                                     godot_int,
                                                                                 p_data:
                                                                                     u8)
                                                                ->
                                                                    godot_error,
    pub godot_pool_byte_array_invert: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_byte_array),
    pub godot_pool_byte_array_push_back: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_byte_array,
                                                                                    p_data:
                                                                                        u8),
    pub godot_pool_byte_array_remove: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_byte_array,
                                                                                 p_idx:
                                                                                     godot_int),
    pub godot_pool_byte_array_resize: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_byte_array,
                                                                                 p_size:
                                                                                     godot_int),
    pub godot_pool_byte_array_set: unsafe extern "C" fn(p_self:
                                                                                  *mut godot_pool_byte_array,
                                                                              p_idx:
                                                                                  godot_int,
                                                                              p_data:
                                                                                  u8),
    pub godot_pool_byte_array_get: unsafe extern "C" fn(p_self:
                                                                                  *const godot_pool_byte_array,
                                                                              p_idx:
                                                                                  godot_int)
                                                             -> u8,
    pub godot_pool_byte_array_size: unsafe extern "C" fn(p_self:
                                                                                   *const godot_pool_byte_array)
                                                              -> godot_int,
    pub godot_pool_byte_array_destroy: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_byte_array),
    pub godot_pool_int_array_new: unsafe extern "C" fn(r_dest:
                                                                                 *mut godot_pool_int_array),
    pub godot_pool_int_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                      *mut godot_pool_int_array,
                                                                                  p_src:
                                                                                      *const godot_pool_int_array),
    pub godot_pool_int_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                            *mut godot_pool_int_array,
                                                                                        p_a:
                                                                                            *const godot_array),
    pub godot_pool_int_array_append: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_int_array,
                                                                                p_data:
                                                                                    godot_int),
    pub godot_pool_int_array_append_array: unsafe extern "C" fn(p_self:
                                                                                          *mut godot_pool_int_array,
                                                                                      p_array:
                                                                                          *const godot_pool_int_array),
    pub godot_pool_int_array_insert: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_int_array,
                                                                                p_idx:
                                                                                    godot_int,
                                                                                p_data:
                                                                                    godot_int)
                                                               ->
                                                                   godot_error,
    pub godot_pool_int_array_invert: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_int_array),
    pub godot_pool_int_array_push_back: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_int_array,
                                                                                   p_data:
                                                                                       godot_int),
    pub godot_pool_int_array_remove: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_int_array,
                                                                                p_idx:
                                                                                    godot_int),
    pub godot_pool_int_array_resize: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_int_array,
                                                                                p_size:
                                                                                    godot_int),
    pub godot_pool_int_array_set: unsafe extern "C" fn(p_self:
                                                                                 *mut godot_pool_int_array,
                                                                             p_idx:
                                                                                 godot_int,
                                                                             p_data:
                                                                                 godot_int),
    pub godot_pool_int_array_get: unsafe extern "C" fn(p_self:
                                                                                 *const godot_pool_int_array,
                                                                             p_idx:
                                                                                 godot_int)
                                                            -> godot_int,
    pub godot_pool_int_array_size: unsafe extern "C" fn(p_self:
                                                                                  *const godot_pool_int_array)
                                                             -> godot_int,
    pub godot_pool_int_array_destroy: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_int_array),
    pub godot_pool_real_array_new: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_pool_real_array),
    pub godot_pool_real_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                       *mut godot_pool_real_array,
                                                                                   p_src:
                                                                                       *const godot_pool_real_array),
    pub godot_pool_real_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                             *mut godot_pool_real_array,
                                                                                         p_a:
                                                                                             *const godot_array),
    pub godot_pool_real_array_append: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_real_array,
                                                                                 p_data:
                                                                                     godot_real),
    pub godot_pool_real_array_append_array: unsafe extern "C" fn(p_self:
                                                                                           *mut godot_pool_real_array,
                                                                                       p_array:
                                                                                           *const godot_pool_real_array),
    pub godot_pool_real_array_insert: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_real_array,
                                                                                 p_idx:
                                                                                     godot_int,
                                                                                 p_data:
                                                                                     godot_real)
                                                                ->
                                                                    godot_error,
    pub godot_pool_real_array_invert: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_real_array),
    pub godot_pool_real_array_push_back: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_real_array,
                                                                                    p_data:
                                                                                        godot_real),
    pub godot_pool_real_array_remove: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_real_array,
                                                                                 p_idx:
                                                                                     godot_int),
    pub godot_pool_real_array_resize: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_real_array,
                                                                                 p_size:
                                                                                     godot_int),
    pub godot_pool_real_array_set: unsafe extern "C" fn(p_self:
                                                                                  *mut godot_pool_real_array,
                                                                              p_idx:
                                                                                  godot_int,
                                                                              p_data:
                                                                                  godot_real),
    pub godot_pool_real_array_get: unsafe extern "C" fn(p_self:
                                                                                  *const godot_pool_real_array,
                                                                              p_idx:
                                                                                  godot_int)
                                                             -> godot_real,
    pub godot_pool_real_array_size: unsafe extern "C" fn(p_self:
                                                                                   *const godot_pool_real_array)
                                                              -> godot_int,
    pub godot_pool_real_array_destroy: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_real_array),
    pub godot_pool_string_array_new: unsafe extern "C" fn(r_dest:
                                                                                    *mut godot_pool_string_array),
    pub godot_pool_string_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                         *mut godot_pool_string_array,
                                                                                     p_src:
                                                                                         *const godot_pool_string_array),
    pub godot_pool_string_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                               *mut godot_pool_string_array,
                                                                                           p_a:
                                                                                               *const godot_array),
    pub godot_pool_string_array_append: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_string_array,
                                                                                   p_data:
                                                                                       *const godot_string),
    pub godot_pool_string_array_append_array: unsafe extern "C" fn(p_self:
                                                                                             *mut godot_pool_string_array,
                                                                                         p_array:
                                                                                             *const godot_pool_string_array),
    pub godot_pool_string_array_insert: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_string_array,
                                                                                   p_idx:
                                                                                       godot_int,
                                                                                   p_data:
                                                                                       *const godot_string)
                                                                  ->
                                                                      godot_error,
    pub godot_pool_string_array_invert: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_string_array),
    pub godot_pool_string_array_push_back: unsafe extern "C" fn(p_self:
                                                                                          *mut godot_pool_string_array,
                                                                                      p_data:
                                                                                          *const godot_string),
    pub godot_pool_string_array_remove: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_string_array,
                                                                                   p_idx:
                                                                                       godot_int),
    pub godot_pool_string_array_resize: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_string_array,
                                                                                   p_size:
                                                                                       godot_int),
    pub godot_pool_string_array_set: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_pool_string_array,
                                                                                p_idx:
                                                                                    godot_int,
                                                                                p_data:
                                                                                    *const godot_string),
    pub godot_pool_string_array_get: unsafe extern "C" fn(p_self:
                                                                                    *const godot_pool_string_array,
                                                                                p_idx:
                                                                                    godot_int)
                                                               ->
                                                                   godot_string,
    pub godot_pool_string_array_size: unsafe extern "C" fn(p_self:
                                                                                     *const godot_pool_string_array)
                                                                -> godot_int,
    pub godot_pool_string_array_destroy: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_string_array),
    pub godot_pool_vector2_array_new: unsafe extern "C" fn(r_dest:
                                                                                     *mut godot_pool_vector2_array),
    pub godot_pool_vector2_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_pool_vector2_array,
                                                                                      p_src:
                                                                                          *const godot_pool_vector2_array),
    pub godot_pool_vector2_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                                *mut godot_pool_vector2_array,
                                                                                            p_a:
                                                                                                *const godot_array),
    pub godot_pool_vector2_array_append: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector2_array,
                                                                                    p_data:
                                                                                        *const godot_vector2),
    pub godot_pool_vector2_array_append_array: unsafe extern "C" fn(p_self:
                                                                                              *mut godot_pool_vector2_array,
                                                                                          p_array:
                                                                                              *const godot_pool_vector2_array),
    pub godot_pool_vector2_array_insert: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector2_array,
                                                                                    p_idx:
                                                                                        godot_int,
                                                                                    p_data:
                                                                                        *const godot_vector2)
                                                                   ->
                                                                       godot_error,
    pub godot_pool_vector2_array_invert: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector2_array),
    pub godot_pool_vector2_array_push_back: unsafe extern "C" fn(p_self:
                                                                                           *mut godot_pool_vector2_array,
                                                                                       p_data:
                                                                                           *const godot_vector2),
    pub godot_pool_vector2_array_remove: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector2_array,
                                                                                    p_idx:
                                                                                        godot_int),
    pub godot_pool_vector2_array_resize: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector2_array,
                                                                                    p_size:
                                                                                        godot_int),
    pub godot_pool_vector2_array_set: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_vector2_array,
                                                                                 p_idx:
                                                                                     godot_int,
                                                                                 p_data:
                                                                                     *const godot_vector2),
    pub godot_pool_vector2_array_get: unsafe extern "C" fn(p_self:
                                                                                     *const godot_pool_vector2_array,
                                                                                 p_idx:
                                                                                     godot_int)
                                                                ->
                                                                    godot_vector2,
    pub godot_pool_vector2_array_size: unsafe extern "C" fn(p_self:
                                                                                      *const godot_pool_vector2_array)
                                                                 ->
                                                                     godot_int,
    pub godot_pool_vector2_array_destroy: unsafe extern "C" fn(p_self:
                                                                                         *mut godot_pool_vector2_array),
    pub godot_pool_vector3_array_new: unsafe extern "C" fn(r_dest:
                                                                                     *mut godot_pool_vector3_array),
    pub godot_pool_vector3_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_pool_vector3_array,
                                                                                      p_src:
                                                                                          *const godot_pool_vector3_array),
    pub godot_pool_vector3_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                                *mut godot_pool_vector3_array,
                                                                                            p_a:
                                                                                                *const godot_array),
    pub godot_pool_vector3_array_append: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector3_array,
                                                                                    p_data:
                                                                                        *const godot_vector3),
    pub godot_pool_vector3_array_append_array: unsafe extern "C" fn(p_self:
                                                                                              *mut godot_pool_vector3_array,
                                                                                          p_array:
                                                                                              *const godot_pool_vector3_array),
    pub godot_pool_vector3_array_insert: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector3_array,
                                                                                    p_idx:
                                                                                        godot_int,
                                                                                    p_data:
                                                                                        *const godot_vector3)
                                                                   ->
                                                                       godot_error,
    pub godot_pool_vector3_array_invert: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector3_array),
    pub godot_pool_vector3_array_push_back: unsafe extern "C" fn(p_self:
                                                                                           *mut godot_pool_vector3_array,
                                                                                       p_data:
                                                                                           *const godot_vector3),
    pub godot_pool_vector3_array_remove: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector3_array,
                                                                                    p_idx:
                                                                                        godot_int),
    pub godot_pool_vector3_array_resize: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_pool_vector3_array,
                                                                                    p_size:
                                                                                        godot_int),
    pub godot_pool_vector3_array_set: unsafe extern "C" fn(p_self:
                                                                                     *mut godot_pool_vector3_array,
                                                                                 p_idx:
                                                                                     godot_int,
                                                                                 p_data:
                                                                                     *const godot_vector3),
    pub godot_pool_vector3_array_get: unsafe extern "C" fn(p_self:
                                                                                     *const godot_pool_vector3_array,
                                                                                 p_idx:
                                                                                     godot_int)
                                                                ->
                                                                    godot_vector3,
    pub godot_pool_vector3_array_size: unsafe extern "C" fn(p_self:
                                                                                      *const godot_pool_vector3_array)
                                                                 ->
                                                                     godot_int,
    pub godot_pool_vector3_array_destroy: unsafe extern "C" fn(p_self:
                                                                                         *mut godot_pool_vector3_array),
    pub godot_pool_color_array_new: unsafe extern "C" fn(r_dest:
                                                                                   *mut godot_pool_color_array),
    pub godot_pool_color_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                                        *mut godot_pool_color_array,
                                                                                    p_src:
                                                                                        *const godot_pool_color_array),
    pub godot_pool_color_array_new_with_array: unsafe extern "C" fn(r_dest:
                                                                                              *mut godot_pool_color_array,
                                                                                          p_a:
                                                                                              *const godot_array),
    pub godot_pool_color_array_append: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_color_array,
                                                                                  p_data:
                                                                                      *const godot_color),
    pub godot_pool_color_array_append_array: unsafe extern "C" fn(p_self:
                                                                                            *mut godot_pool_color_array,
                                                                                        p_array:
                                                                                            *const godot_pool_color_array),
    pub godot_pool_color_array_insert: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_color_array,
                                                                                  p_idx:
                                                                                      godot_int,
                                                                                  p_data:
                                                                                      *const godot_color)
                                                                 ->
                                                                     godot_error,
    pub godot_pool_color_array_invert: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_color_array),
    pub godot_pool_color_array_push_back: unsafe extern "C" fn(p_self:
                                                                                         *mut godot_pool_color_array,
                                                                                     p_data:
                                                                                         *const godot_color),
    pub godot_pool_color_array_remove: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_color_array,
                                                                                  p_idx:
                                                                                      godot_int),
    pub godot_pool_color_array_resize: unsafe extern "C" fn(p_self:
                                                                                      *mut godot_pool_color_array,
                                                                                  p_size:
                                                                                      godot_int),
    pub godot_pool_color_array_set: unsafe extern "C" fn(p_self:
                                                                                   *mut godot_pool_color_array,
                                                                               p_idx:
                                                                                   godot_int,
                                                                               p_data:
                                                                                   *const godot_color),
    pub godot_pool_color_array_get: unsafe extern "C" fn(p_self:
                                                                                   *const godot_pool_color_array,
                                                                               p_idx:
                                                                                   godot_int)
                                                              -> godot_color,
    pub godot_pool_color_array_size: unsafe extern "C" fn(p_self:
                                                                                    *const godot_pool_color_array)
                                                               -> godot_int,
    pub godot_pool_color_array_destroy: unsafe extern "C" fn(p_self:
                                                                                       *mut godot_pool_color_array),
    pub godot_array_new: unsafe extern "C" fn(r_dest:
                                                                        *mut godot_array),
    pub godot_array_new_copy: unsafe extern "C" fn(r_dest:
                                                                             *mut godot_array,
                                                                         p_src:
                                                                             *const godot_array),
    pub godot_array_new_pool_color_array: unsafe extern "C" fn(r_dest:
                                                                                         *mut godot_array,
                                                                                     p_pca:
                                                                                         *const godot_pool_color_array),
    pub godot_array_new_pool_vector3_array: unsafe extern "C" fn(r_dest:
                                                                                           *mut godot_array,
                                                                                       p_pv3a:
                                                                                           *const godot_pool_vector3_array),
    pub godot_array_new_pool_vector2_array: unsafe extern "C" fn(r_dest:
                                                                                           *mut godot_array,
                                                                                       p_pv2a:
                                                                                           *const godot_pool_vector2_array),
    pub godot_array_new_pool_string_array: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_array,
                                                                                      p_psa:
                                                                                          *const godot_pool_string_array),
    pub godot_array_new_pool_real_array: unsafe extern "C" fn(r_dest:
                                                                                        *mut godot_array,
                                                                                    p_pra:
                                                                                        *const godot_pool_real_array),
    pub godot_array_new_pool_int_array: unsafe extern "C" fn(r_dest:
                                                                                       *mut godot_array,
                                                                                   p_pia:
                                                                                       *const godot_pool_int_array),
    pub godot_array_new_pool_byte_array: unsafe extern "C" fn(r_dest:
                                                                                        *mut godot_array,
                                                                                    p_pba:
                                                                                        *const godot_pool_byte_array),
    pub godot_array_set: unsafe extern "C" fn(p_self:
                                                                        *mut godot_array,
                                                                    p_idx:
                                                                        godot_int,
                                                                    p_value:
                                                                        *const godot_variant),
    pub godot_array_get: unsafe extern "C" fn(p_self:
                                                                        *const godot_array,
                                                                    p_idx:
                                                                        godot_int)
                                                   -> godot_variant,
    pub godot_array_operator_index: unsafe extern "C" fn(p_self:
                                                                                   *mut godot_array,
                                                                               p_idx:
                                                                                   godot_int)
                                                              ->
                                                                  *mut godot_variant,
    pub godot_array_append: unsafe extern "C" fn(p_self:
                                                                           *mut godot_array,
                                                                       p_value:
                                                                           *const godot_variant),
    pub godot_array_clear: unsafe extern "C" fn(p_self:
                                                                          *mut godot_array),
    pub godot_array_count: unsafe extern "C" fn(p_self:
                                                                          *const godot_array,
                                                                      p_value:
                                                                          *const godot_variant)
                                                     -> godot_int,
    pub godot_array_empty: unsafe extern "C" fn(p_self:
                                                                          *const godot_array)
                                                     -> godot_bool,
    pub godot_array_erase: unsafe extern "C" fn(p_self:
                                                                          *mut godot_array,
                                                                      p_value:
                                                                          *const godot_variant),
    pub godot_array_front: unsafe extern "C" fn(p_self:
                                                                          *const godot_array)
                                                     -> godot_variant,
    pub godot_array_back: unsafe extern "C" fn(p_self:
                                                                         *const godot_array)
                                                    -> godot_variant,
    pub godot_array_find: unsafe extern "C" fn(p_self:
                                                                         *const godot_array,
                                                                     p_what:
                                                                         *const godot_variant,
                                                                     p_from:
                                                                         godot_int)
                                                    -> godot_int,
    pub godot_array_find_last: unsafe extern "C" fn(p_self:
                                                                              *const godot_array,
                                                                          p_what:
                                                                              *const godot_variant)
                                                         -> godot_int,
    pub godot_array_has: unsafe extern "C" fn(p_self:
                                                                        *const godot_array,
                                                                    p_value:
                                                                        *const godot_variant)
                                                   -> godot_bool,
    pub godot_array_hash: unsafe extern "C" fn(p_self:
                                                                         *const godot_array)
                                                    -> godot_int,
    pub godot_array_insert: unsafe extern "C" fn(p_self:
                                                                           *mut godot_array,
                                                                       p_pos:
                                                                           godot_int,
                                                                       p_value:
                                                                           *const godot_variant),
    pub godot_array_invert: unsafe extern "C" fn(p_self:
                                                                           *mut godot_array),
    pub godot_array_pop_back: unsafe extern "C" fn(p_self:
                                                                             *mut godot_array)
                                                        -> godot_variant,
    pub godot_array_pop_front: unsafe extern "C" fn(p_self:
                                                                              *mut godot_array)
                                                         -> godot_variant,
    pub godot_array_push_back: unsafe extern "C" fn(p_self:
                                                                              *mut godot_array,
                                                                          p_value:
                                                                              *const godot_variant),
    pub godot_array_push_front: unsafe extern "C" fn(p_self:
                                                                               *mut godot_array,
                                                                           p_value:
                                                                               *const godot_variant),
    pub godot_array_remove: unsafe extern "C" fn(p_self:
                                                                           *mut godot_array,
                                                                       p_idx:
                                                                           godot_int),
    pub godot_array_resize: unsafe extern "C" fn(p_self:
                                                                           *mut godot_array,
                                                                       p_size:
                                                                           godot_int),
    pub godot_array_rfind: unsafe extern "C" fn(p_self:
                                                                          *const godot_array,
                                                                      p_what:
                                                                          *const godot_variant,
                                                                      p_from:
                                                                          godot_int)
                                                     -> godot_int,
    pub godot_array_size: unsafe extern "C" fn(p_self:
                                                                         *const godot_array)
                                                    -> godot_int,
    pub godot_array_sort: unsafe extern "C" fn(p_self:
                                                                         *mut godot_array),
    pub godot_array_sort_custom: unsafe extern "C" fn(p_self:
                                                                                *mut godot_array,
                                                                            p_obj:
                                                                                *mut godot_object,
                                                                            p_func:
                                                                                *const godot_string),
    pub godot_array_destroy: unsafe extern "C" fn(p_self:
                                                                            *mut godot_array),
    pub godot_dictionary_new: unsafe extern "C" fn(r_dest:
                                                                             *mut godot_dictionary),
    pub godot_dictionary_new_copy: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_dictionary,
                                                                              p_src:
                                                                                  *const godot_dictionary),
    pub godot_dictionary_destroy: unsafe extern "C" fn(p_self:
                                                                                 *mut godot_dictionary),
    pub godot_dictionary_size: unsafe extern "C" fn(p_self:
                                                                              *const godot_dictionary)
                                                         -> godot_int,
    pub godot_dictionary_empty: unsafe extern "C" fn(p_self:
                                                                               *const godot_dictionary)
                                                          -> godot_bool,
    pub godot_dictionary_clear: unsafe extern "C" fn(p_self:
                                                                               *mut godot_dictionary),
    pub godot_dictionary_has: unsafe extern "C" fn(p_self:
                                                                             *const godot_dictionary,
                                                                         p_key:
                                                                             *const godot_variant)
                                                        -> godot_bool,
    pub godot_dictionary_has_all: unsafe extern "C" fn(p_self:
                                                                                 *const godot_dictionary,
                                                                             p_keys:
                                                                                 *const godot_array)
                                                            -> godot_bool,
    pub godot_dictionary_erase: unsafe extern "C" fn(p_self:
                                                                               *mut godot_dictionary,
                                                                           p_key:
                                                                               *const godot_variant),
    pub godot_dictionary_hash: unsafe extern "C" fn(p_self:
                                                                              *const godot_dictionary)
                                                         -> godot_int,
    pub godot_dictionary_keys: unsafe extern "C" fn(p_self:
                                                                              *const godot_dictionary)
                                                         -> godot_array,
    pub godot_dictionary_values: unsafe extern "C" fn(p_self:
                                                                                *const godot_dictionary)
                                                           -> godot_array,
    pub godot_dictionary_get: unsafe extern "C" fn(p_self:
                                                                             *const godot_dictionary,
                                                                         p_key:
                                                                             *const godot_variant)
                                                        -> godot_variant,
    pub godot_dictionary_set: unsafe extern "C" fn(p_self:
                                                                             *mut godot_dictionary,
                                                                         p_key:
                                                                             *const godot_variant,
                                                                         p_value:
                                                                             *const godot_variant),
    pub godot_dictionary_operator_index: unsafe extern "C" fn(p_self:
                                                                                        *mut godot_dictionary,
                                                                                    p_key:
                                                                                        *const godot_variant)
                                                                   ->
                                                                       *mut godot_variant,
    pub godot_dictionary_next: unsafe extern "C" fn(p_self:
                                                                              *const godot_dictionary,
                                                                          p_key:
                                                                              *const godot_variant)
                                                         ->
                                                             *mut godot_variant,
    pub godot_dictionary_operator_equal: unsafe extern "C" fn(p_self:
                                                                                        *const godot_dictionary,
                                                                                    p_b:
                                                                                        *const godot_dictionary)
                                                                   ->
                                                                       godot_bool,
    pub godot_dictionary_to_json: unsafe extern "C" fn(p_self:
                                                                                 *const godot_dictionary)
                                                            -> godot_string,
    pub godot_node_path_new: unsafe extern "C" fn(r_dest:
                                                                            *mut godot_node_path,
                                                                        p_from:
                                                                            *const godot_string),
    pub godot_node_path_new_copy: unsafe extern "C" fn(r_dest:
                                                                                 *mut godot_node_path,
                                                                             p_src:
                                                                                 *const godot_node_path),
    pub godot_node_path_destroy: unsafe extern "C" fn(p_self:
                                                                                *mut godot_node_path),
    pub godot_node_path_as_string: unsafe extern "C" fn(p_self:
                                                                                  *const godot_node_path)
                                                             -> godot_string,
    pub godot_node_path_is_absolute: unsafe extern "C" fn(p_self:
                                                                                    *const godot_node_path)
                                                               -> godot_bool,
    pub godot_node_path_get_name_count: unsafe extern "C" fn(p_self:
                                                                                       *const godot_node_path)
                                                                  ->
                                                                      godot_int,
    pub godot_node_path_get_name: unsafe extern "C" fn(p_self:
                                                                                 *const godot_node_path,
                                                                             p_idx:
                                                                                 godot_int)
                                                            -> godot_string,
    pub godot_node_path_get_subname_count: unsafe extern "C" fn(p_self:
                                                                                          *const godot_node_path)
                                                                     ->
                                                                         godot_int,
    pub godot_node_path_get_subname: unsafe extern "C" fn(p_self:
                                                                                    *const godot_node_path,
                                                                                p_idx:
                                                                                    godot_int)
                                                               ->
                                                                   godot_string,
    pub godot_node_path_get_property: unsafe extern "C" fn(p_self:
                                                                                     *const godot_node_path)
                                                                ->
                                                                    godot_string,
    pub godot_node_path_is_empty: unsafe extern "C" fn(p_self:
                                                                                 *const godot_node_path)
                                                            -> godot_bool,
    pub godot_node_path_operator_equal: unsafe extern "C" fn(p_self:
                                                                                       *const godot_node_path,
                                                                                   p_b:
                                                                                       *const godot_node_path)
                                                                  ->
                                                                      godot_bool,
    pub godot_plane_new_with_reals: unsafe extern "C" fn(r_dest:
                                                                                   *mut godot_plane,
                                                                               p_a:
                                                                                   godot_real,
                                                                               p_b:
                                                                                   godot_real,
                                                                               p_c:
                                                                                   godot_real,
                                                                               p_d:
                                                                                   godot_real),
    pub godot_plane_new_with_vectors: unsafe extern "C" fn(r_dest:
                                                                                     *mut godot_plane,
                                                                                 p_v1:
                                                                                     *const godot_vector3,
                                                                                 p_v2:
                                                                                     *const godot_vector3,
                                                                                 p_v3:
                                                                                     *const godot_vector3),
    pub godot_plane_new_with_normal: unsafe extern "C" fn(r_dest:
                                                                                    *mut godot_plane,
                                                                                p_normal:
                                                                                    *const godot_vector3,
                                                                                p_d:
                                                                                    godot_real),
    pub godot_plane_as_string: unsafe extern "C" fn(p_self:
                                                                              *const godot_plane)
                                                         -> godot_string,
    pub godot_plane_normalized: unsafe extern "C" fn(p_self:
                                                                               *const godot_plane)
                                                          -> godot_plane,
    pub godot_plane_center: unsafe extern "C" fn(p_self:
                                                                           *const godot_plane)
                                                      -> godot_vector3,
    pub godot_plane_get_any_point: unsafe extern "C" fn(p_self:
                                                                                  *const godot_plane)
                                                             ->
                                                                 godot_vector3,
    pub godot_plane_is_point_over: unsafe extern "C" fn(p_self:
                                                                                  *const godot_plane,
                                                                              p_point:
                                                                                  *const godot_vector3)
                                                             -> godot_bool,
    pub godot_plane_distance_to: unsafe extern "C" fn(p_self:
                                                                                *const godot_plane,
                                                                            p_point:
                                                                                *const godot_vector3)
                                                           -> godot_real,
    pub godot_plane_has_point: unsafe extern "C" fn(p_self:
                                                                              *const godot_plane,
                                                                          p_point:
                                                                              *const godot_vector3,
                                                                          p_epsilon:
                                                                              godot_real)
                                                         -> godot_bool,
    pub godot_plane_project: unsafe extern "C" fn(p_self:
                                                                            *const godot_plane,
                                                                        p_point:
                                                                            *const godot_vector3)
                                                       -> godot_vector3,
    pub godot_plane_intersect_3: unsafe extern "C" fn(p_self:
                                                                                *const godot_plane,
                                                                            r_dest:
                                                                                *mut godot_vector3,
                                                                            p_b:
                                                                                *const godot_plane,
                                                                            p_c:
                                                                                *const godot_plane)
                                                           -> godot_bool,
    pub godot_plane_intersects_ray: unsafe extern "C" fn(p_self:
                                                                                   *const godot_plane,
                                                                               r_dest:
                                                                                   *mut godot_vector3,
                                                                               p_from:
                                                                                   *const godot_vector3,
                                                                               p_dir:
                                                                                   *const godot_vector3)
                                                              -> godot_bool,
    pub godot_plane_intersects_segment: unsafe extern "C" fn(p_self:
                                                                                       *const godot_plane,
                                                                                   r_dest:
                                                                                       *mut godot_vector3,
                                                                                   p_begin:
                                                                                       *const godot_vector3,
                                                                                   p_end:
                                                                                       *const godot_vector3)
                                                                  ->
                                                                      godot_bool,
    pub godot_plane_operator_neg: unsafe extern "C" fn(p_self:
                                                                                 *const godot_plane)
                                                            -> godot_plane,
    pub godot_plane_operator_equal: unsafe extern "C" fn(p_self:
                                                                                   *const godot_plane,
                                                                               p_b:
                                                                                   *const godot_plane)
                                                              -> godot_bool,
    pub godot_plane_set_normal: unsafe extern "C" fn(p_self:
                                                                               *mut godot_plane,
                                                                           p_normal:
                                                                               *const godot_vector3),
    pub godot_plane_get_normal: unsafe extern "C" fn(p_self:
                                                                               *const godot_plane)
                                                          -> godot_vector3,
    pub godot_plane_get_d: unsafe extern "C" fn(p_self:
                                                                          *const godot_plane)
                                                     -> godot_real,
    pub godot_plane_set_d: unsafe extern "C" fn(p_self:
                                                                          *mut godot_plane,
                                                                      p_d:
                                                                          godot_real),
    pub godot_rect2_new_with_position_and_size: unsafe extern "C" fn(r_dest:
                                                                                               *mut godot_rect2,
                                                                                           p_pos:
                                                                                               *const godot_vector2,
                                                                                           p_size:
                                                                                               *const godot_vector2),
    pub godot_rect2_new: unsafe extern "C" fn(r_dest:
                                                                        *mut godot_rect2,
                                                                    p_x:
                                                                        godot_real,
                                                                    p_y:
                                                                        godot_real,
                                                                    p_width:
                                                                        godot_real,
                                                                    p_height:
                                                                        godot_real),
    pub godot_rect2_as_string: unsafe extern "C" fn(p_self:
                                                                              *const godot_rect2)
                                                         -> godot_string,
    pub godot_rect2_get_area: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect2)
                                                        -> godot_real,
    pub godot_rect2_intersects: unsafe extern "C" fn(p_self:
                                                                               *const godot_rect2,
                                                                           p_b:
                                                                               *const godot_rect2)
                                                          -> godot_bool,
    pub godot_rect2_encloses: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect2,
                                                                         p_b:
                                                                             *const godot_rect2)
                                                        -> godot_bool,
    pub godot_rect2_has_no_area: unsafe extern "C" fn(p_self:
                                                                                *const godot_rect2)
                                                           -> godot_bool,
    pub godot_rect2_clip: unsafe extern "C" fn(p_self:
                                                                         *const godot_rect2,
                                                                     p_b:
                                                                         *const godot_rect2)
                                                    -> godot_rect2,
    pub godot_rect2_merge: unsafe extern "C" fn(p_self:
                                                                          *const godot_rect2,
                                                                      p_b:
                                                                          *const godot_rect2)
                                                     -> godot_rect2,
    pub godot_rect2_has_point: unsafe extern "C" fn(p_self:
                                                                              *const godot_rect2,
                                                                          p_point:
                                                                              *const godot_vector2)
                                                         -> godot_bool,
    pub godot_rect2_grow: unsafe extern "C" fn(p_self:
                                                                         *const godot_rect2,
                                                                     p_by:
                                                                         godot_real)
                                                    -> godot_rect2,
    pub godot_rect2_expand: unsafe extern "C" fn(p_self:
                                                                           *const godot_rect2,
                                                                       p_to:
                                                                           *const godot_vector2)
                                                      -> godot_rect2,
    pub godot_rect2_operator_equal: unsafe extern "C" fn(p_self:
                                                                                   *const godot_rect2,
                                                                               p_b:
                                                                                   *const godot_rect2)
                                                              -> godot_bool,
    pub godot_rect2_get_position: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rect2)
                                                            -> godot_vector2,
    pub godot_rect2_get_size: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect2)
                                                        -> godot_vector2,
    pub godot_rect2_set_position: unsafe extern "C" fn(p_self:
                                                                                 *mut godot_rect2,
                                                                             p_pos:
                                                                                 *const godot_vector2),
    pub godot_rect2_set_size: unsafe extern "C" fn(p_self:
                                                                             *mut godot_rect2,
                                                                         p_size:
                                                                             *const godot_vector2),
    pub godot_rect3_new: unsafe extern "C" fn(r_dest:
                                                                        *mut godot_rect3,
                                                                    p_pos:
                                                                        *const godot_vector3,
                                                                    p_size:
                                                                        *const godot_vector3),
    pub godot_rect3_get_position: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rect3)
                                                            -> godot_vector3,
    pub godot_rect3_set_position: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rect3,
                                                                             p_v:
                                                                                 *const godot_vector3),
    pub godot_rect3_get_size: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect3)
                                                        -> godot_vector3,
    pub godot_rect3_set_size: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect3,
                                                                         p_v:
                                                                             *const godot_vector3),
    pub godot_rect3_as_string: unsafe extern "C" fn(p_self:
                                                                              *const godot_rect3)
                                                         -> godot_string,
    pub godot_rect3_get_area: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect3)
                                                        -> godot_real,
    pub godot_rect3_has_no_area: unsafe extern "C" fn(p_self:
                                                                                *const godot_rect3)
                                                           -> godot_bool,
    pub godot_rect3_has_no_surface: unsafe extern "C" fn(p_self:
                                                                                   *const godot_rect3)
                                                              -> godot_bool,
    pub godot_rect3_intersects: unsafe extern "C" fn(p_self:
                                                                               *const godot_rect3,
                                                                           p_with:
                                                                               *const godot_rect3)
                                                          -> godot_bool,
    pub godot_rect3_encloses: unsafe extern "C" fn(p_self:
                                                                             *const godot_rect3,
                                                                         p_with:
                                                                             *const godot_rect3)
                                                        -> godot_bool,
    pub godot_rect3_merge: unsafe extern "C" fn(p_self:
                                                                          *const godot_rect3,
                                                                      p_with:
                                                                          *const godot_rect3)
                                                     -> godot_rect3,
    pub godot_rect3_intersection: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rect3,
                                                                             p_with:
                                                                                 *const godot_rect3)
                                                            -> godot_rect3,
    pub godot_rect3_intersects_plane: unsafe extern "C" fn(p_self:
                                                                                     *const godot_rect3,
                                                                                 p_plane:
                                                                                     *const godot_plane)
                                                                ->
                                                                    godot_bool,
    pub godot_rect3_intersects_segment: unsafe extern "C" fn(p_self:
                                                                                       *const godot_rect3,
                                                                                   p_from:
                                                                                       *const godot_vector3,
                                                                                   p_to:
                                                                                       *const godot_vector3)
                                                                  ->
                                                                      godot_bool,
    pub godot_rect3_has_point: unsafe extern "C" fn(p_self:
                                                                              *const godot_rect3,
                                                                          p_point:
                                                                              *const godot_vector3)
                                                         -> godot_bool,
    pub godot_rect3_get_support: unsafe extern "C" fn(p_self:
                                                                                *const godot_rect3,
                                                                            p_dir:
                                                                                *const godot_vector3)
                                                           -> godot_vector3,
    pub godot_rect3_get_longest_axis: unsafe extern "C" fn(p_self:
                                                                                     *const godot_rect3)
                                                                ->
                                                                    godot_vector3,
    pub godot_rect3_get_longest_axis_index: unsafe extern "C" fn(p_self:
                                                                                           *const godot_rect3)
                                                                      ->
                                                                          godot_int,
    pub godot_rect3_get_longest_axis_size: unsafe extern "C" fn(p_self:
                                                                                          *const godot_rect3)
                                                                     ->
                                                                         godot_real,
    pub godot_rect3_get_shortest_axis: unsafe extern "C" fn(p_self:
                                                                                      *const godot_rect3)
                                                                 ->
                                                                     godot_vector3,
    pub godot_rect3_get_shortest_axis_index: unsafe extern "C" fn(p_self:
                                                                                            *const godot_rect3)
                                                                       ->
                                                                           godot_int,
    pub godot_rect3_get_shortest_axis_size: unsafe extern "C" fn(p_self:
                                                                                           *const godot_rect3)
                                                                      ->
                                                                          godot_real,
    pub godot_rect3_expand: unsafe extern "C" fn(p_self:
                                                                           *const godot_rect3,
                                                                       p_to_point:
                                                                           *const godot_vector3)
                                                      -> godot_rect3,
    pub godot_rect3_grow: unsafe extern "C" fn(p_self:
                                                                         *const godot_rect3,
                                                                     p_by:
                                                                         godot_real)
                                                    -> godot_rect3,
    pub godot_rect3_get_endpoint: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rect3,
                                                                             p_idx:
                                                                                 godot_int)
                                                            -> godot_vector3,
    pub godot_rect3_operator_equal: unsafe extern "C" fn(p_self:
                                                                                   *const godot_rect3,
                                                                               p_b:
                                                                                   *const godot_rect3)
                                                              -> godot_bool,
    pub godot_rid_new: unsafe extern "C" fn(r_dest:
                                                                      *mut godot_rid),
    pub godot_rid_get_id: unsafe extern "C" fn(p_self:
                                                                         *const godot_rid)
                                                    -> godot_int,
    pub godot_rid_new_with_resource: unsafe extern "C" fn(r_dest:
                                                                                    *mut godot_rid,
                                                                                p_from:
                                                                                    *const godot_object),
    pub godot_rid_operator_equal: unsafe extern "C" fn(p_self:
                                                                                 *const godot_rid,
                                                                             p_b:
                                                                                 *const godot_rid)
                                                            -> godot_bool,
    pub godot_rid_operator_less: unsafe extern "C" fn(p_self:
                                                                                *const godot_rid,
                                                                            p_b:
                                                                                *const godot_rid)
                                                           -> godot_bool,
    pub godot_transform_new_with_axis_origin: unsafe extern "C" fn(r_dest:
                                                                                             *mut godot_transform,
                                                                                         p_x_axis:
                                                                                             *const godot_vector3,
                                                                                         p_y_axis:
                                                                                             *const godot_vector3,
                                                                                         p_z_axis:
                                                                                             *const godot_vector3,
                                                                                         p_origin:
                                                                                             *const godot_vector3),
    pub godot_transform_new: unsafe extern "C" fn(r_dest:
                                                                            *mut godot_transform,
                                                                        p_basis:
                                                                            *const godot_basis,
                                                                        p_origin:
                                                                            *const godot_vector3),
    pub godot_transform_get_basis: unsafe extern "C" fn(p_self:
                                                                                  *const godot_transform)
                                                             -> godot_basis,
    pub godot_transform_set_basis: unsafe extern "C" fn(p_self:
                                                                                  *mut godot_transform,
                                                                              p_v:
                                                                                  *mut godot_basis),
    pub godot_transform_get_origin: unsafe extern "C" fn(p_self:
                                                                                   *const godot_transform)
                                                              ->
                                                                  godot_vector3,
    pub godot_transform_set_origin: unsafe extern "C" fn(p_self:
                                                                                   *mut godot_transform,
                                                                               p_v:
                                                                                   *mut godot_vector3),
    pub godot_transform_as_string: unsafe extern "C" fn(p_self:
                                                                                  *const godot_transform)
                                                             -> godot_string,
    pub godot_transform_inverse: unsafe extern "C" fn(p_self:
                                                                                *const godot_transform)
                                                           ->
                                                               godot_transform,
    pub godot_transform_affine_inverse: unsafe extern "C" fn(p_self:
                                                                                       *const godot_transform)
                                                                  ->
                                                                      godot_transform,
    pub godot_transform_orthonormalized: unsafe extern "C" fn(p_self:
                                                                                        *const godot_transform)
                                                                   ->
                                                                       godot_transform,
    pub godot_transform_rotated: unsafe extern "C" fn(p_self:
                                                                                *const godot_transform,
                                                                            p_axis:
                                                                                *const godot_vector3,
                                                                            p_phi:
                                                                                godot_real)
                                                           ->
                                                               godot_transform,
    pub godot_transform_scaled: unsafe extern "C" fn(p_self:
                                                                               *const godot_transform,
                                                                           p_scale:
                                                                               *const godot_vector3)
                                                          -> godot_transform,
    pub godot_transform_translated: unsafe extern "C" fn(p_self:
                                                                                   *const godot_transform,
                                                                               p_ofs:
                                                                                   *const godot_vector3)
                                                              ->
                                                                  godot_transform,
    pub godot_transform_looking_at: unsafe extern "C" fn(p_self:
                                                                                   *const godot_transform,
                                                                               p_target:
                                                                                   *const godot_vector3,
                                                                               p_up:
                                                                                   *const godot_vector3)
                                                              ->
                                                                  godot_transform,
    pub godot_transform_xform_plane: unsafe extern "C" fn(p_self:
                                                                                    *const godot_transform,
                                                                                p_v:
                                                                                    *const godot_plane)
                                                               ->
                                                                   godot_plane,
    pub godot_transform_xform_inv_plane: unsafe extern "C" fn(p_self:
                                                                                        *const godot_transform,
                                                                                    p_v:
                                                                                        *const godot_plane)
                                                                   ->
                                                                       godot_plane,
    pub godot_transform_new_identity: unsafe extern "C" fn(r_dest:
                                                                                     *mut godot_transform),
    pub godot_transform_operator_equal: unsafe extern "C" fn(p_self:
                                                                                       *const godot_transform,
                                                                                   p_b:
                                                                                       *const godot_transform)
                                                                  ->
                                                                      godot_bool,
    pub godot_transform_operator_multiply: unsafe extern "C" fn(p_self:
                                                                                          *const godot_transform,
                                                                                      p_b:
                                                                                          *const godot_transform)
                                                                     ->
                                                                         godot_transform,
    pub godot_transform_xform_vector3: unsafe extern "C" fn(p_self:
                                                                                      *const godot_transform,
                                                                                  p_v:
                                                                                      *const godot_vector3)
                                                                 ->
                                                                     godot_vector3,
    pub godot_transform_xform_inv_vector3: unsafe extern "C" fn(p_self:
                                                                                          *const godot_transform,
                                                                                      p_v:
                                                                                          *const godot_vector3)
                                                                     ->
                                                                         godot_vector3,
    pub godot_transform_xform_rect3: unsafe extern "C" fn(p_self:
                                                                                    *const godot_transform,
                                                                                p_v:
                                                                                    *const godot_rect3)
                                                               ->
                                                                   godot_rect3,
    pub godot_transform_xform_inv_rect3: unsafe extern "C" fn(p_self:
                                                                                        *const godot_transform,
                                                                                    p_v:
                                                                                        *const godot_rect3)
                                                                   ->
                                                                       godot_rect3,
    pub godot_transform2d_new: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_transform2d,
                                                                          p_rot:
                                                                              godot_real,
                                                                          p_pos:
                                                                              *const godot_vector2),
    pub godot_transform2d_new_axis_origin: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_transform2d,
                                                                                      p_x_axis:
                                                                                          *const godot_vector2,
                                                                                      p_y_axis:
                                                                                          *const godot_vector2,
                                                                                      p_origin:
                                                                                          *const godot_vector2),
    pub godot_transform2d_as_string: unsafe extern "C" fn(p_self:
                                                                                    *const godot_transform2d)
                                                               ->
                                                                   godot_string,
    pub godot_transform2d_inverse: unsafe extern "C" fn(p_self:
                                                                                  *const godot_transform2d)
                                                             ->
                                                                 godot_transform2d,
    pub godot_transform2d_affine_inverse: unsafe extern "C" fn(p_self:
                                                                                         *const godot_transform2d)
                                                                    ->
                                                                        godot_transform2d,
    pub godot_transform2d_get_rotation: unsafe extern "C" fn(p_self:
                                                                                       *const godot_transform2d)
                                                                  ->
                                                                      godot_real,
    pub godot_transform2d_get_origin: unsafe extern "C" fn(p_self:
                                                                                     *const godot_transform2d)
                                                                ->
                                                                    godot_vector2,
    pub godot_transform2d_get_scale: unsafe extern "C" fn(p_self:
                                                                                    *const godot_transform2d)
                                                               ->
                                                                   godot_vector2,
    pub godot_transform2d_orthonormalized: unsafe extern "C" fn(p_self:
                                                                                          *const godot_transform2d)
                                                                     ->
                                                                         godot_transform2d,
    pub godot_transform2d_rotated: unsafe extern "C" fn(p_self:
                                                                                  *const godot_transform2d,
                                                                              p_phi:
                                                                                  godot_real)
                                                             ->
                                                                 godot_transform2d,
    pub godot_transform2d_scaled: unsafe extern "C" fn(p_self:
                                                                                 *const godot_transform2d,
                                                                             p_scale:
                                                                                 *const godot_vector2)
                                                            ->
                                                                godot_transform2d,
    pub godot_transform2d_translated: unsafe extern "C" fn(p_self:
                                                                                     *const godot_transform2d,
                                                                                 p_offset:
                                                                                     *const godot_vector2)
                                                                ->
                                                                    godot_transform2d,
    pub godot_transform2d_xform_vector2: unsafe extern "C" fn(p_self:
                                                                                        *const godot_transform2d,
                                                                                    p_v:
                                                                                        *const godot_vector2)
                                                                   ->
                                                                       godot_vector2,
    pub godot_transform2d_xform_inv_vector2: unsafe extern "C" fn(p_self:
                                                                                            *const godot_transform2d,
                                                                                        p_v:
                                                                                            *const godot_vector2)
                                                                       ->
                                                                           godot_vector2,
    pub godot_transform2d_basis_xform_vector2: unsafe extern "C" fn(p_self:
                                                                                              *const godot_transform2d,
                                                                                          p_v:
                                                                                              *const godot_vector2)
                                                                         ->
                                                                             godot_vector2,
    pub godot_transform2d_basis_xform_inv_vector2: unsafe extern "C" fn(p_self:
                                                                                                  *const godot_transform2d,
                                                                                              p_v:
                                                                                                  *const godot_vector2)
                                                                             ->
                                                                                 godot_vector2,
    pub godot_transform2d_interpolate_with: unsafe extern "C" fn(p_self:
                                                                                           *const godot_transform2d,
                                                                                       p_m:
                                                                                           *const godot_transform2d,
                                                                                       p_c:
                                                                                           godot_real)
                                                                      ->
                                                                          godot_transform2d,
    pub godot_transform2d_operator_equal: unsafe extern "C" fn(p_self:
                                                                                         *const godot_transform2d,
                                                                                     p_b:
                                                                                         *const godot_transform2d)
                                                                    ->
                                                                        godot_bool,
    pub godot_transform2d_operator_multiply: unsafe extern "C" fn(p_self:
                                                                                            *const godot_transform2d,
                                                                                        p_b:
                                                                                            *const godot_transform2d)
                                                                       ->
                                                                           godot_transform2d,
    pub godot_transform2d_new_identity: unsafe extern "C" fn(r_dest:
                                                                                       *mut godot_transform2d),
    pub godot_transform2d_xform_rect2: unsafe extern "C" fn(p_self:
                                                                                      *const godot_transform2d,
                                                                                  p_v:
                                                                                      *const godot_rect2)
                                                                 ->
                                                                     godot_rect2,
    pub godot_transform2d_xform_inv_rect2: unsafe extern "C" fn(p_self:
                                                                                          *const godot_transform2d,
                                                                                      p_v:
                                                                                          *const godot_rect2)
                                                                     ->
                                                                         godot_rect2,
    pub godot_variant_get_type: unsafe extern "C" fn(p_v:
                                                                               *const godot_variant)
                                                          ->
                                                              godot_variant_type,
    pub godot_variant_new_copy: unsafe extern "C" fn(r_dest:
                                                                               *mut godot_variant,
                                                                           p_src:
                                                                               *const godot_variant),
    pub godot_variant_new_nil: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_variant),
    pub godot_variant_new_bool: unsafe extern "C" fn(p_v:
                                                                               *mut godot_variant,
                                                                           p_b:
                                                                               godot_bool),
    pub godot_variant_new_uint: unsafe extern "C" fn(r_dest:
                                                                               *mut godot_variant,
                                                                           p_i:
                                                                               u64),
    pub godot_variant_new_int: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_variant,
                                                                          p_i:
                                                                              i64),
    pub godot_variant_new_real: unsafe extern "C" fn(r_dest:
                                                                               *mut godot_variant,
                                                                           p_r:
                                                                               f64),
    pub godot_variant_new_string: unsafe extern "C" fn(r_dest:
                                                                                 *mut godot_variant,
                                                                             p_s:
                                                                                 *const godot_string),
    pub godot_variant_new_vector2: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_variant,
                                                                              p_v2:
                                                                                  *const godot_vector2),
    pub godot_variant_new_rect2: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_rect2:
                                                                                *const godot_rect2),
    pub godot_variant_new_vector3: unsafe extern "C" fn(r_dest:
                                                                                  *mut godot_variant,
                                                                              p_v3:
                                                                                  *const godot_vector3),
    pub godot_variant_new_transform2d: unsafe extern "C" fn(r_dest:
                                                                                      *mut godot_variant,
                                                                                  p_t2d:
                                                                                      *const godot_transform2d),
    pub godot_variant_new_plane: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_plane:
                                                                                *const godot_plane),
    pub godot_variant_new_quat: unsafe extern "C" fn(r_dest:
                                                                               *mut godot_variant,
                                                                           p_quat:
                                                                               *const godot_quat),
    pub godot_variant_new_rect3: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_rect3:
                                                                                *const godot_rect3),
    pub godot_variant_new_basis: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_basis:
                                                                                *const godot_basis),
    pub godot_variant_new_transform: unsafe extern "C" fn(r_dest:
                                                                                    *mut godot_variant,
                                                                                p_trans:
                                                                                    *const godot_transform),
    pub godot_variant_new_color: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_color:
                                                                                *const godot_color),
    pub godot_variant_new_node_path: unsafe extern "C" fn(r_dest:
                                                                                    *mut godot_variant,
                                                                                p_np:
                                                                                    *const godot_node_path),
    pub godot_variant_new_rid: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_variant,
                                                                          p_rid:
                                                                              *const godot_rid),
    pub godot_variant_new_object: unsafe extern "C" fn(r_dest:
                                                                                 *mut godot_variant,
                                                                             p_obj:
                                                                                 *const godot_object),
    pub godot_variant_new_dictionary: unsafe extern "C" fn(r_dest:
                                                                                     *mut godot_variant,
                                                                                 p_dict:
                                                                                     *const godot_dictionary),
    pub godot_variant_new_array: unsafe extern "C" fn(r_dest:
                                                                                *mut godot_variant,
                                                                            p_arr:
                                                                                *const godot_array),
    pub godot_variant_new_pool_byte_array: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_variant,
                                                                                      p_pba:
                                                                                          *const godot_pool_byte_array),
    pub godot_variant_new_pool_int_array: unsafe extern "C" fn(r_dest:
                                                                                         *mut godot_variant,
                                                                                     p_pia:
                                                                                         *const godot_pool_int_array),
    pub godot_variant_new_pool_real_array: unsafe extern "C" fn(r_dest:
                                                                                          *mut godot_variant,
                                                                                      p_pra:
                                                                                          *const godot_pool_real_array),
    pub godot_variant_new_pool_string_array: unsafe extern "C" fn(r_dest:
                                                                                            *mut godot_variant,
                                                                                        p_psa:
                                                                                            *const godot_pool_string_array),
    pub godot_variant_new_pool_vector2_array: unsafe extern "C" fn(r_dest:
                                                                                             *mut godot_variant,
                                                                                         p_pv2a:
                                                                                             *const godot_pool_vector2_array),
    pub godot_variant_new_pool_vector3_array: unsafe extern "C" fn(r_dest:
                                                                                             *mut godot_variant,
                                                                                         p_pv3a:
                                                                                             *const godot_pool_vector3_array),
    pub godot_variant_new_pool_color_array: unsafe extern "C" fn(r_dest:
                                                                                           *mut godot_variant,
                                                                                       p_pca:
                                                                                           *const godot_pool_color_array),
    pub godot_variant_as_bool: unsafe extern "C" fn(p_self:
                                                                              *const godot_variant)
                                                         -> godot_bool,
    pub godot_variant_as_uint: unsafe extern "C" fn(p_self:
                                                                              *const godot_variant)
                                                         -> u64,
    pub godot_variant_as_int: unsafe extern "C" fn(p_self:
                                                                             *const godot_variant)
                                                        -> i64,
    pub godot_variant_as_real: unsafe extern "C" fn(p_self:
                                                                              *const godot_variant)
                                                         -> f64,
    pub godot_variant_as_string: unsafe extern "C" fn(p_self:
                                                                                *const godot_variant)
                                                           -> godot_string,
    pub godot_variant_as_vector2: unsafe extern "C" fn(p_self:
                                                                                 *const godot_variant)
                                                            -> godot_vector2,
    pub godot_variant_as_rect2: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_rect2,
    pub godot_variant_as_vector3: unsafe extern "C" fn(p_self:
                                                                                 *const godot_variant)
                                                            -> godot_vector3,
    pub godot_variant_as_transform2d: unsafe extern "C" fn(p_self:
                                                                                     *const godot_variant)
                                                                ->
                                                                    godot_transform2d,
    pub godot_variant_as_plane: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_plane,
    pub godot_variant_as_quat: unsafe extern "C" fn(p_self:
                                                                              *const godot_variant)
                                                         -> godot_quat,
    pub godot_variant_as_rect3: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_rect3,
    pub godot_variant_as_basis: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_basis,
    pub godot_variant_as_transform: unsafe extern "C" fn(p_self:
                                                                                   *const godot_variant)
                                                              ->
                                                                  godot_transform,
    pub godot_variant_as_color: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_color,
    pub godot_variant_as_node_path: unsafe extern "C" fn(p_self:
                                                                                   *const godot_variant)
                                                              ->
                                                                  godot_node_path,
    pub godot_variant_as_rid: unsafe extern "C" fn(p_self:
                                                                             *const godot_variant)
                                                        -> godot_rid,
    pub godot_variant_as_object: unsafe extern "C" fn(p_self:
                                                                                *const godot_variant)
                                                           ->
                                                               *mut godot_object,
    pub godot_variant_as_dictionary: unsafe extern "C" fn(p_self:
                                                                                    *const godot_variant)
                                                               ->
                                                                   godot_dictionary,
    pub godot_variant_as_array: unsafe extern "C" fn(p_self:
                                                                               *const godot_variant)
                                                          -> godot_array,
    pub godot_variant_as_pool_byte_array: unsafe extern "C" fn(p_self:
                                                                                         *const godot_variant)
                                                                    ->
                                                                        godot_pool_byte_array,
    pub godot_variant_as_pool_int_array: unsafe extern "C" fn(p_self:
                                                                                        *const godot_variant)
                                                                   ->
                                                                       godot_pool_int_array,
    pub godot_variant_as_pool_real_array: unsafe extern "C" fn(p_self:
                                                                                         *const godot_variant)
                                                                    ->
                                                                        godot_pool_real_array,
    pub godot_variant_as_pool_string_array: unsafe extern "C" fn(p_self:
                                                                                           *const godot_variant)
                                                                      ->
                                                                          godot_pool_string_array,
    pub godot_variant_as_pool_vector2_array: unsafe extern "C" fn(p_self:
                                                                                            *const godot_variant)
                                                                       ->
                                                                           godot_pool_vector2_array,
    pub godot_variant_as_pool_vector3_array: unsafe extern "C" fn(p_self:
                                                                                            *const godot_variant)
                                                                       ->
                                                                           godot_pool_vector3_array,
    pub godot_variant_as_pool_color_array: unsafe extern "C" fn(p_self:
                                                                                          *const godot_variant)
                                                                     ->
                                                                         godot_pool_color_array,
    pub godot_variant_call: unsafe extern "C" fn(p_self:
                                                                           *mut godot_variant,
                                                                       p_method:
                                                                           *const godot_string,
                                                                       p_args:
                                                                           *mut *const godot_variant,
                                                                       p_argcount:
                                                                           godot_int,
                                                                       r_error:
                                                                           *mut godot_variant_call_error)
                                                      -> godot_variant,
    pub godot_variant_has_method: unsafe extern "C" fn(p_self:
                                                                                 *const godot_variant,
                                                                             p_method:
                                                                                 *const godot_string)
                                                            -> godot_bool,
    pub godot_variant_operator_equal: unsafe extern "C" fn(p_self:
                                                                                     *const godot_variant,
                                                                                 p_other:
                                                                                     *const godot_variant)
                                                                ->
                                                                    godot_bool,
    pub godot_variant_operator_less: unsafe extern "C" fn(p_self:
                                                                                    *const godot_variant,
                                                                                p_other:
                                                                                    *const godot_variant)
                                                               -> godot_bool,
    pub godot_variant_hash_compare: unsafe extern "C" fn(p_self:
                                                                                   *const godot_variant,
                                                                               p_other:
                                                                                   *const godot_variant)
                                                              -> godot_bool,
    pub godot_variant_booleanize: unsafe extern "C" fn(p_self:
                                                                                 *const godot_variant)
                                                            -> godot_bool,
    pub godot_variant_destroy: unsafe extern "C" fn(p_self:
                                                                              *mut godot_variant),
    pub godot_string_new: unsafe extern "C" fn(r_dest:
                                                                         *mut godot_string),
    pub godot_string_new_copy: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_string,
                                                                          p_src:
                                                                              *const godot_string),
    pub godot_string_new_data: unsafe extern "C" fn(r_dest:
                                                                              *mut godot_string,
                                                                          p_contents:
                                                                              *const libc::c_char,
                                                                          p_size:
                                                                              libc::c_int),
    pub godot_string_new_unicode_data: unsafe extern "C" fn(r_dest:
                                                                                      *mut godot_string,
                                                                                  p_contents:
                                                                                      *const wchar_t,
                                                                                  p_size:
                                                                                      libc::c_int),
    pub godot_string_get_data: unsafe extern "C" fn(p_self:
                                                                              *const godot_string,
                                                                          p_dest:
                                                                              *mut libc::c_char,
                                                                          p_size:
                                                                              *mut libc::c_int),
    pub godot_string_operator_index: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_string,
                                                                                p_idx:
                                                                                    godot_int)
                                                               ->
                                                                   *mut wchar_t,
    pub godot_string_c_str: unsafe extern "C" fn(p_self:
                                                                           *const godot_string)
                                                      -> *const libc::c_char,
    pub godot_string_unicode_str: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            ->
                                                                *const wchar_t,
    pub godot_string_operator_equal: unsafe extern "C" fn(p_self:
                                                                                    *const godot_string,
                                                                                p_b:
                                                                                    *const godot_string)
                                                               -> godot_bool,
    pub godot_string_operator_less: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string,
                                                                               p_b:
                                                                                   *const godot_string)
                                                              -> godot_bool,
    pub godot_string_operator_plus: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string,
                                                                               p_b:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_length: unsafe extern "C" fn(p_self:
                                                                            *const godot_string)
                                                       -> godot_int,
    pub godot_string_begins_with: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string,
                                                                             p_string:
                                                                                 *const godot_string)
                                                            -> godot_bool,
    pub godot_string_begins_with_char_array: unsafe extern "C" fn(p_self:
                                                                                            *const godot_string,
                                                                                        p_char_array:
                                                                                            *const libc::c_char)
                                                                       ->
                                                                           godot_bool,
    pub godot_string_bigrams: unsafe extern "C" fn(p_self:
                                                                             *const godot_string)
                                                        -> godot_array,
    pub godot_string_chr: unsafe extern "C" fn(p_character:
                                                                         wchar_t)
                                                    -> godot_string,
    pub godot_string_ends_with: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_string:
                                                                               *const godot_string)
                                                          -> godot_bool,
    pub godot_string_find: unsafe extern "C" fn(p_self:
                                                                          *const godot_string,
                                                                      p_what:
                                                                          godot_string)
                                                     -> godot_int,
    pub godot_string_find_from: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_what:
                                                                               godot_string,
                                                                           p_from:
                                                                               godot_int)
                                                          -> godot_int,
    pub godot_string_findmk: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_keys:
                                                                            *const godot_array)
                                                       -> godot_int,
    pub godot_string_findmk_from: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string,
                                                                             p_keys:
                                                                                 *const godot_array,
                                                                             p_from:
                                                                                 godot_int)
                                                            -> godot_int,
    pub godot_string_findmk_from_in_place: unsafe extern "C" fn(p_self:
                                                                                          *const godot_string,
                                                                                      p_keys:
                                                                                          *const godot_array,
                                                                                      p_from:
                                                                                          godot_int,
                                                                                      r_key:
                                                                                          *mut godot_int)
                                                                     ->
                                                                         godot_int,
    pub godot_string_findn: unsafe extern "C" fn(p_self:
                                                                           *const godot_string,
                                                                       p_what:
                                                                           godot_string)
                                                      -> godot_int,
    pub godot_string_findn_from: unsafe extern "C" fn(p_self:
                                                                                *const godot_string,
                                                                            p_what:
                                                                                godot_string,
                                                                            p_from:
                                                                                godot_int)
                                                           -> godot_int,
    pub godot_string_find_last: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_what:
                                                                               godot_string)
                                                          -> godot_int,
    pub godot_string_format: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_values:
                                                                            *const godot_variant)
                                                       -> godot_string,
    pub godot_string_format_with_custom_placeholder: unsafe extern "C" fn(p_self:
                                                                                                    *const godot_string,
                                                                                                p_values:
                                                                                                    *const godot_variant,
                                                                                                p_placeholder:
                                                                                                    *const libc::c_char)
                                                                               ->
                                                                                   godot_string,
    pub godot_string_hex_encode_buffer: unsafe extern "C" fn(p_buffer:
                                                                                       *const u8,
                                                                                   p_len:
                                                                                       godot_int)
                                                                  ->
                                                                      godot_string,
    pub godot_string_hex_to_int: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           -> godot_int,
    pub godot_string_hex_to_int_without_prefix: unsafe extern "C" fn(p_self:
                                                                                               *const godot_string)
                                                                          ->
                                                                              godot_int,
    pub godot_string_insert: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_at_pos:
                                                                            godot_int,
                                                                        p_string:
                                                                            godot_string)
                                                       -> godot_string,
    pub godot_string_is_numeric: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           -> godot_bool,
    pub godot_string_is_subsequence_of: unsafe extern "C" fn(p_self:
                                                                                       *const godot_string,
                                                                                   p_string:
                                                                                       *const godot_string)
                                                                  ->
                                                                      godot_bool,
    pub godot_string_is_subsequence_ofi: unsafe extern "C" fn(p_self:
                                                                                        *const godot_string,
                                                                                    p_string:
                                                                                        *const godot_string)
                                                                   ->
                                                                       godot_bool,
    pub godot_string_lpad: unsafe extern "C" fn(p_self:
                                                                          *const godot_string,
                                                                      p_min_length:
                                                                          godot_int)
                                                     -> godot_string,
    pub godot_string_lpad_with_custom_character: unsafe extern "C" fn(p_self:
                                                                                                *const godot_string,
                                                                                            p_min_length:
                                                                                                godot_int,
                                                                                            p_character:
                                                                                                *const godot_string)
                                                                           ->
                                                                               godot_string,
    pub godot_string_match: unsafe extern "C" fn(p_self:
                                                                           *const godot_string,
                                                                       p_wildcard:
                                                                           *const godot_string)
                                                      -> godot_bool,
    pub godot_string_matchn: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_wildcard:
                                                                            *const godot_string)
                                                       -> godot_bool,
    pub godot_string_md5: unsafe extern "C" fn(p_md5:
                                                                         *const u8)
                                                    -> godot_string,
    pub godot_string_num: unsafe extern "C" fn(p_num:
                                                                         f64)
                                                    -> godot_string,
    pub godot_string_num_int64: unsafe extern "C" fn(p_num:
                                                                               i64,
                                                                           p_base:
                                                                               godot_int)
                                                          -> godot_string,
    pub godot_string_num_int64_capitalized: unsafe extern "C" fn(p_num:
                                                                                           i64,
                                                                                       p_base:
                                                                                           godot_int,
                                                                                       p_capitalize_hex:
                                                                                           godot_bool)
                                                                      ->
                                                                          godot_string,
    pub godot_string_num_real: unsafe extern "C" fn(p_num:
                                                                              f64)
                                                         -> godot_string,
    pub godot_string_num_scientific: unsafe extern "C" fn(p_num:
                                                                                    f64)
                                                               ->
                                                                   godot_string,
    pub godot_string_num_with_decimals: unsafe extern "C" fn(p_num:
                                                                                       f64,
                                                                                   p_decimals:
                                                                                       godot_int)
                                                                  ->
                                                                      godot_string,
    pub godot_string_pad_decimals: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string,
                                                                              p_digits:
                                                                                  godot_int)
                                                             -> godot_string,
    pub godot_string_pad_zeros: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_digits:
                                                                               godot_int)
                                                          -> godot_string,
    pub godot_string_replace_first: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string,
                                                                               p_key:
                                                                                   godot_string,
                                                                               p_with:
                                                                                   godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_replace: unsafe extern "C" fn(p_self:
                                                                             *const godot_string,
                                                                         p_key:
                                                                             godot_string,
                                                                         p_with:
                                                                             godot_string)
                                                        -> godot_string,
    pub godot_string_replacen: unsafe extern "C" fn(p_self:
                                                                              *const godot_string,
                                                                          p_key:
                                                                              godot_string,
                                                                          p_with:
                                                                              godot_string)
                                                         -> godot_string,
    pub godot_string_rfind: unsafe extern "C" fn(p_self:
                                                                           *const godot_string,
                                                                       p_what:
                                                                           godot_string)
                                                      -> godot_int,
    pub godot_string_rfindn: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_what:
                                                                            godot_string)
                                                       -> godot_int,
    pub godot_string_rfind_from: unsafe extern "C" fn(p_self:
                                                                                *const godot_string,
                                                                            p_what:
                                                                                godot_string,
                                                                            p_from:
                                                                                godot_int)
                                                           -> godot_int,
    pub godot_string_rfindn_from: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string,
                                                                             p_what:
                                                                                 godot_string,
                                                                             p_from:
                                                                                 godot_int)
                                                            -> godot_int,
    pub godot_string_rpad: unsafe extern "C" fn(p_self:
                                                                          *const godot_string,
                                                                      p_min_length:
                                                                          godot_int)
                                                     -> godot_string,
    pub godot_string_rpad_with_custom_character: unsafe extern "C" fn(p_self:
                                                                                                *const godot_string,
                                                                                            p_min_length:
                                                                                                godot_int,
                                                                                            p_character:
                                                                                                *const godot_string)
                                                                           ->
                                                                               godot_string,
    pub godot_string_similarity: unsafe extern "C" fn(p_self:
                                                                                *const godot_string,
                                                                            p_string:
                                                                                *const godot_string)
                                                           -> godot_real,
    pub godot_string_sprintf: unsafe extern "C" fn(p_self:
                                                                             *const godot_string,
                                                                         p_values:
                                                                             *const godot_array,
                                                                         p_error:
                                                                             *mut godot_bool)
                                                        -> godot_string,
    pub godot_string_substr: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_from:
                                                                            godot_int,
                                                                        p_chars:
                                                                            godot_int)
                                                       -> godot_string,
    pub godot_string_to_double: unsafe extern "C" fn(p_self:
                                                                               *const godot_string)
                                                          -> f64,
    pub godot_string_to_float: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_real,
    pub godot_string_to_int: unsafe extern "C" fn(p_self:
                                                                            *const godot_string)
                                                       -> godot_int,
    pub godot_string_camelcase_to_underscore: unsafe extern "C" fn(p_self:
                                                                                             *const godot_string)
                                                                        ->
                                                                            godot_string,
    pub godot_string_camelcase_to_underscore_lowercased: unsafe extern "C" fn(p_self:
                                                                                                        *const godot_string)
                                                                                   ->
                                                                                       godot_string,
    pub godot_string_capitalize: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           -> godot_string,
    pub godot_string_char_to_double: unsafe extern "C" fn(p_what:
                                                                                    *const libc::c_char)
                                                               -> f64,
    pub godot_string_char_to_int: unsafe extern "C" fn(p_what:
                                                                                 *const libc::c_char)
                                                            -> godot_int,
    pub godot_string_wchar_to_int: unsafe extern "C" fn(p_str:
                                                                                  *const wchar_t)
                                                             -> i64,
    pub godot_string_char_to_int_with_len: unsafe extern "C" fn(p_what:
                                                                                          *const libc::c_char,
                                                                                      p_len:
                                                                                          godot_int)
                                                                     ->
                                                                         godot_int,
    pub godot_string_char_to_int64_with_len: unsafe extern "C" fn(p_str:
                                                                                            *const wchar_t,
                                                                                        p_len:
                                                                                            libc::c_int)
                                                                       ->
                                                                           i64,
    pub godot_string_hex_to_int64: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string)
                                                             -> i64,
    pub godot_string_hex_to_int64_with_prefix: unsafe extern "C" fn(p_self:
                                                                                              *const godot_string)
                                                                         ->
                                                                             i64,
    pub godot_string_to_int64: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> i64,
    pub godot_string_unicode_char_to_double: unsafe extern "C" fn(p_str:
                                                                                            *const wchar_t,
                                                                                        r_end:
                                                                                            *mut *const wchar_t)
                                                                       ->
                                                                           f64,
    pub godot_string_get_slice_count: unsafe extern "C" fn(p_self:
                                                                                     *const godot_string,
                                                                                 p_splitter:
                                                                                     godot_string)
                                                                -> godot_int,
    pub godot_string_get_slice: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_splitter:
                                                                               godot_string,
                                                                           p_slice:
                                                                               godot_int)
                                                          -> godot_string,
    pub godot_string_get_slicec: unsafe extern "C" fn(p_self:
                                                                                *const godot_string,
                                                                            p_splitter:
                                                                                wchar_t,
                                                                            p_slice:
                                                                                godot_int)
                                                           -> godot_string,
    pub godot_string_split: unsafe extern "C" fn(p_self:
                                                                           *const godot_string,
                                                                       p_splitter:
                                                                           *const godot_string)
                                                      -> godot_array,
    pub godot_string_split_allow_empty: unsafe extern "C" fn(p_self:
                                                                                       *const godot_string,
                                                                                   p_splitter:
                                                                                       *const godot_string)
                                                                  ->
                                                                      godot_array,
    pub godot_string_split_floats: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string,
                                                                              p_splitter:
                                                                                  *const godot_string)
                                                             -> godot_array,
    pub godot_string_split_floats_allows_empty: unsafe extern "C" fn(p_self:
                                                                                               *const godot_string,
                                                                                           p_splitter:
                                                                                               *const godot_string)
                                                                          ->
                                                                              godot_array,
    pub godot_string_split_floats_mk: unsafe extern "C" fn(p_self:
                                                                                     *const godot_string,
                                                                                 p_splitters:
                                                                                     *const godot_array)
                                                                ->
                                                                    godot_array,
    pub godot_string_split_floats_mk_allows_empty: unsafe extern "C" fn(p_self:
                                                                                                  *const godot_string,
                                                                                              p_splitters:
                                                                                                  *const godot_array)
                                                                             ->
                                                                                 godot_array,
    pub godot_string_split_ints: unsafe extern "C" fn(p_self:
                                                                                *const godot_string,
                                                                            p_splitter:
                                                                                *const godot_string)
                                                           -> godot_array,
    pub godot_string_split_ints_allows_empty: unsafe extern "C" fn(p_self:
                                                                                             *const godot_string,
                                                                                         p_splitter:
                                                                                             *const godot_string)
                                                                        ->
                                                                            godot_array,
    pub godot_string_split_ints_mk: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string,
                                                                               p_splitters:
                                                                                   *const godot_array)
                                                              -> godot_array,
    pub godot_string_split_ints_mk_allows_empty: unsafe extern "C" fn(p_self:
                                                                                                *const godot_string,
                                                                                            p_splitters:
                                                                                                *const godot_array)
                                                                           ->
                                                                               godot_array,
    pub godot_string_split_spaces: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string)
                                                             -> godot_array,
    pub godot_string_char_lowercase: unsafe extern "C" fn(p_char:
                                                                                    wchar_t)
                                                               -> wchar_t,
    pub godot_string_char_uppercase: unsafe extern "C" fn(p_char:
                                                                                    wchar_t)
                                                               -> wchar_t,
    pub godot_string_to_lower: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_string,
    pub godot_string_to_upper: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_string,
    pub godot_string_get_basename: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string)
                                                             -> godot_string,
    pub godot_string_get_extension: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_left: unsafe extern "C" fn(p_self:
                                                                          *const godot_string,
                                                                      p_pos:
                                                                          godot_int)
                                                     -> godot_string,
    pub godot_string_ord_at: unsafe extern "C" fn(p_self:
                                                                            *const godot_string,
                                                                        p_idx:
                                                                            godot_int)
                                                       -> wchar_t,
    pub godot_string_plus_file: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_file:
                                                                               *const godot_string)
                                                          -> godot_string,
    pub godot_string_right: unsafe extern "C" fn(p_self:
                                                                           *const godot_string,
                                                                       p_pos:
                                                                           godot_int)
                                                      -> godot_string,
    pub godot_string_strip_edges: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string,
                                                                             p_left:
                                                                                 godot_bool,
                                                                             p_right:
                                                                                 godot_bool)
                                                            -> godot_string,
    pub godot_string_strip_escapes: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_erase: unsafe extern "C" fn(p_self:
                                                                           *mut godot_string,
                                                                       p_pos:
                                                                           godot_int,
                                                                       p_chars:
                                                                           godot_int),
    pub godot_string_ascii: unsafe extern "C" fn(p_self:
                                                                           *mut godot_string,
                                                                       result:
                                                                           *mut libc::c_char),
    pub godot_string_ascii_extended: unsafe extern "C" fn(p_self:
                                                                                    *mut godot_string,
                                                                                result:
                                                                                    *mut libc::c_char),
    pub godot_string_utf8: unsafe extern "C" fn(p_self:
                                                                          *mut godot_string,
                                                                      result:
                                                                          *mut libc::c_char),
    pub godot_string_parse_utf8: unsafe extern "C" fn(p_self:
                                                                                *mut godot_string,
                                                                            p_utf8:
                                                                                *const libc::c_char)
                                                           -> godot_bool,
    pub godot_string_parse_utf8_with_len: unsafe extern "C" fn(p_self:
                                                                                         *mut godot_string,
                                                                                     p_utf8:
                                                                                         *const libc::c_char,
                                                                                     p_len:
                                                                                         godot_int)
                                                                    ->
                                                                        godot_bool,
    pub godot_string_chars_to_utf8: unsafe extern "C" fn(p_utf8:
                                                                                   *const libc::c_char)
                                                              ->
                                                                  godot_string,
    pub godot_string_chars_to_utf8_with_len: unsafe extern "C" fn(p_utf8:
                                                                                            *const libc::c_char,
                                                                                        p_len:
                                                                                            godot_int)
                                                                       ->
                                                                           godot_string,
    pub godot_string_hash: unsafe extern "C" fn(p_self:
                                                                          *const godot_string)
                                                     -> u32,
    pub godot_string_hash64: unsafe extern "C" fn(p_self:
                                                                            *const godot_string)
                                                       -> u64,
    pub godot_string_hash_chars: unsafe extern "C" fn(p_cstr:
                                                                                *const libc::c_char)
                                                           -> u32,
    pub godot_string_hash_chars_with_len: unsafe extern "C" fn(p_cstr:
                                                                                         *const libc::c_char,
                                                                                     p_len:
                                                                                         godot_int)
                                                                    -> u32,
    pub godot_string_hash_utf8_chars: unsafe extern "C" fn(p_str:
                                                                                     *const wchar_t)
                                                                -> u32,
    pub godot_string_hash_utf8_chars_with_len: unsafe extern "C" fn(p_str:
                                                                                              *const wchar_t,
                                                                                          p_len:
                                                                                              godot_int)
                                                                         ->
                                                                             u32,
    pub godot_string_md5_buffer: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           ->
                                                               godot_pool_byte_array,
    pub godot_string_md5_text: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_string,
    pub godot_string_sha256_buffer: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_pool_byte_array,
    pub godot_string_sha256_text: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            -> godot_string,
    pub godot_string_empty: unsafe extern "C" fn(p_self:
                                                                           *const godot_string)
                                                      -> godot_bool,
    pub godot_string_get_base_dir: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string)
                                                             -> godot_string,
    pub godot_string_get_file: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_string,
    pub godot_string_humanize_size: unsafe extern "C" fn(p_size:
                                                                                   usize)
                                                              ->
                                                                  godot_string,
    pub godot_string_is_abs_path: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            -> godot_bool,
    pub godot_string_is_rel_path: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            -> godot_bool,
    pub godot_string_is_resource_file: unsafe extern "C" fn(p_self:
                                                                                      *const godot_string)
                                                                 ->
                                                                     godot_bool,
    pub godot_string_path_to: unsafe extern "C" fn(p_self:
                                                                             *const godot_string,
                                                                         p_path:
                                                                             *const godot_string)
                                                        -> godot_string,
    pub godot_string_path_to_file: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string,
                                                                              p_path:
                                                                                  *const godot_string)
                                                             -> godot_string,
    pub godot_string_simplify_path: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_c_escape: unsafe extern "C" fn(p_self:
                                                                              *const godot_string)
                                                         -> godot_string,
    pub godot_string_c_escape_multiline: unsafe extern "C" fn(p_self:
                                                                                        *const godot_string)
                                                                   ->
                                                                       godot_string,
    pub godot_string_c_unescape: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           -> godot_string,
    pub godot_string_http_escape: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            -> godot_string,
    pub godot_string_http_unescape: unsafe extern "C" fn(p_self:
                                                                                   *const godot_string)
                                                              ->
                                                                  godot_string,
    pub godot_string_json_escape: unsafe extern "C" fn(p_self:
                                                                                 *const godot_string)
                                                            -> godot_string,
    pub godot_string_word_wrap: unsafe extern "C" fn(p_self:
                                                                               *const godot_string,
                                                                           p_chars_per_line:
                                                                               godot_int)
                                                          -> godot_string,
    pub godot_string_xml_escape: unsafe extern "C" fn(p_self:
                                                                                *const godot_string)
                                                           -> godot_string,
    pub godot_string_xml_escape_with_quotes: unsafe extern "C" fn(p_self:
                                                                                            *const godot_string)
                                                                       ->
                                                                           godot_string,
    pub godot_string_xml_unescape: unsafe extern "C" fn(p_self:
                                                                                  *const godot_string)
                                                             -> godot_string,
    pub godot_string_percent_decode: unsafe extern "C" fn(p_self:
                                                                                    *const godot_string)
                                                               ->
                                                                   godot_string,
    pub godot_string_percent_encode: unsafe extern "C" fn(p_self:
                                                                                    *const godot_string)
                                                               ->
                                                                   godot_string,
    pub godot_string_is_valid_float: unsafe extern "C" fn(p_self:
                                                                                    *const godot_string)
                                                               -> godot_bool,
    pub godot_string_is_valid_hex_number: unsafe extern "C" fn(p_self:
                                                                                         *const godot_string,
                                                                                     p_with_prefix:
                                                                                         godot_bool)
                                                                    ->
                                                                        godot_bool,
    pub godot_string_is_valid_html_color: unsafe extern "C" fn(p_self:
                                                                                         *const godot_string)
                                                                    ->
                                                                        godot_bool,
    pub godot_string_is_valid_identifier: unsafe extern "C" fn(p_self:
                                                                                         *const godot_string)
                                                                    ->
                                                                        godot_bool,
    pub godot_string_is_valid_integer: unsafe extern "C" fn(p_self:
                                                                                      *const godot_string)
                                                                 ->
                                                                     godot_bool,
    pub godot_string_is_valid_ip_address: unsafe extern "C" fn(p_self:
                                                                                         *const godot_string)
                                                                    ->
                                                                        godot_bool,
    pub godot_string_destroy: unsafe extern "C" fn(p_self:
                                                                             *mut godot_string),
    pub godot_object_destroy: unsafe extern "C" fn(p_o:
                                                                             *mut godot_object),
    pub godot_global_get_singleton: unsafe extern "C" fn(p_name:
                                                                                   *mut libc::c_char)
                                                              ->
                                                                  *mut godot_object,
    pub godot_method_bind_get_method: unsafe extern "C" fn(p_classname:
                                                                                     *const libc::c_char,
                                                                                 p_methodname:
                                                                                     *const libc::c_char)
                                                                ->
                                                                    *mut godot_method_bind,
    pub godot_method_bind_ptrcall: unsafe extern "C" fn(p_method_bind:
                                                                                  *mut godot_method_bind,
                                                                              p_instance:
                                                                                  *mut godot_object,
                                                                              p_args:
                                                                                  *mut *const libc::c_void,
                                                                              p_ret:
                                                                                  *mut libc::c_void),
    pub godot_method_bind_call: unsafe extern "C" fn(p_method_bind:
                                                                               *mut godot_method_bind,
                                                                           p_instance:
                                                                               *mut godot_object,
                                                                           p_args:
                                                                               *mut *const godot_variant,
                                                                           p_arg_count:
                                                                               libc::c_int,
                                                                           p_call_error:
                                                                               *mut godot_variant_call_error)
                                                          -> godot_variant,
    pub godot_get_class_constructor: unsafe extern "C" fn(p_classname:
                                                                                    *const libc::c_char)
                                                               ->
                                                                   godot_class_constructor,
    pub godot_alloc: unsafe extern "C" fn(p_bytes:
                                                                    libc::c_int)
                                               -> *mut libc::c_void,
    pub godot_realloc: unsafe extern "C" fn(p_ptr:
                                                                      *mut libc::c_void,
                                                                  p_bytes:
                                                                      libc::c_int)
                                                 -> *mut libc::c_void,
    pub godot_free: unsafe extern "C" fn(p_ptr:
                                                                   *mut libc::c_void),
    pub godot_print_error: unsafe extern "C" fn(p_description:
                                                                          *const libc::c_char,
                                                                      p_function:
                                                                          *const libc::c_char,
                                                                      p_file:
                                                                          *const libc::c_char,
                                                                      p_line:
                                                                          libc::c_int),
    pub godot_print_warning: unsafe extern "C" fn(p_description:
                                                                            *const libc::c_char,
                                                                        p_function:
                                                                            *const libc::c_char,
                                                                        p_file:
                                                                            *const libc::c_char,
                                                                        p_line:
                                                                            libc::c_int),
    pub godot_print: unsafe extern "C" fn(p_message:
                                                                    *const godot_string),
    pub godot_nativescript_register_class: unsafe extern "C" fn(p_gdnative_handle:
                                                                                          *mut libc::c_void,
                                                                                      p_name:
                                                                                          *const libc::c_char,
                                                                                      p_base:
                                                                                          *const libc::c_char,
                                                                                      p_create_func:
                                                                                          godot_instance_create_func,
                                                                                      p_destroy_func:
                                                                                          godot_instance_destroy_func),
    pub godot_nativescript_register_tool_class: unsafe extern "C" fn(p_gdnative_handle:
                                                                                               *mut libc::c_void,
                                                                                           p_name:
                                                                                               *const libc::c_char,
                                                                                           p_base:
                                                                                               *const libc::c_char,
                                                                                           p_create_func:
                                                                                               godot_instance_create_func,
                                                                                           p_destroy_func:
                                                                                               godot_instance_destroy_func),
    pub godot_nativescript_register_method: unsafe extern "C" fn(p_gdnative_handle:
                                                                                           *mut libc::c_void,
                                                                                       p_name:
                                                                                           *const libc::c_char,
                                                                                       p_function_name:
                                                                                           *const libc::c_char,
                                                                                       p_attr:
                                                                                           godot_method_attributes,
                                                                                       p_method:
                                                                                           godot_instance_method),
    pub godot_nativescript_register_property: unsafe extern "C" fn(p_gdnative_handle:
                                                                                             *mut libc::c_void,
                                                                                         p_name:
                                                                                             *const libc::c_char,
                                                                                         p_path:
                                                                                             *const libc::c_char,
                                                                                         p_attr:
                                                                                             *mut godot_property_attributes,
                                                                                         p_set_func:
                                                                                             godot_property_set_func,
                                                                                         p_get_func:
                                                                                             godot_property_get_func),
    pub godot_nativescript_register_signal: unsafe extern "C" fn(p_gdnative_handle:
                                                                                           *mut libc::c_void,
                                                                                       p_name:
                                                                                           *const libc::c_char,
                                                                                       p_signal:
                                                                                           *const godot_signal),
    pub godot_nativescript_get_userdata: unsafe extern "C" fn(p_instance:
                                                                                        *mut godot_object)
                                                                   ->
                                                                       *mut libc::c_void,
}}