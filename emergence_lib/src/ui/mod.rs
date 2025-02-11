//! Manages all UI across the entire game.

use crate::ui::{
    production_statistics::ProductionStatisticsPlugin, select_structure::SelectStructurePlugin,
    select_terraforming::SelectTerraformingPlugin, selection_panel::HoverDetailsPlugin,
};
use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

mod intent;
mod production_statistics;
mod select_structure;
mod select_terraforming;
mod selection_panel;
mod wheel_menu;

/// The font handles for the `FiraSans` font family.
///
/// This is cached in a resource to improve performance.
#[derive(Debug, Resource)]
struct FiraSansFontFamily {
    /// The font to use for regular text.
    regular: Handle<Font>,
}

/// Struct to build the UI plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        info!("Building UI plugin...");

        let asset_server = app.world.get_resource::<AssetServer>().unwrap();

        app.insert_resource(FiraSansFontFamily {
            regular: asset_server.load("fonts/FiraSans-Medium.ttf"),
        })
        .add_startup_system(setup_ui.in_base_set(StartupSet::PreStartup))
        .add_plugin(ScreenDiagnosticsPlugin::default())
        .add_plugin(ScreenFrameDiagnosticsPlugin)
        .add_plugin(HoverDetailsPlugin)
        .add_plugin(ProductionStatisticsPlugin)
        .add_plugin(SelectStructurePlugin)
        .add_plugin(SelectTerraformingPlugin);
    }
}

/// The UI panel on the left side.
#[derive(Debug, Component)]
struct LeftPanel;

/// The UI panel on the right side.
#[derive(Debug, Component)]
struct RightPanel;

/// Create the basic UI layout.
fn setup_ui(mut commands: Commands) {
    // UI layout
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // UI panel on the left side
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Px(200.), Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                },
                LeftPanel,
            ));

            // UI panel on the right side
            parent.spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        size: Size::new(Val::Px(400.), Val::Percent(100.)),
                        ..default()
                    },
                    ..default()
                },
                RightPanel,
            ));
        });
}
