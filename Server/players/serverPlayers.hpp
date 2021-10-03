#ifndef serverPlayers_hpp
#define serverPlayers_hpp

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14

#include <utility>
#include "items.hpp"
#include "entities.hpp"
#include "movingType.hpp"
#include "inventory.hpp"
#include "serverNetworking.hpp"

class ServerPlayerData {
public:
    ServerPlayerData(char*& iter);
    ServerPlayerData() = default;
    
    void serialize(std::vector<char>& serial) const;
    
    std::string name;
    int x, y;
    Inventory inventory;
};

class ServerPlayer : public Entity {
    Connection* connection = nullptr;
public:
    explicit ServerPlayer(const ServerPlayerData& data) : Entity(EntityType::PLAYER, data.x, data.y), name(std::move(data.name)), inventory(data.inventory) { friction = false; }
    std::string name;
    
    Inventory inventory;
    MovingType moving_type = MovingType::STANDING;
    void setConnection(Connection* connection);
    Connection* getConnection();
    
    bool breaking = false;
    unsigned short breaking_x = 0, breaking_y = 0;
    
    unsigned short getWidth() override { return PLAYER_WIDTH * 2; }
    unsigned short getHeight() override { return PLAYER_HEIGHT * 2; }
    
    bool isColliding(Blocks* blocks) override;
};

struct BlockEvents {
    void (*onUpdate)(Blocks* blocks, unsigned short x, unsigned short y) = nullptr;
    void (*onRightClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
    void (*onLeftClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
};

class ServerPacketEvent {
public:
    ServerPacketEvent(sf::Packet& packet, PacketType packet_type, ServerPlayer* player) : packet(packet), packet_type(packet_type), player(player) {}
    sf::Packet& packet;
    PacketType packet_type;
    ServerPlayer* player;
};

class ServerPlayers : EventListener<BlockChangeEvent>, EventListener<ServerNewConnectionEvent>, EventListener<ServerConnectionWelcomeEvent>, EventListener<ServerPacketEvent> {
    Entities* entities;
    Blocks* blocks;
    Items* items;
    ServerNetworking* networking_manager;
    
    std::vector<ServerPlayerData*> all_players;

    BlockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(ServerNewConnectionEvent& event) override;
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(ServerPacketEvent& event) override;
    
    void leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y, unsigned short tick_length);
public:
    ServerPlayers(Blocks* blocks, Entities* entities, Items* items, ServerNetworking* networking_manager);
    void init();
    void rightClickEvent(ServerPlayer* player, unsigned short x, unsigned short y);
    
    const std::vector<ServerPlayerData*>& getAllPlayers() { return all_players; }
    
    ServerPlayer* getPlayerByName(const std::string& name);
    ServerPlayer* addPlayer(const std::string& name);
    char* addPlayerFromSerial(char* iter);
    void savePlayer(ServerPlayer* player);
    ServerPlayerData* getPlayerData(const std::string& name);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void lookForItemsThatCanBePickedUp();
    void getPacketsFromPlayers();
    
    EventSender<ServerPacketEvent> packet_event;
    
    ~ServerPlayers();
};

#endif
