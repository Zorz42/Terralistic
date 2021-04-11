//
//  inventoryRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 03/04/2021.
//

#ifndef inventoryRenderer_hpp
#define inventoryRenderer_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"
#include "networkingModule.hpp"

struct inventoryRenderer : gfx::sceneModule, packetListener {
    networkingManager* networking_manager;
    inventory* player_inventory;
    bool multiplayer;
    inventoryRenderer(inventory* player_inventory, networkingManager* manager, bool multiplayer) : packetListener(manager), networking_manager(manager), player_inventory(player_inventory), multiplayer(multiplayer) {}
    
    void init() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
    void onPacket(packets::packet packet) override;
    
private:
    void renderItem(inventoryItem* item, int x, int y, int i);
    void selectSlot(char slot);
    void updateStackTexture(int i);
    
    gfx::rect inventory_slots[20],
    select_rect{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::top},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::image stack_textures[20], mouse_stack_texture;
};

#endif /* inventoryRenderer_hpp */
