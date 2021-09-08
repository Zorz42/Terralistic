#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>
#include "packetType.hpp"
#include "serverPlayers.hpp"

class Connection {
    sf::TcpSocket* socket;
    sf::Packet master_packet;
public:
    explicit Connection(sf::TcpSocket* socket) : socket(socket) {}
    ServerPlayer* player = nullptr;
    void send(sf::Packet& packet);
    void send(std::vector<char>& data);
    sf::Socket::Status receive(sf::Packet& packet);
    std::string getIpAddress();
    void freeSocket();
    void flushPacket();
};

class ServerNetworkingManager : EventListener<ServerBlockChangeEvent>, EventListener<ServerBlockBreakStageChangeEvent>, EventListener<ServerLiquidChangeEvent>, EventListener<ServerItemCreationEvent>, EventListener<ServerEntityDeletionEvent>, EventListener<ServerEntityVelocityChangeEvent>, EventListener<ServerInventoryItemStackChangeEvent>, EventListener<ServerInventoryItemTypeChangeEvent>, EventListener<RecipeAvailabilityChangeEvent>, EventListener<ServerLightChangeEvent> {
    std::vector<Connection> connections;
    sf::TcpListener listener;
    
    ServerBlocks* blocks;
    ServerEntities* entities;
    Players* players;
    
    void onEvent(ServerBlockChangeEvent& event) override;
    void onEvent(ServerBlockBreakStageChangeEvent& event) override;
    void onEvent(ServerLiquidChangeEvent& event) override;
    void onEvent(ServerItemCreationEvent& event) override;
    void onEvent(ServerEntityDeletionEvent& event) override;
    void onEvent(ServerEntityVelocityChangeEvent& event) override;
    void onEvent(ServerInventoryItemStackChangeEvent& event) override;
    void onEvent(ServerInventoryItemTypeChangeEvent& event) override;
    void onEvent(RecipeAvailabilityChangeEvent& event) override;
    void onEvent(ServerLightChangeEvent& event) override;
    
    void onPacket(sf::Packet& packet, PacketType packet_type, Connection& conn);
    
    void sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack);
public:
    ServerNetworkingManager(ServerBlocks* blocks, ServerEntities* entities, Players* players) : blocks(blocks), entities(entities), players(players) {}
    
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    void flushPackets();
    
    bool accept_itself = false;
};

#endif
