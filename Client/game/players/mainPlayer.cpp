#include "clientPlayers.hpp"

void ClientPlayers::init() {
    manager->packet_event.addListener(this);
}

void ClientPlayers::stop() {
    manager->packet_event.removeListener(this);
}

#define RUN_SPEED 18
#define WALK_SPEED 10
#define SNEAK_SPEED 4
#define JUMP_VELOCITY 50

void ClientPlayers::update(float frame_length) {
    if(main_player) {
        static int prev_x, prev_y;
        float vel_x_change = 0, vel_y_change = 0;
        
        MovingType prev_moving_type = main_player->moving_type;
        
        if(getKeyState(gfx::Key::SHIFT)) {
            if(getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D))
                main_player->moving_type = MovingType::SNEAK_WALKING;
            else
                main_player->moving_type = MovingType::SNEAKING;
        } else if(getKeyState(gfx::Key::CTRL) && (getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D)))
            main_player->moving_type = MovingType::RUNNING;
        else if(getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D))
            main_player->moving_type = MovingType::WALKING;
        else
            main_player->moving_type = MovingType::STANDING;
        
        
        if((getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::WALKING) != walking_right) {
            walking_right = getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::WALKING;
            if(walking_right) {
                vel_x_change += WALK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change -= WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::WALKING) != walking_left) {
            walking_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::WALKING;
            if(walking_left) {
                vel_x_change -= WALK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change += WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::SNEAK_WALKING) != sneaking_right) {
            sneaking_right = getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::SNEAK_WALKING;
            if(sneaking_right) {
                vel_x_change += SNEAK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change -= SNEAK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::SNEAK_WALKING) != sneaking_left) {
            sneaking_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::SNEAK_WALKING;
            if(sneaking_left) {
                vel_x_change -= SNEAK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change += SNEAK_SPEED;
        }
        
        if((getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::RUNNING) != running_right) {
            running_right = getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::RUNNING;
            if(running_right) {
                vel_x_change += RUN_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change -= RUN_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::RUNNING) != running_left) {
            running_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::RUNNING;
            if(running_left) {
                vel_x_change -= RUN_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = gfx::getTicks();
            } else
                vel_x_change += RUN_SPEED;
        }
        
        
        if(getKeyState(gfx::Key::SPACE) && main_player->isTouchingGround(blocks)) {
            vel_y_change -= JUMP_VELOCITY;
            main_player->has_jumped = true;
            sf::Packet packet;
            packet << PacketType::PLAYER_JUMPED;
            manager->sendPacket(packet);
        }
        
        float speed_multiplier = 1;
        
        int starting_x = (main_player->getX()) / (BLOCK_WIDTH * 2);
        int starting_y = (main_player->getY()) / (BLOCK_WIDTH * 2);
        int ending_x = (main_player->getX() + PLAYER_WIDTH * 2 - 1) / (BLOCK_WIDTH * 2);
        int ending_y = (main_player->getY() + PLAYER_HEIGHT * 2 - 1) / (BLOCK_WIDTH * 2);
        
        for(int x = starting_x; x <= ending_x; x++)
            for(int y = starting_y; y <= ending_y; y++)
                speed_multiplier = std::min(speed_multiplier, liquids->getLiquidInfo(x, y).speed_multiplier);
        
        blocks->view_x += (main_player->getX() - blocks->view_x + PLAYER_WIDTH) / 8;
        blocks->view_y += (main_player->getY() - blocks->view_y + PLAYER_HEIGHT) / 8;
        if(blocks->view_x < gfx::getWindowWidth() / 2)
            blocks->view_x = gfx::getWindowWidth() / 2;
        if(blocks->view_y < gfx::getWindowHeight() / 2)
            blocks->view_y = gfx::getWindowHeight() / 2;
        if(blocks->view_x >= blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2)
            blocks->view_x = blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2;
        if(blocks->view_y >= blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2)
            blocks->view_y = blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2;
        
        if(vel_x_change || vel_y_change) {
            entities->addVelocityX(main_player, vel_x_change);
            entities->addVelocityY(main_player, vel_y_change);
            
            sf::Packet packet;
            packet << PacketType::PLAYER_VELOCITY << main_player->getVelocityX() << main_player->getVelocityY();
            manager->sendPacket(packet);
        }
        
        if(prev_moving_type != main_player->moving_type) {
            sf::Packet packet;
            packet << PacketType::PLAYER_MOVING_TYPE << (unsigned char)main_player->moving_type;
            manager->sendPacket(packet);
        }
        
        prev_x = main_player->getX();
        prev_y = main_player->getY();
    }
}
