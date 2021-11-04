#pragma once

#include "properties.hpp"
#include "events.hpp"
#include "items.hpp"

#define INVENTORY_SIZE 20

class ItemStack {
public:
    ItemStack(ItemType* type, unsigned short stack) : type(type), stack(stack) {}
    ItemStack() = default;
    ItemType* type = &ItemTypes::nothing;
    unsigned short stack = 0;
};


class Recipe {
public:
    std::map<ItemType*, unsigned short> ingredients;
    ItemStack result;
};


class InventoryItemChangeEvent {
public:
    InventoryItemChangeEvent(char item_pos) : item_pos(item_pos) {}
    char item_pos;
};


class Recipes {
    std::vector<Recipe*> recipes;
public:
    void registerARecipe(Recipe* recipe);
    const std::vector<Recipe*>& getAllRecipes();
};


class Inventory {
    Items* items;
    Recipes* recipes;
    
    ItemStack mouse_item;
    unsigned int item_counts[(int)ItemTypeOld::NUM_ITEMS];
    std::vector<const Recipe*> available_recipes;
    ItemStack inventory_arr[INVENTORY_SIZE];
    bool hasIngredientsForRecipe(const Recipe* recipe);
public:
    Inventory(Items* items, Recipes* recipes);
    
    unsigned char selected_slot = 0;
    
    const std::vector<const Recipe*>& getAvailableRecipes();
    void updateAvailableRecipes();
    
    char addItem(ItemType* id, int quantity);
    char removeItem(ItemType* id, int quantity);
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
