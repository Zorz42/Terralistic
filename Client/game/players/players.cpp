#include <cassert>
#include "clientPlayers.hpp"

ClientPlayers::ClientPlayers(NetworkingManager* manager, ClientBlocks* world_map, ResourcePack* resource_pack, std::string username) :
manager(manager), blocks(world_map), resource_pack(resource_pack), main_player(std::move(username)) {}

ClientPlayer::ClientPlayer(const std::string& name, int x, int y, unsigned short id) : name(name), x(x), y(y), id(id) {
    name_text.renderText(name, WHITE);
}

void ClientPlayers::renderPlayers() {
    for(ClientPlayer* i : other_players)
        render(*i);
    
    render(main_player);
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
