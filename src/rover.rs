use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub struct RoverPlugin;

#[derive(Component)]
enum SpeechState {
    Inactive,
    Waiting,
    Thinking,
    Talking,
}

#[derive(Component)]
struct Rover;
#[derive(Component)]
struct chatMessage;

#[derive(Component)]
struct userInput;


#[derive(Component)]
struct roverSprite;

//RESOURCES

#[derive(Default)]
struct UserInput{
    val: String,
}

struct RoverSpeech {
    val:String,
}



impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.
        add_startup_system(setup_rover)
        .add_system(
            text_input
                .run_if_not(RoverIsInactive)
        ).add_system(
            despawn_rover
                .run_if(RoverIsInactive)
        ).add_system(
            RoverChatUpdate
                .run_if(RoverShouldTalk)
            )
        .insert_resource(RoverSpeech {val : "eat shit".to_string()})
        .run();
    }
}

fn setup_rover(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn_bundle(UiCameraBundle::default());

    commands.spawn()
        .insert(Rover)
        .insert(SpeechState::Talking);

    commands.spawn_bundle(
        TextBundle::from_sections([
            TextSection::new(
                "Rover: ",
                TextStyle {
                    font: asset_server.load("Jersey.ttf"),
                    font_size: 20.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("Jersey.ttf"),
                font_size: 20.0,
                color: Color::GOLD,
            }),
        ]).with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(25.0),
                left: Val::Px(0.0),
                ..default()
            },
            ..default()
        }
    )).insert(chatMessage);


    commands.spawn_bundle(
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
    ).insert(userInput);
    //.insert(Rover);

   commands.spawn().insert_bundle(SpriteBundle{
    texture: asset_server.load("rover-1.png"),
    transform: Transform::from_xyz(-300.0,-350.0,1.0),
    ..default()
    }).insert(roverSprite);


}   

fn RoverShouldTalk(q: Query<&SpeechState, With<Rover>>) -> bool {
    let (sp) = q.single();
    return matches!(sp, SpeechState::Talking);
}

fn RoverIsInactive(q: Query<&SpeechState, With<Rover>>) -> bool {
    let (sp) = q.single();
    return matches!(sp, SpeechState::Inactive);
}

fn RoverChatUpdate(
    mut tquery: Query<&mut Text, With<chatMessage>>,
    sp: Res<RoverSpeech>){
        //let rover = rv.single();
        for mut rtext in tquery.iter_mut(){
            rtext.sections[1].value = sp.val.to_string();
    }
}

fn despawn_rover(mut commands: Commands, 
    query: Query<Entity, With<chatMessage>>,
    rovQuery: Query<Entity, With<roverSprite>>){
        //let rover = rovQuery.single();
        for rv in rovQuery.iter() {
            commands.entity(rv).despawn();
        }
        
        for ent in query.iter() {
            commands.entity(ent).despawn_recursive();
            
        }
}

fn text_input(
    mut commands: Commands, 
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
  /*  usrResText: Res<userInput>,*/
    mut string: Local<String>,
    mut rv: Query<&mut SpeechState, With<Rover>>,
    mut text: Query<&mut Text, With<userInput>>,
) {
    let mut userText = text.single_mut();
    let mut rover = rv.single_mut();
    for ev in char_evr.iter() {
        string.push(ev.char);
        userText.sections[0].value = format!("{}{}", userText.sections[0].value, ev.char);
        
    }
    
    if keys.just_pressed(KeyCode::Return) {
    //   let tokens = parser(&*string);
    //   for str in tokens.iter() {
    //     println!("{}", str);
    //   }
      userText.sections[0].value = format!("");
      //commands.insert_resource(UserInput {val : format!("{}", string)});
        string.clear();
    }
    if keys.just_pressed(KeyCode::Escape) {
        *rover = SpeechState::Inactive;
        info!("escape pressed");
    }
}


fn parser(input: &str) ->Vec<&str> {
    let mut strings = Vec::new();
    let split = input.split(" ");
    for s in split {
        strings.push(s);
    }  
    strings
}

fn stemmer(mut strings: Vec<&str>) ->Vec<&str>  {
    let mut i=0;
    let mut new_strings=Vec::new();
    let stopword = vec!["a","about","above","across","after","afterwards","again","against","all", "almost","purpose"];
    for s in strings{
         if stopword.contains(&&s)==false{
              new_strings.push(s);
         }
        i+=1;
    }
    new_strings
}

// // fn answerer(int: procVal){
// //     let mut tquery = Query<&mut Text, With<RoverText>>;

// //     for mut text in &mut tquery {
// //         text.sections[1].value = format!("eat shit and die");
// // }
// // }

