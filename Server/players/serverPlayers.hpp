#ifndef serverPlayers_hpp
#define serverPlayers_hpp

#include <utility>
#include "items.hpp"
#include "player.hpp"
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

class ServerPlayer : public Player, EventListener<InventoryItemChangeEvent> {
    Connection* connection = nullptr;
    
    void onEvent(InventoryItemChangeEvent& event) override;
public:
    ServerPlayer(const ServerPlayerData& data) : Player(data.x, data.y, data.name), inventory(data.inventory) { friction = false; inventory.item_change_event.addListener(this); }
    
    Inventory inventory;
    
    void setConnection(Connection* connection);
    Connection* getConnection();
    
    bool breaking = false;
    int breaking_x = 0, breaking_y = 0;
    
    void destruct();
};

struct BlockEvents {
    void (*onUpdate)(Blocks* blocks, unsigned short x, unsigned short y) = nullptr;
    void (*onRightClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
    void (*onLeftClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
};

class ServerPacketEvent {
public:
    ServerPacketEvent(sf::Packet& packet, ClientPacketType packet_type, ServerPlayer* player) : packet(packet), packet_type(packet_type), player(player) {}
    sf::Packet& packet;
    ClientPacketType packet_type;
    ServerPlayer* player;
};

class ServerPlayers : public ServerModule, EventListener<BlockChangeEvent>, EventListener<ServerNewConnectionEvent>, EventListener<ServerConnectionWelcomeEvent>, EventListener<ServerPacketEvent>, EventListener<ServerDisconnectEvent> {
    Entities* entities;
    Blocks* blocks;
    Items* items;
    ServerNetworking* networking;
    
    std::vector<ServerPlayerData*> all_players;

    BlockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    void onEvent(BlockChangeEvent& event) override;
    void onEvent(ServerNewConnectionEvent& event) override;
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(ServerPacketEvent& event) override;
    void onEvent(ServerDisconnectEvent& event) override;
    
    void leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y);
    void rightClickEvent(ServerPlayer* player, unsigned short x, unsigned short y);
    
    void init() override;
    void update(float frame_length) override;
    void stop() override;
    
public:
    ServerPlayers(Blocks* blocks, Entities* entities, Items* items, ServerNetworking* networking) : blocks(blocks), entities(entities), items(items), networking(networking) {}
    
    const std::vector<ServerPlayerData*>& getAllPlayers() { return all_players; }
    
    ServerPlayer* getPlayerByName(const std::string& name);
    ServerPlayer* addPlayer(const std::string& name);
    char* addPlayerFromSerial(char* iter);
    void savePlayer(ServerPlayer* player);
    ServerPlayerData* getPlayerData(const std::string& name);
    
    EventSender<ServerPacketEvent> packet_event;
};

#endif
