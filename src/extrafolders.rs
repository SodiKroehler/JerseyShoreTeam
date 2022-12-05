use bevy::prelude::*;
use iyes_loopless::prelude::*;


use super::GameState;
use super::CONSTANTS;
use super::deflections::Password;
// use super::DeflectionsPlugin;

const z_offset:f32 = CONSTANTS::Z_EXTRAFOLDER;

pub struct ExtraFoldersPlugin;

#[derive(Component)]
struct EmailButton;

#[derive(Component)]
struct EmailText;

#[derive(Component)]
struct Email;

#[derive(Component)]
struct CloseButton;

#[derive(Component)]
enum FileButton {
    Audio,
    Text,
    Picture,
}

#[derive(Component)]
struct Documents;


impl Plugin for ExtraFoldersPlugin {
    fn build(&self, app: &mut App) {
       app
            .add_startup_system(_set_up)
           .add_enter_system(GameState::Email, open_email) 
           .add_exit_system(GameState::Email, close_email) 
           .add_enter_system(GameState::Folder, open_docs) 
           .add_exit_system(GameState::Folder, close_docs) 
           .add_system(handle_email_password.run_in_state(GameState::Email))                                     
           .add_system(handle_close_button
                            .run_in_state(GameState::Email)                                      
                            .run_in_state(GameState::Folder))                                      
           .add_system(handle_close_button.run_in_state(GameState::Email))                                      
           .add_system(handle_email_button.run_in_state(GameState::InGame));                                      
    }
}

fn _set_up(mut commands: Commands, 
            asset_server: Res<AssetServer>){

    //email icon/button
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., CONSTANTS::Z_UI+1.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(340.0),
                bottom: Val::Px(-5.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("email_logo.png")),
        ..default()
    }).insert(EmailButton);

//     commands.spawn_bundle(SpriteBundle {
//         texture: asset_server.load("email_logo.png"),
//         transform: Transform::from_xyz(-290.0, -346.0, CONSTANTS::Z_UI+1.0),
//         ..default()
//     }).insert(EmailButton);
}

fn open_email(mut commands: Commands, 
    asset_server: Res<AssetServer>){

        commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("dialog.png"),
            transform: Transform::from_xyz(0.0, 0.0, z_offset),
            ..default()
        }).insert(Email);
        
        //close button
        commands.spawn_bundle(ButtonBundle {
            transform: Transform::from_xyz(0.,0., z_offset + 3.0),
            style: Style {
                size: Size::new(Val::Px(40.0), Val::Px(40.0)),
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(460.0),
                    top: Val::Px(260.0),
                    ..default()
                },
                ..default()
            },
            image:UiImage(asset_server.load("close_icon.png")),
            ..default()
        }).insert(CloseButton)
        .insert(Email);

        //submit button
        commands.spawn_bundle(ButtonBundle {
            transform: Transform::from_xyz(0.,0., z_offset + 3.0),
            style: Style {
                size: Size::new(Val::Px(100.0), Val::Px(35.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    right: Val::Px(468.0),
                    bottom: Val::Px(265.0),
                    ..default()
                },
                ..default()
            },
            color: CONSTANTS::XP_BLUE.into(),
            ..default()
        }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Enter",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: CONSTANTS::c(String::from("FCF8ED")),
                },
            ));
        }).insert(Email);

        // portal title
         commands.spawn_bundle(NodeBundle{
            transform: Transform::from_xyz(0.,0., z_offset + 2.0),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(489.0), //400 too left
                    bottom: Val::Px(425.0), //415 too low
                    ..default()
                },
                ..default()
            },
            color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Jersey Shore Super Email",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: CONSTANTS::BACKGROUND,
                },
            ));
        }).insert(Email);

        //prompt text
        commands.spawn_bundle(NodeBundle{
            transform: Transform::from_xyz(0.,0., z_offset + 2.0),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(489.0),
                    bottom: Val::Px(370.0),
                    ..default()
                },
                ..default()
            },
            color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
            ..default()
        }).with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    // "C::/UsersBratwurstIII/Documents",
                    String::from(r"Please enter your password\:"),
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                ));
            }).insert(Email);

        //actual text entry
        commands.spawn_bundle(NodeBundle{
            transform: Transform::from_xyz(0.,0., z_offset + 1.0),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(489.0),
                    bottom: Val::Px(325.0), //300 too low 340 too high
                    ..default()
                },
                ..default()
            },
            color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
            ..default()
        }).with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "",
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::BLACK,
                    },
                )).insert(EmailText);
            }).insert(Email);

}

