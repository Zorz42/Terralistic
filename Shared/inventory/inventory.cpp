#include <algorithm>
#include "inventory.hpp"

void Inventory::setItem(char pos, ItemStack item) {
    item_counts[(int)getItem(pos).type->id] -= getItem(pos).stack;
    item_counts[(int)item.type->id] += item.stack;
    ItemStack& item_stack = pos == -1 ? mouse_item : inventory_arr[pos];
    item_stack = item;
    updateAvailableRecipes();
    
    InventoryItemChangeEvent event(pos);
    item_change_event.call(event);
}

unsigned short Inventory::increaseStack(char pos, unsigned short stack) {
    int stack_to_be = getItem(pos).stack + stack, result;
    if(stack_to_be > getItem(pos).type->stack_size)
        stack_to_be = getItem(pos).type->stack_size;
    result = stack_to_be - getItem(pos).stack;
    setItem(pos, ItemStack(getItem(pos).type, stack_to_be));
    return (unsigned short)result;
}

unsigned short Inventory::decreaseStack(char pos, unsigned short stack) {
    if(stack >= getItem(pos).stack) {
        unsigned short prev_stack = getItem(pos).stack;
        setItem(pos, ItemStack(&ItemTypes::nothing, 0));
        return prev_stack;
    } else {
        setItem(pos, ItemStack(getItem(pos).type, getItem(pos).stack - stack));
        return stack;
    }
}

char* Inventory::loadFromSerial(char *iter) {
    for(ItemStack& item : inventory_arr) {
        item.type = items->getItemTypeById(*iter++);
        item.stack = 0;
        for(int i = 0; i < sizeof(short); i++)
            item.stack += (unsigned int)*iter++ << i * 8;
        item_counts[(int)item.type->id] += item.stack;
    }
    updateAvailableRecipes();
    return iter;
}

Inventory::Inventory(Items* items, Recipes* recipes) : items(items), recipes(recipes) {
    item_counts = new unsigned int[items->getNumItemTypes()];
    for(int i = 0; i < items->getNumItemTypes(); i++)
        item_counts[i] = 0;
}

char Inventory::addItem(ItemType* id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(getItem(i).type == id) {
            quantity -= increaseStack(i, (unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(getItem(i).type == &ItemTypes::nothing) {
            setItem(i, ItemStack(id, getItem(i).stack));
            quantity -= increaseStack(i, (unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

char Inventory::removeItem(ItemType* id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].type == id) {
            quantity -= decreaseStack(i, (unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    if(mouse_item.type == id) {
        quantity -= decreaseStack(-1, (unsigned short)quantity);
        if(!quantity)
            return (char)-1;
    }
    return -1;
}

ItemStack Inventory::getItem(char pos) {
    if(pos == -1)
        return mouse_item;
    return inventory_arr[pos];
}

ItemStack Inventory::getSelectedSlot() {
    return getItem(selected_slot);
}

void Inventory::swapWithMouseItem(char pos) {
    ItemStack temp = mouse_item;
    mouse_item = inventory_arr[pos];
    inventory_arr[pos] = temp;
}

void Inventory::serialize(std::vector<char> &serial) const {
    for(ItemStack item : inventory_arr) {
        serial.push_back((char)item.type->id);
        serial.insert(serial.end(), {0, 0});
        for(int i = 0; i < sizeof(short); i++)
            serial[serial.size() - sizeof(short) + i] = (short)(unsigned char)item.stack >> i * 8;
    }
}

bool Inventory::hasIngredientsForRecipe(const Recipe* recipe) {
    return std::all_of(recipe->ingredients.begin(), recipe->ingredients.end(), [this](auto ingredient){ return item_counts[(int)ingredient.first->id] >= ingredient.second; });
}

const std::vector<const Recipe*>& Inventory::getAvailableRecipes() {
    return available_recipes;
}

void Inventory::updateAvailableRecipes() {
    available_recipes.clear();
    for(const Recipe* recipe : recipes->getAllRecipes())
        if(hasIngredientsForRecipe(recipe))
            available_recipes.emplace_back(recipe);
}

void Recipes::registerARecipe(Recipe* recipe) {
    recipes.push_back(recipe);
}

const std::vector<Recipe*>& Recipes::getAllRecipes() {
    return recipes;
}

Inventory& Inventory::operator=(Inventory& inventory) {
    items = inventory.items;
    recipes = inventory.recipes;
    
    selected_slot = inventory.selected_slot;
    for(int i = 0; i < INVENTORY_SIZE; i++)
        inventory_arr[i] = inventory.inventory_arr[i];
    mouse_item = inventory.mouse_item;
    item_counts = new unsigned int[items->getNumItemTypes()];
    for(int i = 0; i < items->getNumItemTypes(); i++)
        item_counts[i] = inventory.item_counts[i];
    available_recipes = inventory.available_recipes;
    
    return *this;
}

Inventory::~Inventory() {
    delete[] item_counts;
}
