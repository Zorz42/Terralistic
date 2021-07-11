//
//  textures.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 28/06/2021.
//

#ifndef textures_hpp
#define textures_hpp

#include "properties.hpp"

#ifdef __APPLE__

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#else
#include "graphics.hpp"
#endif

const gfx::image& getBlockTexture(BlockType type);
const gfx::image& getItemTexture(ItemType type);
const gfx::image& getItemTextTexture(ItemType type);
const gfx::image& getLiquidTexture(LiquidType type);
const gfx::image& getBreakingTexture();
void loadTextures();

#endif /* textures_hpp */
