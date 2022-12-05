// extern crate rust_stemmers;
use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::{OpenOptions};
use std::io::{prelude::*, Write, BufReader};
// use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
// use stop_words;


use crate::CONSTANTS;
use super::GameState;
use super::maphs;


//stuff for egui
// use bevy_inspector_egui::{InspectorPlugin, Inspectable};
// #[derive(Inspectable, Default)]
// struct Data {
//     should_render: bool,
//     text: String,
//     #[inspectable(min = 42.0, max = 100.0)]
//     size: f32,
// }
// use bevy_inspector_egui::WorldInspectorPlugin;
// end stuff for egui
pub struct RoverPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RoverState {
    Inactive,
    Listening,
    Thinking,
    Talking,
}

#[derive(Component)]
struct Rover;
#[derive(Component)]
struct ChatMessage;
#[derive(Component)]
struct ChatParent;

#[derive(Component)]
struct RoverSprite;

#[derive(Component)]
struct UserInputBox;

#[derive(Component)]
struct UserInputBoxButton;



//RESOURCES


#[derive(Default)]
pub struct LastChat{
    pub name: String,
    pub val: String,
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
pub struct Dict <T>{
    pub items: Vec<T>
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
            .add_exit_system(RoverState::Listening, chat_update)
            .add_enter_system(RoverState::Talking, chat_update)
            // .add_plugin(WorldInspectorPlugin::new())
            // .add_system(handle_user_input_focus.run_not_in_state(GameState::Rover))
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
        }).insert(UserInputBoxButton);
    }).insert(UserInputBox);
}   


// fn handle_user_input_focus(mut commands: Commands,
//     b_query: Query<&Interaction, (Changed<Interaction>, With<UserInputBoxButton>)>,
//     // itext: Query<(Entity, &Children),With<UserInputBox>>,
//     mut inter_query: Query<&Interaction, Changed<Interaction>>){ 

//     // let (c_node, c_kids) = itext.single();
//     let clickable = b_query.single();
//     // //info!("{:?}", c_kids[1].id());
//     // let clickable = inter_query.get(c_kids[0]).unwrap();

//     // info!("{:?}", clickable);
//     match *clickable {
//         Interaction::Clicked => {
//             info!("userinputtextclicked");
//             commands.insert_resource(NextState(GameState::Rover));
//         }
//             Interaction::Hovered => {
//             // *color = HOVERED_BUTTON.into();
//         }
//             Interaction::None => {
//             //  *color = XP_BLUE.into();
//         }
//     }
// }

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
    gstate: Res<CurrentState<GameState>>,
    c_child: Query<&mut Text, (With<Parent>, With<ChatMessage>)>,
    c_parent: Query<Entity, (With<ChatParent>, With<Children>)>,
    mut sp: ResMut<LastChat>){

        if !(format!("{gstate:?}").contains("Rover")) {
            return; //just a fail safe, in case the exit system on 
            //listening triggers when leaving rover
        }
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
            ])).insert(ChatMessage)
                .id();
        
        let big_parent = c_parent.single();
        commands.entity(big_parent).push_children(&[newmsg]);
        
        // info!("currstate: {:?}", format!("{rstate:?}") );

        if format!("{rstate:?}").contains("Thinking") {
                //just came from listening, so outputting user msg
                info!("usrmsg: {}", sp.val);
        } else if format!("{rstate:?}").contains("Talking"){
            //is currently talking, so outputting rover
            // info!("{:?}", rstate);
            commands.insert_resource(NextState(RoverState::Listening));
            sp.val = "".to_owned();
        }       
   
}

fn open_rover(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    user_text_q: Query<(Entity, &Children), With<UserInputBox>>,
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
            }).insert(ChatParent)
            .id();

        let current_speaker = "Rover: ";
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
                    "Hello! ".to_owned(),
                    TextStyle {
                        font: asset_server.load("Jersey.ttf"),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ])).insert(ChatMessage).id();   

        commands.entity(big_parent).push_children(&[first_line]);

        commands.spawn().insert_bundle(SpriteBundle{
            texture: asset_server.load("rover-1.png"),
            sprite: Sprite{
                custom_size: Some(Vec2::new(20.0,20.0)),
                ..default()
            },
            transform: Transform::from_xyz(-270.0,-375.0,2.0),
            ..default()
            }).insert(RoverSprite);
    
        commands.insert_resource(NextState(RoverState::Listening));
            
    }

