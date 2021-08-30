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

void ClientInventoryItem::render() const {
    const gfx::Image& texture = resource_pack->getItemTexture(type);
    texture.render(4, x + INVENTORY_UI_SPACING / 2, y + INVENTORY_UI_SPACING / 2);
    
    if(stack > 1)
        stack_texture.render(1, x + BLOCK_WIDTH * 4 - stack_texture.getTextureWidth() + INVENTORY_UI_SPACING / 2, y + BLOCK_WIDTH * 4 - stack_texture.getTextureHeight() + INVENTORY_UI_SPACING / 2);
}

void ClientInventoryItem::renderWithBack() const {
    gfx::Color color = GREY;
    color.a = TRANSPARENCY;
    gfx::RectShape(x, y, INVENTORY_ITEM_BACK_RECT_WIDTH, INVENTORY_ITEM_BACK_RECT_WIDTH).render(isHovered() ? GREY : color);
    render();
}

ClientInventoryItem& ClientInventoryItem::operator=(const ClientInventoryItem& item) {
    resource_pack = item.resource_pack;
    type = item.type;
    setStack(item.getStack());
    return *this;
}

bool ClientInventoryItem::isHovered() const {
    return gfx::getMouseX() > x && gfx::getMouseY() > y && gfx::getMouseX() < x + INVENTORY_ITEM_BACK_RECT_WIDTH && gfx::getMouseY() < y + INVENTORY_ITEM_BACK_RECT_WIDTH;
}

void DisplayRecipe::render() {
    result_display.renderWithBack();
}

DisplayRecipe::DisplayRecipe(const Recipe* recipe, ResourcePack* resource_pack, int x, int y) : recipe(recipe), result_display(resource_pack) {
    result_display.x = x;
    result_display.y = y;
    result_display.type = recipe->result.type;
    result_display.setStack(recipe->result.stack);
    for(const ItemStack& ingredient : recipe->ingredients) {
        ingredients.emplace_back(resource_pack);
        ingredients.back().type = ingredient.type;
        ingredients.back().setStack(ingredient.stack);
    }
}

void DisplayRecipe::renderIngredients(int x, int y) {
    x += SPACING / 2;
    y += SPACING / 2;
    for(ClientInventoryItem& ingredient : ingredients) {
        ingredient.x = x;
        x += INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING / 2;
        ingredient.y = y;
        ingredient.renderWithBack();
    }
}
