extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{PosNormTex, DrawShaded, Projection, Camera, Light, PointLight, Shape, Mesh, Material, MaterialDefaults, Texture, VirtualKeyCode},
    core::{Transform, transform::TransformBundle},
    utils::application_root_dir,
    assets::AssetLoaderSystemData,
    input::{get_key, is_close_requested, is_key_down},
    ecs::prelude::Entity,
};

struct Example {
    pub thing: Option<Entity>,
}

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        let mut camera_trans = Transform::default();
        camera_trans.set_z(-4.0);
        //camera_trans.set_rotation_euler(0.0, 0.0, 90.0);
        camera_trans.face_towards([0.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into());

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
        let light_point = PointLight {
            color: [1.0, 1.0, 1.0, 1.0].into(),
            intensity: 3.0,
            radius: 5.0,
            ..PointLight::default()
        };
        world
            .create_entity()
            .with(Light::Point(light_point))
            .with(light_transform)
            .build();

        // And an object.
        let thing_pos = Transform::default();
        let thing_mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                Shape::Cylinder(32, None).generate::<Vec<PosNormTex>>(None),
                (),
            )
        });

        let thing_albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data([1.0, 0.0, 1.0, 1.0].into(), ())
        });

        let thing_material = Material {
            albedo: thing_albedo,
            ..material_defaults.clone()
        };
        self.thing = Some(world
            .create_entity()
            .with(thing_pos)
            .with(thing_mesh)
            .with(thing_material)
            .build());
    }

    fn handle_event( &mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> SimpleTrans {
        let StateData { world, .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
            match get_key(&event) {
                _ => {}
            }
        }
        Trans::None
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
    let mut game = Application::new("./", Example { thing: None }, game_data)?;

    game.run();

    Ok(())
}
