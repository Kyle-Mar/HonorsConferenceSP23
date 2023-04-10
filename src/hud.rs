use crate::app_state::AppState;
use bevy::{prelude::*};

#[derive(Component)]
pub struct HealthBar;

fn setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(50.0),
                    height: Val::Percent(10.0),
                },
                margin: UiRect { left: Val::Percent(1.), top: Val::Percent(1.), ..default()},
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: BackgroundColor(Color::GRAY),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                        },
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::GREEN),
                    ..default()
                },
                HealthBar,
            ));
        });
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::InGame)));
    }
}
