#pragma once
#include "entities.hpp"

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14

enum class MovingType {STANDING, WALKING, SNEAKING, SNEAK_WALKING, RUNNING};

class Player : public Entity {
    bool isColliding(Blocks* blocks) override;
public:
    Player(int x, int y, const std::string& name, short health, int id=0) : Entity(EntityType::PLAYER, x, y, id), name(name), health(health) {}
    
    int getWidth() override;
    int getHeight() override;

    const std::string name;
    bool flipped = false;
    short health;
    int getHealth();

    MovingType moving_type = MovingType::STANDING;
};
