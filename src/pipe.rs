use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PipePlugin;

#[derive(Component)]
pub struct Pipe;

/// This plugin handles Pipe related stuff like movement
/// Pipe logic is only active during the State `GameState::Playing`
impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pipe.run_if(in_state(GameState::Playing)))
            .add_systems(Update, move_pipe.run_if(in_state(GameState::Playing)));
    }
}

fn spawn_pipe(time: Res<Time>, mut commands: Commands, textures: Res<TextureAssets>) {
    let elapsed = time.elapsed_seconds_wrapped();
    if elapsed > 2. && elapsed % 1.5 == 0. {
        println!("Adding pipe");
        commands
            .spawn(SpriteBundle {
                texture: textures.texture_pipe.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.))
                    .with_scale(Vec3::new(0.75, 1., 1.)),
                ..Default::default()
            })
            .insert(Pipe);
    }
}

fn move_pipe(time: Res<Time>, mut pipe_query: Query<&mut Transform, With<Pipe>>) {}
