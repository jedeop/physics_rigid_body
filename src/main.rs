use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

const WINDOW_SIZE: f32 = 700.;
const COUNT: usize = 5;

#[derive(Component)]
struct RigidBody {
    mass: f32,
    velocity: Vec3,
}

fn startup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();
    for _ in 0..COUNT {
        let mass = rng.gen_range(10.0..50.0);
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(mass).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform::from_translation(Vec3::new(
                    rng.gen_range(-200..=200) as f32,
                    rng.gen_range(-200..=200) as f32,
                    0.,
                )),
                ..default()
            },
            RigidBody {
                mass,
                velocity: Vec3 {
                    x: rng.gen_range(-100.0..=100.0) as f32,
                    y: rng.gen_range(-300.0..=300.0) as f32,
                    // y: 0.,
                    z: 0.,
                },
            },
        ));
    }
}

fn velocity(mut query: Query<(&RigidBody, &mut Transform)>, time: Res<Time>) {
    for (rigid_body, mut transform) in &mut query {
        transform.translation += rigid_body.velocity * time.delta_seconds();
    }
}

fn collide(mut query: Query<(&mut RigidBody, &mut Transform)>, time: Res<Time>) {
    let mut iter = query.iter_combinations_mut();
    while let Some([mut a1, mut a2]) = iter.fetch_next() {
        let distance = (a1.1.translation + a1.0.velocity * time.delta_seconds())
            .distance(a2.1.translation + a2.0.velocity * time.delta_seconds());
        if distance <= a1.0.mass + a2.0.mass {
            let normal = (a2.1.translation - a1.1.translation).normalize();

            let normal_a1 = a1.0.velocity.dot(normal) * normal;
            let normal_a2 = a2.0.velocity.dot(normal) * normal;
            let tangent_a1 = a1.0.velocity - normal_a1;
            let tangent_a2 = a2.0.velocity - normal_a2;

            let new_normal_a1 = ((2. * a2.0.mass * normal_a2)
                + normal_a1 * (a1.0.mass - a2.0.mass))
                / (a1.0.mass + a2.0.mass);
            let new_normal_a2 = ((2. * a1.0.mass * normal_a1)
                + normal_a2 * (a2.0.mass - a1.0.mass))
                / (a1.0.mass + a2.0.mass);

            a1.0.velocity = tangent_a1 + new_normal_a1;
            a2.0.velocity = tangent_a2 + new_normal_a2;
        }
    }
}
fn collide_wall(mut query: Query<(&mut RigidBody, &Transform)>, time: Res<Time>) {
    for (mut rigid_body, transform) in &mut query {
        let pos = transform.translation + rigid_body.velocity * time.delta_seconds();
        if pos.x < -WINDOW_SIZE / 2. + rigid_body.mass || pos.x > WINDOW_SIZE / 2. - rigid_body.mass
        {
            rigid_body.velocity.x *= -1.;
        }
        if pos.y < -WINDOW_SIZE / 2. + rigid_body.mass || pos.y > WINDOW_SIZE / 2. - rigid_body.mass
        {
            rigid_body.velocity.y *= -1.;
        }
    }
}

fn gravity(mut query: Query<&mut RigidBody>, time: Res<Time>) {
    for mut rigid_body in &mut query {
        rigid_body.velocity.y -= 98.0 * 3.0 * time.delta_seconds();
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "강체 물리 엔진".to_string(),
                width: WINDOW_SIZE,
                height: WINDOW_SIZE,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(startup_system)
        .add_system(velocity)
        .add_system(collide)
        .add_system(collide_wall)
        .add_system(gravity)
        .run();
}
