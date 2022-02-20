use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)
            .add_system(reset_player)
            .add_system(player_jump)
            .add_system(detect_airborne)
            // .add_system(detect_floor_collision)
            // .add_system(move_player)
        ;
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    jump_component: JumpComponent,
    #[bundle]
    sprite_bundle: SpriteBundle,
    #[bundle]
    rigid_body: RigidBodyBundle,
    #[bundle]
    collider: ColliderBundle,
    rigid_body_position_sync: RigidBodyPositionSync,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        let sprite_bundle = SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        };

        let rigid_body = RigidBodyBundle {
            mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
            activation: RigidBodyActivation::cannot_sleep().into(),
            ccd: RigidBodyCcd { ccd_enabled: true, ..Default::default()}.into(),
            position: Vec2::new(0.0, 0.0).into(),
            ..Default::default()
        };

        let collider = ColliderBundle {
            shape: ColliderShape::cuboid(0.5, 0.5).into(),
            flags: ColliderFlags {
                active_events: ActiveEvents::CONTACT_EVENTS,
                ..Default::default()
            }.into(),
            ..Default::default()
        };

        Self {
            player: Player::default(),
            jump_component: JumpComponent::default(),
            sprite_bundle,
            rigid_body,
            collider,
            rigid_body_position_sync: RigidBodyPositionSync::Discrete,
        }
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Component, Default, Debug)]
pub struct JumpComponent {
    airborne: bool,
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PlayerBundle::default());
}

fn reset_player(input: Res<Input<KeyCode>>, mut query: Query<&mut RigidBodyPositionComponent, With<Player>>) {

    let mut position_component = query.single_mut();

    if input.just_pressed(KeyCode::R) {
        position_component.position.translation = Vec2::new(0.0, 0.0).into();
    }
}

fn player_jump(input: Res<Input<KeyCode>>, mut query: Query<(&mut RigidBodyVelocityComponent, &mut JumpComponent), With<Player>>) {
    let (mut velocity, mut jump_component) = query.single_mut();

    if input.pressed(KeyCode::Space) && !jump_component.airborne {
        velocity.linvel = Vec2::new(0., 4.).into();
        jump_component.airborne = true;
    }
}

fn detect_airborne(mut query: Query<(Entity, &mut JumpComponent)>, mut contact_events: EventReader<ContactEvent>) {
    for contact_event in contact_events.iter() {
        for (entity, mut jump_component) in query.iter_mut() {
            if let ContactEvent::Started(handle_1, handle_2) = &contact_event {
                if handle_1.entity() == entity || handle_2.entity() == entity {
                    jump_component.airborne = false;
                }
            }
        }
    }
}
