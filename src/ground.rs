use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct GroundPlugin;

#[derive(Component)]
pub struct Ground;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn(SpriteBundle {
        texture: textures.texture_ground.clone(),
        transform: Transform::from_translation(Vec3::new(0.,0.,1.0)),
        ..Default::default()
    })
    .insert(Ground);
}