#include "clientPlayers.hpp"
#include "readOpa.hpp"

void ClientPlayers::init() {
    networking->packet_event.addListener(this);
}

void ClientPlayers::loadTextures() {
    loadOpa(player_texture, resource_pack->getFile("/misc/player.opa"));
}

void ClientPlayers::stop() {
    networking->packet_event.removeListener(this);
}

#define RUN_SPEED 18
#define WALK_SPEED 10
#define SNEAK_SPEED 4
#define JUMP_VELOCITY 50

void ClientPlayers::updateParallel(float frame_length) {
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
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change -= WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::WALKING) != walking_left) {
            walking_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::WALKING;
            if(walking_left) {
                vel_x_change -= WALK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change += WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::SNEAK_WALKING) != sneaking_right) {
            sneaking_right = getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::SNEAK_WALKING;
            if(sneaking_right) {
                vel_x_change += SNEAK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change -= SNEAK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::SNEAK_WALKING) != sneaking_left) {
            sneaking_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::SNEAK_WALKING;
            if(sneaking_left) {
                vel_x_change -= SNEAK_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change += SNEAK_SPEED;
        }
        
        if((getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::RUNNING) != running_right) {
            running_right = getKeyState(gfx::Key::D) && main_player->moving_type == MovingType::RUNNING;
            if(running_right) {
                vel_x_change += RUN_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change -= RUN_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::RUNNING) != running_left) {
            running_left = getKeyState(gfx::Key::A) && main_player->moving_type == MovingType::RUNNING;
            if(running_left) {
                vel_x_change -= RUN_SPEED;
                if(!main_player->started_moving)
                    main_player->started_moving = timer.getTimeElapsed();
            } else
                vel_x_change += RUN_SPEED;
        }
        
        
        if(getKeyState(gfx::Key::SPACE) && main_player->isTouchingGround(blocks)) {
            vel_y_change -= JUMP_VELOCITY;
            main_player->has_jumped = true;
            Packet packet;
            packet << ClientPacketType::PLAYER_JUMPED;
            networking->sendPacket(packet);
        }
        
        float speed_multiplier = 1;
        
        int starting_x = (main_player->getX()) / (BLOCK_WIDTH * 2);
        int starting_y = (main_player->getY()) / (BLOCK_WIDTH * 2);
        int ending_x = (main_player->getX() + PLAYER_WIDTH * 2 - 1) / (BLOCK_WIDTH * 2);
        int ending_y = (main_player->getY() + PLAYER_HEIGHT * 2 - 1) / (BLOCK_WIDTH * 2);
        
        for(int x = starting_x; x <= ending_x; x++)
            for(int y = starting_y; y <= ending_y; y++)
                speed_multiplier = std::min(speed_multiplier, liquids->getLiquidType(x, y)->speed_multiplier);
        
        camera->setX(main_player->getX() + PLAYER_WIDTH);
        camera->setY(main_player->getY() + PLAYER_HEIGHT);
        
        if(camera->getTargetX() < gfx::getWindowWidth() / 2)
            camera->setX(gfx::getWindowWidth() / 2);
        if(camera->getTargetY() < gfx::getWindowHeight() / 2)
            camera->setY(gfx::getWindowHeight() / 2);
        if(camera->getTargetX() >= blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2)
            camera->setX(blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2);
        if(camera->getTargetY() >= blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2)
            camera->setY(blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2);
        
        if(vel_x_change || vel_y_change) {
            entities->addVelocityX(main_player, vel_x_change);
            entities->addVelocityY(main_player, vel_y_change);
            
            Packet packet;
            packet << ClientPacketType::PLAYER_VELOCITY << main_player->getVelocityX() << main_player->getVelocityY();
            networking->sendPacket(packet);
        }
        
        if(prev_moving_type != main_player->moving_type) {
            Packet packet;
            packet << ClientPacketType::PLAYER_MOVING_TYPE << (int)main_player->moving_type;
            networking->sendPacket(packet);
        }
        
        prev_x = main_player->getX();
        prev_y = main_player->getY();
    } else {
        walking_left = false;
        walking_right = false;
        sneaking_left = false;
        sneaking_right = false;
        running_left = false;
        running_right = false;
    }
}
