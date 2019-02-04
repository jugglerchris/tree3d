extern crate amethyst;

use amethyst::{
    prelude::*,
    renderer::{PosNormTex, DrawShaded, DrawSkybox, Projection, Camera, Light, PointLight, MeshData, Mesh, Material, MaterialDefaults, Texture, VirtualKeyCode, DisplayConfig, Pipeline, Stage, RenderBundle},
    core::{Transform, transform::{TransformBundle, Parent}},
    controls::{ArcBallControlBundle, ArcBallControlTag, FlyControlTag},
    utils::{application_root_dir, scene::BasicScenePrefab, auto_fov::{AutoFov, AutoFovSystem}},
    assets::{AssetLoaderSystemData, Handle, PrefabLoader, PrefabLoaderSystem, RonFormat},
    input::{get_key, is_close_requested, is_key_down, InputBundle},
    ecs::prelude::Entity,
};
use std::f32::consts::PI;


struct Example {
    pub thing: Option<Entity>,
}

impl Example {
    fn make_tree_at(&mut self,
                    world: &mut World,
                    mesh: &Handle<Mesh>,
                    material: &Material,
                    pos: &Transform,
                    parent: Option<Entity>,
                    depth: usize)
    -> Entity
    {
        let mut entity_builder = world
                .create_entity()
                .with(pos.clone())
                .with(mesh.clone())
                .with(material.clone());
        if let Some(parent) = parent {
            entity_builder = entity_builder.with(Parent { entity: parent });
        }
        let entity = entity_builder.build();
        if depth > 0 {
            let mut child_transform = Transform::default();
            //child_transform.set_x(-0.5);
            child_transform.set_scale(0.8, 0.8, 0.8);
            child_transform.set_y(1.0);
            child_transform.yaw_local(PI/2.);
            child_transform.roll_local(0.3);
            self.make_tree_at(world, mesh, material, &child_transform, Some(entity), depth-1);
            child_transform.roll_local(-0.7);
            child_transform.set_scale(0.45, 0.45, 0.8);
            self.make_tree_at(world, mesh, material, &child_transform, Some(entity), depth-1);
        }
        entity
    }

    fn make_trunk_mesh(&mut self, world: &mut World) -> Handle<Mesh> {
        const POINTS: usize = 20;

        let mut points = vec![];
        for i in 0..POINTS {
            let angle_start = (i as f32) * 2.*PI / (POINTS as f32);
            let angle_next = ((i+1) as f32) * 2.*PI / (POINTS as f32);
            let angle_middle = (i as f32 + 0.5) * 2.*PI / (POINTS as f32);
            // Bottom right
            points.push(
                PosNormTex {
                    position: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    normal: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });
            // Bottom left
            points.push(
                PosNormTex {
                    position: [ angle_start.cos(), 0.0, angle_start.sin() ].into(),
                    normal: [ angle_start.cos(), 0.0, angle_start.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });
            // Top middle
            points.push(
                PosNormTex {
                    position: [ angle_middle.cos(), 1.0, angle_middle.sin() ].into(),
                    normal: [ angle_middle.cos(), 0.0, angle_middle.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });

            let angle_nextmid = ((i+1) as f32 + 0.5) * 2.*PI / (POINTS as f32);
            // Bottom middle
            points.push(
                PosNormTex {
                    position: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    normal: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });
            // Top left
            points.push(
                PosNormTex {
                    position: [ angle_middle.cos(), 1.0, angle_middle.sin() ].into(),
                    normal: [ angle_middle.cos(), 0.0, angle_middle.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });
            // Top right
            points.push(
                PosNormTex {
                    position: [ angle_nextmid.cos(), 1.0, angle_nextmid.sin() ].into(),
                    normal: [ angle_nextmid.cos(), 0.0, angle_nextmid.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                });
        }

        world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                MeshData::PosNormTex(points),
                (),
            )
        })
    }

    fn make_tree(&mut self, world: &mut World) {
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        let mut thing_pos = Transform::default();
        thing_pos.set_scale(0.2, 1.0, 0.2);
        //thing_pos.face_towards([0.0, -10.0, 0.0].into(), [0.0, 0.0, 1.0].into());
        let thing_mesh = self.make_trunk_mesh(world);

        let thing_albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data([0.4, 0.4, 0.0, 1.0].into(), ())
        });

        let thing_material = Material {
            albedo: thing_albedo,
            ..material_defaults.clone()
        };
        self.thing = Some(self.make_tree_at(world,
                                            &thing_mesh,
                                            &thing_material,
                                            &thing_pos,
                                            None,
                                            5));
    }
}

type MyPrefabData = BasicScenePrefab<Vec<PosNormTex>>;

impl SimpleState for Example {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let mut camera_trans = Transform::default();
        camera_trans.set_z(-4.0);
        camera_trans.set_x(-4.0);
        camera_trans.set_y(1.0);
        //camera_trans.set_rotation_euler(0.0, 0.0, 90.0);
        camera_trans.face_towards([0.0, 0.0, 0.0].into(), [0.0, 1.0, 0.0].into());

        // Make the initial background scene
        {
            let handle = world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
                loader.load("resources/scene.ron", RonFormat, (), ())
            });
            world.create_entity().with(handle).build();
        }

        // Make an tree.
        self.make_tree(world);

        // Make the camera
        world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.3, 1.0471975512)))
            .with(camera_trans)
            .with(ArcBallControlTag {
                target: self.thing.unwrap(),
                distance: 6.0,
            })
            .with(FlyControlTag)
            .with(AutoFov::new())
            .build();

        // Make a light
        let mut light_transform = Transform::default();
        light_transform.set_x(-4.0);
        light_transform.set_y(12.0);
        light_transform.set_z(-4.0);
        let light_point = PointLight {
            color: [1.0, 1.0, 1.0, 1.0].into(),
            intensity: 130.0,
            radius: 5.0,
            ..PointLight::default()
        };
        world
            .create_entity()
            .with(Light::Point(light_point))
            .with(light_transform)
            .build();
    }

    fn handle_event( &mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent, ) -> SimpleTrans {
        let StateData { world: _, .. } = data;
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
        application_root_dir().unwrap().to_str().unwrap()
    );

    let render_bundle = {
        let display_config = DisplayConfig::load(&path);
        let pipe = Pipeline::build().with_stage(
            Stage::with_backbuffer()
                .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
                .with_pass(DrawShaded::<PosNormTex>::new())
                .with_pass(DrawSkybox::new()),
        );
        RenderBundle::new(pipe, Some(display_config))
    };

    let key_bindings_path = format!("{}/resources/input.ron", application_root_dir().unwrap().to_str().unwrap());
    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with(AutoFovSystem, "", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<String, String>::new().with_bindings_from_file(&key_bindings_path)?)?
        .with_bundle(ArcBallControlBundle::<String, String>::new())?
        .with_bundle(render_bundle)?;
    let mut game = Application::new("./", Example { thing: None }, game_data)?;

    game.run();

    Ok(())
}
