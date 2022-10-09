#include "player.hpp"

int Player::getWidth() {
    return PLAYER_WIDTH * 2;
}
int Player::getHeight() {
    return PLAYER_HEIGHT * 2;
}

bool Player::isColliding(Blocks* blocks, Direction direction, float colliding_x, float colliding_y) {
    bool result = isCollidingWithBlocks(blocks, direction, colliding_x, colliding_y);
    
    if(!result && moving_type == MovingType::SNEAK_WALKING && isCollidingWithBlocks(blocks, Direction::DOWN, getX(), getY() + 1) &&
       (!isCollidingWithBlocks(blocks, Direction::DOWN, getX() + 1, getY() + 1) || !isCollidingWithBlocks(blocks, Direction::DOWN, getX() - 1, getY() + 1)))
        result = false;
    
    int starting_x = colliding_x / (BLOCK_WIDTH * 2);
    int ending_x = (colliding_x + getWidth() - 1) / (BLOCK_WIDTH * 2);
    int ending_y = (colliding_y + getHeight() - 1) / (BLOCK_WIDTH * 2);
    
    if(!result && int(colliding_y + getHeight()) % (BLOCK_WIDTH * 2) == 1 && direction == Direction::DOWN && (getVelocityY() > 3 || moving_type != MovingType::SNEAKING)) {
        for(int x_ = starting_x; x_ <= ending_x; x_++)
            if(blocks->getBlockType(x_, ending_y)->feet_collidable)
                result = true;
    }
    
    return result;
}

void Player::setHealth(int health_) {
    int old_health = health;
    health = health_;
    PlayerHealthChangeEvent event(this, old_health);
    health_change_event.call(event);
}

int Player::getHealth() const {
    return health;
}
