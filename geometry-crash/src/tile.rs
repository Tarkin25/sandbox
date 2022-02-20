use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    #[bundle]
    sprite: SpriteBundle,
    rigid_body_position_sync: RigidBodyPositionSync,
}

impl TileBundle {
    pub fn new(width: f32, height: f32, position: Vec2) -> Self {
        let rigid_body = RigidBodyBundle {
            position: position.into(),
            body_type: RigidBodyType::Static.into(),
            ..Default::default()
        };

        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(width / 2., height / 2.).into(),
            ..Default::default()
        };

        let sprite = SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(width, height)),
                color: Color::rgb(0., 0., 1.),
                ..Default::default()
            },
            ..Default::default()
        };

        Self {
            rigid_body,
            collider,
            sprite,
            rigid_body_position_sync: RigidBodyPositionSync::Discrete,
        }
    }
}