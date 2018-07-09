use fnv::FnvHashMap;
use cgmath::{Vector2, vec2};
use pixel_num::sub_pixel_i64::{self, SubPixelI64};
use shape::Shape;
use axis_aligned_rect::AxisAlignedRect;
use loose_quad_tree::LooseQuadTree;

#[derive(Default, Debug)]
pub struct InputModel {
    left: SubPixelI64,
    right: SubPixelI64,
    up: SubPixelI64,
    down: SubPixelI64,
}

fn clamp_float(value: f32) -> SubPixelI64 {
    SubPixelI64::new_pixels_f32(value).clamp_zero_one_pixel()
}

impl InputModel {
    pub fn set_left(&mut self, value: f32) {
        self.left = clamp_float(value);
    }
    pub fn set_right(&mut self, value: f32) {
        self.right = clamp_float(value);
    }
    pub fn set_up(&mut self, value: f32) {
        self.up = clamp_float(value);
    }
    pub fn set_down(&mut self, value: f32) {
        self.down = clamp_float(value);
    }
    fn horizontal(&self) -> SubPixelI64 {
        self.right - self.left
    }
    fn vertical(&self) -> SubPixelI64 {
        self.down - self.up
    }
    fn movement(&self) -> Vector2<SubPixelI64> {
        sub_pixel_i64::normalize_vector_if_longer_than_one(vec2(
            self.horizontal(),
            self.vertical(),
        ))
    }
}

fn update_player_velocity(
    _current_velocity: Vector2<SubPixelI64>,
    input_model: &InputModel,
) -> Vector2<SubPixelI64> {
    const MULTIPLIER: i64 = 4;
    input_model.movement() * SubPixelI64::new(MULTIPLIER)
}

pub type EntityId = u32;

#[derive(Default)]
struct EntityIdAllocator {
    next: u32,
}

impl EntityIdAllocator {
    fn allocate(&mut self) -> EntityId {
        let id = self.next;
        self.next += 1;
        id
    }
    fn reset(&mut self) {
        self.next = 0;
    }
}

#[derive(Debug)]
struct SpatialInfo {
    entity_id: EntityId,
}

type SpatialLooseQuadTree = LooseQuadTree<SpatialInfo, SubPixelI64>;

pub struct RenderUpdate<'a> {
    pub position: Vector2<SubPixelI64>,
    pub shape: &'a Shape<SubPixelI64>,
    pub colour: [f32; 3],
}

pub struct GameState {
    player_id: Option<EntityId>,
    entity_id_allocator: EntityIdAllocator,
    position: FnvHashMap<EntityId, Vector2<SubPixelI64>>,
    shape: FnvHashMap<EntityId, Shape<SubPixelI64>>,
    colour: FnvHashMap<EntityId, [f32; 3]>,
    velocity: FnvHashMap<EntityId, Vector2<SubPixelI64>>,
    quad_tree: SpatialLooseQuadTree,
}

impl GameState {
    pub fn new(size_hint: Vector2<f32>) -> Self {
        Self {
            player_id: None,
            entity_id_allocator: Default::default(),
            position: Default::default(),
            shape: Default::default(),
            colour: Default::default(),
            velocity: Default::default(),
            quad_tree: LooseQuadTree::new(vec2(
                SubPixelI64::new_pixels_f32(size_hint.x),
                SubPixelI64::new_pixels_f32(size_hint.y),
            )),
        }
    }
    fn clear(&mut self) {
        self.player_id = None;
        self.entity_id_allocator.reset();
        self.position.clear();
        self.shape.clear();
        self.colour.clear();
        self.velocity.clear();
    }
    pub fn init_demo(&mut self) {
        self.clear();
        let player_id = self.entity_id_allocator.allocate();
        self.player_id = Some(player_id);
        self.position.insert(
            player_id,
            vec2(
                SubPixelI64::new_pixels_f32(32.),
                SubPixelI64::new_pixels_f32(64.),
            ),
        );

        self.shape.insert(
            player_id,
            Shape::AxisAlignedRect(AxisAlignedRect::new(vec2(
                SubPixelI64::new_pixels_f32(32.),
                SubPixelI64::new_pixels_f32(64.),
            ))),
        );
        self.colour.insert(player_id, [1., 0., 0.]);
        self.velocity.insert(
            player_id,
            vec2(
                SubPixelI64::new_pixels_f32(0.),
                SubPixelI64::new_pixels_f32(0.),
            ),
        );
    }
    pub fn render_updates(&self) -> impl Iterator<Item = RenderUpdate> {
        let position = &self.position;
        position.iter().filter_map(move |(id, &position)| {
            self.shape.get(id).and_then(|shape| {
                self.colour.get(id).map(|&colour| RenderUpdate {
                    position,
                    shape,
                    colour,
                })
            })
        })
    }
    pub fn update(&mut self, input_model: &InputModel) {
        let player_id = self.player_id.expect("No player id");
        if let Some(velocity) = self.velocity.get_mut(&player_id) {
            *velocity = update_player_velocity(*velocity, input_model);
        }
        for (id, velocity) in self.velocity.iter() {
            if let Some(position) = self.position.get_mut(id) {
                *position += *velocity;
            }
        }
    }
}
