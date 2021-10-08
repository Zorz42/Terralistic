#include <iostream>
#include "serverNetworking.hpp"
#include "print.hpp"
#include "compress.hpp"
#include "graphics.hpp"

void Connection::send(sf::Packet& packet) {
    sf::Socket::Status status = sf::Socket::Partial;
    while(status == sf::Socket::Partial)
        status = socket->send(packet);
}

sf::Socket::Status Connection::receive(sf::Packet& packet) {
    return socket->receive(packet);
}

bool Connection::hasBeenGreeted() {
    return greeted;
}

void Connection::greet() {
    greeted = true;
}

std::string Connection::getIpAddress() {
    return socket->getRemoteAddress().toString();
}

Connection::~Connection() {
    delete socket;
}

void Connection::send(std::vector<char>& data) {
    int size = (int)data.size();
    int bytes_sent = 0;
    while(bytes_sent < sizeof(int)) {
        size_t sent;
        socket->send((char*)&size, sizeof(int), sent);
        bytes_sent += sent;
    }
    
    bytes_sent = 0;
    while(bytes_sent < data.size()) {
        size_t sent;
        socket->send(&data[bytes_sent], (int)data.size() - bytes_sent, sent);
        bytes_sent += sent;
    }
}

void Connection::pushPacket(sf::Packet& packet, PacketType type) {
    packet_buffer.push({packet, type});
}

bool Connection::hasPacketInBuffer() {
    return !packet_buffer.empty();
}

std::pair<sf::Packet, PacketType> Connection::getPacket() {
    auto result = packet_buffer.front();
    packet_buffer.pop();
    return result;
}

void ServerNetworking::init() {
    listener.listen(port);
    listener.setBlocking(false);
}

void ServerNetworking::sendToEveryone(sf::Packet& packet, Connection* exclusion) {
    for(Connection* connection : connections)
        if(exclusion == nullptr || exclusion != connection)
            connection->send(packet);
}

void ServerNetworking::update(float frame_length) {
    static sf::TcpSocket *socket = new sf::TcpSocket;
    while(true) {
        if(listener.accept(*socket) != sf::Socket::NotReady) {
            if(!is_private || socket->getRemoteAddress().toString() == "127.0.0.1") {
                socket->setBlocking(false);
                Connection* connection = new Connection(socket);
                connections.push_back(connection);
            } else {
                delete socket;
            }
            socket = new sf::TcpSocket;
        } else
            break;
    }
    
    sf::Packet packet;
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i]->hasBeenGreeted()) {
            while(true) {
                sf::Socket::Status status = connections[i]->receive(packet);
                if(status == sf::Socket::NotReady)
                    break;
                else if(status == sf::Socket::Disconnected) {
                    ServerDisconnectEvent event(connections[i]);
                    disconnect_event.call(event);
                    
                    print::info(connections[i]->getIpAddress() + " disconnected (" + std::to_string(connections.size()) + " players online)");
                    delete connections[i];
                    connections.erase(connections.begin() + i);
                    
                    //sf::Packet entity_packet;
                    //entity_packet << PacketType::ENTITY_DELETION << connections[i].player->id;
                    //sendToEveryone(entity_packet);
                    
                    break;
                } else if(status == sf::Socket::Done) {
                    unsigned char packet_type;
                    packet >> packet_type;
                    
                    connections[i]->pushPacket(packet, (PacketType)packet_type);
                }
            }
        } else if(connections[i]->receive(packet) != sf::Socket::NotReady) {
            ServerConnectionWelcomeEvent event(connections[i], packet);
            connection_welcome_event.call(event);
            
            sf::Packet welcome_packet;
            welcome_packet << WelcomePacketType::WELCOME;
            connections[i]->send(welcome_packet);
            connections[i]->greet();
            
            ServerNewConnectionEvent event2(connections[i]);
            new_connection_event.call(event2);
            
            /*
            bool already_exists = false;
            for(Entity* entity : entities->getEntities())
                if(entity->type == EntityType::PLAYER) {
                    ServerPlayer* player = (ServerPlayer*)entity;
                    if(player->name == player_name)
                        already_exists = true;
                }
            */
            /*if(already_exists) {
                sf::Packet kick_packet;
                kick_packet << PacketType::KICK << "You are already logged in from another location";
                connections[i].send(kick_packet);
                connections.erase(connections.begin() + i);
            } else {
                player->inventory.item_change_event.addListener(&connections[i]);
                
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
            }*/
        }
    }
}

void ServerNetworking::stop() {
    if(!is_private) {
        sf::Packet kick_packet;
        kick_packet << PacketType::KICK << std::string("Server stopped!");
        sendToEveryone(kick_packet);
    }
    
    listener.close();
}
