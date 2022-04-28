use bevy::prelude::*;


pub struct ComponentsPlugin;
impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<TileComponent>()
            .register_type::<Paused>()
            .register_type::<CamScale>()
            .register_type::<FollowCam>()
            .register_type::<MousePosition>();
    }
}


//* TileSize *//
#[derive(Default)]
pub struct TileSize(pub usize);


//* TileMap *//
#[derive(Default)]
pub struct TileMap {
    pub width: usize,
    pub height: usize,
}


#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct Tile;



#[derive(Component)]
pub struct TileName(pub &'static str);

#[derive(Component)]
pub struct ZIndex(pub f32);

#[derive(Component)]
pub struct Position3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}



impl Default for ZIndex {
    fn default() -> Self {
        return ZIndex(0.0);
    }
}

impl Default for Position3D {
    fn default() -> Self {
        return Position3D {
            x: 0.,
            y: 0.,
            z: 0.,
        };
    }
}

impl Position3D {
    #[inline]
    pub const fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
        }
    }
}


#[derive(Bundle)]
pub struct TileBundle {
    pub name: Name,
    pub index_pos: IndexPos,
    pub tile: Tile,
    pub tile_just_spawned: TileJustSpawned,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub position: Position3D,
    // pub global_transform: GlobalTransform,
    // pub position: Position2D,
    // pub z_index: ZIndex,
}

impl Default for TileBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Tile"),
            index_pos: Default::default(),
            // position: Default::default(),
            // z_index: Default::default(),
            tile_just_spawned: TileJustSpawned,
            tile: Tile,
            position: Default::default(),
            // global_transform: Default::default(),
            sprite: SpriteSheetBundle { 
                visibility: Visibility {
                    is_visible: false
                },
                sprite: TextureAtlasSprite {
                    index: 18,
                    ..default()
                },
                ..default() 
            },
        }
    }
}


//* TileJustSpawned ...med lidt info omkring hvad der skal gøres *//
#[derive(Component)]
pub struct TileJustSpawned;


//* Animation Timer *//
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


//* Tile Component *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TileComponent {
    pub frame_max: usize,
    pub selected: bool,
}


//* Tile Preview || Falsk tile *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TilePreview;


//* Mouse Position *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct MousePosition {
    pub raw_x: f32, // den rigtige/interne position. Kommer formentlig ikke til at skulle bruges til noget.
    pub raw_y: f32,
    pub x: f32, // den opskalerede position.
    pub y: f32,
}


//* Paused *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Paused(pub bool);


//* CamScale *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct CamScale(pub f32);



//* IndexPos *//
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct IndexPos(pub usize);


//* FollowCam *//
/// FollowCam gør det muligt at låse en entity fast på skærmen
/// Via offset kan man bestemme hvor på skærmen objektet skal være.
#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct FollowCam {
    pub offset_x: f32,
    pub offset_y: f32,
}