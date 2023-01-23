//! Tools for the player to interact with the world

use bevy::prelude::{App, Plugin, SystemLabel};

pub mod abilities;
pub mod camera;
pub mod cursor;
pub mod intent;
pub mod organism_details;
pub mod tile_selection;
pub mod zoning;

/// All of the code needed for users to interact with the simulation.
pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(camera::CameraPlugin)
            .add_plugin(abilities::AbilitiesPlugin)
            .add_plugin(cursor::CursorTilePosPlugin)
            .add_plugin(intent::IntentPlugin)
            .add_plugin(organism_details::DetailsPlugin)
            .add_plugin(tile_selection::TileSelectionPlugin)
            .add_plugin(zoning::ZoningPlugin);

        #[cfg(feature = "debug_tools")]
        app.add_plugin(debug_tools::DebugToolsPlugin);
    }
}

/// Public system sets for player interaction, used for system ordering and config
#[derive(SystemLabel, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InteractionSystem {
    /// Moves the camera
    MoveCamera,
    /// Cursor position is set
    ComputeCursorPos,
    /// Tiles are selected
    SelectTiles,
    /// Held structure is selected
    SelectStructure,
    /// Replenishes the [`IntentPool`](intent::IntentPool) of the hive mind
    ReplenishIntent,
    /// Apply zoning to tiles
    ApplyZoning,
    /// Use intent-spending abilities
    UseAbilities,
}
