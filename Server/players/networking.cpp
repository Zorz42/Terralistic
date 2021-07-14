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

            //Packet spawn_packet(PacketType::SPAWN_POS, sizeof(curr_player->y) + sizeof(curr_player->x));
            //spawn_packet << curr_player->y << curr_player->x;
            //event.conn.sendPacket(spawn_packet);

            /*for(player* player : online_players) {
                Packet join_packet(PacketType::PLAYER_JOIN, sizeof(player->x) + sizeof(player->y) + sizeof(player->id) + (int)player->name.size() + 1);
                join_packet << player->x << player->y << player->id << player->name;
                curr_player->conn->sendPacket(join_packet);
            }

            for(const item& i : parent_items->getItems()) {
                Packet item_packet(PacketType::ITEM_CREATION, sizeof(i.x) + sizeof(i.y) + sizeof(i.getId()) + sizeof(char));
                item_packet << i.x << i.y << i.getId() << (char)i.getItemId();
                curr_player->conn->sendPacket(item_packet);
            }*/

            for(inventoryItem& curr_item : curr_player->player_inventory.inventory_arr) // send the whole inventory
                if(curr_item.getId() != ItemType::NOTHING)
                    curr_item.sendPacket();

            /*Packet join_packet_out(PacketType::PLAYER_JOIN, sizeof(curr_player->x) + sizeof(curr_player->y) + sizeof(curr_player->id) + (int)curr_player->name.size() + 1);
            join_packet_out << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
            manager->sendToEveryone(join_packet_out, curr_player->conn);*/

            //curr_player->conn->registered = true;

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
                ServerPacketEvent(packet, *curr_player).call();
            } else
                break;
        }
    }
}
