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
struct Fanfic;

#[derive(Component)]
struct FidoTheDog;

#[derive(Component)]
struct Documents;


#[derive(Component)]
struct FilePreview;


impl Plugin for ExtraFoldersPlugin {
    fn build(&self, app: &mut App) {
       app
            .add_startup_system(_set_up)
           .add_enter_system(GameState::Email, open_email) 
           .add_exit_system(GameState::Email, close_email) 
           .add_enter_system(GameState::Folder, open_docs) 
           .add_exit_system(GameState::Folder, close_docs) 
           .add_system(handle_email_password.run_in_state(GameState::Email))                                     
           .add_system(handle_close_button.run_in_state(GameState::Folder))                                      
           .add_system(handle_close_button.run_in_state(GameState::Email))                                      
           .add_system(open_photo.run_in_state(GameState::Folder))                                      
           .add_system(open_fanfic.run_in_state(GameState::Folder))                                      
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
                    font: asset_server.load(CONSTANTS::FONT_FILE),
                    font_size: 20.0,
                    color: CONSTANTS::BACKGROUND,
                },
            ));
        }).insert(Email);

        // portal title
         commands.spawn_bundle(NodeBundle{
            transform: Transform::from_xyz(0.,0., z_offset + 2.0),
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(489.0), 
                    bottom: Val::Px(425.0),
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
                    font: asset_server.load(CONSTANTS::FONT_FILE),
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
                        font: asset_server.load(CONSTANTS::FONT_FILE),
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
                        font: asset_server.load(CONSTANTS::FONT_FILE),
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
        transform: Transform::from_xyz(0.0, 4.0, z_offset+1.0),
        sprite: Sprite{
            custom_size: Some(Vec2::new(900.0 - 50.0, 718.0-50.0)),
            ..default()
        },
        ..default()
    }).insert(Documents);

      //close button
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0., z_offset + 3.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: Val::Px(215.0),
                top: Val::Px(20.0),
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
                left: Val::Px(225.0),
                top: Val::Px(30.0),
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
                font: asset_server.load(CONSTANTS::FONT_FILE),
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
                left: Val::Px(235.0), 
                top: Val::Px(79.0), 
                ..default()
            },
            ..default()
        },
        color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            r"C:\\Users\\Bratwurst III\\My Documents",
            TextStyle {
                font: asset_server.load(CONSTANTS::FONT_FILE),
                font_size: 20.0,
                color: Color::BLACK,
            },
        ));
    }).insert(Documents);

     //fanfic button
     commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0.,  z_offset+5.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(320.0),
                bottom: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("file_icon.png")),
        ..default()
    }).insert(Fanfic).insert(Documents);

    //fanfic button description
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0.,  z_offset+5.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(260.0),
                bottom: Val::Px(450.0),
                ..default()
            },
            ..default()
        },
        color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "my_super_secret_document.txt",
            TextStyle {
                font: asset_server.load(CONSTANTS::FONT_FILE),
                font_size: 20.0,
                color: Color::BLACK,
            },
        ));
    }).insert(Documents);

    //  //audio button
    //  commands.spawn_bundle(ButtonBundle {
    //     transform: Transform::from_xyz(0.,0., CONSTANTS::Z_UI+1.0),
    //     style: Style {
    //         size: Size::new(Val::Px(40.0), Val::Px(40.0)),
    //         position_type: PositionType::Absolute,
    //         position: UiRect {
    //             left: Val::Px(400.0),
    //             bottom: Val::Px(500.0),
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     image:UiImage(asset_server.load("music_icon.png")),
    //     ..default()
    // }).insert(FileButton::Audio).insert(Documents);

    //open fido button
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0.,  z_offset+5.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(480.0),
                bottom: Val::Px(500.0),
                ..default()
            },
            ..default()
        },
        image:UiImage(asset_server.load("file_icon.png")),
        ..default()
    }).insert(FidoTheDog).insert(Documents);

    //fido button description
    commands.spawn_bundle(ButtonBundle {
        transform: Transform::from_xyz(0.,0.,  z_offset+5.0),
        style: Style {
            size: Size::new(Val::Px(40.0), Val::Px(40.0)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(480.0),
                bottom: Val::Px(450.0),
                ..default()
            },
            ..default()
        },
        color: bevy::prelude::UiColor(Color::rgba(0.0,0.0,0.0,0.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle::from_section(
            "babyfido.png",
            TextStyle {
                font: asset_server.load(CONSTANTS::FONT_FILE),
                font_size: 20.0,
                color: Color::BLACK,
            },
        ));
    }).insert(Documents);
}

