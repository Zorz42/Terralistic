#include "playerHandler.hpp"

void PlayerHandler::render() {
    for(OtherPlayer* i : other_players)
        render(*i);
    
    render(main_player);
}

OtherPlayer* PlayerHandler::getPlayerById(unsigned short id) {
    for(OtherPlayer* i : other_players)
        if(i->id == id)
            return i;
    assert(false);
    return nullptr;
}

void PlayerHandler::onEvent(ClientPacketEvent &event) {
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
            
            OtherPlayer* curr_player = getPlayerById(id);
            curr_player->flipped = flipped;
            curr_player->x = x;
            curr_player->y = y;
            break;
        }
        case PacketType::SPAWN_POS: {
            int x, y;
            event.packet >> x >> y;
            main_player.x = x;
            main_player.y = y;
            world_map->view_x = x;
            world_map->view_y = y;
            received_spawn_coords = true;
            break;
        }
        default:;
    }
}
