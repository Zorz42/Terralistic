//
//  textures.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#ifndef textures_hpp
#define textures_hpp

#include "properties.hpp"

#include "graphics.hpp"

const gfx::Image& getBlockTexture(BlockType type);
const gfx::Image& getItemTexture(ItemType type);
const gfx::Image& getItemTextTexture(ItemType type);
const gfx::Image& getLiquidTexture(LiquidType type);
const gfx::Image& getBreakingTexture();
void loadTextures();

#endif /* textures_hpp */
