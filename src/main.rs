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
use nalgebra::{Vector3, Vector4, UnitQuaternion, U3, Unit};
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
            let mut scale = Transform::default();
            //scale.set_scale(0.9, 0.9, 0.9);
            let mut child_transform = Transform::default();
            child_transform.move_up(1.0);
            //child_transform.yaw_local(PI/2.);
            child_transform.roll_local(0.3);
            //scale.concat(&child_transform);
            //child_transform.concat(&scale);
            //child_transform = scale;
            self.make_tree_at(world, mesh, material, &child_transform, Some(entity), depth-1);
            /*
            child_transform.roll_local(-1.4);
            child_transform.set_scale(0.5, 0.9, 0.5);
            self.make_tree_at(world, mesh, material, &child_transform, Some(entity), depth-1);
            */
        }
        entity
    }

    fn make_cylinder(&mut self, world: &mut World) -> Handle<Mesh> {
        const POINTS: usize = 20;

        let mut points = vec![];
        for i in 0..POINTS {
            let angle_start = (i as f32) * 2.*PI / (POINTS as f32);
            let angle_next = ((i+1) as f32) * 2.*PI / (POINTS as f32);
            let angle_middle = (i as f32 + 0.5) * 2.*PI / (POINTS as f32);
            let angle_nextmid = ((i+1) as f32 + 0.5) * 2.*PI / (POINTS as f32);
            let bottom_right =  PosNormTex {
                    position: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    normal: [ angle_next.cos(), 0.0, angle_next.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            let bottom_left =  PosNormTex {
                    position: [ angle_start.cos(), 0.0, angle_start.sin() ].into(),
                    normal: [ angle_start.cos(), 0.0, angle_start.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            let top_middle = PosNormTex {
                    position: [ angle_middle.cos(), 1.0, angle_middle.sin() ].into(),
                    normal: [ angle_middle.cos(), 0.0, angle_middle.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            let next_middle = PosNormTex {
                    position: [ angle_nextmid.cos(), 1.0, angle_nextmid.sin() ].into(),
                    normal: [ angle_nextmid.cos(), 0.0, angle_nextmid.sin() ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            let centre_top = PosNormTex {
                    position: [ 0.0, 1.0, 0.0 ].into(),
                    normal: [ 0.0, 1.0, 0.0 ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            let centre_bot = PosNormTex {
                    position: [ 0.0, 0.0, 0.0 ].into(),
                    normal: [ 0.0, -1.0, 0.0 ].into(),
                    tex_coord: [0.0, 0.0].into(),
                };
            // Bottom half triangle
            points.push(bottom_right);
            points.push(bottom_left);
            points.push(top_middle);

            // Top half triangle
            points.push(bottom_right);
            points.push(top_middle);
            points.push(next_middle);

            // Top cap piece
            points.push(top_middle);
            points.push(centre_top);
            points.push(next_middle);

            // Bottom cap piece
            points.push(bottom_left);
            points.push(bottom_right);
            points.push(centre_bot);
        }

        world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
            loader.load_from_data(
                MeshData::PosNormTex(points),
                (),
            )
        })
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
    fn make_tree_points(&self, points: &mut Vec<PosNormTex>, depth: usize) {
        const POINTS: usize = 40;
        // We divide into two halves
        assert_eq!(POINTS % 4, 0);

        struct BranchPoint {
            pos: [f32; 3],
            out_normal: [f32; 3],
            radius: f32,
        }

        // For now assume that the first half of the base of the
        // trunk matches up with the first branch point, and the
        // second half the other.
        let branch_points = [
            BranchPoint {
                pos: [0.0, 3.0, 0.5],
                out_normal: [0.0, 9.0f32.sqrt(), 2.0f32.sqrt()],
                radius: 0.7,
            },
            BranchPoint {
                pos: [0.0, 3.0, -0.5],
                out_normal: [0.0, 9.0f32.sqrt(), -2.0f32.sqrt()],
                radius: 0.7,
            },
        ];

        for _ in 0..1 {
            let mut points_bot = Vec::new();

            let angle_delta = (2.*PI) / (POINTS as f32);
            for i in 0..POINTS {
                let angle = angle_delta * (i as f32);
                points_bot.push(
                    PosNormTex {
                        position: [ angle.cos(), 0.0, angle.sin() ].into(),
                        normal: [ angle.cos(), 0.0, angle.sin() ].into(),
                        tex_coord: [0.0, 0.0].into(),
                    });
            }
            // Top points are ahead by half a step
            let mut points_top = Vec::new();

            let mut sub_transforms = Vec::new();

            for branch in &branch_points {
                let mut transform = Transform::default();
                transform.set_scale(branch.radius, branch.radius, branch.radius);
                let rot =  UnitQuaternion::rotation_between(
                    &Vector3::new(0.0, 1.0, 0.0),
                    &branch.out_normal.into());
                if let Some(rot) = rot {
                    transform.set_rotation(rot);
                }
                transform.set_position(branch.pos.into());
                let matrix = transform.matrix();
                if depth > 1 {
                    sub_transforms.push(matrix.clone());
                }
                for i in 0..POINTS/2 {
                    let u = (i as f32 + 0.5) / ((POINTS/2) as f32);
                    let u = if branch.pos[2] < 0.0 { u + 0.25 } else { u + 0.75 };
                    let u = u % 1.0;
                    let angle = u * 2.0 * PI;
                    let pos = Vector4::new(angle.cos(), 0.0, angle.sin(), 1.0);
                    let pos = matrix * pos;
                    points_top.push(
                        PosNormTex {
                            position: [*pos.get(0).unwrap(), *pos.get(1).unwrap(), *pos.get(2).unwrap()].into(),
                            normal: [ angle.cos(), 0.0, angle.sin() ].into(),
                            tex_coord: [0.0, 0.0].into(),
                        });
                }
            }

            for i in 0..POINTS {
                let i0 = i;
                let i1 = (i+1) % POINTS;
                points.push(points_bot[i0]);
                points.push(points_top[i0]);
                points.push(points_bot[i1]);

                points.push(points_top[i0]);
                points.push(points_top[i1]);
                points.push(points_bot[i1]);
            }

            if depth > 1 {
                let mut sub_points = Vec::new();
                self.make_tree_points(&mut sub_points, depth-1);
                for transform in &sub_transforms {
                    let mut twist = Transform::default();
                    let rot =  UnitQuaternion::from_axis_angle(&Unit::new_normalize(Vector3::new(0.0, 1.0, 0.0)), PI/2.0);
                    twist.set_rotation(rot);
                    let twist_mat = twist.matrix();
                    for PosNormTex { position, normal, tex_coord } in &sub_points {
                        let pos = Vector4::new(*position.get(0).unwrap(), *position.get(1).unwrap(), *position.get(2).unwrap(), 1.0);
                        let pos = transform * twist_mat * pos;

                        let normal = Vector4::new(*normal.get(0).unwrap(), *normal.get(1).unwrap(), *normal.get(2).unwrap(), 1.0);
                        let normal = transform * twist_mat * normal;

                        points.push(PosNormTex {
                            position: pos.fixed_rows::<U3>(0).into(),
                            normal: normal.fixed_rows::<U3>(0).into(),
                            tex_coord: *tex_coord
                        });
                    }
                }
            }
        }
    }

    fn make_tree_mesh(&mut self, world: &mut World, depth: usize) -> Handle<Mesh> {
        let mut points = Vec::new();
        self.make_tree_points(&mut points, depth);

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
        let thing_mesh = self.make_cylinder(world);

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
    fn make_tree2(&mut self, world: &mut World) {
        let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();

        let mut thing_pos = Transform::default();
        thing_pos.set_scale(1.0, 1.0, 1.0);
        //thing_pos.face_towards([0.0, -10.0, 0.0].into(), [0.0, 0.0, 1.0].into());
        let thing_mesh = self.make_tree_mesh(world, 8);

        let thing_albedo = world.exec(|loader: AssetLoaderSystemData<'_, Texture>| {
            loader.load_from_data([0.4, 0.4, 0.0, 1.0].into(), ())
        });

        let thing_material = Material {
            albedo: thing_albedo,
            ..material_defaults.clone()
        };
        self.thing = Some(
           world.create_entity()
                .with(thing_pos)
                .with(thing_mesh)
                .with(thing_material)
                .build());
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
        //self.make_tree(world);
        self.make_tree2(world);

        // Make the camera
        world
            .create_entity()
            .with(Camera::from(Projection::perspective(1.3, 1.0471975512)))
            .with(camera_trans)
            .with(ArcBallControlTag {
                target: self.thing.unwrap(),
                distance: 16.0,
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
            .with(light_transform.clone())
            .build();
        light_transform.set_z(4.0);
        let light_point2 = PointLight {
            color: [0.3, 1.0, 0.3, 1.0].into(),
            intensity: 130.0,
            radius: 5.0,
            ..PointLight::default()
        };
        world
            .create_entity()
            .with(Light::Point(light_point2))
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
