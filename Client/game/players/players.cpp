#include <cassert>
#include "clientPlayers.hpp"

ClientPlayers::ClientPlayers(NetworkingManager* manager, ClientBlocks* blocks, Liquids* liquids, ResourcePack* resource_pack, Entities* entities, const std::string& username) :
manager(manager), blocks(blocks), liquids(liquids), resource_pack(resource_pack), entities(entities), username(username) {}

ClientPlayer::ClientPlayer(const std::string& name, int x, int y, unsigned short id) : Player(x, y, name, id) {
    name_text.loadFromText(name, WHITE);
    friction = false;
}

void ClientPlayers::render() {
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER)
            render(*(ClientPlayer*)entity);
}

#define HEADER_MARGIN 4
#define HEADER_PADDING 2

void ClientPlayers::render(ClientPlayer& player_to_draw) {
    if(player_to_draw.isTouchingGround(blocks) && player_to_draw.getVelocityY() == 0)
        player_to_draw.has_jumped = false;
    
    if(player_to_draw.getVelocityX())
        player_to_draw.flipped = player_to_draw.getVelocityX() < 0;
    
    if(player_to_draw.moving_type == MovingType::STANDING)
        player_to_draw.started_moving = 0;
    
    if(player_to_draw.moving_type == MovingType::SNEAKING)
        player_to_draw.texture_frame = 10;
    else if(player_to_draw.has_jumped)
        player_to_draw.texture_frame = 0;
    else if(player_to_draw.moving_type == MovingType::WALKING && player_to_draw.has_moved_x)
        player_to_draw.texture_frame = (gfx::getTicks() - player_to_draw.started_moving) / 70 % 9 + 1;
    else if(player_to_draw.moving_type == MovingType::SNEAK_WALKING) {
        if(player_to_draw.has_moved_x)
            player_to_draw.texture_frame = (gfx::getTicks() - player_to_draw.started_moving) / 150 % 6 + 10;
        else
            player_to_draw.texture_frame = 10;
    } else if(player_to_draw.moving_type == MovingType::RUNNING && player_to_draw.has_moved_x)
        player_to_draw.texture_frame = (gfx::getTicks() - player_to_draw.started_moving) / 80 % 8 + 16;
    else
        player_to_draw.texture_frame = 1;
    
    int player_x = gfx::getWindowWidth() / 2 + player_to_draw.getX() - blocks->view_x;
    int player_y = gfx::getWindowHeight() / 2 + player_to_draw.getY() - blocks->view_y;
    resource_pack->getPlayerTexture().render(2, player_x, player_y, {(short)(player_to_draw.texture_frame * PLAYER_WIDTH), 0, (unsigned short)(PLAYER_WIDTH), (unsigned short)(PLAYER_HEIGHT)}, player_to_draw.flipped);
    if(player_to_draw.name != "_") {
        int header_x = gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.getY() + PLAYER_WIDTH - blocks->view_x,
        header_y = gfx::getWindowHeight() / 2 + player_to_draw.getY() - blocks->view_y - player_to_draw.name_text.getTextureHeight() - HEADER_MARGIN;
        gfx::RectShape(header_x - HEADER_PADDING, header_y - HEADER_PADDING, player_to_draw.name_text.getTextureWidth() + 2 * HEADER_PADDING, player_to_draw.name_text.getTextureHeight() + 2 * HEADER_PADDING).render(BLACK);
        player_to_draw.name_text.render(1, header_x, header_y);
    }
}

ClientPlayer* ClientPlayers::getPlayerById(unsigned short id) {
    for(Entity* entity : entities->getEntities())
        if(entity->type == EntityType::PLAYER && entity->id == id)
            return (ClientPlayer*)entity;
    assert(false);
    return nullptr;
}

void ClientPlayers::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::PLAYER_JOIN: {
            int x, y;
            unsigned short id;
            unsigned char moving_type;
            std::string name;
            event.packet >> x >> y >> id >> name >> moving_type;
            ClientPlayer* new_player = new ClientPlayer(name, x, y, id);
            new_player->moving_type = (MovingType)moving_type;
            entities->registerEntity(new_player);
            if(name == username)
                main_player = new_player;
            
            break;
        }
        case PacketType::PLAYER_MOVING_TYPE: {
            unsigned short id;
            unsigned char moving_type;
            event.packet >> moving_type >> id;
            if(id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->moving_type = (MovingType)moving_type;
            }
            
            break;
        }
        case PacketType::PLAYER_JUMPED: {
            unsigned short id;
            event.packet >> id;
            if(id != main_player->id) {
                ClientPlayer* player = getPlayerById(id);
                player->has_jumped = true;
            }
            
            break;
        }
        default:;
    }
}
