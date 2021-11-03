#pragma once

#include "properties.hpp"
#include "events.hpp"

#define INVENTORY_SIZE 20

struct ItemStack {
    ItemStack(ItemTypeOld type, unsigned short stack) : type(type), stack(stack) {}
    ItemStack() = default;
    ItemTypeOld type = ItemTypeOld::NOTHING;
    unsigned short stack = 0;
};


class InventoryItemChangeEvent {
public:
    InventoryItemChangeEvent(char item_pos) : item_pos(item_pos) {}
    char item_pos;
};


class Inventory {
    ItemStack mouse_item;
    unsigned int item_counts[(int)ItemTypeOld::NUM_ITEMS];
    std::vector<const RecipeOld*> available_recipes;
    ItemStack inventory_arr[INVENTORY_SIZE];
    bool hasIngredientsForRecipe(const RecipeOld& recipe);
public:
    Inventory();
    
    unsigned char selected_slot = 0;
    
    const std::vector<const RecipeOld*>& getAvailableRecipes();
    void updateAvailableRecipes();
    
    char addItem(ItemTypeOld id, int quantity);
    char removeItem(ItemTypeOld id, int quantity);
    void setItem(char pos, ItemStack item);
    ItemStack getItem(char pos);
    
    ItemStack getSelectedSlot();
    void swapWithMouseItem(char pos);
    
    unsigned short increaseStack(char pos, unsigned short stack);
    unsigned short decreaseStack(char pos, unsigned short stack);
    
    void serialize(std::vector<char>& serial) const;
    char* loadFromSerial(char* iter);
    
    EventSender<InventoryItemChangeEvent> item_change_event;
};
