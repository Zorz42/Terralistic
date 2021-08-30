#ifndef serverPlayers_hpp
#define serverPlayers_hpp

#define INVENTORY_SIZE 20

#include <utility>

#include "serverItems.hpp"

class ServerInventory;

class InventoryItem : ItemStack {
    ServerInventory* inventory;
public:
    InventoryItem() : inventory(nullptr) {}
    explicit InventoryItem(ServerInventory* holder) : inventory(holder) {}
    
    char* loadFromSerial(char* iter);
    void serialize(std::vector<char>& serial) const;
    
    ItemType getType() const { return type; }
    void setType(ItemType type_);
    void setTypeDirectly(ItemType type_);
    
    const ItemInfo& getUniqueItem() const;
    unsigned short getStack() const;
    void setStack(unsigned short stack_);
    void setStackDirectly(unsigned short stack_);
    
    unsigned short increaseStack(unsigned short stack_);
    unsigned short decreaseStack(unsigned short stack_);
    
    short getPosInInventory();
    ServerInventory* getInventory() { return inventory; }
};

class ServerInventory {
    friend InventoryItem;
    InventoryItem mouse_item;
    unsigned int item_counts[(int)ItemType::NUM_ITEMS];
    std::vector<const Recipe*> available_recipes;
public:
    bool hasIngredientsForRecipe(const Recipe& recipe);
    const std::vector<const Recipe*>& getAvailableRecipes();
    void updateAvailableRecipes();
    ServerInventory();
    InventoryItem inventory_arr[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    char removeItem(ItemType id, int quantity);
    unsigned char selected_slot = 0;
    InventoryItem* getSelectedSlot();
    void swapWithMouseItem(InventoryItem* item);
};

class ServerPlayer {
    static inline unsigned int curr_id = 0;
public:
    explicit ServerPlayer(std::string name) : id(curr_id++), name(std::move(name)) {}
    ServerPlayer(char*& iter);
    std::string name;
    const unsigned short id;
    
    bool flipped = false;
    int x = 0, y = 0;
    unsigned short sight_width = 0, sight_height = 0;
    int sight_x = 0, sight_y = 0;
    unsigned short getSightBeginX() const;
    unsigned short getSightEndX() const;
    unsigned short getSightBeginY() const;
    unsigned short getSightEndY() const;
    
    ServerInventory inventory;
    
    bool breaking = false;
    unsigned short breaking_x = 0, breaking_y = 0;
    
    void serialize(std::vector<char>& serial) const;
};

struct blockEvents {
    void (*onUpdate)(ServerBlocks*, ServerBlock*) = nullptr;
    void (*onRightClick)(ServerBlock*, ServerPlayer*) = nullptr;
    void (*onLeftClick)(ServerBlock*, ServerPlayer*) = nullptr;
};

class Players : EventListener<ServerBlockUpdateEvent> {
    ServerItems* items;
    ServerBlocks* blocks;
    
    std::vector<ServerPlayer*> all_players;
    std::vector<ServerPlayer*> online_players;
    
    void onEvent(ServerBlockUpdateEvent& event) override;

    blockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    void leftClickEvent(ServerBlock this_block, ServerPlayer* peer, unsigned short tick_length);
public:
    Players(ServerBlocks* parent_blocks, ServerItems* parent_items);
    
    void rightClickEvent(ServerBlock this_block, ServerPlayer* peer);
    
    const std::vector<ServerPlayer*>& getAllPlayers() { return all_players; }
    const std::vector<ServerPlayer*>& getOnlinePlayers() { return online_players; }
    
    ServerPlayer* getPlayerByName(const std::string& name);
    ServerPlayer* addPlayer(const std::string& name);
    char* addPlayerFromSerial(char* iter);
    void removePlayer(ServerPlayer* player);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocksInVisibleAreas();
    void lookForItemsThatCanBePickedUp();
    
    ~Players() override;
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

class RecipeAvailabilityChangeEvent : public Event<RecipeAvailabilityChangeEvent> {
public:
    RecipeAvailabilityChangeEvent(ServerInventory* inventory) : inventory(inventory) {}
    ServerInventory* inventory;
};

#endif
