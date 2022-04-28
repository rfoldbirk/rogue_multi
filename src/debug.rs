use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        // if cfg!(debug_assertion) {
        app.add_plugin(WorldInspectorPlugin::new());
        // }
    }
}