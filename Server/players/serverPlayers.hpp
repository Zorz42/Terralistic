#pragma once
#include "serverBlocks.hpp"
#include "player.hpp"
#include "inventory.hpp"
#include "serverNetworking.hpp"
#include "worldSaver.hpp"
#include "liquids.hpp"

class ServerPlayerData {
public:
    ServerPlayerData(Items* items, Recipes* recipes) : inventory(items, recipes) {}
    
    std::string name;
    int x = 0, y = 0;
    int health = 0;
    Inventory inventory;
};

class ServerPlayer : public Player, EventListener<InventoryItemChangeEvent> {
    Connection* connection = nullptr;
    
    void onEvent(InventoryItemChangeEvent& event) override;
public:
    explicit ServerPlayer(const ServerPlayerData& data) : Player(data.x, data.y , data.name), inventory(data.inventory), health(data.health) { friction = false; inventory.item_change_event.addListener(this); }

    Inventory inventory;
    
    void setConnection(Connection* connection);
    Connection* getConnection();

    int health;
    bool breaking = false;
    int breaking_x = 0, breaking_y = 0;
    
    ~ServerPlayer() override;
};

class BlockBehaviour {
public:
    Blocks* blocks;
    Walls* walls;
    Liquids* liquids;
    BlockBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : blocks(blocks), walls(walls), liquids(liquids) {}
    virtual void onUpdate(int x, int y) {}
    virtual void onRandomTick(int x, int y) {}
    virtual void onRightClick(int x, int y, ServerPlayer* player) {}
    virtual void onLeftClick(int x, int y, ServerPlayer* player) {
        if(blocks->getBlockType(x, y)->break_time != UNBREAKABLE)
            blocks->startBreakingBlock(x, y);
    }
};

class AirBehaviour : public BlockBehaviour {
    void onRightClick(int x, int y, ServerPlayer* player) override;
public:
    AirBehaviour(Blocks* blocks, Walls* walls, Liquids* liquids) : BlockBehaviour(blocks, walls, liquids) {}
};

class ServerPacketEvent {
public:
    ServerPacketEvent(sf::Packet& packet, ClientPacketType packet_type, ServerPlayer* player) : packet(packet), packet_type(packet_type), player(player) {}
    sf::Packet& packet;
    ClientPacketType packet_type;
    ServerPlayer* player;
};

class ServerPlayers : public ServerModule, EventListener<BlockUpdateEvent>, EventListener<BlockRandomTickEvent>, EventListener<ServerNewConnectionEvent>, EventListener<ServerConnectionWelcomeEvent>, EventListener<ServerPacketEvent>, EventListener<ServerDisconnectEvent>, EventListener<WorldSaveEvent>, EventListener<WorldLoadEvent>, EventListener<EntityAbsoluteVelocityChangeEvent> {
    Entities* entities;
    ServerBlocks* blocks;
    Walls* walls;
    Items* items;
    Recipes* recipes;
    ServerNetworking* networking;
    WorldSaver* world_saver;
    
    std::vector<ServerPlayerData*> all_players;

    BlockBehaviour **blocks_behaviour = nullptr;
    
    void onEvent(BlockUpdateEvent& event) override;
    void onEvent(BlockRandomTickEvent& event) override;
    void onEvent(ServerNewConnectionEvent& event) override;
    void onEvent(ServerConnectionWelcomeEvent& event) override;
    void onEvent(ServerPacketEvent& event) override;
    void onEvent(ServerDisconnectEvent& event) override;
    void onEvent(WorldSaveEvent& event) override;
    void onEvent(WorldLoadEvent& event) override;
    void onEvent(EntityAbsoluteVelocityChangeEvent& event) override;
    
    void leftClickEvent(ServerPlayer* player, int x, int y);
    void rightClickEvent(ServerPlayer* player, int x, int y);
    
    void init() override;
    void postInit() override;
    void update(float frame_length) override;
    void stop() override;
    
    BlockBehaviour default_behaviour;
    AirBehaviour air_behaviour;
    
public:
    ServerPlayers(ServerBlocks* blocks, Walls* walls, Liquids* liquids, Entities* entities, Items* items, ServerNetworking* networking, Recipes* recipes, WorldSaver* world_saver) : blocks(blocks), walls(walls), entities(entities), items(items), networking(networking), recipes(recipes), world_saver(world_saver), default_behaviour(blocks, walls, liquids), air_behaviour(blocks, walls, liquids) {}
    
    ServerPlayer* getPlayerByName(const std::string& name);
    ServerPlayer* addPlayer(const std::string& name);
    void savePlayer(ServerPlayer* player);
    ServerPlayerData* getPlayerData(const std::string& name);
    void setPlayerHealth(ServerPlayer* player, int health);
    void resetPlayer(ServerPlayer* player);
    
    std::vector<char> toSerial();
    void fromSerial(const std::vector<char>& serial);
    
    BlockBehaviour*& getBlockBehaviour(BlockType* type);
    
    EventSender<ServerPacketEvent> packet_event;
};
