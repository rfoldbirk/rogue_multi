use bevy::prelude::*;

use crate::components::*;



pub struct TestPlugin;
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_platform);

        app.add_system(ensure_correct_position_from_cam);
    }
}



fn spawn_platform(mut commands: Commands) {
    let mut tiles = Vec::new();

    let test_map = [
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    ];

    for i in test_map {
        if i == 0 {
            tiles.push(
                commands.spawn_bundle(TileBundle {
                    index_pos: IndexPos(i),
                    position: Position3D::from_xyz(4., 4., 0.),
                    ..default()
                }).id()
            );
        }
    }

    commands.spawn_bundle(TransformBundle {..default()})
        .insert(Name::new("Test Platform"))
        .push_children(&tiles);
}




fn ensure_correct_position_from_cam(
    mut q: Query<(&mut Transform, &FollowCam)>,
    q_cam: Query<&Transform, (With<Camera>, Without<FollowCam>)>,
) {
    let camera_pos = q_cam.single();

    for (mut pos, fc) in q.iter_mut() {
        pos.translation = camera_pos.translation + Vec3::new(fc.offset_x, fc.offset_y, 0.);
        pos.translation.z = 0.0;
    }
}