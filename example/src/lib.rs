#[macro_use]
extern crate godot;


gdclass! {
    class RustTest: godot::types::MeshInstance {
        fields {
        }
        constructor(godot_info) {
            RustTest {
                godot_info: godot_info,
            }
        }

        export fn _ready(&mut self) {
            let p = self.godot_parent();
            p.set_physics_process(true);
        }

        export fn _physics_process(&mut self, _delta: f64) {
            let p = self.godot_parent();
            p.rotate_y(0.05);
        }
    }
}

gd_init! {
    RustTest
}