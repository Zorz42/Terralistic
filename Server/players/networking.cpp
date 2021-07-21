#include "players.hpp"
#include "print.hpp"

void players::openSocket(unsigned short port) {
    listener.listen(port);
    listener.setBlocking(false);
}

void players::closeSocket() {
    listener.close();
}

void players::sendToEveryone(sf::Packet& packet, Player* exclusion) {
    for(Player* curr_player : online_players)
        if(curr_player != exclusion)
            curr_player->socket->send(packet);
}

void players::checkForNewConnections() {
    static sf::TcpSocket *socket = new sf::TcpSocket;
    while(true) {
        if(listener.accept(*socket) != sf::Socket::NotReady) {
            socket->setBlocking(false);
            pending_connections.push_back(socket);
            socket = new sf::TcpSocket;
        } else
            break;
    }
}

void players::getPacketsFromPlayers() {
    for(int i = 0; i < pending_connections.size(); i++) {
        sf::Packet packet;
        if(pending_connections[i]->receive(packet) != sf::Socket::NotReady) {
            std::string player_name;
            packet >> player_name;
            Player* curr_player = getPlayerByName(player_name);
            curr_player->socket = pending_connections[i];
            
            sf::Packet spawn_packet;
            spawn_packet << PacketType::SPAWN_POS << curr_player->x << curr_player->y;
            curr_player->socket->send(spawn_packet);

            for(Player* player : online_players) {
                sf::Packet join_packet;
                join_packet << PacketType::PLAYER_JOIN << player->x << player->y << player->id << player->name;
                curr_player->socket->send(join_packet);
            }

            for(const Item& curr_item : parent_items->getItems()) {
                sf::Packet item_packet;
                item_packet << PacketType::ITEM_CREATION << curr_item.getX() << curr_item.getY() << curr_item.getId() << (unsigned char)curr_item.getType();
                curr_player->socket->send(item_packet);
            }

            for(InventoryItem& curr_item : curr_player->inventory.inventory_arr)
                if(curr_item.getType() != ItemType::NOTHING)
                    sendInventoryItemPacket(curr_item, curr_item.getType(), curr_item.getStack());
            
            sf::Packet join_packet;
            join_packet << PacketType::PLAYER_JOIN << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
            sendToEveryone(join_packet);

            online_players.push_back(curr_player);

            print::info(curr_player->name + " (" + curr_player->socket->getRemoteAddress().toString() + ") connected (" + std::to_string(online_players.size()) + " players online)");
            
            pending_connections.erase(pending_connections.begin() + i);
        }
    }
    
    for(Player* curr_player : online_players) {
        while(true) {
            sf::Packet packet;
            sf::Socket::Status status = curr_player->socket->receive(packet);
            if(status == sf::Socket::NotReady)
                break;
            else if(status == sf::Socket::Disconnected) {
                print::info(curr_player->name + " (" + curr_player->socket->getRemoteAddress().toString() + ") disconnected (" + std::to_string(online_players.size() - 1) + " players online)");
                delete curr_player->socket;
                for(auto i = online_players.begin(); i != online_players.end(); i++)
                    if((*i)->id == curr_player->id) {
                        online_players.erase(i);
                        break;
                    }

                sf::Packet quit_packet;
                quit_packet << PacketType::PLAYER_QUIT << curr_player->id;
                sendToEveryone(quit_packet);
                
                break;
            } else if(status == sf::Socket::Done) {
                unsigned char packet_type;
                packet >> packet_type;
                ServerPacketEvent(packet, (PacketType)packet_type, *curr_player).call();
            }
        }
    }
}
