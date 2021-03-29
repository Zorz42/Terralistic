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
};

inline int view_x, view_y;
inline gfx::sprite player;

inline inventory::inventoryItem *hovered = nullptr;
inline inventory::inventory player_inventory;

}

#endif /* movementHandler_hpp */
