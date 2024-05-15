use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

mod controller;

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Pendulum;

// Simulation parameters
const MAX_PLATFORM_VEL: f32 = 1000.0; // Maximum absolute platform velocity
const BALL_DENSITY: f32 = 1.0; // Pendulum ball density
const AIR_RESISTANCE: f32 = 20.0; // Pendulum ball air resistance

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, step)
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Platform
    let platform = commands
        .spawn(Platform)
        .insert(RigidBody::KinematicVelocityBased)
        .insert(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(50.0, 25.0))),
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            ..default()
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)))
        .insert(Velocity::zero())
        .id();

    // Pendulum Arm
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 0.0))
        .local_anchor2(Vec2::new(50.0, 0.0));

    commands
        .spawn(Pendulum)
        .insert(RigidBody::Dynamic)
        // Meshes
        .with_children(|children| {
            children
                .spawn(MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(100.0, 10.0))),
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
                .spawn(Collider::cuboid(50.0, 5.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
                .insert(ColliderMassProperties::Density(0.0)); // The arm is massless
            children
                .spawn(Collider::ball(20.0))
                .insert(TransformBundle::from(Transform::from_xyz(-50.0, 0.0, 0.0)))
                .insert(ColliderMassProperties::Density(BALL_DENSITY));
        })
        // Air resistance
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: AIR_RESISTANCE,
        })
        .insert(TransformBundle::from(Transform::from_xyz(
            -10.0, -12.5, 0.0,
        )))
        .insert(ImpulseJoint::new(platform, joint));
}

fn step(
    mut platform_vel: Query<&mut Velocity, With<Platform>>,
    mut body_sleep: Query<&mut Sleeping, With<RigidBody>>,
    pendulum_pos: Query<&Transform, With<Pendulum>>,
) {
    // Ensure nothing sleeps
    body_sleep.iter_mut().for_each(|mut body| {
        body.sleeping = false;
    });

    // Get pendulum state
    let position = pendulum_pos.get_single().unwrap();

    // Add offset to angle to make pendulum down 0 deg
    // TODO Figure out why there is an error in the angle
    let theta = f32::to_degrees(position.rotation.z) - 45.0 + 4.538864;
    println!("Pendulum angle: {}", theta);

    // Control
    let mut velocity = platform_vel.get_single_mut().unwrap();
    velocity.linvel.x = controller::get_vel(theta);

    // Fixed max platform velocity
    if velocity.linvel.x.abs() > MAX_PLATFORM_VEL {
        velocity.linvel.x = MAX_PLATFORM_VEL * velocity.linvel.x.signum();
    }
}
