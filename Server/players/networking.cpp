//
//  networking.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 14/07/2021.
//

#include "players.hpp"
#include "print.hpp"

void players::openSocket(unsigned short port) {
    listener.listen(port);
    listener.setBlocking(false);
}

void players::closeSocket() {
    listener.close();
}

void players::sendToEveryone(sf::Packet& packet, player* exclusion) {
    for(player* curr_player : online_players)
        if(curr_player != exclusion)
            curr_player->socket->send(packet);
}

void players::checkForNewConnections() {
    while(true) {
        sf::TcpSocket* socket = new sf::TcpSocket();
        if(listener.accept(*socket) != sf::Socket::NotReady) {
            socket->setBlocking(false);
            pending_connections.push_back(socket);
        } else {
            delete socket;
            break;
        }
    }
}

void players::getPacketsFromPlayers() {
    for(int i = 0; i < pending_connections.size(); i++) {
        sf::Packet packet;
        if(pending_connections[i]->receive(packet) != sf::Socket::NotReady) {
            std::string player_name;
            packet >> player_name;
            player* curr_player = getPlayerByName(player_name);
            curr_player->socket = pending_connections[i];
            
            sf::Packet spawn_packet;
            spawn_packet << PacketType::SPAWN_POS << curr_player->x << curr_player->y;
            curr_player->socket->send(spawn_packet);

            for(player* player : online_players) {
                sf::Packet join_packet;
                join_packet << PacketType::PLAYER_JOIN << player->x << player->y << player->id << player->name;
                curr_player->socket->send(join_packet);
            }

            for(const item& curr_item : parent_items->getItems()) {
                sf::Packet item_packet;
                item_packet << PacketType::ITEM_CREATION << curr_item.x << curr_item.y << curr_item.getId() << (unsigned char)curr_item.getItemId();
                curr_player->socket->send(item_packet);
            }

            for(inventoryItem& curr_item : curr_player->player_inventory.inventory_arr) // send the whole inventory
                if(curr_item.getId() != ItemType::NOTHING)
                    curr_item.sendPacket();
            
            sf::Packet join_packet;
            join_packet << PacketType::PLAYER_JOIN << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
            sendToEveryone(join_packet);

            online_players.push_back(curr_player);

            print::info(curr_player->name + " (" + curr_player->socket->getRemoteAddress().toString() + ") connected (" + std::to_string(online_players.size()) + " players online)");
            
            pending_connections.erase(pending_connections.begin() + i);
        }
    }
    
    for(player* curr_player : online_players) {
        while(true) {
            sf::Packet packet;
            if(curr_player->socket->receive(packet) != sf::Socket::NotReady) {
                unsigned char packet_type;
                packet >> packet_type;
                ServerPacketEvent(packet, (PacketType)packet_type, *curr_player).call();
            } else
                break;
        }
    }
}
