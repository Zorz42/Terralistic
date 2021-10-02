#ifndef serverNetworking_hpp
#define serverNetworking_hpp

#include <vector>
#include <string>
#include "packetType.hpp"
#include "serverPlayers.hpp"
#include "commands.hpp"

class Connection : public EventListener<InventoryItemChangeEvent> {
    sf::TcpSocket* socket;
    sf::Packet master_packet;
    
    void onEvent(InventoryItemChangeEvent& event) override;
public:
    Connection(sf::TcpSocket* socket) : socket(socket) {}
    ServerPlayer* player = nullptr;
    void send(sf::Packet& packet);
    void send(std::vector<char>& data);
    sf::Socket::Status receive(sf::Packet& packet);
    std::string getIpAddress();
    void freeSocket();
    void flushPacket();
};

class ServerNetworkingManager : EventListener<BlockChangeEvent>, EventListener<BlockBreakStageChangeEvent>, EventListener<LiquidChangeEvent>, EventListener<ItemCreationEvent>, EventListener<EntityDeletionEvent>, EventListener<EntityVelocityChangeEvent>, EventListener<EntityPositionChangeEvent> {
    std::vector<Connection> connections;
    sf::TcpListener listener;
    
    Blocks* blocks;
    Liquids* liquids;
    ServerPlayers* players;
    Items* items;
    Entities* entities;
    Commands commands;
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(BlockBreakStageChangeEvent& event) override;
    void onEvent(LiquidChangeEvent& event) override;
    void onEvent(ItemCreationEvent& event) override;
    void onEvent(EntityDeletionEvent& event) override;
    void onEvent(EntityVelocityChangeEvent& event) override;
    void onEvent(EntityPositionChangeEvent& event) override;
    
    void onPacket(sf::Packet& packet, PacketType packet_type, Connection& conn);
public:
    ServerNetworkingManager(Blocks* blocks, Liquids* liquids, ServerPlayers* players, Items* items, Entities* entities) : blocks(blocks), liquids(liquids), players(players), items(items), entities(entities), commands(blocks, players, items, entities) {}

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
