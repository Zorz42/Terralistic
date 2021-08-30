#include "clientPlayers.hpp"

ClientPlayers::ClientPlayers(NetworkingManager* manager, ClientBlocks* world_map, ResourcePack* resource_pack, int x, int y, std::string username) :
manager(manager), blocks(world_map), resource_pack(resource_pack), main_player(x, y, std::move(username)) {
    world_map->view_x = x + getPlayerWidth();
    world_map->view_y = y + getPlayerHeight();
}

void ClientPlayers::renderPlayers() {
    for(OtherPlayer* i : other_players)
        render(*i);
    
    render(main_player);
}

OtherPlayer* ClientPlayers::getPlayerById(unsigned short id) {
    for(OtherPlayer* i : other_players)
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
            OtherPlayer* new_player = new OtherPlayer(name, x, y, id);
            new_player->name_text.renderText(new_player->name, BLACK);
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
        default:;
    }
}
