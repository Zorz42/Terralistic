#include "clientInventory.hpp"

const ItemInfo& ClientInventoryItem::getUniqueItem() const {
    return ::getItemInfo(type);
}

void ClientInventoryItem::setStack(unsigned short stack_) {
    if(stack != stack_) {
        stack = stack_;
        stack_texture.renderText(std::to_string(stack));
    }
}

unsigned short ClientInventoryItem::getStack() const {
    return stack;
}

unsigned short ClientInventoryItem::increaseStack(unsigned short stack_) {
    int stack_to_be = stack + stack_, result;
    if(stack_to_be > getUniqueItem().stack_size)
        stack_to_be = getUniqueItem().stack_size;
    result = stack_to_be - stack;
    setStack((unsigned short)stack_to_be);
    return (unsigned short)result;
}

void ClientInventoryItem::render(int x, int y) const {
    const gfx::Image& texture = resource_pack->getItemTexture(type);
    texture.render(4, x, y);
    
    if(stack > 1)
        stack_texture.render(1, x + BLOCK_WIDTH * 2 - stack_texture.getTextureWidth(), y + BLOCK_WIDTH * 2 - stack_texture.getTextureHeight());
}

ClientInventoryItem& ClientInventoryItem::operator=(const ClientInventoryItem& item) {
    resource_pack = item.resource_pack;
    type = item.type;
    setStack(item.getStack());
    return *this;
}

void DisplayRecipe::updateResult() {
    result_display.type = recipe->result.type;
    result_display.setStack(recipe->result.stack);
}

void DisplayRecipe::render(int x, int y) const {
    result_display.render(x, y);
}

void DisplayRecipe::setResourcePack(ResourcePack *resource_pack) {
    result_display = ClientInventoryItem(resource_pack);
}
