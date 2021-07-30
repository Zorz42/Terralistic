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

//#define VELOCITY 30
//#define JUMP_VELOCITY 80
#define VELOCITY 90
#define JUMP_VELOCITY 240

void ClientPlayers::onKeyDown(gfx::Key key) {
    switch(key) {
        case gfx::Key::SPACE:
            if(!key_up) {
                key_up = true;
                jump = true;
            }
            break;
        case gfx::Key::A:
            if(!key_left) {
                key_left = true;
                main_player.velocity_x -= VELOCITY;
            }
            break;
        case gfx::Key::D:
            if(!key_right) {
                key_right = true;
                main_player.velocity_x += VELOCITY;
            }
            break;
        default:;
    }
}

void ClientPlayers::onKeyUp(gfx::Key key) {
    switch (key) {
        case gfx::Key::SPACE:
            if(key_up) {
                key_up = false;
                jump = false;
                if(main_player.velocity_y < -10)
                    main_player.velocity_y = -10;
            }
            break;
        case gfx::Key::A:
            if(key_left) {
                key_left = false;
                main_player.velocity_x += VELOCITY;
            }
            break;
        case gfx::Key::D:
            if(key_right) {
                key_right = false;
                main_player.velocity_x -= VELOCITY;
            }
            break;
        default:;
    }
}

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
        if(main_player.velocity_x)
            main_player.flipped = main_player.velocity_x < 0;
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
        
        main_player.velocity_y = isPlayerTouchingGround() && main_player.velocity_y >= 0 ? short(0) : short(main_player.velocity_y + gfx::getDeltaTime() / 4 * speed_multiplier);
        
        int move_x = float(main_player.velocity_x * gfx::getDeltaTime() / 100) * speed_multiplier, move_y = float(main_player.velocity_y * gfx::getDeltaTime() / 100) * speed_multiplier;
        
        for(int i = 0; i < move_x; i++) {
            main_player.x++;
            if(isPlayerColliding()) {
                main_player.x--;
                break;
            }
        }
        for(int i = 0; i > move_x; i--) {
            main_player.x--;
            if(isPlayerColliding()) {
                main_player.x++;
                break;
            }
        }
        for(int i = 0; i < move_y; i++) {
            main_player.y++;
            if(isPlayerColliding()) {
                main_player.y--;
                break;
            }
        }
        for(int i = 0; i > move_y; i--) {
            main_player.y--;
            if(isPlayerColliding()) {
                main_player.y++;
                if(main_player.velocity_y < 0)
                    main_player.velocity_y = 0;
                break;
            }
        }
        if(isPlayerTouchingGround() && jump) {
            main_player.velocity_y = -JUMP_VELOCITY;
            jump = false;
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
    }
}
