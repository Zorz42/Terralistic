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

struct module : public gfx::sceneModule<game>, packetListener {
    module(game* scene, networkingManager* manager) : gfx::sceneModule<game>(scene), packetListener(manager) {}
    void onKeyUp(gfx::key key) override;
    void onKeyDown(gfx::key key) override;
    void init() override;
    void update() override;
    void render() override;
    void selectSlot(char slot);
    void onPacket(packets::packet packet) override;
    
private:
    bool key_up = false, jump = false, key_left = false, key_right = false;
    int position_x, position_y;
    short velocity_x = 0, velocity_y = 0;
    
    bool isPlayerColliding();
    bool touchingGround();
};

inline int view_x, view_y;
inline gfx::sprite player;

inline inventory::inventoryItem *hovered = nullptr;

}

#endif /* movementHandler_hpp */
