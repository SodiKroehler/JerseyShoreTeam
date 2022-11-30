use bevy::{prelude::*};
use crate::CONSTANTS;

use super::GameState;
use iyes_loopless::prelude::*;
use super::CONSTANTS::*;

pub struct UiPlugin;

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);

#[derive(Component)]
struct pause_screen;

#[derive(Component)]
struct start_button;

#[derive(Component)]
struct userInput;


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
      // .add_startup_system(setup_ui)
        .add_system (
            process_triggers
                .run_in_state(GameState::InGame))
        .add_startup_system(spawn_xp_ui_elems)
        .add_enter_system(GameState::Ending, setup_credits)
        .add_system (roll_credits.run_in_state(GameState::Ending))
        .add_system (handle_user_input_focus.run_in_state(GameState::InGame))
        .add_exit_system(GameState::Ending, exit_credits)
        .add_enter_system(GameState::Paused, enter_paused)
        .add_system(pause_button.run_in_state(GameState::Paused))
        .add_system(handle_start_button.run_in_state(GameState::InGame))
        .add_exit_system(GameState::Paused, exit_paused);
    }
}

fn spawn_xp_ui_elems(mut commands: Commands, asset_server: Res<AssetServer>){
    
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("taskbar.png"),
        transform: Transform::from_xyz(0.0, -346.0, 1.),
        ..default()
    });

    commands.spawn_bundle(ButtonBundle {
        style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(31.0)),
                // justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            color: START_GREEN.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        }).insert(start_button);
}

fn process_triggers(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
    }
    if kbd.just_pressed(KeyCode::B) {
        commands.insert_resource(NextState(GameState::Rover));
    }
    if kbd.just_pressed(KeyCode::C) {
        commands.insert_resource(NextState(GameState::Ending));
    }
}

fn setup_credits(mut commands: Commands,
                asset_server: Res<AssetServer>){
    let initial_offset: f32 = 640. + (1280.*3.);
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("credit-sheet.png"),
            transform: Transform::from_xyz(initial_offset, 0., 1.),
			..default()
		}).insert(CreditTimer(Timer::from_seconds(5., true)));
}

fn roll_credits(
	time: Res<Time>,
    mut commands: Commands,
	mut popup: Query<(&mut CreditTimer, &mut Transform)>) {

    let counter = -4800.0;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.x -= 1280.;
            // if counter == transform.translation.x {
            if counter == -4800.0 {
                timer.pause();
                //exit.send(AppExit);
                commands.insert_resource(NextState(GameState::Paused));
            }
		}
	}
}

fn exit_credits(mut commands: Commands,
                qu: Query<Entity, With<CreditTimer>>) {

    for ent in qu.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

fn enter_paused(mut commands: Commands,
                asset_server: Res<AssetServer>){

    commands.spawn_bundle(NodeBundle{
        transform: Transform::from_xyz(0.0,0.0,14.0),
        style: Style {
            size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
            ..default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.60).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            transform: Transform::from_xyz(0.0,0.0,15.0),
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: XP_BLUE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Play!",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
    }).insert(pause_screen);

}

fn pause_button(mut commands: Commands,
                mut inter_query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>, 
                                Without<start_button>)>){

    for (interaction, mut color) in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                commands.insert_resource(NextState(GameState::InGame));
            }
            Interaction::Hovered => {
               // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = XP_BLUE.into();
            }
        }
    }
}

fn exit_paused(mut commands: Commands,
                q : Query<Entity, With<pause_screen>>){
    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

fn handle_start_button(mut commands: Commands,
    mut inter_query: Query<&Interaction,
                    (Changed<Interaction>, With<Button>, 
                    With<start_button>)>){

    for interaction in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::Rover));
            }
            Interaction::Hovered => {
            // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
              //  *color = XP_BLUE.into();
            }
        }
    }
}

fn handle_user_input_focus(mut commands: Commands,
    mut inter_query: Query<&Interaction,
                    (Changed<Interaction>, With<Button>, 
                    With<userInput>)>){
    for interaction in &mut inter_query {
       info!("{:?}", interaction);
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::Rover));
            }
            Interaction::Hovered => {
                // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                //  *color = XP_BLUE.into();
            }
        }
    }
}

pub fn spawn_blue_screen_of_death(mut commands: Commands,
                                    asset_server: Res<AssetServer>){
    info!("death to america and to butter sauce");

    commands.spawn_bundle(NodeBundle{
        transform: Transform::from_xyz(0.0,0.0,20.0),
        style: Style {
            size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
            ..default()
        },
        color: CONSTANTS::XP_BLUE.into(),
        ..default()
    }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "A problem has been detected and the game has been stopped
                to prevent damage to your computer.  TOO_HIGH_TRUTH_VALUE_FOR_STATEMENT",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        });
}
