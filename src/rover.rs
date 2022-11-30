extern crate rust_stemmers;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::{OpenOptions};
use std::io::{self, prelude::*, Write, BufReader};
use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use stop_words;
use crate::CONSTANTS;

use super::GameState;
use super::maphs;


//stuff for egui
use bevy_inspector_egui::{InspectorPlugin, Inspectable};
#[derive(Inspectable, Default)]
struct Data {
    should_render: bool,
    text: String,
    #[inspectable(min = 42.0, max = 100.0)]
    size: f32,
}
use bevy_inspector_egui::WorldInspectorPlugin;
// end stuff for egui
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
struct roverSprite;

#[derive(Component)]
struct userInput;

//RESOURCES

#[derive(Default)]
struct Stage{
    val: isize,
}

#[derive(Default)]
struct LastChat{
    name: String,
    val: String,
}

#[derive(Serialize, Deserialize)]
struct Question{
    id: isize,
    question: String,
    answer: String,
    vector: Vec<f64>,
    priority: isize,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dict <T>{
    items: Vec<T>
}

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_rover)
            .add_loopless_state(RoverState::Listening)
            .add_enter_system(GameState::Rover, open_rover)
            .add_exit_system(GameState::Rover, close_rover)
            .add_system(
                text_input
                    .run_in_state(GameState::Rover)
                    .run_in_state(RoverState::Listening))
            .add_enter_system(RoverState::Thinking, get_rover_response)
            .add_enter_system(RoverState::Thinking, chat_update)
            .add_enter_system(RoverState::Talking, chat_update)
            //.add_plugin(WorldInspectorPlugin::new())
            .insert_resource(Stage {val : 0})
            .insert_resource(LastChat {name : "Rover:".to_string(),
                                        val : "".to_string()});  
                                        
    }
}

fn setup_rover(mut commands: Commands, asset_server: Res<AssetServer>) {

    commands.spawn_bundle(
        NodeBundle{
            // transform: Transform::from_xyz(.0,-350.0,2.0),
            color: Color::rgba(0.0, 0.0, 0.15, 0.45).into(),
            style: Style {
                size: Size::new(Val::Px(200.0), Val::Px(31.0)),
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(110.0),
                    ..default()
                },
                ..default()
            },
            focus_policy: bevy::ui::FocusPolicy::Pass,
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn_bundle(
            TextBundle::from_section(
                "Ask Me Anything!",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
        );
    }).with_children(|parent| {
        parent.spawn_bundle(ButtonBundle {
            color: Color::rgba(0.999, 0.0, 0.15, 0.45).into(),
            ..default()
        });
    }).insert(userInput);
}   

// fn debug_current_state(state: Res<CurrentState<GameState>>,
//                         rstate: Res<CurrentState<RoverState>>) {
//     if (format!("{rstate:?}") == "RoverState::Listening") {
//         info!("listening");
//     }

//     if rstate.is_changed() {
//         println!("Detected state change to {:?}!", rstate);
//     }
// }

fn chat_update(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    rstate: Res<CurrentState<RoverState>>,
    c_child: Query<&mut Text, (With<Parent>, With<chatMessage>)>,
    c_parent: Query<Entity, (With<chatParent>, With<Children>)>,
    mut sp: ResMut<LastChat>){

        let current_speaker = &sp.name;     
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
            ])).insert(chatMessage)
                .id();

        let big_parent = c_parent.single();
        commands.entity(big_parent).push_children(&[newmsg]);

        if (format!("{rstate:?}") == "RoverState::Thinking") {
            //just came from listening, so outputting user msg
            // info!("thinking");
        } else {
            //is currently talking, so outputting rover
            // info!("{:?}", rstate);
            commands.insert_resource(NextState(RoverState::Listening));
            sp.val = "".to_owned();
        }       
   
}

