//
//  players.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 22/06/2021.
//

#ifndef players_hpp
#define players_hpp

#define INVENTORY_SIZE 20

#include "items.hpp"

class inventory;
class player;

class inventoryItem {
    unsigned short stack;
    inventory* holder;
    itemType item_id;
public:
    inventoryItem() : holder(nullptr), item_id(itemType::NOTHING), stack(0) {}
    explicit inventoryItem(inventory* holder) : holder(holder), item_id(itemType::NOTHING), stack(0) {}

    inline itemType getId() { return item_id; }
    void setId(itemType id);
    [[nodiscard]] const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    [[nodiscard]] unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    void sendPacket();
};

class inventory {
    friend inventoryItem;
    inventoryItem mouse_item;
    player* owner;
public:
    explicit inventory(player* owner);
    inventoryItem inventory_arr[INVENTORY_SIZE];
    char addItem(itemType id, int quantity);
    bool open = false;
    char selected_slot = 0;
    inventoryItem* getSelectedSlot();
    void swapWithMouseItem(inventoryItem* item);
};

class player {
public:
    explicit player(unsigned short id) : id(id), player_inventory(this) {}
    connection* conn = nullptr;
    const unsigned short id;
    bool flipped = false;
    int x = 0, y = 0;
    unsigned short sight_width = 0, sight_height = 0;
    inventory player_inventory;
    unsigned short breaking_x{}, breaking_y{};
    bool breaking = false;
    std::string name;
};

class players;

struct blockEvents {
    void (*onBreak)(blocks*, players*, block*) = nullptr;
    void (*onRightClick)(block*, player*) = nullptr;
    void (*onLeftClick)(block*, player*) = nullptr;
};

class players : serverPacketListener {
    items* parent_items;
    blocks* parent_blocks;
    serverNetworkingManager* manager;
    
    std::vector<player*> all_players;
    std::vector<player*> online_players;
    
    void onPacket(Packet& packet, connection& conn) override;
    
    void leftClickEvent(block this_block, connection& connection, unsigned short tick_length);
    void rightClickEvent(block this_block, player* peer);
    
    blockEvents custom_block_events[(int)blockType::NUM_BLOCKS];
public:
    players(serverNetworkingManager* manager_, blocks* parent_blocks_, items* parent_items_);
    
    inline const std::vector<player*>& getAllPlayers() { return all_players; }
    inline const std::vector<player*>& getOnlinePlayers() { return online_players; }
    
    player* getPlayerByConnection(connection* conn);
    player* getPlayerByName(const std::string& name);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocks();
    void lookForItems();
    
    void saveTo(std::string path);
    void loadFrom(std::string path);
    
    void breakBlock(block* this_block);
    
    ~players();
};

#endif /* players_hpp */
