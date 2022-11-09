extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use stop_words;
use super::GameState;

pub struct RoverPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RoverState {
    Inactive,
    Listening,
    Thinking,
    Talking,
}

#[derive(Component)]
struct Rover;
#[derive(Component)]
struct chatMessage;
#[derive(Component)]
struct chatParent;

#[derive(Component)]
struct userInput;


#[derive(Component)]
struct roverSprite;

//RESOURCES

#[derive(Default)]
struct LastChat{
    name: String,
    val: String,
}




impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_startup_system(setup_rover)
            .add_loopless_state(RoverState::Listening)
            .add_enter_system(GameState::Rover, open_rover)
            .add_exit_system(GameState::Rover, close_rover)
            .add_system(
                text_input
                    .run_in_state(GameState::Rover)
                    .run_in_state(RoverState::Listening))
            .add_system(
                get_rover_response
                    .run_in_state(RoverState::Thinking))
            .add_enter_system(RoverState::Thinking, chat_update)
            .add_enter_system(RoverState::Talking, chat_update)
            .add_system(
                TEMP_move_to_talking
                    .run_in_state(RoverState::Talking))    
            .insert_resource(LastChat {name : "Rover:".to_string(),
                                        val : "eat shit".to_string()});
    }
}

fn setup_rover(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn()
    //     .insert(Rover)
    //   //  .insert(SpeechState::Talking);
}   


fn TEMP_move_to_talking(mut commands: Commands, mut msg: ResMut<LastChat>) {
    commands.insert_resource(NextState(RoverState::Listening));
    msg.val.clear();
}

fn chat_update(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    //tquery: Query<Entity, With<chatMessage>>,
    mut c_child: Query<&mut Text, (With<Parent>, With<chatMessage>)>,
    mut c_parent: Query<Entity, (With<chatParent>, With<Children>)>,
    sp: Res<LastChat>){
        let current_speaker = &sp.name;   
        let mut counter = 1.0;   
        for mut child_transform in c_child.iter_mut() {
            counter+=25.0;
        }
        let newmsg = commands.spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    current_speaker,
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    &sp.val,
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ]).with_style(Style {
                //position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(counter),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            }
        )).insert(chatMessage)
        .id();

        // for mut child_transform in c_child.iter_mut() {
        //     counter+=25.0;
        //     child_transform.sections[0]. position.bottom += counter;
        //     child_transform.sections[1].style.position.bottom += counter;
        //     // `parent` contains the Entity ID we can use
        //     // to query components from the parent:
        //     //let parent_ = q_parent.get(parent.get());
        //    // *child_transform = Transform::from_xyz(0.0,-25.0,1.0);
        //   // child_transform.translation.y += counter;
        //   // info!("transform now: {}", child_transform.translation.y);
        // }

        // for rtext in tquery.iter(){
        let big_parent = c_parent.single();
        commands.entity(big_parent).push_children(&[newmsg]);
        // rtext.sections[1].value = sp.val.to_string();
        //commands.entity(newmsg).  sections[1].value = sp.val;
        
   
}

fn open_rover(
    mut commands: Commands, 
    asset_server: Res<AssetServer>){
        let big_parent = commands.spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    "",
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ]).with_text_alignment(TextAlignment::TOP_CENTER)
        ).insert(userInput)
        .insert(chatParent)
        .id();


        let current_speaker = "Rover:";
        let first_line = commands.spawn_bundle(
            TextBundle::from_sections([
                TextSection::new(
                    current_speaker,
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    "eat shit".to_owned(),
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ]).with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(25.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            }
        )).insert(chatMessage)
        .id();   

        commands.entity(big_parent).push_children(&[first_line]);

        commands.spawn().insert_bundle(SpriteBundle{
            texture: asset_server.load("rover-1.png"),
            transform: Transform::from_xyz(-300.0,-325.0,1.0),
            ..default()
            }).insert(roverSprite);
    }

fn close_rover(mut commands: Commands, 
    query: Query<Entity, With<chatMessage>>,
    rov_query: Query<Entity, With<roverSprite>>,
    usr_input_query: Query<Entity, With<userInput>>){
        for rv in rov_query.iter() {
            commands.entity(rv).despawn();
        }
        
        for ent in query.iter() {
            commands.entity(ent).despawn_recursive();
            
        }
        for ent in usr_input_query.iter() {
            commands.entity(ent).despawn_recursive();
            
        }
}

fn text_input(
    mut commands: Commands, 
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut msg: ResMut<LastChat>,
    mut userText: Query<&mut Text, With<userInput>>,) {

        let mut userText = userText.single_mut();

        //let mut k_input :String = "".to_owned();

        for ev in char_evr.iter() {
            //info!(input);
            msg.val.push(ev.char);
            userText.sections[0].value = format!("{}{}", userText.sections[0].value, ev.char);
            
        }
        if keys.just_pressed(KeyCode::Return) {      
            info!("input now: {}", msg.val);
            userText.sections[0].value = format!("");
            //msg.val = format!("{}", input);
            msg.name = "root: ".to_owned();
            //.clear();
            commands.insert_resource(NextState(RoverState::Thinking));
        }
        

        if keys.just_pressed(KeyCode::Escape) {
           // commands.insert_resource(NextState(RoverState::Listening));
            commands.insert_resource(NextState(GameState::InGame));
            info!("escape pressed");
        }
}

fn get_rover_response(
    mut commands: Commands,
    mut msg: ResMut<LastChat>,){

    let tokens = parser(&(msg.val)[..]);
    let stemmed_tokens = stemmer(tokens);
    //println!("Stemmed parsed tokens:");
    // for str in stemmed_tokens.iter() {
    //     println!("{}", str);
    // }
    msg.val = "eat shit".to_owned();
    msg.name = "rover:".to_owned();
    commands.insert_resource(NextState(RoverState::Talking));     
}

fn parser(input: &str) ->Vec<String> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s.to_lowercase()); 
    }  
    strings
}

fn stemmer(mut strings: Vec<String>) ->Vec<String>  {
    let mut new_strings=Vec::new();
    let stopwords = stop_words::get("english");
    let en_stemmer = Stemmer::create(Algorithm::English);
    for s in strings{
         if stopwords.contains(&&s)==false{
            new_strings.push(en_stemmer.stem(&s).into_owned());
         }
    }
    new_strings
}

// // fn answerer(int: procVal){
// //     let mut tquery = Query<&mut Text, With<RoverText>>;

// //     for mut text in &mut tquery {
// //         text.sections[1].value = format!("eat shit and die");
// // }
// // }

