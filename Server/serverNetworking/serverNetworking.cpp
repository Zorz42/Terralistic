#include "serverNetworking.hpp"
#include "print.hpp"

void NetworkingManager::openSocket(unsigned short port) {
    listener.listen(port);
    listener.setBlocking(false);
}

void NetworkingManager::closeSocket() {
    listener.close();
}

void NetworkingManager::sendToEveryone(sf::Packet& packet, Connection* exclusion) {
    for(Connection& connection : connections)
        if(exclusion == nullptr || exclusion->socket != connection.socket)
            connection.socket->send(packet);
}

void NetworkingManager::checkForNewConnections() {
    static sf::TcpSocket *socket = new sf::TcpSocket;
    while(true) {
        if(listener.accept(*socket) != sf::Socket::NotReady) {
            socket->setBlocking(false);
            connections.push_back(socket);
            socket = new sf::TcpSocket;
        } else
            break;
    }
}

void NetworkingManager::getPacketsFromPlayers() {
    sf::Packet packet;
    for(Connection& connection : connections) {
        if(connection.player) {
            while(true) {
                sf::Socket::Status status = connection.socket->receive(packet);
                if(status == sf::Socket::NotReady)
                    break;
                else if(status == sf::Socket::Disconnected) {
                    print::info(connection.player->name + " (" + connection.socket->getRemoteAddress().toString() + ") disconnected (" + std::to_string(players->getOnlinePlayers().size() - 1) + " players online)");
                    delete connection.socket;
                    players->removePlayer(connection.player);
                    
                    sf::Packet quit_packet;
                    quit_packet << PacketType::PLAYER_QUIT << connection.player->id;
                    sendToEveryone(quit_packet);
                    
                    break;
                } else if(status == sf::Socket::Done) {
                    unsigned char packet_type;
                    packet >> packet_type;
                    ServerPacketEvent(packet, (PacketType)packet_type, connection).call();
                }
            }
        } else if(connection.socket->receive(packet) != sf::Socket::NotReady) {
            std::string player_name;
            packet >> player_name;
            Player* player = players->addPlayer(player_name);
            connection.player = player;
            
            sf::Packet spawn_packet;
            spawn_packet << PacketType::SPAWN_POS << player->x << player->y;
            connection.socket->send(spawn_packet);

            for(Player* curr_player : players->getOnlinePlayers())
                if(curr_player != player) {
                    sf::Packet join_packet;
                    join_packet << PacketType::PLAYER_JOIN << curr_player->x << curr_player->y << curr_player->id << curr_player->name;
                    connection.socket->send(join_packet);
                }

            for(const Item& curr_item : items->getItems()) {
                sf::Packet item_packet;
                item_packet << PacketType::ITEM_CREATION << curr_item.getX() << curr_item.getY() << curr_item.getId() << (unsigned char)curr_item.getType();
                connection.socket->send(item_packet);
            }

            for(InventoryItem& curr_item : player->inventory.inventory_arr)
                if(curr_item.getType() != ItemType::NOTHING)
                    sendInventoryItemPacket(connection, curr_item, curr_item.getType(), curr_item.getStack());
            
            sf::Packet join_packet;
            join_packet << PacketType::PLAYER_JOIN << player->x << player->y << player->id << player->name;
            sendToEveryone(join_packet, &connection);

            print::info(player->name + " (" + connection.socket->getRemoteAddress().toString() + ") connected (" + std::to_string(players->getOnlinePlayers().size()) + " players online)");
        }
    }
}

