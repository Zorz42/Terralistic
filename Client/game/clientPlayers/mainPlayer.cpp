#include "clientPlayers.hpp"

void ClientPlayers::init() {
    blocks->view_x = main_player.x;
    blocks->view_y = main_player.y;
    
    sf::Packet join_packet;
    join_packet << main_player.name;
    manager->sendPacket(join_packet);
    
    sf::Packet packet;
    packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH);
    manager->sendPacket(packet);
}

#define WALK_SPEED 20
#define SNEAK_SPEED 10
#define JUMP_VELOCITY 70

bool ClientPlayers::isPlayerColliding() {
    if(main_player.x < getPlayerWidth() / 2 || main_player.y < getPlayerHeight() / 2 ||
       main_player.y >= blocks->getWorldHeight() * BLOCK_WIDTH - getPlayerHeight() / 2 ||
       main_player.x >= blocks->getWorldWidth() * BLOCK_WIDTH - getPlayerWidth() / 2)
        return true;

    unsigned short starting_x = (main_player.x - getPlayerWidth() / 2) / BLOCK_WIDTH;
    unsigned short starting_y = (main_player.y - getPlayerHeight() / 2) / BLOCK_WIDTH;
    unsigned short ending_x = (main_player.x + getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
    unsigned short ending_y = (main_player.y + getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            if(blocks->getChunk(x >> 4, y >> 4).getState() != ChunkState::loaded || !blocks->getBlock(x, y).getBlockInfo().ghost)
                return true;
    
    return false;
}

bool ClientPlayers::isPlayerTouchingGround() {
    main_player.y++;
    bool result = isPlayerColliding();
    main_player.y--;
    return result;
}

void ClientPlayers::update() {
    if(received_spawn_coords) {
        if(getKeyState(gfx::Key::SHIFT))
            main_player.moving_type = MovingType::SNEAKING;
        else if(!getKeyState(gfx::Key::D) && !getKeyState(gfx::Key::A))
            main_player.moving_type = MovingType::STANDING;
        else
            main_player.moving_type = MovingType::WALKING;
        
        
        if((getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::WALKING) != walking_right) {
            walking_right = getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::WALKING;
            if(walking_right) {
                main_player.velocity_x += WALK_SPEED;
                if(!main_player.started_walking)
                    main_player.started_walking = gfx::getTicks();
            } else
                main_player.velocity_x -= WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::WALKING) != walking_left) {
            walking_left = getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::WALKING;
            if(walking_left) {
                main_player.velocity_x -= WALK_SPEED;
                if(!main_player.started_walking)
                    main_player.started_walking = gfx::getTicks();
            } else
                main_player.velocity_x += WALK_SPEED;
        }
        
        if((getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::SNEAKING) != sneaking_right) {
            sneaking_right = getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::SNEAKING;
            if(sneaking_right) {
                main_player.velocity_x += SNEAK_SPEED;
                if(!main_player.started_walking)
                    main_player.started_walking = gfx::getTicks();
            } else
                main_player.velocity_x -= SNEAK_SPEED;
        }
        
        if((getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::SNEAKING) != sneaking_left) {
            sneaking_left = getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::SNEAKING;
            if(sneaking_left) {
                main_player.velocity_x -= SNEAK_SPEED;
                if(!main_player.started_walking)
                    main_player.started_walking = gfx::getTicks();
            } else
                main_player.velocity_x += SNEAK_SPEED;
        }
        
        
        if(getKeyState(gfx::Key::SPACE) && isPlayerTouchingGround()) {
            main_player.velocity_y = -JUMP_VELOCITY;
            main_player.has_jumped = true;
        }
            
        if(main_player.velocity_x)
            main_player.flipped = main_player.velocity_x < 0;
        
        main_player.y--;
        if(main_player.velocity_y < 0 && isPlayerColliding())
            main_player.velocity_y = 0;
        main_player.y++;
        
        static unsigned short prev_width = gfx::getWindowWidth(), prev_height = gfx::getWindowHeight();
        if(prev_width != gfx::getWindowWidth() || prev_height != gfx::getWindowHeight()) {
            sf::Packet packet;
            packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / BLOCK_WIDTH) << (unsigned short)(gfx::getWindowHeight() / BLOCK_WIDTH);
            manager->sendPacket(packet);

            prev_width = gfx::getWindowWidth();
            prev_height = gfx::getWindowHeight();
        }
        
        float speed_multiplier = 1;
        
        int prev_x = main_player.x, prev_y = main_player.y, prev_view_x = blocks->view_x, prev_view_y = blocks->view_y;
        
        unsigned short starting_x = (main_player.x - getPlayerWidth() / 2) / BLOCK_WIDTH;
        unsigned short starting_y = (main_player.y - getPlayerHeight() / 2) / BLOCK_WIDTH;
        unsigned short ending_x = (main_player.x + getPlayerWidth() / 2 - 1) / BLOCK_WIDTH;
        unsigned short ending_y = (main_player.y + getPlayerHeight() / 2 - 1) / BLOCK_WIDTH;
        
        for(unsigned short x = starting_x; x <= ending_x; x++)
            for(unsigned short y = starting_y; y <= ending_y; y++)
                speed_multiplier = std::min(speed_multiplier, blocks->getBlock(x, y).getLiquidInfo().speed_multiplier);
        
        main_player.velocity_y += gfx::getDeltaTime() / 4 * speed_multiplier;
        if(isPlayerTouchingGround() && main_player.velocity_y >= 0)
            main_player.velocity_y = 0;
        
        int move_x = float(main_player.velocity_x * gfx::getDeltaTime() / 100) * speed_multiplier;
        int x_factor = move_x > 0 ? 1 : -1;
        for(int i = 0; i < abs(move_x); i++) {
            main_player.x += x_factor;
            if(isPlayerColliding() || (main_player.moving_type == MovingType::SNEAKING && !isPlayerTouchingGround() && !main_player.velocity_y)) {
                main_player.x -= x_factor;
                break;
            }
        }
        
        
        int move_y = float(main_player.velocity_y * gfx::getDeltaTime() / 100) * speed_multiplier;
        int y_factor = move_y > 0 ? 1 : -1;
        for(int i = 0; i < abs(move_y); i++) {
            main_player.y += y_factor;
            if(isPlayerColliding()) {
                main_player.y -= y_factor;
                break;
            }
        }
        
        blocks->view_x += (main_player.x - blocks->view_x) / 8;
        blocks->view_y += (main_player.y - blocks->view_y) / 8;
        if(blocks->view_x < gfx::getWindowWidth() / 2)
            blocks->view_x = gfx::getWindowWidth() / 2;
        if(blocks->view_y < gfx::getWindowHeight() / 2)
            blocks->view_y = gfx::getWindowHeight() / 2;
        if(blocks->view_x >= blocks->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2)
            blocks->view_x = blocks->getWorldWidth() * BLOCK_WIDTH - gfx::getWindowWidth() / 2;
        if(blocks->view_y >= blocks->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2)
            blocks->view_y = blocks->getWorldHeight() * BLOCK_WIDTH - gfx::getWindowHeight() / 2;
        
        if(prev_x != main_player.x || prev_y != main_player.y) {
            sf::Packet packet;
            packet << PacketType::PLAYER_MOVEMENT << main_player.x << main_player.y << main_player.flipped;
            manager->sendPacket(packet);
        }
        
        if(prev_view_x != blocks->view_x || prev_view_y != blocks->view_y) {
            sf::Packet packet;
            packet << PacketType::VIEW_POS_CHANGE << blocks->view_x << blocks->view_y;
            manager->sendPacket(packet);
        }
        
        if(isPlayerTouchingGround() && main_player.velocity_y == 0)
            main_player.has_jumped = false;
        
        if(!main_player.velocity_x && main_player.texture_frame == 3)
            main_player.started_walking = 0;
        
        main_player.texture_frame = 0;
        
        if(main_player.started_walking)
            main_player.texture_frame = std::abs(int(gfx::getTicks() - main_player.started_walking) / 50 % 9 - 4) + 3;
        
        if(main_player.has_jumped)
            main_player.texture_frame = 1;
        
        if(main_player.moving_type == MovingType::SNEAKING)
            main_player.texture_frame = 2;
    }
}
