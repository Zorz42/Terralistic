//
//  itemRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef itemRenderer_hpp
#define itemRenderer_hpp

#include <Graphics/graphics.hpp>

namespace itemRenderer {

void render();

struct uniqueRenderItem {
    gfx::image texture;
    gfx::sprite text_texture;
};
uniqueRenderItem& getUniqueRenderItem(unsigned short id);

}

#endif /* itemRenderer_hpp */
