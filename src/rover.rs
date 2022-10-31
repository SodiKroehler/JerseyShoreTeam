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
struct SpeechVal(String);
#[derive(Component)]
struct Rover;

impl Plugin for RoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_rover)
        .add_system(text_input)
        .add_system(
            RoverRespond
                .run_if(RoverShouldTalk)
            ).run();
    }
}

fn setup_rover(mut commands: Commands, asset_server: Res<AssetServer>) {
    //commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn()
        .insert(Rover)
        .insert(SpeechState)
        .insert(SpeechVal(format("eat shit")));

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
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
    )
    //.insert(RoverText);

}   

fn RoverShouldTalk(rstate: Res<RoverState>) -> bool {
    rstate == RoverState::Talking
}

fn RoverRespond(
    mut tquery: Query<&mut Text, 
                            (With<RoverText>, 
                            /*Changed<RoverRespText>*/)>){
    for mut text in &mut tquery {
                text.sections[1].value = format!("RoverText.val");
    }
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
      let tokens = parser(&*string);
      for str in tokens.iter() {
        println!("{}", str);
      }
        string.clear();
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

// fn answerer(int: procVal){
//     let mut tquery = Query<&mut Text, With<RoverText>>;

//     for mut text in &mut tquery {
//         text.sections[1].value = format!("eat shit and die");
// }
// }

