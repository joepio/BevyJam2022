use bevy::prelude::*;

use crate::assets::{FontAssets, GameState};

pub struct ScoreboardPlugin;

impl Plugin for ScoreboardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Scoreboard { hits: 0, misses: 0 })
            .add_event::<ScoreboardEvent>()
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_scoreboard))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(scoreboard_ui)
                    .with_system(handle_scoreboard_event),
            );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScoreboardEvent {
    Hit,
    Miss,
    Reset,
}

struct Scoreboard {
    pub hits: usize,
    pub misses: usize,
}

fn setup_scoreboard(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "Score: ".to_string(),
                    style: TextStyle {
                        font: font_assets.fira_mono_medium.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.6, 0.6, 0.9),
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: font_assets.fira_mono_medium.clone(),
                        font_size: 32.0,
                        color: Color::rgb(0.9, 0.6, 0.6),
                    },
                },
            ],
            ..Default::default()
        },
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                left: Val::Px(5.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
}

fn scoreboard_ui(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    if !scoreboard.is_changed() {
        // Nothing changed, don't update anything
        return;
    }

    let mut text = query.single_mut();

    let hits = scoreboard.hits;
    let misses = scoreboard.misses;

    if hits > 0 || misses > 0 {
        text.sections[1].value = format!("{:.1}% | ", (hits / (hits + misses)) * 100);
    } else {
        text.sections[1].value = "0".to_string();
    }
}

fn handle_scoreboard_event(
    mut scoreboard: ResMut<Scoreboard>,
    mut events: EventReader<ScoreboardEvent>,
) {
    for event in events.iter() {
        match event {
            ScoreboardEvent::Hit => {
                scoreboard.hits += 1;
            }
            ScoreboardEvent::Miss => {
                scoreboard.misses += 1;
            }
            ScoreboardEvent::Reset => {
                scoreboard.hits = 0;
                scoreboard.misses = 0;
            }
        }
    }
}
