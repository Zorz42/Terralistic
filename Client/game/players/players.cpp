#include <cassert>
#include "clientPlayers.hpp"

ClientPlayers::ClientPlayers(NetworkingManager* manager, ClientBlocks* world_map, ResourcePack* resource_pack, ClientEntities* entities, std::string username) :
manager(manager), blocks(world_map), resource_pack(resource_pack), entities(entities), main_player(std::move(username)) {
    other_players.push_back(&main_player);
}

ClientPlayer::ClientPlayer(const std::string& name, int x, int y, unsigned short id) : name(name), ClientEntity(id, EntityType::PLAYER, x, y) {
    name_text.renderText(name, WHITE);
    friction = false;
}

void ClientPlayers::renderPlayers() {
    for(ClientPlayer* i : other_players)
        render(*i);
}

#define HEADER_MARGIN 4
#define HEADER_PADDING 2

void ClientPlayers::render(ClientPlayer& player_to_draw) {
    resource_pack->getPlayerTexture().render(2,
                                             gfx::getWindowWidth() / 2 + player_to_draw.x - blocks->view_x,
                                             gfx::getWindowHeight() / 2 + player_to_draw.y - blocks->view_y,
                                             {(short)(player_to_draw.texture_frame * PLAYER_WIDTH), 0, (unsigned short)(PLAYER_WIDTH), (unsigned short)(PLAYER_HEIGHT)},
                                             player_to_draw.flipped);
    if(player_to_draw.name != "_") {
        int header_x = gfx::getWindowWidth() / 2 - player_to_draw.name_text.getTextureWidth() / 2 + player_to_draw.x + PLAYER_WIDTH - blocks->view_x,
        header_y = gfx::getWindowHeight() / 2 + player_to_draw.y - blocks->view_y - player_to_draw.name_text.getTextureHeight() - HEADER_MARGIN;
        gfx::RectShape(header_x - HEADER_PADDING, header_y - HEADER_PADDING, player_to_draw.name_text.getTextureWidth() + 2 * HEADER_PADDING, player_to_draw.name_text.getTextureHeight() + 2 * HEADER_PADDING).render(BLACK);
        player_to_draw.name_text.render(1, header_x, header_y);
    }
}

ClientPlayer* ClientPlayers::getPlayerById(unsigned short id) {
    for(ClientPlayer* i : other_players)
        if(i->id == id)
            return i;
    assert(false);
    return nullptr;
}

void ClientPlayers::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::PLAYER_JOIN: {
            int x, y;
            unsigned short id;
            std::string name;
            event.packet >> x >> y >> id >> name;
            ClientPlayer* new_player = new ClientPlayer(name, x, y, id);
            other_players.push_back(new_player);
            break;
        }
        case PacketType::PLAYER_QUIT: {
            unsigned id;
            event.packet >> id;
            for(auto i = other_players.begin(); i != other_players.end(); i++)
                if((*i)->id == id) {
                    other_players.erase(i);
                    break;
                }
            break;
        }
        case PacketType::PLAYER_MOVEMENT: {
            int x, y;
            unsigned short id;
            bool flipped;
            event.packet >> x >> y >> flipped >> id;
            
            ClientPlayer* curr_player = getPlayerById(id);
            curr_player->flipped = flipped;
            curr_player->x = x;
            curr_player->y = y;
            break;
        }
        default:;
    }
}
