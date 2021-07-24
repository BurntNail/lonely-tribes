use amethyst::core::ecs::{Builder, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::Camera;

pub fn init_camera(world: &mut World, wh: (f32, f32)) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(wh.0 * 0.5, wh.1 * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(wh.0, wh.1))
        .with(transform)
        .build();
}
