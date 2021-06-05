 //
//  serverMap.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/04/2021.
//

#ifndef serverMap_hpp
#define serverMap_hpp

#include <vector>
#include <string>
#include <chrono>
#include "serverNetworking.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100
#define INVENTORY_SIZE 20

#define UNBREAKABLE -1

class serverMap : serverPacketListener {
public:
    enum class blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES, SAND, SNOWY_GRASS_BLOCK};
    enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK, WOOD_PLANKS};
    enum class liquidType {EMPTY, WATER};
    enum class flowDirection {NONE, LEFT, RIGHT, BOTH = LEFT | RIGHT};

    static void initBlocks();
    static void initItems();
    static void initLiquids();
    
    class block;
    class player;
    
protected:
    struct uniqueBlock {
        uniqueBlock(const std::string& name, bool ghost, bool only_on_floor, bool transparent, itemType drop, short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(name), drop(drop), break_time(break_time) {}
        
        bool ghost, only_on_floor, transparent;
        std::string name;
        itemType drop;
        short break_time;
        
        void (*onBreak)(serverMap*, block*) = nullptr;
        void (*onRightClick)(block*, player*) = nullptr;
        void (*onLeftClick)(block*, player*) = nullptr;
    };
    
    struct uniqueLiquid {
        uniqueLiquid(unsigned short flow_time) : flow_time(flow_time) {}
        unsigned short flow_time;
    };
    
    struct uniqueItem {
        uniqueItem(std::string name, unsigned short stack_size, blockType places);
        std::string name;
        unsigned short stack_size;
        blockType places;
    };
    
    struct blockData {
        blockData(blockType block_id=blockType::AIR, liquidType liquid_id=liquidType::EMPTY) : block_id(block_id), liquid_id(liquid_id) {}
        
        blockType block_id;
        liquidType liquid_id = liquidType::EMPTY;
        bool light_source = false, update_light = true;
        unsigned short break_progress = 0;
        unsigned char break_stage = 0, liquid_level = 0, light_level = 0;
        unsigned int when_to_update_liquid = 1;
        flowDirection flow_direction = flowDirection::NONE;
        
        uniqueBlock& getUniqueBlock() const;
        uniqueLiquid& getUniqueLiquid() const;
    };
    
    static std::vector<uniqueItem> unique_items;
    static std::vector<uniqueBlock> unique_blocks;
    static std::vector<uniqueLiquid> unique_liquids;

public:
    class block {
        blockData* block_data = nullptr;
        unsigned short x, y;
        serverMap* parent_map;
        
        void syncWithClient();
        void updateNeighbors();
        
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, serverMap* parent_map) : x(x), y(y), block_data(block_data), parent_map(parent_map) {}
        block() = default;
        
        void update();
        void setType(blockType block_id, bool process=true);
        void setType(liquidType liquid_id, bool process=true);
        void setType(blockType block_id, liquidType liquid_id, bool process=true);
        void breakBlock();
        void setBreakProgress(unsigned short ms);
        void lightUpdate();
        void liquidUpdate();
        void setLightSource(unsigned char power);
        void removeLightSource();
        
        inline bool refersToABlock() { return block_data != nullptr; }
        
        inline bool isTransparent() { return block_data->getUniqueBlock().transparent; }
        inline bool isOnlyOnFloor() { return block_data->getUniqueBlock().only_on_floor; }
        inline bool isLightSource() { return block_data->light_source; }
        inline bool isGhost() { return block_data->getUniqueBlock().ghost; }
        inline unsigned short getBreakTime() { return block_data->getUniqueBlock().break_time; }
        inline unsigned char getLightLevel() { return block_data->light_level; }
        inline unsigned short getBreakProgress() { return block_data->break_progress; }
        inline unsigned char getBreakStage() { return block_data->break_stage; }
        inline itemType getDrop() { return block_data->getUniqueBlock().drop; }
        inline blockType getType() { return block_data->block_id; }
        inline liquidType getLiquidType() { return block_data->liquid_id; }
        inline void scheduleLightUpdate() { block_data->update_light = true; }
        inline bool hasScheduledLightUpdate() { return block_data->update_light; }
        inline bool canUpdateLiquid() { return block_data->when_to_update_liquid != 0 && (unsigned int)std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() > block_data->when_to_update_liquid; }
        void setLiquidLevel(unsigned char level);
        inline unsigned char getLiquidLevel() { return block_data->liquid_level; }
        inline unsigned short getFlowTime() { return block_data->getUniqueLiquid().flow_time; }
        inline flowDirection getFlowDirection() { return block_data->flow_direction; }
        inline void setFlowDirection(flowDirection flow_direction) { block_data->flow_direction = flow_direction; }
        
        inline unsigned short getX() { return x; }
        inline unsigned short getY() { return y; }
        
        inline void _setLightLevel(unsigned char level) { block_data->light_level = level; }
        
        void leftClickEvent(connection& connection, unsigned short tick_length);
        void rightClickEvent(player* peer);
    };
    
    struct item {
        void create(itemType item_id_, int x_, int y_, unsigned short id_, serverMap& world_serverMap);
        void destroy(serverMap& world_serverMap);
        int x, y;
        void update(float frame_length, serverMap& world_serverMap);
        bool colliding(serverMap& world_serverMap) const;
        uniqueItem& getUniqueItem() const;
        unsigned short getId() { return id; }
        itemType getItemId() { return item_id; }
    protected:
        int velocity_x, velocity_y;
        unsigned short id;
        itemType item_id;
    };
    
    class inventory;
    
    struct inventoryItem {
        inventoryItem() : holder(nullptr), item_id(serverMap::itemType::NOTHING), stack(0) {}
        explicit inventoryItem(inventory* holder) : holder(holder), item_id(serverMap::itemType::NOTHING), stack(0) {}
        explicit inventoryItem(inventory* holder, itemType item_id) : holder(holder), item_id(item_id), stack(1) {}
        
        inline itemType getId() { return item_id; }
        void setId(itemType id);
        uniqueItem& getUniqueItem() const;
        void setStack(unsigned short stack_);
        unsigned short getStack() const;
        unsigned short increaseStack(unsigned short stack_);
        bool decreaseStack(unsigned short stack_);
        void sendPacket();
    private:
        unsigned short stack;
        inventory* holder;
        itemType item_id;
    };
    
    struct inventory {
        friend inventoryItem;
    public:
        inventory(player* owner);
        inventoryItem inventory_arr[INVENTORY_SIZE];
        char addItem(itemType id, int quantity);
        bool open = false;
        char selected_slot = 0;
        inventoryItem* getSelectedSlot();
        void swapWithMouseItem(inventoryItem* item);
    private:
        inventoryItem mouse_item;
        player* owner;
    };
    
    class player {
    public:
        player(unsigned short id, serverMap* world_map) : id(id), inventory(this) {}
        connection* conn = nullptr;
        const unsigned short id;
        bool flipped = false;
        int x = 0, y = 0;
        unsigned short sight_width = 0, sight_height = 0;
        inventory inventory;
        unsigned short breaking_x, breaking_y;
        bool breaking = false;
        std::string name;
    };
    
