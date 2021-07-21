#ifndef players_hpp
#define players_hpp

#define INVENTORY_SIZE 20

#include "items.hpp"

class Inventory;
class Player;

class InventoryItem {
    unsigned short stack;
    Inventory* inventory;
    ItemType type;
public:
    InventoryItem() : inventory(nullptr), type(ItemType::NOTHING), stack(0) {}
    explicit InventoryItem(Inventory* holder) : inventory(holder), type(ItemType::NOTHING), stack(0) {}

    inline ItemType getType() { return type; }
    void setType(ItemType type_);
    void setTypeWithoutProcessing(ItemType type_);
    const ItemInfo& getUniqueItem() const;
    void setStack(unsigned short stack_);
    void setStackWithoutProcessing(unsigned short stack_);
    unsigned short getStack() const;
    unsigned short increaseStack(unsigned short stack_);
    bool decreaseStack(unsigned short stack_);
    unsigned char getPosInInventory();
    void syncWithClient();
    inline Inventory* getInventory() { return inventory; } 
};

class Inventory {
    friend InventoryItem;
    InventoryItem mouse_item;
    Player* player;
public:
    explicit Inventory(Player* owner);
    InventoryItem inventory_arr[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    bool open = false;
    unsigned char selected_slot = 0;
    InventoryItem* getSelectedSlot();
    void swapWithMouseItem(InventoryItem* item);
    inline Player& getPlayer() { return *player; }
};

class Player {
    static inline unsigned int curr_id = 0;
public:
    explicit Player() : id(curr_id++), inventory(this) {}
    std::string name;
    
    bool disconnected = false;
    
    const unsigned short id;
    bool flipped = false;
    int x = 0, y = 0;
    unsigned short sight_width = 0, sight_height = 0;
    Inventory inventory;
    
    bool breaking = false;
    unsigned short breaking_x{}, breaking_y{};
};

class Players;

struct blockEvents {
    void (*onUpdate)(Blocks*, Block*) = nullptr;
    void (*onRightClick)(Block*, Player*) = nullptr;
    void (*onLeftClick)(Block*, Player*) = nullptr;
};

class ServerInventoryItemTypeChangeEvent : public Event<ServerInventoryItemTypeChangeEvent> {
public:
    ServerInventoryItemTypeChangeEvent(InventoryItem& item, ItemType type) : item(item), type(type) {}
    InventoryItem& item;
    ItemType type;
};

class ServerInventoryItemStackChangeEvent : public Event<ServerInventoryItemStackChangeEvent> {
public:
    ServerInventoryItemStackChangeEvent(InventoryItem& item, unsigned short stack) : item(item), stack(stack) {}
    InventoryItem& item;
    unsigned short stack;
};

class Players : EventListener<ServerBlockUpdateEvent> {
    Items* parent_items;
    Blocks* parent_blocks;
    
    std::vector<Player*> all_players;
    std::vector<Player*> online_players;
    
    void onEvent(ServerBlockUpdateEvent& event) override;

    blockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
public:
    Players(Blocks* parent_blocks_, Items* parent_items_);
    
    void leftClickEvent(Block this_block, Player* peer, unsigned short tick_length);
    void rightClickEvent(Block this_block, Player* peer);
    
    inline const std::vector<Player*>& getAllPlayers() { return all_players; }
    inline const std::vector<Player*>& getOnlinePlayers() { return online_players; }
    
    Player* getPlayerByName(const std::string& name);
    
    Player* addPlayer(const std::string& name);
    void removePlayer(Player* player);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocks();
    void lookForItems();
    
    void saveTo(std::string path);
    void loadFrom(std::string path);
    
    void sendInventoryItemPacket(InventoryItem& item, ItemType type, unsigned short stack);
    
    bool accept_itself = false;
    
    ~Players();
};

#endif /* players_hpp */
