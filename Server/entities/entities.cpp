#include "serverEntities.hpp"

void ServerEntities::updateAllEntities(float frame_length) {
    for(ServerEntity* entity : entities)
        entity->updateEntity(blocks, frame_length);
}

void ServerEntities::registerEntity(ServerEntity* entity) {
    entities.push_back(entity);
}

void ServerEntities::removeEntity(ServerEntity *entity) {
    ServerEntityDeletionEvent event(*entity);
    event.call();
    entities.erase(std::find(entities.begin(), entities.end(), entity));
}

void ServerEntity::updateEntity(ServerBlocks* blocks, float frame_length) {
    if(gravity)
        velocity_y += frame_length / 5.f;
    
    if(friction) {
        velocity_y *= 0.99f;
        velocity_x *= isTouchingGround(blocks) ? 0.9f : 0.99f;
    }
    
    float y_to_be = y + float(velocity_y * frame_length) / 100;
    float move_y = y_to_be - y;
    int y_factor = move_y > 0 ? 1 : -1;
    for(int i = 0; i < std::abs(move_y); i++) {
        y += y_factor;
        if(isColliding(blocks)) {
            y -= y_factor;
            velocity_y = 0;
            break;
        }
    }
    if(velocity_y)
        y = y_to_be;
    
    float x_to_be = x + float(velocity_x * frame_length) / 100;
    float move_x = x_to_be - x;
    int x_factor = move_x > 0 ? 1 : -1;
    bool has_collided_x = false;
    for(int i = 0; i < std::abs(move_x); i++) {
        x += x_factor;
        if(isColliding(blocks)) {
            x -= x_factor;
            has_collided_x = true;
            break;
        }
    }
    if(!has_collided_x)
        x = x_to_be;
}

bool ServerEntity::isTouchingGround(ServerBlocks* blocks) {
    y++;
    bool is_touching_ground = isColliding(blocks);
    y--;
    return is_touching_ground;
}

bool ServerEntity::isColliding(ServerBlocks* blocks) {
    if(x < 0 || y < 0 ||
       y >= blocks->getHeight() * BLOCK_WIDTH * 2 - getHeight() ||
       x >= blocks->getWidth() * BLOCK_WIDTH * 2 - getWidth())
        return true;

    unsigned short starting_x = x / (BLOCK_WIDTH * 2);
    unsigned short starting_y = y / (BLOCK_WIDTH * 2);
    unsigned short ending_x = (x + getWidth() - 1) / (BLOCK_WIDTH * 2);
    unsigned short ending_y = (y + getHeight() - 1) / (BLOCK_WIDTH * 2);
    
    for(unsigned short x_ = starting_x; x_ <= ending_x; x_++)
        for(unsigned short y_ = starting_y; y_ <= ending_y; y_++)
            if(!blocks->getBlock(x_, y_).getBlockInfo().ghost)
                return true;
    
    return false;
}

void ServerEntity::addVelocityX(float vel_x) {
    velocity_x += vel_x;
    ServerEntityVelocityChangeEvent event(*this);
    event.call();
}

void ServerEntity::addVelocityY(float vel_y) {
    velocity_y += vel_y;
    ServerEntityVelocityChangeEvent event(*this);
    event.call();
}

void ServerEntity::setVelocityX(float vel_x) {
    velocity_x = vel_x;
    ServerEntityVelocityChangeEvent event(*this);
    event.call();
}

void ServerEntity::setVelocityY(float vel_y) {
    velocity_y = vel_y;
    ServerEntityVelocityChangeEvent event(*this);
    event.call();
}
