#pragma once
#include "entities.hpp"

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14

enum class MovingType {STANDING, WALKING, SNEAKING, SNEAK_WALKING, RUNNING};

class Player : public Entity {
    int getWidth() override;
    int getHeight() override;
    
    bool isColliding(Blocks* blocks) override;
public:
    Player(int x, int y, const std::string& name, int id=0) : Entity(EntityType::PLAYER, x, y, id), name(name) {}
    
    const std::string name;
    
    MovingType moving_type = MovingType::STANDING;
};