fn close_docs(mut commands: Commands,
    q: Query<Entity, With<Documents>>){

    for ent in q.iter() {
        commands.entity(ent).despawn_recursive();    
    }
}
fn open_photo(mut commands: Commands,  
    mut inter_query: Query<&Interaction,
    (Changed<Interaction>, With<FidoTheDog>)>,
    asset_server: Res<AssetServer>,
    q : Query<Entity, (With<Fanfic>, With<FilePreview>)>){

    for interaction in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
            // info!("{:?}", ent.get_type_info());
            commands.spawn_bundle(SpriteBundle {
                    texture: asset_server.load("fidoTheDog.png"),
                    transform: Transform::from_xyz(200.0, -50.0, z_offset+5.0),
                    sprite: Sprite{
                        custom_size: Some(Vec2::new(350.0,350.0)),
                        ..default()
                    },
                    ..default()
                }).insert(Documents)
                    .insert(FidoTheDog)
                    .insert(FilePreview);  

                for ent in q.iter() {
                    commands.entity(ent).despawn_recursive();    
                }
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

fn open_fanfic(mut commands: Commands,  
    mut inter_query: Query<&Interaction,
    (Changed<Interaction>, With<Fanfic>)>,
    asset_server: Res<AssetServer>,
    q : Query<Entity, (With<FidoTheDog>, With<FilePreview>)>){

    for interaction in &mut inter_query {
        match *interaction {
            Interaction::Clicked => {
                commands.spawn_bundle(
                    NodeBundle{ 
                        transform: Transform::from_xyz(0.0, 0.0, z_offset+5.0),
                        color: CONSTANTS::uicolor(CONSTANTS::BACKGROUND),
                        style: Style {
                            // size: Size::new(Val::Px(100.0), Val::Px(200.0)),
                            position_type: PositionType::Absolute,
                            justify_content: JustifyContent::FlexStart,
                            overflow: Overflow::Visible,
                            flex_wrap: FlexWrap::WrapReverse,
                            position: UiRect {
                                bottom: Val::Px(100.0),
                                right: Val::Px(250.0),
                                left: Val::Px(655.0),
                                top: Val::Px(150.0),
                            },
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent|{ 
                        parent.spawn_bundle(
                            TextBundle::from_sections([
                                TextSection::new(
                                    "A single household, without much dignity, 
in fair Seaside Heights, New Jersey, 
where we lay our scene.",
                                    TextStyle {
                                        font: asset_server.load(CONSTANTS::FONT_FILE),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                                TextSection::new(
                                    "
 Where ancient grudges were formed by the minute, 
and feuds broke across the eight who lived there.",
                                    TextStyle {
                                        font: asset_server.load(CONSTANTS::FONT_FILE),
                                        font_size: 20.0,
                                        color: Color::BLACK,
                                    },
                                ),
                            ]).with_text_alignment(TextAlignment::TOP_LEFT))
                            .insert(Documents)
                            .insert(Fanfic)
                            .insert(FilePreview);
                        }).insert(Documents)
                            .insert(Fanfic)
                            .insert(FilePreview);
                for ent in q.iter() {
                    commands.entity(ent).despawn_recursive();    
                }
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