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

void DisplayRecipe::render() {
    back_rect.render();
    result_display.render(back_rect.getTranslatedX() + INVENTORY_UI_SPACING / 2, back_rect.getTranslatedY() + INVENTORY_UI_SPACING / 2);
}

DisplayRecipe::DisplayRecipe(const Recipe* recipe, ResourcePack* resource_pack, int x, int y) : recipe(recipe), result_display(resource_pack) {
    back_rect.setX(x);
    back_rect.setY(y);
    back_rect.setWidth(32 + INVENTORY_UI_SPACING);
    back_rect.setHeight(32 + INVENTORY_UI_SPACING);
    back_rect.c = WHITE;
    back_rect.c.a = TRANSPARENCY;
}