protected:
    unsigned short width, height;
    blockData *blocks = nullptr;
    
    void removeNaturalLight(unsigned short x);
    void setNaturalLight(unsigned short x);
    
    void onPacket(packets::packet& packet, connection& conn);
    
    serverNetworkingManager* manager;
    std::vector<item> items;
    std::vector<player*> all_players;
    std::vector<player*> online_players;
    
public:
    serverMap(serverNetworkingManager* manager) : manager(manager), serverPacketListener(manager) { listening_to = {packets::STARTED_BREAKING, packets::STOPPED_BREAKING, packets::RIGHT_CLICK, packets::CHUNK, packets::VIEW_SIZE_CHANGE, packets::PLAYER_MOVEMENT, packets::PLAYER_JOIN, packets::DISCONNECT, packets::INVENTORY_SWAP, packets::HOTBAR_SELECTION}; }
    
    std::vector<player*> getAllPlayers() { return all_players; } 
    
    block getBlock(unsigned short x, unsigned short y);
    
    int getSpawnX();
    int getSpawnY();
    
    inline unsigned short getWorldWidth() { return width; }
    inline unsigned short getWorldHeight() { return height; }
    
    void createWorld(unsigned short width, unsigned short height);
    void setNaturalLight();
    void spawnItem(itemType item_id, int x, int y, short id=-1);
    
    item* getItemById(unsigned short id);
    player* getPlayerByConnection(connection* conn);
    player* getPlayerByName(const std::string& name);
    player* getPlayerById(unsigned short id);
    
    void updateItems(float frame_length);
    void updatePlayersBreaking(unsigned short tick_length);
    void lookForItems(serverMap& world_serverMap);
    void updateBlocks();
    
    ~serverMap();
};

#endif /* serverMap_hpp */
