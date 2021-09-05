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
    const gfx::Image& texture = resource_pack->getItemTexture();
    texture.render(4, x + INVENTORY_UI_SPACING / 2, y + INVENTORY_UI_SPACING / 2, resource_pack->getTextureRectangle(type));
    
    if(stack > 1)
        stack_texture.render(1, x + BLOCK_WIDTH * 4 - stack_texture.getTextureWidth() + INVENTORY_UI_SPACING / 2, y + BLOCK_WIDTH * 4 - stack_texture.getTextureHeight() + INVENTORY_UI_SPACING / 2);
}

void ClientInventoryItem::renderWithBack(unsigned short mouse_x, unsigned short mouse_y) const {
    gfx::Color color = GREY;
    color.a = TRANSPARENCY;
    gfx::RectShape(x, y, INVENTORY_ITEM_BACK_RECT_WIDTH, INVENTORY_ITEM_BACK_RECT_WIDTH).render(isHovered(mouse_x, mouse_y) ? GREY : color);
    render();
}

ClientInventoryItem& ClientInventoryItem::operator=(const ClientInventoryItem& item) {
    resource_pack = item.resource_pack;
    type = item.type;
    setStack(item.getStack());
    return *this;
}

bool ClientInventoryItem::isHovered(unsigned short mouse_x, unsigned short mouse_y) const {
    return mouse_x > x && mouse_y > y && mouse_x < x + INVENTORY_ITEM_BACK_RECT_WIDTH && mouse_y < y + INVENTORY_ITEM_BACK_RECT_WIDTH;
}

void DisplayRecipe::render(unsigned short mouse_x, unsigned short mouse_y) {
    result_display.renderWithBack(mouse_x, mouse_y);
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

void DisplayRecipe::renderIngredients(int x, int y, unsigned short mouse_x, unsigned short mouse_y) {
    x += SPACING / 2;
    y += SPACING / 2;
    for(ClientInventoryItem& ingredient : ingredients) {
        ingredient.x = x;
        x += INVENTORY_ITEM_BACK_RECT_WIDTH + SPACING / 2;
        ingredient.y = y;
        ingredient.renderWithBack(mouse_x, mouse_y);
    }
}
