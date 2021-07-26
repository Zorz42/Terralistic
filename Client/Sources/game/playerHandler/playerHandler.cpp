#include "playerHandler.hpp"

void playerHandler::render() {
    // iterate through every player and render them
    for(OtherPlayer* i : other_players)
        render(i->x, i->y, world_map->view_x, world_map->view_y, i->flipped, i->name_text);
    
    render(player->x, player->y, world_map->view_x, world_map->view_y, player->flipped);
}

OtherPlayer* playerHandler::getPlayerById(unsigned short id) {
    for(OtherPlayer* player : other_players)
        if(player->id == id)
            return player;
    assert(false);
    return nullptr;
}

void playerHandler::onEvent(ClientPacketEvent &event) {
    switch(event.packet_type) {
        case PacketType::PLAYER_JOIN: {
            OtherPlayer* new_player = new OtherPlayer();
            event.packet >> new_player->x >> new_player->y >> new_player->id >> new_player->name;
            new_player->name_text.renderText(new_player->name, {0, 0, 0});
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
            
            OtherPlayer* player = getPlayerById(id);
            player->flipped = flipped;
            player->x = x;
            player->y = y;
            break;
        }
        case PacketType::SPAWN_POS: {
            int x, y;
            event.packet >> x >> y;
            player->x = x;
            player->y = y;
            world_map->view_x = x;
            world_map->view_y = y;
            received_spawn_coords = true;
            break;
        }
        default:;
    }
}
