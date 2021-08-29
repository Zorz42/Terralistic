#include "serverNetworking.hpp"
#include "print.hpp"
#include "compress.hpp"

void Connection::send(sf::Packet& packet) {
    socket->send(packet);
}

void Connection::send(std::vector<char>& data) {
    size_t sent;
    socket->send(&data[0], data.size(), sent);
    if(data.size() != sent)
        print::error("data was not sent properly");
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

void ServerNetworkingManager::getPacketsFromPlayers() {
    sf::Packet packet;
    for(int i = 0; i < connections.size(); i++) {
        if(connections[i].player) {
            while(true) {
                sf::Socket::Status status = connections[i].receive(packet);
                if(status == sf::Socket::NotReady)
                    break;
                else if(status == sf::Socket::Disconnected) {
                    print::info(connections[i].player->name + " (" + connections[i].getIpAddress() + ") disconnected (" + std::to_string(players->getOnlinePlayers().size() - 1) + " players online)");
                    players->removePlayer(connections[i].player);
                    
                    sf::Packet quit_packet;
                    quit_packet << PacketType::PLAYER_QUIT << connections[i].player->id;
                    sendToEveryone(quit_packet);
                    
                    connections[i].freeSocket();
                    connections.erase(connections.begin() + i);
                    
                    break;
                } else if(status == sf::Socket::Done) {
                    unsigned char packet_type;
                    packet >> packet_type;
                    onPacket(packet, (PacketType)packet_type, connections[i]);
                }
            }
        } else if(connections[i].receive(packet) != sf::Socket::NotReady) {
            std::string player_name;
            packet >> player_name;
            ServerPlayer* player = players->addPlayer(player_name);
            connections[i].player = player;
            
            sf::Packet welcome_packet;
            welcome_packet << player->x << player->y;
            welcome_packet << blocks->getWidth() << blocks->getHeight();
            
            std::vector<char> map_data;
            
            for(int x = 0; x < blocks->getWidth(); x++)
                for(int y = 0; y < blocks->getHeight(); y++) {
                    ServerBlock block = blocks->getBlock(x, y);
                    map_data.push_back((char)block.getBlockType());
                    map_data.push_back((char)block.getLiquidType());
                    map_data.push_back((char)block.getLiquidLevel());
                    map_data.push_back((char)block.getLightLevel());
                }
            
            map_data = compress(map_data);
            
            welcome_packet << (unsigned int)map_data.size();
            connections[i].send(welcome_packet);
            connections[i].send(map_data);

            for(ServerPlayer* curr_player : players->getOnlinePlayers())
                if(curr_player != player) {
                    sf::Packet join_packet;
                    join_packet << PacketType::PLAYER_JOIN << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
                    connections[i].send(join_packet);
                }

            for(const ServerItem& curr_item : items->getItems()) {
                sf::Packet item_packet;
                item_packet << PacketType::ITEM_CREATION << curr_item.getX() << curr_item.getY() << curr_item.getId() << (unsigned char)curr_item.getType();
                connections[i].send(item_packet);
            }

            for(InventoryItem& curr_item : player->inventory.inventory_arr)
                if(curr_item.getType() != ItemType::NOTHING)
                    sendInventoryItemPacket(connections[i], curr_item, curr_item.getType(), curr_item.getStack());
            player->inventory.updateAvailableRecipes();
            
            sf::Packet join_packet;
            join_packet << PacketType::PLAYER_JOIN << player->x << player->y << player->id << player->name;
            sendToEveryone(join_packet, &connections[i]);

            print::info(player->name + " (" + connections[i].getIpAddress() + ") connected (" + std::to_string(players->getOnlinePlayers().size()) + " players online)");
        }
    }
}
