#pragma once

#include "events.hpp"
#include "items.hpp"

#define INVENTORY_SIZE 20

class ItemStack {
public:
    ItemStack(ItemType* type, int stack) : type(type), stack(stack) {}
    ItemStack() = default;
    ItemType* type = &ItemTypes::nothing;
    int stack = 0;
};


class Recipe {
public:
    std::map<ItemType*, int> ingredients;
    ItemStack result;
};


class InventoryItemChangeEvent {
public:
    InventoryItemChangeEvent(int item_pos) : item_pos(item_pos) {}
    int item_pos;
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
    int *item_counts = nullptr;
    std::vector<const Recipe*> available_recipes;
    ItemStack inventory_arr[INVENTORY_SIZE];
    bool hasIngredientsForRecipe(const Recipe* recipe);
public:
    Inventory(Items* items, Recipes* recipes);
    
    int selected_slot = 0;
    
    const std::vector<const Recipe*>& getAvailableRecipes();
    void updateAvailableRecipes();
    
    int addItem(ItemType* id, int quantity);
    int removeItem(ItemType* id, int quantity);
    void setItem(int pos, ItemStack item);
    ItemStack getItem(int pos);
    
    ItemStack getSelectedSlot();
    void swapWithMouseItem(int pos);
    
    int increaseStack(int pos, int stack);
    int decreaseStack(int pos, int stack);
    
    void serialize(std::vector<char>& serial) const;
    char* loadFromSerial(char* iter);
    
    Inventory& operator=(Inventory& inventory);
    
    EventSender<InventoryItemChangeEvent> item_change_event;
    
    ~Inventory();
};
