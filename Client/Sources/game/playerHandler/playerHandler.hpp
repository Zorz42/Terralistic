//
//  movementHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef movementHandler_hpp
#define movementHandler_hpp

#include <Graphics/graphics.hpp>
#include "game.hpp"

namespace playerHandler {

struct module : public gfx::sceneModule<game::scene> {
    using gfx::sceneModule<game::scene>::sceneModule;
    void onKeyUp(gfx::key key);
    void onKeyDown(gfx::key key);
    void init();
    void update();
    void render();
    void selectSlot(char slot);
};

inline int view_x, view_y;
inline gfx::sprite player;

inline inventory::inventoryItem *hovered = nullptr;
inline inventory::inventory player_inventory;

}

#endif /* movementHandler_hpp */
