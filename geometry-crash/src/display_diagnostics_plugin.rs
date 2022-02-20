use bevy::diagnostic::{Diagnostic, Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

pub struct DisplayDiagnosticsPlugin;

impl Plugin for DisplayDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_startup_system(setup_diagnostics_text)
            .add_system(display_text)
            .add_system(update_diagnostics_text);
    }
}

#[derive(Component, Default, Debug)]
struct UpdatedText {
    updated_text: Option<String>,
}

fn setup_diagnostics_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    commands.spawn_bundle(TextBundle {
        text: Text::with_section(
            "",
            TextStyle {
                font,
                color: Color::WHITE,
                font_size: 32.0
            },
            Default::default()
        ),
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(5.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    }).insert(UpdatedText::default());
}

fn display_text(mut query: Query<(&mut UpdatedText, &mut Text)>) {
    for (mut stateful_text, mut text) in query.iter_mut() {
        if let Some(updated_text) = stateful_text.updated_text.take() {
            text.sections[0].value = updated_text;
        }
    }
}

fn update_diagnostics_text(mut query: Query<&mut UpdatedText>, diagnostics: Res<Diagnostics>) {
    let mut diagnostics_text = query.single_mut();

    let fps = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::average)
        .unwrap_or(0.0);

    diagnostics_text.updated_text = Some(format!("{:.0} FPS", fps));
}