fn close_rover(mut commands: Commands, 
    query: Query<Entity, With<ChatParent>>,
    rov_query: Query<Entity, With<RoverSprite>>,
    usr_input_query: Query<Entity, With<UserInputBox>>){

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
    user_text: Query<(Entity, &Children), With<UserInputBox>>,
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
    
    let q = msg.val.clone();

    // super::deflections::check_for_funny_values(q); will run first
    let parsed = parser(q);
   // let stemmed = stemmer(parsed);
    let indexed = indexer(parsed);
    let answer = answerer(indexed, 100, asset_server);
    msg.val = answer;
    msg.name = "rover: ".to_owned();
    commands.insert_resource(NextState(RoverState::Talking));     
}

pub fn parser(input: String) ->Vec<String> {
    let mut strings = Vec::new();
    let no_punc = input.replace(&['(',')',',','"','.',';','?',':','\''][..], "");

    let split = no_punc.split(" ");
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
    let raw_dictionary: String = fs::read_to_string("./assets/6000_w2i.json").unwrap();
    let dict: HashMap<String, isize> = serde_json::from_str(&raw_dictionary).unwrap();
    
    let mut ngram = VecDeque::from([1,1,1,1]); // one is not a word
    let mut sent_embed = vec!(0.0; CONSTANTS::H*4);
    let mut i : usize = 0;

    for t in toks.iter(){
        match dict.get(t){
            Some(s) => {
                if i == 0 { //no 3 1's
                    ngram.push_back(*s);
                    ngram.pop_front();
                    continue;
                }
                ngram.push_back(*s);
                ngram.pop_front();
                let embed : Vec<f64> = embedder(&ngram); // [1,1, 92]
                super::maphs::sum(&mut sent_embed, &embed);
                i+=1;
            }    
            None => {   
                let mut file = OpenOptions::new()
                    .append(true)
                    .open("./assets/words_to_add.txt")
                    .unwrap();
                file.write_all(t.as_bytes()).unwrap();
            }
        }
    }
    for i in 0..2{ // two more times to flush it out
        ngram.push_back(1);
        ngram.pop_front();
        let embed = embedder(&ngram); // [1,1, 92]
        super::maphs::sum(&mut sent_embed, &embed);
    }   
    return sent_embed;
}


fn embedder(ngram: &VecDeque<isize>) -> Vec<f64>{

    let mut raw_weights: String = fs::read_to_string("./assets/6000_weights.json").unwrap();
    let mut weights: Vec<Vec<f64>> = serde_json::from_str(&raw_weights).unwrap();

    let mut token_embed = vec![0.0];
    let mut index = 0;
    token_embed.pop();

    for t in ngram.iter(){
        let word_embed = &weights[*t as usize];
        for w in word_embed.iter(){
            token_embed.push(*w);
        }
    }

    return token_embed;
}


fn answerer(idxs: Vec<f64>,
            stage: isize,
            asset_server: Res<AssetServer> ) -> String{
    // let mut raw_qa_list = asset_server.load("questions_answers.json");
    let raw_qa_list: String = fs::read_to_string("./assets/6000_qa_list.json").unwrap();
    let qa_json = serde_json::from_str::<Dict<Question>>(&raw_qa_list).unwrap();
    let mut closest_answer: String = String::from("");
    let mut least_distance = 0.0;
    for p in qa_json.items.iter() {
        let dist = maphs::cos_distance(&idxs, &p.vector);
        // info!("l:{:?}d:{:?}", least_distance, dist);
        if dist > least_distance && p.priority <= stage {
            least_distance = dist;
            closest_answer = p.answer.clone()}
    }
    if least_distance < CONSTANTS::COS_DIST_THRESHOLD {
        info!("{} confidence, sending deflection", least_distance);
        info!("Question was closest to {}", closest_answer);
        return super::deflections::generate_deflection(super::deflections::DeflectionType::NoMatch)
    }
    return closest_answer;
}

