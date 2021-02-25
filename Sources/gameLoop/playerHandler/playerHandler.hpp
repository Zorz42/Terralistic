//
//  movementHandler.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 01/07/2020.
//

#ifndef movementHandler_hpp
#define movementHandler_hpp

#include "objectedGraphicsLibrary.hpp"

namespace playerHandler {

void handleEvents(SDL_Event& event);
void move();
void render();
void doPhysics();
void prepare();
void lookForItems();

inline int position_x, position_y;
inline int view_x, view_y;

inline short velocity_x = 0, velocity_y = 0;
inline ogl::texture player(ogl::center);

void selectSlot(char slot);

inline inventory::inventoryItem *hovered = nullptr;
inline inventory::inventory player_inventory;

}

#endif /* movementHandler_hpp */
