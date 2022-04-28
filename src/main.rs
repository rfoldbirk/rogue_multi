use bevy::prelude::*;
use bevy::render::camera::WindowOrigin::BottomLeft;

mod testing;
mod debug;
mod components;
mod map;

use testing::TestPlugin;
use debug::DebugPlugin;
use map::MapPlugin;
use components::{ComponentsPlugin, CamScale};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 16.0 / 9.0;

fn main() {
    const WIDTH: f32 = 232.0;
    const HEIGHT: f32 = 256.0;
    const SCALE: u8 = 4;

    let mut app = App::new();
    
    
    // Resources
    app
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "Rouge Survival".to_string(),
            width: WIDTH * SCALE as f32,
            height: HEIGHT * SCALE as f32,
            position: Some(Vec2::new(50.0, 50.0)),
            resizable: false,
            ..Default::default()
        });

    // Plugins
    app
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugPlugin)
        .add_plugin(ComponentsPlugin)
        .add_plugin(TestPlugin)
        .add_plugin(MapPlugin);


    // Startup systems
    app.add_startup_system(setup);

    // Systems
    app.add_system(ensure_cam_scale)
        .add_system(process_mouse_pos);
    
    // Run
    app.run();
}



fn setup(mut commands: Commands) {
    // Game Data
    // Information man skal bruge i resten af spillet.

    let cam_scale = 5.;

    commands.spawn()
        .insert(components::MousePosition {
            raw_x: 0.0,
            raw_y: 0.0,
            x: 0.0,
            y: 0.0,
        })
        .insert(components::Paused(false))
        .insert(components::CamScale(cam_scale))
        .insert(Name::new("GameData"));
        
        
        
    // Camera
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.transform.translation.x = 0.;
    camera.transform.translation.y = -196.;
    camera.orthographic_projection.window_origin = BottomLeft;
    camera.orthographic_projection.scale = 1./cam_scale;

    commands.spawn_bundle(camera).insert(Name::new("Camera"));
}


fn process_mouse_pos(
    windows: Res<Windows>,
    mut q_mouse_pos: Query<(&mut components::MousePosition, &CamScale)>,
    q_cam_transformation: Query<&Transform, With<Camera>>,
) {
    // Få musens nuværende position på skærmen.
    let window = windows.get_primary().unwrap();
    let (mut mouse_pos, cam_scale) = q_mouse_pos.single_mut();
    let cam_translation = q_cam_transformation.single().translation;

    if let Some(raw_mouse_pos) = window.cursor_position() {
        // Opdater musens position og beregn den korrekte position i forhold til hvor kameraet er placeret
        mouse_pos.raw_x = raw_mouse_pos.x;
        mouse_pos.raw_x = raw_mouse_pos.y;

        mouse_pos.x = raw_mouse_pos.x / cam_scale.0 + cam_translation.x;
        mouse_pos.y = raw_mouse_pos.y / cam_scale.0 + cam_translation.y;
    }


}


fn ensure_cam_scale(
    mut q: Query<&mut CamScale>,
    mut cam_q: Query<&mut OrthographicProjection, With<Camera>>,
) {
    let mut cam_scale = q.single_mut();
    let mut camera_proj = cam_q.single_mut();

    // Max - kan være at det skal ændres :/
    if cam_scale.0 > 8. {
        cam_scale.0 = 8.;
    }

    // Min
    if cam_scale.0 < 0. {
        cam_scale.0 = 0.;
    }

    // Opdater skalering af kamera
    camera_proj.scale = 1.0/cam_scale.0;
}