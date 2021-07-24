#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>

#include "packetType.hpp"
#include "players.hpp"

class Connection {
    sf::TcpSocket* socket;
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    Player* player = nullptr;
    void send(sf::Packet& packet);
    sf::Socket::Status receive(sf::Packet& packet);
    std::string getIpAddress();
    void freeSocket();
    
};

class NetworkingManager : EventListener<ServerBlockChangeEvent>, EventListener<ServerBlockBreakStageChangeEvent>, EventListener<ServerLiquidChangeEvent>, EventListener<ServerItemCreationEvent>, EventListener<ServerItemDeletionEvent>, EventListener<ServerItemMovementEvent>, EventListener<ServerInventoryItemStackChangeEvent>, EventListener<ServerInventoryItemTypeChangeEvent> {
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
    void onEvent(ServerInventoryItemStackChangeEvent& event) override;
    void onEvent(ServerInventoryItemTypeChangeEvent& event) override;
    
    void onPacket(sf::Packet& packet, PacketType packet_type, Connection& conn);
    
    void sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack);
public:
    NetworkingManager(Blocks* blocks, Items* items, Players* players) : blocks(blocks), items(items), players(players) {}
    
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    
    void syncLightWithPlayers();
    
    bool accept_itself = false;
};

#endif /* serverNetworking_hpp */