fn close_email(mut commands: Commands,
    q : Query<Entity, With<Email>>){

    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}

fn handle_email_password( mut commands: Commands, 
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut current_password: ResMut<Password>,
    p_text: Query<Entity, With<EmailText>>,
    mut text_query: Query<&mut Text>,){

    let p_node = p_text.single();
    let mut pass_text = text_query.get_mut(p_node).unwrap();


    //let mut k_input :String = "".to_owned();

    for ev in char_evr.iter() {
        if ev.char != '\u{8}' {pass_text.sections[0].value.push(ev.char)};
    }

    if keys.just_pressed(KeyCode::Return) {      
        // pass_text.sections[0].value = format!("");
        pass_text.sections[0].value.pop();
        if pass_text.sections[0].value == current_password.val {
            commands.insert_resource(NextState(GameState::Ending));
        }
    }

    if keys.just_pressed(KeyCode::Back) {  
        pass_text.sections[0].value.pop();
    }

    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(GameState::InGame));
    }

}

fn handle_close_button(mut commands: Commands,
    mut inter_query: Query<&Interaction,
                    (Changed<Interaction>, With<CloseButton>)>,){

    for interaction in &mut inter_query {
        info!("close clicked");
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::InGame));
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


fn handle_email_button(mut commands: Commands,
    mut inter_query: Query<&Interaction,
                    (Changed<Interaction>, With<EmailButton>)>){

    for interaction in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                commands.insert_resource(NextState(GameState::Email));
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

fn open_docs(mut commands: Commands, asset_server: Res<AssetServer>){
    //portal frame
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("window.png"),
        transform: Transform::from_xyz(0.0, 0.0, z_offset+1.0),
        sprite: Sprite{
            custom_size: Some(Vec2::new(900.0 - 50.0, 718.0-50.0)),
            ..default()
        },
        ..default()
    }).insert(EmailButton);

      //close button
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., z_offset + 3.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(100.0),
                top: Val::Px(0.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("close_icon.png")),
        ..default()
    }).insert(CloseButton)
        .insert(Documents);

    // portal title
    commands.spawn_bundle(NodeBundle{
        transform: Transform::from_xyz(0.,0., z_offset + 2.0),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(100.0), //400 too left
                top: Val::Px(1.0), //415 too low
                ..default()
            },
            ..default()
        },
        color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "My Documents",
            TextStyle {
                font: asset_server.load("Jersey.ttf"),
                font_size: 20.0,
                color: Color::BLACK,
                // color: CONSTANTS::BACKGROUND,
            },
        ));
    }).insert(Documents);

    //address bar
    commands.spawn_bundle(NodeBundle{
        transform: Transform::from_xyz(0.,0., z_offset + 2.0),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(150.0), 
                top: Val::Px(25.0), 
                ..default()
            },
            ..default()
        },
        color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            String::from("C:\\Users\\Bratwurst III\\My Documents"),
            TextStyle {
                font: asset_server.load("Jersey.ttf"),
                font_size: 20.0,
                color: Color::BLACK,
            },
        ));
    }).insert(Documents);

     //fanfic button
     commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., CONSTANTS::Z_UI+1.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(340.0),
                bottom: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("file_icon.png")),
        ..default()
    }).insert(FileButton::Text).insert(Documents);

     //audio button
     commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., CONSTANTS::Z_UI+1.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(400.0),
                bottom: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("music_icon.png")),
        ..default()
    }).insert(FileButton::Audio).insert(Documents);

    //open fido button
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., CONSTANTS::Z_UI+1.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(460.0),
                bottom: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("file_icon.png")),
        ..default()
    }).insert(FileButton::Picture).insert(Documents);
}

fn close_docs(mut commands: Commands,
    q : Query<Entity, With<Documents>>){

    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}