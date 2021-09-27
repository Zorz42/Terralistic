#include "serverPlayers.hpp"
#include "properties.hpp"

void InventoryItem::setTypeDirectly(ItemType type_) {
    inventory->item_counts[(int)type] -= stack;
    inventory->item_counts[(int)type_] += stack;
    type = type_;
}

void InventoryItem::setType(ItemType type_) {
    if(type != type_) {
        ServerInventoryItemTypeChangeEvent event(*this, type_);
        inventory->getPlayers()->inventory_item_type_change_event.call(event);
        
        if(event.cancelled)
            return;
        
        setTypeDirectly(type_);
        inventory->updateAvailableRecipes();
    }
}

const ItemInfo& InventoryItem::getUniqueItem() const {
    return ::getItemInfo(type);
}

void InventoryItem::setStackDirectly(unsigned short stack_) {
    inventory->item_counts[(int)type] += (int)stack_ - stack;
    stack = stack_;
}

void InventoryItem::setStack(unsigned short stack_) {
    if(stack != stack_) {
        ServerInventoryItemStackChangeEvent event(*this, stack_);
        inventory->getPlayers()->inventory_item_stack_change_event.call(event);
        
        if(event.cancelled)
            return;
        
        setStackDirectly(stack_);
        inventory->updateAvailableRecipes();
        if(!stack)
            setType(ItemType::NOTHING);
    }
}

unsigned short InventoryItem::getStack() const {
    return stack;
}

unsigned short InventoryItem::increaseStack(unsigned short stack_) {
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

unsigned short InventoryItem::decreaseStack(unsigned short stack_) {
    if(stack_ > stack) {
        unsigned short prev_stack = stack;
        setStack(0);
        return prev_stack;
    } else {
        setStack(stack - stack_);
        return stack_;
    }
}

short InventoryItem::getPosInInventory() {
    if(this == &inventory->mouse_item)
        return -1;
    else
        return this - &inventory->inventory_arr[0];
}

ServerInventory::ServerInventory() {
    for(InventoryItem& i : inventory_arr)
        i = InventoryItem(this);
    
    for(unsigned int& i : item_counts)
        i = 0;
}

char ServerInventory::addItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getType() == id) {
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getType() == ItemType::NOTHING) {
            inventory_arr[i].setType(id);
            quantity -= inventory_arr[i].increaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    return -1;
}

char ServerInventory::removeItem(ItemType id, int quantity) {
    for(int i = 0; i < INVENTORY_SIZE; i++)
        if(inventory_arr[i].getType() == id) {
            quantity -= inventory_arr[i].decreaseStack((unsigned short)quantity);
            if(!quantity)
                return (char)i;
        }
    if(mouse_item.getType() == id) {
        quantity -= mouse_item.decreaseStack((unsigned short)quantity);
        if(!quantity)
            return (char)-1;
    }
    return -1;
}

InventoryItem* ServerInventory::getSelectedSlot() {
    return &inventory_arr[(int)selected_slot];
}

void ServerInventory::swapWithMouseItem(InventoryItem* item) {
    InventoryItem temp = mouse_item;
    mouse_item = *item;
    *item = temp;
}

char* InventoryItem::loadFromSerial(char* iter) {
    setTypeDirectly((ItemType)*iter++);
    setStackDirectly(*(short*)iter);
    iter += 2;
    return iter;
}

void InventoryItem::serialize(std::vector<char>& serial) const {
    serial.push_back((char)getType());
    serial.insert(serial.end(), {0, 0});
    *(short*)&serial[serial.size() - 2] = getStack();
}

bool ServerInventory::hasIngredientsForRecipe(const Recipe& recipe) {
    return std::all_of(recipe.ingredients.begin(), recipe.ingredients.end(), [this](const ItemStack& ingredient){ return item_counts[(int)ingredient.type] >= ingredient.stack; });
}

const std::vector<const Recipe*>& ServerInventory::getAvailableRecipes() {
    return available_recipes;
}

void ServerInventory::updateAvailableRecipes() {
    std::vector<const Recipe*> new_available_recipes;
    for(const Recipe& recipe : getRecipes())
        if(hasIngredientsForRecipe(recipe))
            new_available_recipes.emplace_back(&recipe);
    
    if(available_recipes != new_available_recipes) {
        available_recipes = new_available_recipes;
        RecipeAvailabilityChangeEvent event(this);
        getPlayers()->recipe_availability_change_event.call(event);
    }
}

