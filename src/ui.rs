use bevy::{prelude::*};
use super::GameState;
use iyes_loopless::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
      // .add_startup_system(setup_ui)
        .add_system(
            processTriggers
                .run_in_state(GameState::InGame))
        .add_system(processTriggers);
    }
}

fn processTriggers(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
    }
    if kbd.just_pressed(KeyCode::B) {
        commands.insert_resource(NextState(GameState::Rover));
    }
}