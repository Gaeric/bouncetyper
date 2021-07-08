use crate::{game::rigid_body::RigidBody, utility::Damp};
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::ops::Add;

pub struct Player {
    pub speed_limit: f32,
    pub speed: f32,
    pub damp: f32,
}

pub fn player_movement(
    time: Res<Time>,
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut query: Query<(&Player, &mut RigidBody)>,
) {
    if let Ok((player, mut rigid_body)) = query.single_mut() {
        let delta: Vec2 = mouse_motion_event
            .iter()
            .map(|mouse_motion| Vec2::new(mouse_motion.delta.x, -mouse_motion.delta.y))
            .fold(Vec2::ZERO, Vec2::add);

        rigid_body.velocity = rigid_body.velocity.damp(
            (delta * player.speed/ time.delta_seconds()).clamp_length_max(player.speed_limit),
            player.damp,
            time.delta_seconds(),
        );
    }
}
