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
#include "serverNetworking.hpp"

#define BLOCK_WIDTH 16
#define MAX_LIGHT 100
#define INVENTORY_SIZE 20

class serverMap : serverPacketListener {
public:
    enum class blockType {AIR, DIRT, STONE_BLOCK, GRASS_BLOCK, STONE, WOOD, LEAVES};
    enum class itemType {NOTHING, STONE, DIRT, STONE_BLOCK};

    static void initBlocks();
    static void initItems();
    
protected:
    struct uniqueBlock {
        uniqueBlock(std::string  name, bool ghost, bool only_on_floor, bool transparent, itemType drop, unsigned short break_time) : ghost(ghost), only_on_floor(only_on_floor), transparent(transparent), name(std::move(name)), drop(drop), break_time(break_time) {}
        
        bool ghost, only_on_floor, transparent;
        std::string name;
        itemType drop;
        unsigned short break_time;
    };
    
    struct blockData {
        blockData(blockType block_id=blockType::AIR) : block_id(block_id) {}
        
        blockType block_id;
        unsigned char light_level = 0;
        bool light_source = false, update_light = true;
        unsigned short break_progress = 0;
        unsigned char break_stage = 0;
        
        uniqueBlock& getUniqueBlock() const;
    };
    
    struct uniqueItem {
        uniqueItem(std::string name, unsigned short stack_size, blockType places);
        std::string name;
        unsigned short stack_size;
        blockType places;
    };
    
protected:
public: // !!! should be protected
    static std::vector<uniqueItem> unique_items;
    static std::vector<uniqueBlock> unique_blocks;
    
public:
    class block {
        blockData* block_data = nullptr;
        unsigned short x, y;
        serverMap* parent_serverMap;
        
    public:
        block(unsigned short x, unsigned short y, blockData* block_data, serverMap* parent_serverMap) : x(x), y(y), block_data(block_data), parent_serverMap(parent_serverMap) {}
        block() = default;
        
        void update();
        void setType(blockType id, bool process=true);
        void breakBlock();
        void setBreakProgress(unsigned short ms);
        void lightUpdate();
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
        inline void scheduleLightUpdate() { block_data->update_light = true; }
        inline bool hasScheduledLightUpdate() { return block_data->update_light; }
        
        inline unsigned short getX() { return x; }
        inline unsigned short getY() { return y; }
        
        inline void _setLightLevel(unsigned char level) { block_data->light_level = level; }
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
        itemType item_id;
        uniqueItem& getUniqueItem() const;
        void setStack(unsigned short stack_);
        unsigned short getStack() const;
        unsigned short increaseStack(unsigned short stack_);
        bool decreaseStack(unsigned short stack_);
    private:
        unsigned short stack;
        inventory* holder;
    };

    class player;
    
    struct inventory {
        friend inventoryItem;
    public:
        inventory(unsigned short owner_id, serverMap* world_map);
        inventoryItem inventory_arr[INVENTORY_SIZE];
        char addItem(itemType id, int quantity);
        bool open = false;
        char selected_slot = 0;
        inventoryItem* getSelectedSlot();
        void swapWithMouseItem(inventoryItem* item);
        serverMap* world_map;
    private:
        inventoryItem mouse_item;
        unsigned short owner_id;
    };
    
    class player {
    public:
        player(unsigned short id, serverMap* world_map) : id(id), inventory(id, world_map) {}
        connection* conn;
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
    std::vector<player> all_players;
    std::vector<player*> online_players;
    
public:
    serverMap(serverNetworkingManager* manager) : manager(manager), serverPacketListener(manager) { listening_to = {packets::STARTED_BREAKING, packets::STOPPED_BREAKING, packets::RIGHT_CLICK, packets::CHUNK, packets::VIEW_SIZE_CHANGE, packets::PLAYER_MOVEMENT, packets::PLAYER_JOIN, packets::DISCONNECT, packets::INVENTORY_SWAP, packets::HOTBAR_SELECTION}; }
    
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
    void updateLight();
    
    ~serverMap();
};

#endif /* serverMap_hpp */
