use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::controller::Controller;

mod controller;

#[derive(Component)]
struct Cart;

#[derive(Component)]
struct Pendulum;

// Simulation Parameters
const PENDULUM_MASS: f32 = 5.0;
const PENDULUM_LENGTH: f32 = 0.5;
const CART_DAMPING: f32 = 0.5;
const CART_MASS: f32 = 20.0;

const GRAVITY_SCALE: f32 = 1.0;
const PIXELS_PER_METER: f32 = 100.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            PIXELS_PER_METER,
        ))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, step)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ground
    commands
        .spawn(Collider::cuboid(500.0, 10.0))
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1000.0, 20.0))),
            material: materials.add(Color::rgb(1.0, 1.0, 0.0)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -150.0, 0.0)))
        .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_2));

    // Cart
    let cart = commands
        .spawn(Cart)
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(GRAVITY_SCALE))
        // Meshes
        .with_children(|children| {
            // Cart body
            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 25.0))),
                    material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

            // Wheels
            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                    material: materials.add(Color::rgb(1.0, 0.0, 1.0)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(
                    -20.0, -19.0, 0.0,
                )));
            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(5.0))),
                    material: materials.add(Color::rgb(1.0, 0.0, 1.0)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(20.0, -19.0, 0.0)));
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(25.0, 19.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -10.0, 0.0)))
                .insert(ColliderMassProperties::Mass(CART_MASS))
                .insert(CollisionGroups::new(Group::GROUP_2, Group::GROUP_2));
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -4.0, -100.0, 0.0,
        )))
        .insert(ExternalForce {
            force: Vec2::ZERO,
            torque: 0.0,
        })
        // Cart friction
        .insert(Damping {
            linear_damping: CART_DAMPING,
            angular_damping: 0.0,
        })
        .id();

    // Pendulum Arm
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0));

    commands
        .spawn(Pendulum)
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(GRAVITY_SCALE))
        // Meshes
        .with_children(|children| {
            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                        // Multiplied by two to convert from Mesh to Collider distances
                        PENDULUM_LENGTH * PIXELS_PER_METER * 2.0,
                        10.0,
                    ))),
                    material: materials.add(Color::rgb(0.0, 0.0, 1.0)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));

            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Circle::new(20.0))),
                    material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
                    ..default()
                })
                .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)));
        })
        // Colliders
        .with_children(|children| {
            children
                .spawn(Collider::ball(20.0))
                .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)))
                .insert(CollisionGroups::new(Group::GROUP_3, Group::GROUP_3))
                .insert(ColliderMassProperties::Mass(PENDULUM_MASS));
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -10.0, -12.5, 0.0,
        )))
        .insert(ImpulseJoint::new(cart, joint));
}

fn step(
    mut cart_force: Query<&mut ExternalForce, With<Cart>>,
    mut body_sleep: Query<&mut Sleeping, With<RigidBody>>,
    pendulum_pos: Query<&Transform, With<Pendulum>>,
    cart_pos: Query<&Transform, With<Cart>>,
) {
    // Ensure no body sleeps
    body_sleep.iter_mut().for_each(|mut body| {
        body.sleeping = false;
    });

    // Get pendulum state
    let position = pendulum_pos.get_single().unwrap();

    // Add offset to angle to make pendulum up 0 deg
    // TODO: Figure out why there is an error in the angle
    let theta = f32::to_degrees(position.rotation.z) + 139.429924;

    // Get cart state
    let position = cart_pos.get_single().unwrap();
    let x_pos = position.translation.x;

    println!("State -> theta: {} deg, x: {} m", &theta, &x_pos);

    // Get cart actuator
    let mut actuator = cart_force.get_single_mut().unwrap();

    let controller = controller::DumbController::from_target(0.0, 0.0, 100.0);
    actuator.force = Vec2::new(controller.get_force(x_pos, theta), 0.0)
}
