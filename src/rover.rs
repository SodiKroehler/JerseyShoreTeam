extern crate rust_stemmers;
use rust_stemmers::{Algorithm, Stemmer};
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use stop_words;
use super::GameState;


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
        app
            .add_startup_system(setup_rover)
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
            //.add_system(debug_current_state)
            //.add_plugin(InspectorPlugin::<Data>::new())
            .add_plugin(WorldInspectorPlugin::new())
            .add_system(
                TEMP_move_to_talking
                    .run_in_state(RoverState::Talking))    
            .insert_resource(LastChat {name : "Rover:".to_string(),
                                        val : "".to_string()});
    }
}

fn setup_rover(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn()
    //     .insert(Rover)
    //   //  .insert(SpeechState::Talking);
}   

// fn debug_current_state(state: Res<CurrentState<GameState>>,
//                         rstate: Res<CurrentState<RoverState>>) {
//     match *state {
//         GameState::InGame => {
//             info!("ingame");
//         }
//         _ => {
//             info!("somfin else");
//         }
//     }
//     // if rstate.is_changed() {
//     //     println!("Detected state change to {:?}!", rstate);
//     // }
// }

fn TEMP_move_to_talking(mut commands: Commands, mut msg: ResMut<LastChat>) {
    commands.insert_resource(NextState(RoverState::Listening));
    msg.val = "".to_owned();
    
}

fn chat_update(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    //tquery: Query<Entity, With<chatMessage>>,
    c_child: Query<&mut Text, (With<Parent>, With<chatMessage>)>,
    c_parent: Query<Entity, (With<chatParent>, With<Children>)>,
    sp: Res<LastChat>){
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
        
   
}

fn open_rover(
    mut commands: Commands, 
    asset_server: Res<AssetServer>){

        
           
        commands.spawn_bundle(
            NodeBundle{
                color: Color::rgb(0.0, 0.0, 0.15).into(),
                style: Style {
                    size: Size::new(Val::Px(200.0), Val::Px(25.0)),
                    padding: UiRect {
                        left: Val::Px(0.0),
                        right: Val::Px(100.0),
                        top: Val::Px(5.0),
                        bottom: Val::Px(5.0),
                    },
                    ..default()
                },
                ..default()
            }
        ).with_children(|parent| {
            parent.spawn_bundle(
                TextBundle::from_section(
                        "",
                        TextStyle {
                            font: asset_server.load("Jersey.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
            );
        }).insert(userInput);
        


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
                            flex_grow: 100.0,
                            padding: UiRect {
                                left: Val::Px(0.0),
                                right: Val::Px(100.0),
                                top: Val::Px(5.0),
                                bottom: Val::Px(5.0),
                            },
                            min_size: Size::new(Val::Px(100.0), Val::Px(50.0)),
                            max_size: Size::new(Val::Px(100.0), Val::Px(500.0)),
                            position: UiRect {
                                bottom: Val::Px(25.0),
                                left: Val::Px(0.0),
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
            transform: Transform::from_xyz(-300.0,-325.0,1.0),
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
        for ent in usr_input_query.iter() {
            commands.entity(ent).despawn_recursive();
            
        }
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
            msg.val.push(ev.char);
            // user_text.sections[0].value = format!("{}{}", user_text.sections[0].value, ev.char);
            user_text.sections[0].value.push(ev.char);
            
        }
        if keys.just_pressed(KeyCode::Return) {      
            user_text.sections[0].value = format!("");
            msg.name = String::from("Root: ");
            commands.insert_resource(NextState(RoverState::Thinking));
        }

        if keys.just_pressed(KeyCode::Back) {      
            let mut this_crap_thing = msg.val.clone().to_string();
            // this_crap_thing.pop();
            // this_crap_thing.pop();
            // this_crap_thing.push('\0');
            // user_text.sections[0].value = this_crap_thing.clone();
            let mut last_i = msg.val.len();
            last_i -=1;
            user_text.sections[0].value.remove(last_i);
           // info!("{:?}", user_text.sections[0].value);
            msg.val.remove(last_i);
           // info!("{:?}",this_crap_thing);
        }

        if keys.just_pressed(KeyCode::Escape) {
            commands.insert_resource(NextState(RoverState::Inactive));
            commands.insert_resource(NextState(GameState::InGame));
            return;
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

fn stemmer(strings: Vec<String>) ->Vec<String>  {
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

