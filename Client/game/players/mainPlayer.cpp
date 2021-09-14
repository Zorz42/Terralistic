#include "clientPlayers.hpp"

void ClientPlayers::init() {
    sf::Packet packet;
    packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / (BLOCK_WIDTH * 2)) << (unsigned short)(gfx::getWindowHeight() / (BLOCK_WIDTH * 2));
    manager->sendPacket(packet);
}

#define RUN_SPEED 18
#define WALK_SPEED 10
#define SNEAK_SPEED 4
#define JUMP_VELOCITY 50

void ClientPlayers::setMainPlayerPosition(int x, int y) {
    main_player.x = x;
    main_player.y = y;
}

bool ClientPlayers::isPlayerColliding() {
    if(main_player.x < 0 || main_player.y < 0 ||
       main_player.y >= blocks->getHeight() * BLOCK_WIDTH * 2 - PLAYER_HEIGHT * 2 ||
       main_player.x >= blocks->getWidth() * BLOCK_WIDTH * 2 - PLAYER_WIDTH * 2)
        return true;

    unsigned short starting_x = (main_player.x) / (BLOCK_WIDTH * 2);
    unsigned short starting_y = (main_player.y) / (BLOCK_WIDTH * 2);
    unsigned short ending_x = (main_player.x + PLAYER_WIDTH * 2 - 1) / (BLOCK_WIDTH * 2);
    unsigned short ending_y = (main_player.y + PLAYER_HEIGHT * 2 - 1) / (BLOCK_WIDTH * 2);
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            if(!blocks->getBlock(x, y).getBlockInfo().ghost)
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
    static int prev_x, prev_y, prev_view_x, prev_view_y;
    
    if(getKeyState(gfx::Key::SHIFT)) {
        if(getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D))
            main_player.moving_type = MovingType::SNEAK_WALKING;
        else
            main_player.moving_type = MovingType::SNEAKING;
    } else if(getKeyState(gfx::Key::CTRL) && (getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D)))
        main_player.moving_type = MovingType::RUNNING;
    else if(getKeyState(gfx::Key::A) || getKeyState(gfx::Key::D))
        main_player.moving_type = MovingType::WALKING;
    else
        main_player.moving_type = MovingType::STANDING;
    
    
    if((getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::WALKING) != walking_right) {
        walking_right = getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::WALKING;
        if(walking_right) {
            main_player.velocity_x += WALK_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x -= WALK_SPEED;
    }
    
    if((getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::WALKING) != walking_left) {
        walking_left = getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::WALKING;
        if(walking_left) {
            main_player.velocity_x -= WALK_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x += WALK_SPEED;
    }
    
    if((getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::SNEAK_WALKING) != sneaking_right) {
        sneaking_right = getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::SNEAK_WALKING;
        if(sneaking_right) {
            main_player.velocity_x += SNEAK_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x -= SNEAK_SPEED;
    }
    
    if((getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::SNEAK_WALKING) != sneaking_left) {
        sneaking_left = getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::SNEAK_WALKING;
        if(sneaking_left) {
            main_player.velocity_x -= SNEAK_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x += SNEAK_SPEED;
    }
    
    if((getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::RUNNING) != running_right) {
        running_right = getKeyState(gfx::Key::D) && main_player.moving_type == MovingType::RUNNING;
        if(running_right) {
            main_player.velocity_x += RUN_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x -= RUN_SPEED;
    }
    
    if((getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::RUNNING) != running_left) {
        running_left = getKeyState(gfx::Key::A) && main_player.moving_type == MovingType::RUNNING;
        if(running_left) {
            main_player.velocity_x -= RUN_SPEED;
            if(!main_player.started_moving)
                main_player.started_moving = gfx::getTicks();
        } else
            main_player.velocity_x += RUN_SPEED;
    }
    
    
    if(getKeyState(gfx::Key::SPACE) && isPlayerTouchingGround()) {
        main_player.velocity_y = -JUMP_VELOCITY;
        main_player.has_jumped = true;
    }
        
    if(main_player.velocity_x)
        main_player.flipped = main_player.velocity_x < 0;
    
    static unsigned short prev_width = gfx::getWindowWidth(), prev_height = gfx::getWindowHeight();
    if(prev_width != gfx::getWindowWidth() || prev_height != gfx::getWindowHeight()) {
        sf::Packet packet;
        packet << PacketType::VIEW_SIZE_CHANGE << (unsigned short)(gfx::getWindowWidth() / (BLOCK_WIDTH * 2)) << (unsigned short)(gfx::getWindowHeight() / (BLOCK_WIDTH * 2));
        manager->sendPacket(packet);

        prev_width = gfx::getWindowWidth();
        prev_height = gfx::getWindowHeight();
    }
    
    float speed_multiplier = 1;
    
    unsigned short starting_x = (main_player.x) / (BLOCK_WIDTH * 2);
    unsigned short starting_y = (main_player.y) / (BLOCK_WIDTH * 2);
    unsigned short ending_x = (main_player.x + PLAYER_WIDTH * 2 - 1) / (BLOCK_WIDTH * 2);
    unsigned short ending_y = (main_player.y + PLAYER_HEIGHT * 2 - 1) / (BLOCK_WIDTH * 2);
    
    for(unsigned short x = starting_x; x <= ending_x; x++)
        for(unsigned short y = starting_y; y <= ending_y; y++)
            speed_multiplier = std::min(speed_multiplier, blocks->getBlock(x, y).getLiquidInfo().speed_multiplier);
    
    blocks->view_x += (main_player.x - blocks->view_x + PLAYER_WIDTH) / 8;
    blocks->view_y += (main_player.y - blocks->view_y + PLAYER_HEIGHT) / 8;
    if(blocks->view_x < gfx::getWindowWidth() / 2)
        blocks->view_x = gfx::getWindowWidth() / 2;
    if(blocks->view_y < gfx::getWindowHeight() / 2)
        blocks->view_y = gfx::getWindowHeight() / 2;
    if(blocks->view_x >= blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2)
        blocks->view_x = blocks->getWidth() * BLOCK_WIDTH * 2 - gfx::getWindowWidth() / 2;
    if(blocks->view_y >= blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2)
        blocks->view_y = blocks->getHeight() * BLOCK_WIDTH * 2 - gfx::getWindowHeight() / 2;
    
    if(prev_x != main_player.x || prev_y != main_player.y) {
        sf::Packet packet;
        packet << PacketType::PLAYER_MOVEMENT << (int)main_player.x << (int)main_player.y << main_player.flipped;
        manager->sendPacket(packet);
    }
    
    if(prev_view_x != blocks->view_x || prev_view_y != blocks->view_y) {
        sf::Packet packet;
        packet << PacketType::VIEW_POS_CHANGE << blocks->view_x << blocks->view_y;
        manager->sendPacket(packet);
    }
    
    bool has_moved_x = prev_x != int(main_player.x);
    
    if(isPlayerTouchingGround() && main_player.velocity_y == 0)
        main_player.has_jumped = false;
    
    if(main_player.moving_type == MovingType::STANDING)
        main_player.started_moving = 0;
    
    if(main_player.moving_type == MovingType::SNEAKING)
        main_player.texture_frame = 10;
    else if(main_player.has_jumped)
        main_player.texture_frame = 0;
    else if(main_player.moving_type == MovingType::WALKING && has_moved_x)
        main_player.texture_frame = (gfx::getTicks() - main_player.started_moving) / 70 % 9 + 1;
    else if(main_player.moving_type == MovingType::SNEAK_WALKING) {
        if(has_moved_x)
            main_player.texture_frame = (gfx::getTicks() - main_player.started_moving) / 150 % 6 + 10;
        else
            main_player.texture_frame = 10;
    } else if(main_player.moving_type == MovingType::RUNNING && has_moved_x)
        main_player.texture_frame = (gfx::getTicks() - main_player.started_moving) / 80 % 8 + 16;
    else
        main_player.texture_frame = 1;
    
    prev_x = main_player.x;
    prev_y = main_player.y;
    prev_view_x = blocks->view_x;
    prev_view_y = blocks->view_y;
}