void NetworkingManager::onEvent(ServerPacketEvent& event) {
    Player* curr_player = event.conn.player;
    switch (event.packet_type) {
        case PacketType::STARTED_BREAKING: {
            unsigned short x, y;
            event.packet >> x >> y;
            curr_player->breaking_x = x;
            curr_player->breaking_y = y;
            curr_player->breaking = true;
            break;
        }

        case PacketType::STOPPED_BREAKING: {
            curr_player->breaking = false;
            break;
        }

        case PacketType::RIGHT_CLICK: {
            unsigned short x, y;
            event.packet >> x >> y;
            players->rightClickEvent(blocks->getBlock(x, y), curr_player);
            break;
        }

        case PacketType::CHUNK: {
            unsigned short x, y;
            event.packet >> x >> y;
            
            sf::Packet chunk_packet;
            chunk_packet << PacketType::CHUNK << x << y;
            for(int chunk_x = 0; chunk_x < 16; chunk_x++)
                for(int chunk_y = 0; chunk_y < 16; chunk_y++) {
                    Block block = blocks->getBlock((x << 4) + chunk_x, (y << 4) + chunk_y);
                    chunk_packet << (unsigned char)block.getBlockType() << (unsigned char)block.getLiquidType() << (unsigned char)block.getLiquidLevel() << (unsigned char)block.getLightLevel();
                }
            event.conn.socket->send(chunk_packet);
            break;
        }

        case PacketType::VIEW_SIZE_CHANGE: {
            event.packet >> curr_player->sight_width >> curr_player->sight_height;
            break;
        }

        case PacketType::PLAYER_MOVEMENT: {
            event.packet >> curr_player->x >> curr_player->y >> curr_player->flipped;
            
            sf::Packet movement_packet;
            movement_packet << PacketType::PLAYER_MOVEMENT << curr_player->x << curr_player->y << curr_player->flipped << curr_player->id;
            sendToEveryone(movement_packet, &event.conn);
            break;
        }

        case PacketType::INVENTORY_SWAP: {
            unsigned char pos;
            event.packet >> pos;
            curr_player->inventory.swapWithMouseItem(&curr_player->inventory.inventory_arr[pos]);
            break;
        }

        case PacketType::HOTBAR_SELECTION: {
            event.packet >> curr_player->inventory.selected_slot;
            break;
        }

        case PacketType::CHAT: {
            std::string message;
            event.packet >> message;
            std::string chat_format = (curr_player->name == "_" ? "Protagonist" : curr_player->name) + ": " + message;
            print::info(chat_format);
            
            sf::Packet chat_packet;
            chat_packet << PacketType::CHAT << chat_format;
            sendToEveryone(chat_packet);
            break;
        }

        default:;
    }
}

void NetworkingManager::onEvent(ServerBlockChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.type;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerBlockBreakStageChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::BLOCK_PROGRESS_CHANGE << event.block.getX() << event.block.getY() << event.break_stage;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerLiquidChangeEvent& event) {
    sf::Packet packet;
    packet << PacketType::LIQUID_CHANGE << event.block.getX() << event.block.getY() << (unsigned char)event.liquid_type << event.liquid_level;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemCreationEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_CREATION << event.x << event.y << event.id << (unsigned char)event.item_id;
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemDeletionEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_DELETION << (short)event.item_to_delete.getId();
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerItemMovementEvent& event) {
    sf::Packet packet;
    packet << PacketType::ITEM_MOVEMENT << event.moved_item.getX() <<  event.moved_item.getY() << event.moved_item.getId();
    sendToEveryone(packet);
}

void NetworkingManager::onEvent(ServerLightChangeEvent& event) {
    unsigned short x = event.block.getX(), y = event.block.getY();
    sf::Packet packet;
    packet << PacketType::LIGHT_CHANGE << x << y << event.block.getLightLevel();
    
    for(Connection& connection : connections)
        if(connection.player->getSightBeginX() < x && x < connection.player->getSightEndX() && connection.player->getSightBeginY() < y && y < connection.player->getSightEndY())
            connection.socket->send(packet);
}

void NetworkingManager::sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack) {
    sf::Packet packet;
    packet << PacketType::INVENTORY_CHANGE << stack << (unsigned char)type << item.getPosInInventory();
    connection.socket->send(packet);
}

void NetworkingManager::onEvent(ServerInventoryItemStackChangeEvent& event) {
    //sendInventoryItemPacket(event.item, event.item.getType(), event.stack);
}

void NetworkingManager::onEvent(ServerInventoryItemTypeChangeEvent& event) {
    //sendInventoryItemPacket(event.item, event.type, event.item.getStack());
}
