use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};
use crate::app_state::AppState;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::PURPLE),
            ..default()
        },
    ).with_children(|parent|{
        parent.spawn(TextBundle::from_section(
            "Press Space to Start!",
            TextStyle{
                font: asset_server.load("fonts/NotoSans-Black.ttf"),
                font_size: 100.0,
                color: Color::GREEN,
            },
        ).with_style(Style{
            position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Auto),
            ..default()

        })
        .with_text_alignment(TextAlignment::Left));
    });    
    
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin{
    fn build(&self, app: &mut App){
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)));
    }
}