#[macro_use]
extern crate godot;


gdclass! {
    class RustTest: godot::types::MeshInstance {
        fields {
            start: godot::types::Vector3,
            time: f32,
        }
        constructor(godot_info) {
            RustTest {
                godot_info: godot_info,
                start: godot::types::Vector3::new(0.0, 0.0, 0.0),
                time: 0.0,
            }
        }

        export fn _ready(&mut self) {
            let p = self.godot_parent();
            p.set_physics_process(true);
            self.start = p.get_translation();
            gprint_warn!("Start: {:?}", self.start);
            gprint_warn!("Parent name: {:?}", p.get_parent()
                .expect("Missing parent")
                .get_name());

            gprint_warn!("{}", p.get_instance_id());
        }

        export fn _physics_process(&mut self, delta: f64) {
            self.time += delta as f32;
            let p = self.godot_parent();
            p.rotate_y(0.05);
            let offset = godot::types::Vector3::new(0.0, 1.0, 0.0) * self.time.cos() * 0.5;
            p.set_translation(self.start + offset);

            if let Some(mat) = p.get_surface_material(0) {
                let mat = mat.cast::<godot::types::SpatialMaterial>().expect("Incorrect material");
                mat.set_albedo(godot::types::Color::new_rgba(self.time.cos().abs(), 0.0, 0.0, 1.0));
            } else {
                gprint_warn!("No material");
            }
        }
    }
}

gd_init! {
    RustTest
}