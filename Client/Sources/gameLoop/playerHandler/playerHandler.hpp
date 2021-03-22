//
//  movementHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef movementHandler_hpp
#define movementHandler_hpp

#include <Graphics/graphics.hpp>

namespace playerHandler {

struct module : public gfx::sceneModule {
    void onKeyUp(gfx::key key);
    void onKeyDown(gfx::key key);
};

void onKeyUp(gfx::key key);
void onKeyDown(gfx::key key);
void move();
void render();
void doPhysics();
void prepare();
void lookForItems();

inline int view_x, view_y;

inline gfx::sprite player;

void selectSlot(char slot);

inline inventory::inventoryItem *hovered = nullptr;
inline inventory::inventory player_inventory;

}

#endif /* movementHandler_hpp */
