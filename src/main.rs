extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{PosNormTex, DrawShaded, Projection, Camera, Light},
    core::{Transform, transform::TransformBundle},
    utils::application_root_dir,
};
use nalgebra::geometry::Translation3;

struct Example;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut camera_trans = Transform::default();
        camera_trans.set_z(-0.4);

        // Make the camera
        world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.3, 1.0471975512)))
            .with(camera_trans)
            .build();

        // Make a light
        let mut light_transform = Transform::default();
        light_transform.set_x(2.0);
        light_transform.set_y(2.0);
        light_transform.set_z(-2.0);
        world
            .create_entity()
            .with(Light::Point(Default::default()))
            .with(light_transform)
            .build();
    }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/resources/display_config.ron",
        application_root_dir()
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?//RenderBundle::new(pipe, Some(config)))?
        .with_basic_renderer(path, DrawShaded::<PosNormTex>::new(), false)?;
    let mut game = Application::new("./", Example, game_data)?;

    game.run();

    Ok(())
}
