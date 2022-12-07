use std::collections::{HashMap, VecDeque};
use std::fs;
use std::fs::{OpenOptions};
use std::io::{prelude::*, Write};
use serde::{Deserialize, Serialize};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;

use crate::CONSTANTS;
use super::GameState;
use super::maphs;
use super::Stage;


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
    Loading,
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
// #[derive(Default, serde::Deserialize, bevy::reflect::TypeUuid)]
// #[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c47"]
pub struct Question{
    pub id: isize,
    pub question: String,
    pub answer: String,
    pub vector: Vec<f64>,
    pub priority: isize,
}

#[derive(Serialize, Deserialize)]
pub struct Deflection{
    pub answer: String,
    pub vector: Vec<u32>,
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Default, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c48"]
pub struct Q_Dict{
    pub items: Vec<Question>,
}

#[derive(Default, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c49"]
pub struct WordMap{
    pub itemz: HashMap<String, isize>,
}

#[derive(Default, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c50"]

pub struct Weights {
    pub items: Vec<Vec<f64>>,
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Default, serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c51"]
pub struct DeflectDict{ 
    pub items: Vec<Deflection>
}

#[derive(Default)]
struct AssetsLoading {
    items: Vec<HandleUntyped>,
}


#[derive(Default)]
pub struct Dictionaries {
    pub q_dict: Handle<Q_Dict>,
    pub w2i_dict: Handle<WordMap>,
    pub weight_dict: Handle<Weights>,
    pub deflect_dict: Handle<DeflectDict>,
}


impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_rover)
            .add_loopless_state(RoverState::Loading)
            .add_system(check_if_loaded.run_in_state(RoverState::Loading))
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
            .add_plugin(JsonAssetPlugin::<WordMap>::new(&["w2idict"]))
            .add_plugin(JsonAssetPlugin::<DeflectDict>::new(&["ddict"]))
            .add_plugin(JsonAssetPlugin::<Weights>::new(&["wdict"]))
            .add_plugin(JsonAssetPlugin::<Q_Dict>::new(&["qdict"]))
            .insert_resource(AssetsLoading {items: Vec::new()})
            .insert_resource(LastChat {name : "Rover: ".to_string(),
                                        val : "How are you?".to_string()});  
                                        
    }
}

fn check_if_loaded(mut commands: Commands,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>){

    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.items.iter().map(|h| h.id)) {
        LoadState::Failed => {
            info!("Problem loading asset")
        }
        LoadState::Loaded => {
            info!("loaded all assets");
            commands.remove_resource::<AssetsLoading>();
            commands.insert_resource(NextState(RoverState::Listening));
        }
        _ => {
            // info!("still waitin on assets");
        }
    }
}


