use bevy::prelude::*;

use crate::components::*;



pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(TileMap {
                width: 20,
                height: 20,
            })
            .insert_resource(TileSize(8))
            .add_startup_system(spawn_fake_tile)
            .add_startup_system(spawn_tile_map)
            .add_system(mouse_hover_over_tiles)
            .add_system(correct_newly_spawned_tile)
            .add_system(ensure_spritesheet_limit);
    }
}


fn spawn_tile_map(mut commands: Commands) {
    let layer_names = ["Background"]; //, "Middle", "Top"];

    let mut layers = Vec::new();
    let mut i = 0;

    for name in layer_names {
        let layer = commands.spawn()
            .insert(Name::new(name))
            .insert(Map)
            .insert(Visibility { is_visible: true })
            .insert(Transform {
                translation: Vec3::new(0., 0., i as f32),
                ..default()
            })
            .insert(GlobalTransform::default())
            .id();

        layers.push(layer);
        i += 1;
    }

    commands.spawn()
        .insert(Name::new("TileMap"))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .push_children(&layers);
}



fn spawn_fake_tile(mut commands: Commands, tile_size: Res<TileSize>) {
    let tile_size = tile_size.0 as f32;

    // Jeg laver en "falsk" tile, som kan flyttes rundt på de forskellige steder, for at vise hvad jeg markerer.    
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(1., 1., 1., 0.5),
            custom_size: Some(Vec2::new(tile_size, tile_size)),
            ..default()
        },
        ..default()
    })
        .insert(TilePreview)
        .insert(Transform {
            translation: Vec3::new(0.0, 0.0, 4.),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(Name::new("TileSelector"));
}


fn mouse_hover_over_tiles(
    mut commands: Commands,
    mut q_preview_tile: Query<(&mut Transform, &mut Visibility), With<TilePreview>>,
    q_tile_map: Query<(Entity, &Transform, &Visibility), (With<Map>, Without<TilePreview>)>,
    q_mouse_pos: Query<&MousePosition>,
    q_existing_tiles: Query<&IndexPos, With<Tile>>,
    mouse_button_input: Res<Input<MouseButton>>,
    tile_map: Res<TileMap>,
    tile_size: Res<TileSize>,
) {
    let tile_size = tile_size.0 as f32;
    let mouse_pos = q_mouse_pos.single();

    
    let mut found = false;
    let (mut p_tile, mut appereance) = q_preview_tile.single_mut();
    

    for x in 0..tile_map.width {
        if found { break; }
        
        let raw_x = x;
        let x = (x as f32)*tile_size; // 0, 8, 16, 


        for y in 0..tile_map.height {
            let raw_y = y;
            let y = (y as f32)*-tile_size;

            // find ud af om den kollidere
            if mouse_pos.x >= x && mouse_pos.x <= x+tile_size && mouse_pos.y >= y && mouse_pos.y <= y+tile_size {
                p_tile.translation.x = x+tile_size/2.; // 0-4, 8-4, 
                p_tile.translation.y = y+tile_size/2.;

                if mouse_button_input.just_pressed(MouseButton::Left) {
                    let mut placed = false;

                    
                    // Find det rigtige tilemap og placer den nye tile der
                    
                    for (t_map, t_transform, visibility) in q_tile_map.iter() {
                        if placed { break }
                        if !visibility.is_visible { continue }
                        let existing_tiles = q_existing_tiles.iter();
                        let mut safe_to_spawn = true;

                        let proposed_index = raw_y * tile_map.width + raw_x;
                        for old_tile_index in existing_tiles {
                            if proposed_index == old_tile_index.0 {
                                safe_to_spawn = false;
                            }
                        }

                        if !safe_to_spawn {continue}

                        
                        let new_tile = commands.spawn_bundle(TileBundle {
                            index_pos: IndexPos(proposed_index),
                            position: Position3D::from_xyz(p_tile.translation.x, p_tile.translation.y, t_transform.translation.z),
                            ..default()
                        }).id();


                        // Skubber den nye tile til tilemappet.
                        commands.entity(t_map).push_children(&[new_tile]);

                        placed = true;
                    }
                }

                found = true;
                break;
            }
        }
    }

    appereance.is_visible = found;
}


fn correct_newly_spawned_tile(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut q: Query<(
        Entity,
        &Position3D,
        &mut Transform,
        &mut Visibility,
    ), With<TileJustSpawned>>,
    tile_size: Res<TileSize>,
    asset_server: Res<AssetServer>,
) {
    let tile_size = tile_size.0 as f32;

    // Ret alle nye enheder
    for (
        entity,
        position,
        mut transform,
        mut visibility,
    ) in q.iter_mut() {       

        let texture_handle = asset_server.load("Tilemap/colored_tilemap_packed.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(tile_size as f32, tile_size as f32), 16, 10);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);


        // opdater position. det er dog kun selve feltet, hvis position ændres. Spritet lever altså lidt i sin egen værden :))
        transform.translation = Vec3::new(position.x, position.y, position.z);
        visibility.is_visible = true;
        
        commands.entity(entity)
            .remove::<Position3D>()
            .remove::<Handle<TextureAtlas>>()
            .insert(texture_atlas_handle)
            .remove::<TileJustSpawned>();

    }
}


fn create_tiles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let tile_size = 8;

    let texture_handle = asset_server.load("Tilemap/colored_tilemap_packed.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(tile_size as f32, tile_size as f32), 17, 10);
    let _texture_atlas_handle = texture_atlases.add(texture_atlas);
    
      


    let _map_id = commands.spawn()
        .insert(Name::new("TileMap"))
        .insert(Transform::default())
        .insert(GlobalTransform::default());
}



// Programmet crasher når man vælger en sprite som ikke findes. Derfor sørger dette system for at det ikke er muligt :)
fn ensure_spritesheet_limit(mut query: Query<(&mut TextureAtlasSprite, &TileComponent), With<TileComponent>>) {
    for (mut text_atlas_sprite, tile) in query.iter_mut() {
        if text_atlas_sprite.index > tile.frame_max {
            text_atlas_sprite.index = tile.frame_max;
        }
    }
}