// use std::path::absolute;

use bevy::{prelude::*, app::AppExit};
use bevy::time::Stopwatch;
use super::GameState;
use iyes_loopless::prelude::*;
use super::CONSTANTS;


pub struct UiPlugin;

#[derive(Component, Deref, DerefMut)]
struct CreditTimer(Timer);

#[derive(Component)]
struct PauseScreen;

#[derive(Component)]
struct StartButton;

// #[derive(Component)]
// struct user_input_box;

#[derive(Component)]
struct GameDuration {
    time: Stopwatch,
}

#[derive(Component)]
struct Clock;

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
        .add_exit_system(GameState::Ending, exit_credits)
        .add_enter_system(GameState::Paused, enter_paused)
        .add_system(pause_button.run_in_state(GameState::Paused))
        .add_system(handle_start_button.run_in_state(GameState::InGame))
        // .insert_resource(GameDuration {time : 0.0})
        .add_system(tick_tock.run_not_in_state(GameState::Paused))
        .add_exit_system(GameState::Paused, exit_paused);
    }
}

fn spawn_xp_ui_elems(mut commands: Commands, asset_server: Res<AssetServer>){
    
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("taskbar.png"),
        transform: Transform::from_xyz(0.0, -346.0, CONSTANTS::Z_PAUSE),
        ..default()
    });
    
    //START BUTTON
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.0,0.0, CONSTANTS::Z_UI + 2.0),
        style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(31.0)),
                // justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            color: CONSTANTS::START_GREEN.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load(CONSTANTS::FONT_FILE),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ));
        }).insert(StartButton);

        //CLOCK
        commands.spawn_bundle(ButtonBundle {
            transform: Transform::from_xyz(0.0,0.0, CONSTANTS::Z_UI + 2.0),
            style: Style {
                    size: Size::new(Val::Px(100.0), Val::Px(31.0)),
                    // justify_content: JustifyContent::FlexStart,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position: UiRect {
                        right: Val::Px(1.0),
                        ..default()
                    },
                    ..default()
                },
                color: CONSTANTS::XP_BLUE.into(),
                ..default()
            }).with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "0.00",
                    TextStyle {
                        font: asset_server.load(CONSTANTS::FONT_FILE),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ));
            }).insert(Clock)
            .insert(GameDuration {time: Stopwatch::new()});
}

fn process_triggers(mut commands: Commands, kbd: Res<Input<KeyCode>>) {
    if kbd.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::Paused));
    }
    if kbd.just_pressed(KeyCode::T) {
        commands.insert_resource(NextState(GameState::Rover));
    }
    // if kbd.just_pressed(KeyCode::C) {
    //     commands.insert_resource(NextState(GameState::Ending));
    // }
}

fn setup_credits(mut commands: Commands,
                asset_server: Res<AssetServer>){
    let initial_offset: f32 = 640. + (1280.*3.);
    commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("credit-sheet.png"),
            transform: Transform::from_xyz(initial_offset, 0., 10.),
			..default()
		}).insert(CreditTimer(Timer::from_seconds(5., true)));
}

fn roll_credits(
	time: Res<Time>,
    mut commands: Commands,
	mut popup: Query<(&mut CreditTimer, &mut Transform)>,
    mut exit: EventWriter<AppExit>) {

    let counter = -4800.0;
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.x -= 1280.;
            if counter == transform.translation.x {
            // if counter == -4800.0 {
                timer.pause();
                exit.send(AppExit);
                // commands.insert_resource(NextState(GameState::Paused));
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
        transform: Transform::from_xyz(0.0,0.0, CONSTANTS::Z_PAUSE),
        style: Style {
            size: Size::new(Val::Px(1280.0), Val::Px(720.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(0.0),
                bottom: Val::Px(0.0),
                ..default()
            },
            ..default()
        },
        color: Color::rgba(0.0, 0.0, 0.0, 0.60).into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: CONSTANTS::XP_BLUE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Play!",
                TextStyle {
                    font: asset_server.load(CONSTANTS::FONT_FILE),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ));
        });
    }).insert(PauseScreen);

}

fn pause_button(mut commands: Commands,
                mut inter_query: Query<(&Interaction, &mut UiColor),
                                (Changed<Interaction>, With<Button>, 
                                Without<StartButton>)>){

    for (interaction, mut color) in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::InGame));
            }
            Interaction::Hovered => {
               // *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = CONSTANTS::XP_BLUE.into();
            }
        }
    }
}

fn exit_paused(mut commands: Commands,
                q : Query<Entity, With<PauseScreen>>){
    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

fn handle_start_button(mut commands: Commands,
    mut inter_query: Query<&Interaction,
                    (Changed<Interaction>, With<Button>, 
                    With<StartButton>)>){

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

// fn add_cursor_to_user_input(mut windows: ResMut<Windows>){

//     let window = windows.get_primary_mut().unwrap();
//     window.set_cursor_icon(CursorIcon::Text);
        // .add_system(add_cursor_to_user_input.run_in_state(GameState::Rover))


// }

pub fn spawn_blue_screen_of_death(mut commands: Commands,
                                    asset_server: Res<AssetServer>){
    info!("death to america and to butter sauce");

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("blue_screen_of_death.png"),
            transform: Transform::from_xyz(0.0, 0., CONSTANTS::Z_PANIC),
            ..default()
        });
}

fn tick_tock(
	time: Res<Time>,
    commands: Commands,
    mut q_clocktime: Query<&mut GameDuration>,
    clocktext: Query<(Entity, &Children), With<Clock>>,
    mut text_query: Query<&mut Text>,) {

    let mut total_game_time = q_clocktime.single_mut();

    let (ctext_node, ctext_kids) = clocktext.single();
    let mut ctext = text_query.get_mut(ctext_kids[0]).unwrap();
    let c_t: f64= total_game_time.time.elapsed_secs().into();
    ctext.sections[0].value = format!("{:.2}", c_t/60.0);
    //info!("{:?}", total_game_time.time.elapsed_secs());
    total_game_time.time.tick(time.delta());
}