fn setup_rover(mut commands: Commands, mut loading: ResMut<AssetsLoading>, asset_server: Res<AssetServer>) {

    let q_handle: Handle<Q_Dict> = asset_server.load("6000.qdict");
    let s_handle: Handle<Weights> = asset_server.load("6000.wdict");
    let h_handle: Handle<WordMap> = asset_server.load("6000.w2idict");
    let d_handle: Handle<DeflectDict> = asset_server.load("deflections.ddict");
    
    loading.items.push(q_handle.clone_untyped());
    loading.items.push(s_handle.clone_untyped());
    loading.items.push(h_handle.clone_untyped());
    loading.items.push(d_handle.clone_untyped());
    
    commands.insert_resource(Dictionaries { q_dict: q_handle,
                                            w2i_dict: h_handle,
                                            weight_dict: s_handle,
                                            deflect_dict: d_handle});

    commands.spawn_bundle(
        NodeBundle{
            color: Color::rgba(0.0, 0.0, 0.15, 0.45).into(),
            // transform: Transform::from_xyz(.0,-350.0,2.0),
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
                    font: asset_server.load(CONSTANTS::FONT_FILE),
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

fn chat_update(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    rstate: Res<CurrentState<RoverState>>,
    gstate: Res<CurrentState<GameState>>,
    c_child: Query<&mut Text, (With<Parent>, With<ChatMessage>)>,
    c_parent: Query<Entity, (With<ChatParent>, With<Children>)>,
    mut sp: ResMut<LastChat>){


        // info!("currstate: {:?}", format!("{rstate:?}") );


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
                        font: asset_server.load(CONSTANTS::FONT_FILE),
                        font_size: 20.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    &sp.val,
                    TextStyle {
                        font: asset_server.load(CONSTANTS::FONT_FILE),
                        font_size: 20.0,
                        color: Color::WHITE,
                    },
                ),
            ])).insert(ChatMessage)
                .id();
        
        let big_parent = c_parent.single();
        commands.entity(big_parent).push_children(&[newmsg]);
        
        if (sp.name == "Rover: " && sp.val == "How are you?") {
            sp.val = String::from(""); // to stop weird chat update
        }

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
                        font: asset_server.load(CONSTANTS::FONT_FILE),
                        font_size: 20.0,
                        color: Color::GOLD,
                    },
                ),
                TextSection::new(
                    "Hello! ".to_owned(),
                    TextStyle {
                        font: asset_server.load(CONSTANTS::FONT_FILE),
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
            transform: Transform::from_xyz(0.0,0.0, CONSTANTS::Z_UI + 2.0),
            // transform: Transform::from_xyz(-270.0,-375.0, CONSTANTS::Z_UI + 2.0),
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
    stage: ResMut<Stage>,
    asset_server: Res<AssetServer>,
    handles: Res<Dictionaries>,
    w2i: ResMut<Assets<WordMap>>,
    weights: ResMut<Assets<Weights>>,
    deflect_dict: ResMut<Assets<DeflectDict>>,
    q_dict: ResMut<Assets<Q_Dict>>){
    
    let q = msg.val.clone();

    // super::deflections::check_for_funny_values(q); will run first
    let parsed = parser(q);
   // let stemmed = stemmer(parsed);
    let indexed = indexer(parsed, &handles, &w2i, &weights);
    let answer = answerer(indexed, stage.val +5, asset_server, &handles, &q_dict, &deflect_dict);
    msg.val = answer;
    msg.name = "Rover: ".to_owned();
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
fn indexer (toks: Vec<String>,
            handles: &Res<Dictionaries>,
            w2i: &ResMut<Assets<WordMap>>, 
            weights:  &ResMut<Assets<Weights>>) -> Vec<f64> {

    // let raw_dictionary: String = fs::read_to_string("./assets/6000_w2i.json").unwrap();
    // let dict: HashMap<String, isize> = serde_json::from_str(&raw_dictionary).unwrap();
    
    if let Some(dict) = w2i.get(&handles.w2i_dict) {

        let mut ngram = VecDeque::from([1,1,1,1]); // one is not a word
        let mut sent_embed = vec!(0.0; CONSTANTS::H*4);
        let mut i : usize = 0;

        for t in toks.iter(){
            match dict.itemz.get(t){
                Some(s) => {
                    if i == 0 { //no 3 1's
                        ngram.push_back(*s);
                        ngram.pop_front();
                        continue;
                    }
                    ngram.push_back(*s);
                    ngram.pop_front();
                    let embed : Vec<f64> = embedder(&ngram, handles, weights); // [1,1, 92]
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
            let embed = embedder(&ngram, handles, weights); // [1,1, 92]
            super::maphs::sum(&mut sent_embed, &embed);
        }   
        return sent_embed;
    } else {
        info!("i have no idea what to");
        return vec![0.0];
    }
}


fn embedder(ngram: &VecDeque<isize>,             
    handles: &Res<Dictionaries>,
    raw_weights: &ResMut<Assets<Weights>>) -> Vec<f64>{

    // let mut raw_weights: String = fs::read_to_string("./assets/6000_weights.json").unwrap();
    // let mut weights: Vec<Vec<f64>> = serde_json::from_str(&raw_weights).unwrap();

    if let Some(weights) = raw_weights.get(&handles.weight_dict) {

        let mut token_embed = vec![0.0];
        let index = 0;
        token_embed.pop();

        for t in ngram.iter(){
            let word_embed = &weights.items[*t as usize];
            for w in word_embed.iter(){
                token_embed.push(*w);
            }
        }

        return token_embed;
    } else {
        info!("weird resource issue in embedder");
        return vec![0.0];
    }
}


fn answerer(idxs: Vec<f64>,
            stage: isize,
            asset_server: Res<AssetServer>,
            handles: &Res<Dictionaries>,
            q_dict: &ResMut<Assets<Q_Dict>>,
            deflect_dict: &ResMut<Assets<DeflectDict>>) -> String{
    // let mut raw_qa_list = asset_server.load("questions_answers.json");
    // let raw_qa_list: String = fs::read_to_string("./assets/6000_qa_list.json").unwrap();
    // let qa_json = serde_json::from_str::<Dict<Question>>(&raw_qa_list).unwrap();
    
    if let Some(qa_json) = q_dict.get(&handles.q_dict) {

        let mut closest_answer: String = String::from("");
        let mut least_distance = 0.0;
        let mut closest_answer_stage = 0;
        for p in qa_json.items.iter() {
            let dist = maphs::cos_distance(&idxs, &p.vector);
            // info!("l:{:?}d:{:?}", least_distance, dist);
            if dist > least_distance{
                least_distance = dist;
                closest_answer_stage = p.priority;
                closest_answer = p.answer.clone();
            }
        }
        if least_distance < CONSTANTS::COS_DIST_THRESHOLD {
            info!("{} confidence, sending deflection", least_distance);
            info!("Question was closest to {}", closest_answer);
            return super::deflections::generate_deflection(
                                super::deflections::DeflectionType::NoMatch, 
                                handles,deflect_dict)
        }
        if closest_answer_stage > stage {
            info!("priority error (had {}, required {})", stage, closest_answer_stage);
            return super::deflections::generate_deflection(
                super::deflections::DeflectionType::StageTooLow, 
                handles,deflect_dict);
        }
        info!("stage: {:?}", stage);
        return closest_answer;
    }else {
        info!("resource loading issue");
        return String::from("um my guts are falling out");
    }
}