fn open_rover(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    user_text_q: Query<(Entity, &Children),(With<userInput>)>,
    mut text_query: Query<&mut Text>){          
        
        let (user_text_node, user_text_kids) = user_text_q.single();
        let mut user_text = text_query.get_mut(user_text_kids[0]).unwrap();
        user_text.sections[0].value = String::from("");

        let big_parent = commands.spawn_bundle(
            NodeBundle{ 
                color: Color::rgb(0.0, 0.0, 0.15).into(),
                style: Style {
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    flex_basis: Val::Px(50.0),
                    overflow: Overflow::Hidden,
                    flex_direction: FlexDirection::ColumnReverse,
                    position_type: PositionType::Absolute,
                    flex_wrap: FlexWrap::Wrap,
                    flex_grow: 10.0,
                    padding: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(100.0),
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    min_size: Size::new(Val::Px(10.0), Val::Px(50.0)),
                    max_size: Size::new(Val::Px(100.0), Val::Px(500.0)),
                    position: UiRect {
                        bottom: Val::Px(31.0),
                        left: Val::Px(110.0),
                        ..default()
                    },
                    ..default()
                },
                ..default()
            }).insert(chatParent)
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
            ])).insert(chatMessage).id();   

        commands.entity(big_parent).push_children(&[first_line]);

        commands.spawn().insert_bundle(SpriteBundle{
            texture: asset_server.load("rover-1.png"),
            transform: Transform::from_xyz(-270.0,-325.0,1.0),
            ..default()
            }).insert(roverSprite);
    
        commands.insert_resource(NextState(RoverState::Listening));
            
    }

fn close_rover(mut commands: Commands, 
    query: Query<Entity, With<chatParent>>,
    rov_query: Query<Entity, With<roverSprite>>,
    usr_input_query: Query<Entity, With<userInput>>){

        for rv in rov_query.iter() {
            commands.entity(rv).despawn();
        }
        
        for ent in query.iter() {
            commands.entity(ent).despawn_recursive();
            
        }
        // for ent in usr_input_query.iter() {
        //     commands.entity(ent).despawn_recursive();
        // }
}

fn text_input(
    mut commands: Commands, 
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut msg: ResMut<LastChat>,
    user_text: Query<(Entity, &Children),(With<userInput>)>,
    mut text_query: Query<&mut Text>,) {

        let (user_text_node, user_text_kids) = user_text.single();
        let mut user_text = text_query.get_mut(user_text_kids[0]).unwrap();


        //let mut k_input :String = "".to_owned();

        for ev in char_evr.iter() {
            if ev.char != '\u{8}' {msg.val.push(ev.char)};
            // user_text.sections[0].value = format!("{}{}", user_text.sections[0].value, ev.char);
            if ev.char != '\u{8}' {user_text.sections[0].value.push(ev.char)};
            
        }
        if keys.just_pressed(KeyCode::Return) {      
            user_text.sections[0].value = format!("");
            msg.val.pop();
            msg.name = String::from("User: ");
            // info!("{:?}", msg.val);
            commands.insert_resource(NextState(RoverState::Thinking));
        }

        if keys.just_pressed(KeyCode::Back) {  
            user_text.sections[0].value.pop();
            msg.val.pop();
        }

        if keys.just_pressed(KeyCode::Escape) {
            commands.insert_resource(NextState(RoverState::Inactive));
            commands.insert_resource(NextState(GameState::InGame));
            return;
        }
}

fn get_rover_response(
    mut commands: Commands,
    mut msg: ResMut<LastChat>,
    mut stage: ResMut<Stage>,
    asset_server: Res<AssetServer>){

    //check for funny vals
    let question = msg.val.clone();
    if question == String::from("farnan is great"){
        commands.insert_resource(NextState(GameState::Paused));
        super::ui::spawn_blue_screen_of_death(commands, asset_server);
        return;
    };
    
    let parsed = parser(question);
   // let stemmed = stemmer(parsed);
    let indexed = indexer(parsed);
    let answer = answerer(indexed, 100, asset_server);
    // msg.val = String::from("eat shit");
    msg.val = answer;
    msg.name = "rover: ".to_owned();
    commands.insert_resource(NextState(RoverState::Talking));     
}

fn parser(input: String) ->Vec<String> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s.to_lowercase()); 
    }  
    strings
}

