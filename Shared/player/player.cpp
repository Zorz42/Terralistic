#include "player.hpp"

int Player::getWidth() {
    return PLAYER_WIDTH * 2;
}
int Player::getHeight() {
    return PLAYER_HEIGHT * 2;
}

bool Player::isColliding(Blocks* blocks) {
    return isCollidingWithBlocks(blocks) ||
    (
     moving_type == MovingType::SNEAK_WALKING && isCollidingWithBlocks(blocks, getX(), getY() + 1) &&
     (!isCollidingWithBlocks(blocks, getX() + 1, getY() + 1) || !isCollidingWithBlocks(blocks, getX() - 1, getY() + 1))
     );
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
