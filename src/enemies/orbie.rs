use bevy::prelude::*;
use heron::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody};

use crate::{assets::ModelAssets, Layer};

use super::{Enemy, EnemyBehaviour, EnemyLastFired};

#[derive(Component, Default)]
pub struct OrbieEnemy;

impl EnemyBehaviour for OrbieEnemy {
    fn spawn(commands: &mut Commands, transform: Transform, model_assets: &ModelAssets) -> Entity {
        commands
            .spawn_bundle((transform, GlobalTransform::default()))
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Sphere { radius: 2.7 })
            .insert(CollisionLayers::all::<Layer>().with_group(Layer::Enemy))
            .insert(PhysicMaterial {
                density: 0.0,
                ..Default::default()
            })
            .insert(EnemyLastFired(Timer::from_seconds(0.8, true)))
            .insert(Enemy)
            .insert(OrbieEnemy)
            .with_children(|parent| {
                parent.spawn_scene(model_assets.unit2.clone());
            })
            .id()

        // -----------------
        // --- UNIT TEST ---
        // -----------------

        // let unit1 = model_assets.unit1.clone();
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(0.0, 18.0, -100.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit1.clone());
        //     });
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(5.0, 18.0, -100.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit1.clone());
        //     });
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(-5.0, 18.0, -100.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit1.clone());
        //     });

        // let unit2 = model_assets.unit2.clone();
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(0.0, 22.0, -120.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit2.clone());
        //     });
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(8.0, 22.0, -120.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit2.clone());
        //     });
        // commands
        //     .spawn_bundle((
        //         Transform::from_xyz(-8.0, 22.0, -120.0),
        //         GlobalTransform::identity(),
        //     ))
        //     .with_children(|parent| {
        //         parent.spawn_scene(unit2.clone());
        //     });
    }
}
