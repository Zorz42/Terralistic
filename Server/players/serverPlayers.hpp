#ifndef serverPlayers_hpp
#define serverPlayers_hpp

#define INVENTORY_SIZE 20

#define PLAYER_HEIGHT 24
#define PLAYER_WIDTH 14

#include <utility>
#include "items.hpp"
#include "entities.hpp"
#include "movingType.hpp"

class ServerInventoryItemChangeEvent {
public:
    ServerInventoryItemChangeEvent(char item_pos) : item_pos(item_pos) {}
    char item_pos;
};

class RecipeAvailabilityChangeEvent {};


class ServerInventory {
    ItemStack mouse_item;
    unsigned int item_counts[(int)ItemType::NUM_ITEMS];
    std::vector<const Recipe*> available_recipes;
    ItemStack inventory_arr[INVENTORY_SIZE];
    bool hasIngredientsForRecipe(const Recipe& recipe);
public:
    ServerInventory(char*& iter);
    ServerInventory();
    
    unsigned char selected_slot = 0;
    
    const std::vector<const Recipe*>& getAvailableRecipes();
    void updateAvailableRecipes();
    
    char addItem(ItemType id, int quantity);
    char removeItem(ItemType id, int quantity);
    void setItem(char pos, ItemStack item);
    ItemStack getItem(char pos);
    
    ItemStack getSelectedSlot();
    void swapWithMouseItem(char pos);
    
    unsigned short increaseStack(char pos, unsigned short stack);
    unsigned short decreaseStack(char pos, unsigned short stack);
    
    void serialize(std::vector<char>& serial) const;
    
    EventSender<ServerInventoryItemChangeEvent> item_change_event;
    EventSender<RecipeAvailabilityChangeEvent> recipe_availability_change_event;
};

class ServerPlayerData {
public:
    ServerPlayerData(char*& iter);
    ServerPlayerData() = default;
    
    void serialize(std::vector<char>& serial) const;
    
    std::string name;
    int x, y;
    ServerInventory inventory;
};

class ServerPlayer : public Entity {
public:
    explicit ServerPlayer(const ServerPlayerData& data) : Entity(EntityType::PLAYER, data.x, data.y), name(std::move(data.name)), inventory(data.inventory) { friction = false; }
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
    
    unsigned short getWidth() override { return PLAYER_WIDTH * 2; }
    unsigned short getHeight() override { return PLAYER_HEIGHT * 2; }
    
    bool isColliding(Blocks* blocks) override;
};

struct BlockEvents {
    void (*onUpdate)(Blocks* blocks, unsigned short x, unsigned short y) = nullptr;
    void (*onRightClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
    void (*onLeftClick)(Blocks* blocks, unsigned short x, unsigned short y, ServerPlayer* player) = nullptr;
};

class ServerPlayers : EventListener<BlockChangeEvent> {
    Entities* entities;
    Blocks* blocks;
    Items* items;
    
    std::vector<ServerPlayerData*> all_players;

    BlockEvents custom_block_events[(int)BlockType::NUM_BLOCKS];
    
    void onEvent(BlockChangeEvent& event) override;
    
    void leftClickEvent(ServerPlayer* player, unsigned short x, unsigned short y, unsigned short tick_length);
public:
    ServerPlayers(Blocks* blocks, Entities* entities, Items* items);
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
    
    ~ServerPlayers();
};

#endif
