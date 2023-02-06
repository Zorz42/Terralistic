use super::{blocks::*, entities::*};

//TODO: write tests

const PLAYER_HEIGHT: i32 = 24;
const PLAYER_WIDTH: i32 = 16;
pub const PLAYER_MAX_HEALTH: i32 = 80;

/**enum of possible movement types for the player*/
#[derive(PartialEq, Copy, Clone)]
pub enum MovingType {
    Standing,
    Walking,
    Sneaking,
    SneakWalking,
    Running,
}

/**event that is fired when the player's health changes*/
pub struct PlayerHealthChangeEvent {
    pub player_id: u32,
    /**health before the change*/
    pub old_health: i32,
}

/**struct with all the information about the player*/
pub struct Player {
    entity: Entity,
    health: i32,
    pub name: String,
    pub flipped: bool,
    pub moving_type: MovingType,
}

impl Player {
    pub fn new(x: i32, y: i32, name: String, health: i32, id: u32) -> Player {
        Player {
            entity: Entity::new(EntityType::PLAYER, x, y, id),
            health,
            name,
            flipped: false,
            moving_type: MovingType::Standing,
        }
    }
    /**sets the player's health to the given value*/
    pub fn set_health(&mut self, health: i32) {
        self.health = health;
    }
    /**returns the player's health*/
    pub fn get_health(&self) -> i32 {
        self.health
    }
    pub fn get_id(&self) -> u32 {
        self.entity.id
    }
}
impl EntityObject for Player {
    fn get_width(&self) -> i32 {
        PLAYER_WIDTH * 2
    }
    fn get_height(&self) -> i32 {
        PLAYER_HEIGHT * 2
    }
    fn is_colliding(
        &self,
        blocks: &Blocks,
        direction: Direction,
        colliding_x: f32,
        colliding_y: f32,
    ) -> bool {
        let mut result = self.is_colliding_with_block(blocks, direction, colliding_x, colliding_y);

        if !result
            && self.moving_type == MovingType::SneakWalking
            && self.is_colliding_with_block(
                blocks,
                Direction::DOWN,
                self.get_x(),
                self.get_y() + 1.0,
            )
            && (!self.is_colliding_with_block(
                blocks,
                Direction::DOWN,
                self.get_x() + 1.0,
                self.get_y() + 1.0,
            ) || !self.is_colliding_with_block(
                blocks,
                Direction::DOWN,
                self.get_x() - 1.0,
                self.get_y() + 1.0,
            ))
        {
            result = false;
        }

        let starting_x = (colliding_x / (BLOCK_WIDTH as f32 * 2.0)) as i32;
        let ending_x =
            ((colliding_x + self.get_width() as f32 - 1.0) / (BLOCK_WIDTH as f32 * 2.0)) as i32;
        let ending_y =
            ((colliding_y + self.get_height() as f32 - 1.0) / (BLOCK_WIDTH as f32 * 2.0)) as i32;

        if !result
            && (colliding_y as i32 + self.get_height()) % (BLOCK_WIDTH * 2) == 1
            && direction == Direction::DOWN
            && (self.get_velocity_y() > 3.0 || self.moving_type != MovingType::Sneaking)
        {
            for x in starting_x..=ending_x {
                if blocks
                    .get_block_type_at(x, ending_y)
                    .unwrap()
                    .feet_collidable
                {
                    result = true;
                    break;
                }
            }
        }
        result
    }
    fn is_colliding_with_block(
        &self,
        blocks: &Blocks,
        direction: Direction,
        colliding_x: f32,
        colliding_y: f32,
    ) -> bool {
        self.entity
            .is_colliding_with_block(blocks, direction, colliding_x, colliding_y)
    }
    fn update_entity(&mut self, blocks: &Blocks) {
        self.entity.update_entity(blocks);
    }
    fn is_touching_ground(&self, blocks: &Blocks) -> bool {
        self.entity.is_touching_ground(blocks)
    }
    fn get_x(&self) -> f32 {
        self.entity.get_x()
    }
    fn get_y(&self) -> f32 {
        self.entity.get_y()
    }
    fn get_velocity_x(&self) -> f32 {
        self.entity.get_velocity_x()
    }
    fn get_velocity_y(&self) -> f32 {
        self.entity.get_velocity_y()
    }
}

