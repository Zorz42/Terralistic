#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>
#include "packetType.hpp"
#include "serverPlayers.hpp"
#include "commands.hpp"

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

class ServerNetworkingManager : EventListener<BlockChangeEvent>, EventListener<BlockBreakStageChangeEvent>, EventListener<LiquidChangeEvent>, EventListener<ServerItemCreationEvent>, EventListener<ServerItemDeletionEvent>, EventListener<EntityVelocityChangeEvent>, EventListener<ServerInventoryItemStackChangeEvent>, EventListener<ServerInventoryItemTypeChangeEvent>, EventListener<RecipeAvailabilityChangeEvent>, EventListener<EntityPositionChangeEvent> {
    std::vector<Connection> connections;
    sf::TcpListener listener;
    
    Blocks* blocks;
    Liquids* liquids;
    ServerPlayers* players;
    ServerItems* items;
    Entities* entities;
    Commands commands;
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockBreakStageChangeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    void onEvent(ServerItemCreationEvent& event) override;
    void onEvent(ServerItemDeletionEvent& event) override;
    void onEvent(EntityVelocityChangeEvent& event) override;
    void onEvent(ServerInventoryItemStackChangeEvent& event) override;
    void onEvent(ServerInventoryItemTypeChangeEvent& event) override;
    void onEvent(RecipeAvailabilityChangeEvent& event) override;
    void onEvent(EntityPositionChangeEvent& event) override;
    
    void onPacket(sf::Packet& packet, PacketType packet_type, Connection& conn);
    
    static void sendInventoryItemPacket(Connection& connection, InventoryItem& item, ItemType type, unsigned short stack);
public:
    ServerNetworkingManager(Blocks* blocks, Liquids* liquids, ServerPlayers* players, ServerItems* items, Entities* entities) : blocks(blocks), liquids(liquids), players(players), items(items), entities(entities), commands(blocks, players, items, entities) {}

    void init();
    
    void sendToEveryone(sf::Packet& packet, Connection* exclusion=nullptr);
    
    void openSocket(unsigned short port);
    void closeSocket();
    
    void checkForNewConnections();
    void getPacketsFromPlayers();
    void flushPackets();
    void syncEntityPositions();
    
    bool accept_itself = false;
};

#endif
