//
//  itemRenderer.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 09/04/2021.
//

#include "renderMap.hpp"

renderMap::uniqueRenderItem* renderMap::unique_render_items;

renderMap::uniqueRenderItem& renderMap::getUniqueRenderItem(map::itemType id) {
    return unique_render_items[(int)id];
}

void renderMap::loadItems() {
    unique_render_items = new renderMap::uniqueRenderItem[map::unique_items.size()];
    for(int i = 0; i < map::unique_items.size(); i++) {
        unique_render_items[i].texture.setTexture(map::unique_items[i].name == "nothing" ? nullptr : gfx::loadImageFile("texturePack/items/" + map::unique_items[i].name + ".png"));
        unique_render_items[i].texture.scale = 2;
        unique_render_items[i].text_texture.setTexture(gfx::renderText(map::unique_items[i].name, {255, 255, 255}));
        unique_render_items[i].text_texture.scale = 2;
    }
}

void renderMap::renderItems() {
    for(map::item& i : items)
        gfx::render(unique_render_items[(int)i.getItemId()].texture, short(i.x / 100 - view_x + gfx::getWindowWidth() / 2), short(i.y / 100 - view_y + gfx::getWindowHeight() / 2));
}
