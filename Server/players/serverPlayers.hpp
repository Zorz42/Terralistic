#ifndef serverPlayers_hpp
#define serverPlayers_hpp

#define INVENTORY_SIZE 20

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14

#include <utility>
#include "serverItems.hpp"
#include "entities.hpp"
#include "movingType.hpp"

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

class ServerPlayers;

class ServerInventory {
    friend InventoryItem;
    InventoryItem mouse_item;
    unsigned int item_counts[(int)ItemType::NUM_ITEMS];
    std::vector<const Recipe*> available_recipes;
    ServerPlayers* players;
public:
    ServerInventory(ServerPlayers* players);
    bool hasIngredientsForRecipe(const Recipe& recipe);
    const std::vector<const Recipe*>& getAvailableRecipes();
    void updateAvailableRecipes();
    InventoryItem inventory_arr[INVENTORY_SIZE];
    char addItem(ItemType id, int quantity);
    char removeItem(ItemType id, int quantity);
    unsigned char selected_slot = 0;
    InventoryItem* getSelectedSlot();
    void swapWithMouseItem(InventoryItem* item);
    ServerPlayers* getPlayers() { return players; }
};

class ServerPlayer : public Entity {
public:
    explicit ServerPlayer(ServerPlayers* players, int x, int y, std::string name) : Entity(EntityType::PLAYER, x, y), name(std::move(name)), inventory(players) { friction = false; }
    explicit ServerPlayer(ServerPlayers* players, char*& iter);
    std::string name;
    
    unsigned short sight_width = 0, sight_height = 0;
    int sight_x = 0, sight_y = 0;
    unsigned short getSightBeginX() const;
    unsigned short getSightEndX() const;
    unsigned short getSightBeginY() const;
    unsigned short getSightEndY() const;
    
    ServerInventory inventory;
    MovingType moving_type = MovingType::STANDING;
    
    bool breaking = false;
    unsigned short breaking_x = 0, breaking_y = 0;
    
    void serialize(std::vector<char>& serial) const;
    
    unsigned short getWidth() override { return PLAYER_WIDTH * 2; }
    unsigned short getHeight() override { return PLAYER_HEIGHT * 2; }
    
    bool isColliding(Blocks* blocks) override;
};

struct BlockEvents {
    void (*onUpdate)(Blocks* blocks, unsigned short x, unsigned short y) = nullptr;
    void (*onRightClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
    void (*onLeftClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
};

class ServerInventoryItemTypeChangeEvent {
public:
    ServerInventoryItemTypeChangeEvent(InventoryItem& item, ItemType type) : item(item), type(type) {}
    InventoryItem& item;
    ItemType type;
};

class ServerInventoryItemStackChangeEvent {
public:
    ServerInventoryItemStackChangeEvent(InventoryItem& item, unsigned short stack) : item(item), stack(stack) {}
    InventoryItem& item;
    unsigned short stack;
};



class RecipeAvailabilityChangeEvent {
public:
    explicit RecipeAvailabilityChangeEvent(ServerInventory* inventory) : inventory(inventory) {}
    ServerInventory* inventory;
};

class ServerPlayers : EventListener<BlockChangeEvent> {
    Entities* entities;
    Blocks* blocks;
    ServerItems* items;
    
    std::vector<ServerPlayer*> all_players;
    std::vector<ServerPlayer*> online_players;

    BlockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    void onEvent(BlockChangeEvent& event) override;
    
    void leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y, unsigned short tick_length);
public:
    ServerPlayers(Blocks* blocks, Entities* entities, ServerItems* items);
    void init();
    void rightClickEvent(ServerPlayer* player, unsigned short x, unsigned short y);
    
    const std::vector<ServerPlayer*>& getAllPlayers() { return all_players; }
    const std::vector<ServerPlayer*>& getOnlinePlayers() { return online_players; }
    
    ServerPlayer* getPlayerByName(const std::string& name);
    ServerPlayer* addPlayer(const std::string& name);
    char* addPlayerFromSerial(char* iter);
    void removePlayer(ServerPlayer* player);
    
    void updatePlayersBreaking(unsigned short tick_length);
    void updateBlocksInVisibleAreas();
    void lookForItemsThatCanBePickedUp();
    
    EventSender<ServerInventoryItemTypeChangeEvent> inventory_item_type_change_event;
    EventSender<ServerInventoryItemStackChangeEvent> inventory_item_stack_change_event;
    EventSender<RecipeAvailabilityChangeEvent> recipe_availability_change_event;
    
    ~ServerPlayers();
};

#endif
