#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>

#include "packetType.hpp"
#include "players.hpp"

class Connection {
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    sf::TcpSocket* socket;
    Player* player = nullptr;
};

class ServerPacketEvent : public Event<ServerPacketEvent> {
public:
    ServerPacketEvent(sf::Packet& packet, PacketType packet_type, Connection& conn) : packet(packet), packet_type(packet_type), conn(conn) {}
    sf::Packet& packet;
    PacketType packet_type;
    Connection& conn;
};

class NetworkingManager : EventListener<ServerPacketEvent>, EventListener<ServerBlockChangeEvent>, EventListener<ServerBlockBreakStageChangeEvent>, EventListener<ServerLiquidChangeEvent>, EventListener<ServerItemCreationEvent>, EventListener<ServerItemDeletionEvent>, EventListener<ServerItemMovementEvent>/*, EventListener<ServerInventoryItemStackChangeEvent>, EventListener<ServerInventoryItemTypeChangeEvent>*/ {
    void onEvent(ServerPacketEvent& event) override;
    std::vector<Connection> connections;
    sf::TcpListener listener;
    
    Blocks* blocks;
    Items* items;
    Players* players;
    
    void onEvent(ServerBlockChangeEvent& event) override;
    void onEvent(ServerBlockBreakStageChangeEvent& event) override;
    void onEvent(ServerLiquidChangeEvent& event) override;
    void onEvent(ServerItemCreationEvent& event) override;
    void onEvent(ServerItemDeletionEvent& event) override;
    void onEvent(ServerItemMovementEvent& event) override;
    //void onEvent(ServerInventoryItemStackChangeEvent& event) override;
    //void onEvent(ServerInventoryItemTypeChangeEvent& event) override;
public:
    NetworkingManager(Blocks* blocks, Items* items, Players* players) : blocks(blocks), items(items), players(players) {}
    
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
};

#endif /* serverNetworking_hpp */
