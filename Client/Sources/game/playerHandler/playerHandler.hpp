//
//  movementHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef movementHandler_hpp
#define movementHandler_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"
#include "networkingModule.hpp"

namespace playerHandler {

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void onKeyUp(gfx::key key);
    void onKeyDown(gfx::key key);
    void init();
    void update();
    void render();
    void selectSlot(char slot);
    void onPacket(packets::packet packet);
    
private:
    bool key_up = false, jump = false;
    gfx::rect inventory_slots[20],
    select_rect{0, 5, 2 * (BLOCK_WIDTH + 10), 2 * (BLOCK_WIDTH + 10), {50, 50, 50}, gfx::top},
    under_text_rect{0, 0, 0, 0, {0, 0, 0}};
    gfx::image stack_textures[20], mouse_stack_texture;
    int position_x, position_y;
    short velocity_x = 0, velocity_y = 0;
    
    bool isPlayerColliding();
    bool touchingGround();
    void renderItem(inventory::inventoryItem* item, int x, int y, int i);
    void updateStackTexture(int i);
};

inline int view_x, view_y;
inline gfx::sprite player;

inline inventory::inventoryItem *hovered = nullptr;
inline inventory::inventory player_inventory;

}

#endif /* movementHandler_hpp */
