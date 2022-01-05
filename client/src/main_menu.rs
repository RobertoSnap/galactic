use bevy::prelude::*;
pub struct MainMenuPlugin;

use kayak_ui::bevy::{BevyContext, FontMapping, UICameraBundle};
use kayak_ui::core::Index;
use kayak_ui::core::{
    render, rsx,
    styles::{Style, StyleProp, Units},
    widget,
};
use kayak_ui::widgets::{App as UiApp, Text, Window};

use galactic_core::game_state::GameState;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(host_game);
        // app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(render_main_menu));
        // app.add_startup_system(startup)
        //     .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(render_main_menu));
    }
}

fn render_main_menu(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn_bundle(UICameraBundle::new());

    font_mapping.add(asset_server.load("roboto.kayak_font"));
    let context = BevyContext::new(|context| {
        render! {
            <UiApp>
                <MainMenu />
            </UiApp>
        }
    });
    commands.insert_resource(context);
}

fn host_game(mut game_state: ResMut<State<GameState>>) {
    if &GameState::MainMenu == game_state.current() {
        game_state.set(GameState::Host).unwrap();
    }
}

#[widget]
fn MainMenu() {
    let text_styles = Style {
        bottom: StyleProp::Value(Units::Stretch(1.0)),
        left: StyleProp::Value(Units::Stretch(0.1)),
        right: StyleProp::Value(Units::Stretch(0.1)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        width: StyleProp::Value(Units::Stretch(1.0)),
        height: StyleProp::Value(Units::Pixels(28.0)),
        ..Default::default()
    };

    rsx! {
        <>
        <Text styles={Some(text_styles)} size={32.0} content={format!("Galactic").to_string()}>{}</Text>
        </>
    }
}
