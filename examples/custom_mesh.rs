extern crate native;
extern crate kiss3d;
extern crate nalgebra;

use nalgebra::na::{Vec3, Rotation};
use kiss3d::window::Window;
use kiss3d::resource::Mesh;
use kiss3d::light;

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
    Window::spawn("Kiss3d: cube", proc(window) {
        let a = Vec3::new(-1.0, -1.0, 0.0);
        let b = Vec3::new(1.0, -1.0, 0.0);
        let c = Vec3::new(0.0, 1.0, 0.0);

        let vertices = ~[a, b, c];
        let indices  = ~[Vec3::new(0u32, 1, 2)];

        let mesh = Mesh::new(vertices, indices, None, None, false);

        let mut c = window.add_mesh(mesh, 1.0);

        c.set_color(1.0, 0.0, 0.0);

        window.set_light(light::StickToCamera);

        window.render_loop(|_| {
            c.prepend_rotation(&Vec3::new(0.0f32, 0.014, 0.0))
        })
    })
}
