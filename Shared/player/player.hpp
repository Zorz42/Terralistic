#pragma once
#include <utility>

#include "entities.hpp"

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14
#define PLAYER_MAX_HEALTH 80

enum class MovingType {STANDING, WALKING, SNEAKING, SNEAK_WALKING, RUNNING};

class Player;

class PlayerHealthChangeEvent {
public:
    PlayerHealthChangeEvent(Player* player, int prev_health) : player(player), prev_health(prev_health) {}
    Player* player;
    int prev_health;
};

class Player : public Entity {
    bool isColliding(Blocks* blocks) override;
    int health;
public:
    Player(int x, int y, std::string name, int health, int id=0) : Entity(EntityType::PLAYER, x, y, id), name(std::move(name)), health(health) {}
    
    int getWidth() override;
    int getHeight() override;

    void setHealth(int health);
    int getHealth() const;
    
    const std::string name;
    bool flipped = false;
    
    EventSender<PlayerHealthChangeEvent> health_change_event;
    
    MovingType moving_type = MovingType::STANDING;
};
