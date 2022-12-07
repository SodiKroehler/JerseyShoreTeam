use bevy::{app::AppExit, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContext, egui};

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

use crate::GameState;

pub struct GameMenuPlugin;

const USERNAME: &'static str = "11";
const PASSWORD: &'static str = "22";

#[derive(Default, Debug)]
pub struct UserData {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MenuState {
    Main,
    UserLogin,
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    NewGame,
    Quit,
}

#[derive(Component)]
struct GameMenuButton;

#[derive(Component)]
struct OnMainMenuScreen;
#[derive(Component)]
struct OnUserLoginScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MenuState::Main)
            .init_resource::<UserData>()
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(menu_setup))
            .add_system_set(SystemSet::on_enter(MenuState::Main).with_system(main_menu_setup))
            .add_system_set(SystemSet::on_update(MenuState::UserLogin).with_system(user_login))
            .add_system_set(
                SystemSet::on_exit(MenuState::UserLogin)
                    .with_system(despawn_screen::<OnMainMenuScreen>),
            )
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(menu_action)
                    .with_system(button_system),
            );
    }
}

fn menu_setup(mut menu_state: ResMut<State<MenuState>>) {
    let _ = menu_state.set(MenuState::Main);
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<GameMenuButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        *color = match *interaction {
            Interaction::Clicked => PRESSED_BUTTON.into(),
            Interaction::Hovered => HOVERED_PRESSED_BUTTON.into(),
            Interaction::None => NORMAL_BUTTON.into(),
        }
    }
}

fn main_menu_setup(mut command: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let button_style = Style {
        size: Size::new(Val::Px(300.), Val::Px(100.)),
        margin: UiRect::all(Val::Px(20.)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font: font.clone(),
        font_size: 40.,
        color: TEXT_COLOR,
    };

    command
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: UiColor::from(Color::ORANGE_RED),
            ..Default::default()
        })
        .insert(OnMainMenuScreen)
        .with_children(|parent| {
            parent.spawn().insert_bundle(
                TextBundle::from_section(
                    "Game Menu",
                    TextStyle {
                        font: font.clone(),
                        font_size: 80.,
                        ..Default::default()
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(30.)),
                    ..Default::default()
                }),
            );

            parent
                .spawn()
                .insert_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: UiColor::from(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .insert(MenuButtonAction::NewGame)
                .insert(GameMenuButton)
                .with_children(|parent| {
                    parent.spawn().insert_bundle(TextBundle::from_section(
                        "New Game",
                        button_text_style.clone(),
                    ));
                });

            parent
                .spawn()
                .insert_bundle(ButtonBundle {
                    style: button_style.clone(),
                    color: UiColor::from(Color::rgb(0.15, 0.15, 0.15)),
                    ..Default::default()
                })
                .insert(MenuButtonAction::Quit)
                .insert(GameMenuButton)
                .with_children(|parent| {
                    parent
                        .spawn()
                        .insert_bundle(TextBundle::from_section("Quit", button_text_style.clone()));
                });
        });
}

fn user_login(
    mut user_data: ResMut<UserData>,
    mut egui_ctx: ResMut<EguiContext>,
    mut state: ResMut<State<GameState>>,
    mut menu: ResMut<State<MenuState>>,
) {
    egui::Window::new("My Window")
        .default_size(egui::Vec2::new(300., 200.))
        .resizable(false)
        .title_bar(false)
        .show(egui_ctx.ctx_mut(), |ui| {
            ui.heading("User Login");
            ui.horizontal(|ui| {
                ui.label("username:");
                ui.text_edit_singleline(&mut user_data.username);
            });
            ui.horizontal(|ui| {
                ui.label("password:");
                ui.text_edit_singleline(&mut user_data.password);
            });
            ui.horizontal(|ui| {
                if ui.button("Login").clicked() {
                    println!(
                        "username: {}, password: {}",
                        user_data.username, user_data.password
                    );
                    if &user_data.username == USERNAME && &user_data.password == PASSWORD {
                        menu.set(MenuState::Disabled).unwrap();
                        state.set(GameState::InGame).unwrap();
                    }
                }
            });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<State<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::NewGame => {
                    if menu_state.current() != &MenuState::UserLogin {
                        menu_state.set(MenuState::UserLogin).unwrap();
                    }
                }
            }
        }
    }
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