// fn stemmer(strings: Vec<String>) ->Vec<String>  {
//     let mut new_strings=Vec::new();
//     let stopwords = stop_words::get("english");
//     let en_stemmer = Stemmer::create(Algorithm::English);
//     for s in strings{
//          if stopwords.contains(&&s)==false{
//             new_strings.push(en_stemmer.stem(&s).into_owned());
//          }
//     }
//     new_strings
// }

//returns a h long vector of the sum of all words
fn indexer (toks: Vec<String>) -> Vec<f64> {
    // let mut indexes = Vec::<f64>::new();
    let raw_dictionary: String = fs::read_to_string("./assets/dictionary.json").unwrap();
    let dict: HashMap<String, isize> = serde_json::from_str(&raw_dictionary).unwrap();
    
    let mut raw_weights: String = fs::read_to_string("./assets/weights.json").unwrap();
    let mut weights: Vec<Vec<f64>> = serde_json::from_str(&raw_weights).unwrap();

    let v = dict.keys().len();
    let mut ngram = VecDeque::from([1,1,1,1,1]); // one is not a word
    let mut sent_embed = vec!(0.0; CONSTANTS::H);

    // UNCOMMENT IF SKIPGRAM

    // for t in toks.iter(){
    //     match dict.get(t){
    //         Some(s) => {
    //             //indexes.push(*s as f64);}      // "hello cruel world"
    //             ngram.push_back(*s);
    //             ngram.pop_front();
    //             let embed = embedder(&ngram); // [1,1, 92]
    //             //embed = vec[x,x2, x3 ... h]
    //             maphs::sum(&mut sent_embed, &embed);
    //         }    
    //         None => {fs::write("./assets/words_to_add.txt", t).unwrap();}
    //     }
    // }
    //UNCOMMENT FOR SKIPGRAM

    for t in toks.iter(){
        // info!("t:{:?}", t);
        // info!("dictlen:{:?}", dict.keys().len());
        match dict.get(t){
            Some(s) => {
                // info!("insome, s{:?},t{:?}", s, t);
                if (*s >= weights.len().try_into().unwrap()) {
                    let blank = vec!(0.0; CONSTANTS::H);
                    super::maphs::sum(&mut sent_embed, &blank);

                } else {
                    let embed = &weights[*s as usize];
                    // info!("embedlen:{:?}", embed.len());
                    super::maphs::sum(&mut sent_embed, &embed);
                }
            }    
            None => {   
                // info!("got to none in indexer");
                let mut file = OpenOptions::new()
                    .append(true)
                    .open("./assets/words_to_add.txt")
                    .unwrap();
                file.write_all(t.as_bytes()).unwrap();
            }
        }
    }
    return sent_embed;
}

// fn embedder(token: &VecDeque<isize>) -> Vec<f64>{
//     // let raw_weights: String = fs::read_to_string("./assets/weights.json").unwrap();
//     // let weights: Vec<f64> = serde_json::from_str(&raw_weights).unwrap();
//     let mut token_embed = vec!(0.0; CONSTANTS::H);
//     return token_embed;
// }


fn answerer(idxs: Vec<f64>,
            stage: isize,
            asset_server: Res<AssetServer> ) -> String{
    // let mut raw_qa_list = asset_server.load("questions_answers.json");
    let raw_qa_list: String = fs::read_to_string("./assets/qa_list.json").unwrap();
    let qa_json = serde_json::from_str::<Dict<Question>>(&raw_qa_list).unwrap();
    let mut closest_answer: String = String::from("");
    let mut least_distance = 500000.0;
    for p in qa_json.items.iter() {
        // info!("in answerer, idxlen{:?},plen{:?}", idxs.len(), p.vector.len());
        let dist = maphs::cos_distance(&idxs, &p.vector);
        // info!("answerer:dist: {:?}", dist);
        if dist < least_distance && p.priority <= stage {
            least_distance = dist;
            closest_answer = p.answer.clone()}
            // info!("chosenq:{:?}", p.question);
    }
    return closest_answer;
}