pub struct Players {
    players: Vec<Player>,
    //player_health_change_event: Sender<PlayerHealthChangeEvent>,
    //player_position_change_event: Sender<EntityPositionChangeEvent>,
    //player_velocity_change_event: Sender<EntityVelocityChangeEvent>,
    //player_absolute_velocity_change_event: Sender<EntityAbsoluteVelocityChangeEvent>,
    //player_deletion_event: Sender<EntityDeletionEvent>
}

impl Default for Players {
    fn default() -> Self {
        Self::new()
    }
}

impl Players {
    pub fn new() -> Players {
        Players {
            players: Vec::new(),
            //player_health_change_event: Sender::new(),
            //player_position_change_event: Sender::new(),
            //player_velocity_change_event: Sender::new(),
            //player_absolute_velocity_change_event: Sender::new(),
            //player_deletion_event: Sender::new()
        }
    }
    /**this function sets the player's health to the given value*/
    pub fn set_health(&mut self, player_id: u32, health: i32) {
        for player in &mut self.players {
            if player.entity.id == player_id {
                player.set_health(health);
                //self.player_health_change_event.send(PlayerHealthChangeEvent::new(player_id, health));
                break;
            }
        }
    }
    /**this function returns the player's health*/
    pub fn get_health(&self, player_id: u32) -> i32 {
        for player in &self.players {
            if player.entity.id == player_id {
                return player.get_health();
            }
        }
        0
    }
}

impl EntityStructTrait<Player> for Players {
    fn update_all_entities(&mut self, blocks: &Blocks) {
        for entity in &mut self.players {
            let old_vel_x = entity.get_velocity_x();
            let old_vel_y = entity.get_velocity_y();
            entity.update_entity(blocks);
            if old_vel_x != entity.get_velocity_x() || old_vel_y != entity.get_velocity_y() {
                let _event =
                    EntityAbsoluteVelocityChangeEvent::new(entity.entity.id, old_vel_x, old_vel_y);
                //self.player_absolute_velocity_change_event.send(event);
            }
        }
    }
    fn register_entity(&mut self, entity: Player) {
        self.players.push(entity);
    }
    fn remove_entity(&mut self, entity_id: u32) {
        let pos = self
            .players
            .iter()
            .position(|entity| entity.entity.id == entity_id);
        if pos.is_none() {
            return;
        }
        let _event = EntityDeletionEvent::new(entity_id);
        //self.player_deletion_event.send(event);
        self.players.remove(pos.unwrap());
    }
    fn get_entity_by_id(&self, entity_id: u32) -> Option<&Player> {
        self.players
            .iter()
            .find(|entity| entity.entity.id == entity_id)
    }
    fn get_entity_by_id_mut(&self, entity_id: u32) -> Option<&Player> {
        self.players
            .iter()
            .find(|entity| entity.entity.id == entity_id)
    }
    fn get_entities(&self) -> &Vec<Player> {
        &self.players
    }
    fn set_velocity_x(&mut self, entity: &mut Player, velocity_x: f32) {
        if entity.entity.get_velocity_x() != velocity_x {
            entity.entity.velocity_x = velocity_x;
            let _event = EntityVelocityChangeEvent::new(entity.entity.id);
            //self.player_velocity_change_event.send(event);
        }
    }
    fn set_velocity_y(&mut self, entity: &mut Player, velocity_y: f32) {
        if entity.entity.get_velocity_y() != velocity_y {
            entity.entity.velocity_y = velocity_y;
            let _event = EntityVelocityChangeEvent::new(entity.entity.id);
            //self.player_velocity_change_event.send(event);
        }
    }
    fn add_velocity_x(&mut self, entity: &mut Player, velocity_x: f32) {
        self.set_velocity_x(entity, entity.entity.get_velocity_x() + velocity_x);
    }
    fn add_velocity_y(&mut self, entity: &mut Player, velocity_y: f32) {
        self.set_velocity_y(entity, entity.entity.get_velocity_y() + velocity_y);
    }
    fn set_x(&mut self, entity: &mut Player, x: f32, send_to_everyone: bool) {
        if entity.entity.get_x() != x {
            entity.entity.x = x;
            if send_to_everyone {
                let _event = EntityPositionChangeEvent::new(entity.entity.id);
                //self.player_position_change_event.send(event);
            }
        }
    }
    fn set_y(&mut self, entity: &mut Player, y: f32, send_to_everyone: bool) {
        if entity.entity.get_y() != y {
            entity.entity.y = y;
            if send_to_everyone {
                let _event = EntityPositionChangeEvent::new(entity.entity.id);
                //self.player_position_change_event.send(event);
            }
        }
    }
}
