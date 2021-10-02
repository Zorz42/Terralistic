#include <iostream>
#include "serverNetworking.hpp"
#include "print.hpp"
#include "compress.hpp"
#include "graphics.hpp"

#define TRANSFER_BUFFER_SIZE 16384 // 2^14

void Connection::send(sf::Packet& packet) {
    master_packet.append(packet.getData(), packet.getDataSize());
}

void Connection::send(std::vector<char>& data) {
    int bytes_sent = 0;
    while(bytes_sent < data.size()) {
        size_t sent;
        socket->send(&data[bytes_sent], (int)data.size() - bytes_sent, sent);
        bytes_sent += sent;
    }
}

sf::Socket::Status Connection::receive(sf::Packet& packet) {
    return socket->receive(packet);
}

std::string Connection::getIpAddress() {
    return socket->getRemoteAddress().toString();
}

void Connection::freeSocket() {
    delete socket;
}

void Connection::flushPacket() {
    if(master_packet.getDataSize()) {
        socket->send(master_packet);
        master_packet.clear();
    }
}

void ServerNetworkingManager::openSocket(unsigned short port) {
    listener.listen(port);
    listener.setBlocking(false);
}

void ServerNetworkingManager::closeSocket() {
    listener.close();
}

void ServerNetworkingManager::sendToEveryone(sf::Packet& packet, Connection* exclusion) {
    for(Connection& connection : connections)
        if(exclusion == nullptr || exclusion->player != connection.player)
            connection.send(packet);
}

void ServerNetworkingManager::checkForNewConnections() {
    static sf::TcpSocket *socket = new sf::TcpSocket;
    while(true) {
        if(listener.accept(*socket) != sf::Socket::NotReady) {
            if(!accept_itself || socket->getRemoteAddress().toString() == "127.0.0.1") {
                socket->setBlocking(false);
                connections.emplace_back(socket);
            } else {
                delete socket;
            }
            socket = new sf::TcpSocket;
        } else
            break;
    }
}

void ServerNetworkingManager::init() {
    blocks->block_change_event.addListener(this);
    blocks->block_break_stage_change_event.addListener(this);
    liquids->liquid_change_event.addListener(this);
    items->item_creation_event.addListener(this);
    entities->entity_deletion_event.addListener(this);
    entities->entity_velocity_change_event.addListener(this);
    entities->entity_position_change_event.addListener(this);
}

void ServerNetworkingManager::getPacketsFromPlayers() {
    sf::Packet packet;
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i].player) {
            while(true) {
                sf::Socket::Status status = connections[i].receive(packet);
                if(status == sf::Socket::NotReady)
                    break;
                else if(status == sf::Socket::Disconnected) {
                    connections[i].player->inventory.item_change_event.removeListener(&connections[i]);
                    
                    players->savePlayer(connections[i].player);
                    entities->removeEntity(connections[i].player);
                    
                    int online_players_count = 0;
                    for(Entity* entity : entities->getEntities())
                        if(entity->type == EntityType::PLAYER)
                            online_players_count++;
                    print::info(connections[i].player->name + " (" + connections[i].getIpAddress() + ") disconnected (" + std::to_string(online_players_count) + " players online)");
                    
                    connections[i].freeSocket();
                    connections.erase(connections.begin() + i);
                    
                    sf::Packet quit_packet;
                    quit_packet << PacketType::ENTITY_DELETION << connections[i].player->id;
                    sendToEveryone(quit_packet);
                    
                    break;
                } else if(status == sf::Socket::Done) {
                    while(!packet.endOfPacket()) {
                        unsigned char packet_type;
                        packet >> packet_type;
                        onPacket(packet, (PacketType)packet_type, connections[i]);
                    }
                }
            }
        } else if(connections[i].receive(packet) != sf::Socket::NotReady) {
            std::string player_name;
            packet >> player_name;
            bool already_exists = false;
            for(Entity* entity : entities->getEntities())
                if(entity->type == EntityType::PLAYER) {
                    ServerPlayer* player = (ServerPlayer*)entity;
                    if(player->name == player_name)
                        already_exists = true;
                }
            
            if(already_exists) {
                sf::Packet kick_packet;
                kick_packet << PacketType::KICK << "You are already logged in from another location";
                connections[i].send(kick_packet);
                connections[i].flushPacket();
                connections.erase(connections.begin() + i);
            } else {
                ServerPlayer* player = players->addPlayer(player_name);
                connections[i].player = player;
                
                player->inventory.item_change_event.addListener(&connections[i]);
                
                std::vector<char> map_data;
                blocks->serialize(map_data);
                liquids->serialize(map_data);
                player->inventory.serialize(map_data);
                
                sf::Packet welcome_packet;
                welcome_packet << PacketType::WELCOME << player->getX() << player->getY() << blocks->getWidth() << blocks->getHeight();
                
                welcome_packet << (unsigned int)map_data.size();
                connections[i].send(welcome_packet);
                connections[i].flushPacket();
                connections[i].send(map_data);
                
                for(Entity* entity : entities->getEntities())
                    if(entity->type == EntityType::PLAYER) {
                        ServerPlayer* curr_player = (ServerPlayer*)entity;
                        sf::Packet join_packet;
                        join_packet << PacketType::PLAYER_JOIN << curr_player->getX() << curr_player->getY() << curr_player->id << curr_player->name << (unsigned char)curr_player->moving_type;
                        connections[i].send(join_packet);
                    }
                
                for(const Entity* entity : entities->getEntities()) {
                    if(entity->type == EntityType::ITEM) {
                        Item* item = (Item*)entity;
                        sf::Packet item_packet;
                        item_packet << PacketType::ITEM_CREATION << item->getX() << item->getY() << item->id << (unsigned char)item->getType();
                        connections[i].send(item_packet);
                    }
                }
                
                sf::Packet join_packet;
                join_packet << PacketType::PLAYER_JOIN << player->getX() << player->getY() << player->id << player->name << (unsigned char)player->moving_type;
                sendToEveryone(join_packet, &connections[i]);

                int online_players_count = 0;
                for(Entity* entity : entities->getEntities())
                    if(entity->type == EntityType::PLAYER)
                        online_players_count++;
                print::info(player->name + " (" + connections[i].getIpAddress() + ") connected (" + std::to_string(online_players_count) + " players online)");
            }
        }
    }
}

void ServerNetworkingManager::flushPackets() {
    for(Connection& connection : connections)
        connection.flushPacket();
}
