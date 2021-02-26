//
//  renderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 20/02/2021.
//

#ifndef renderer_hpp
#define renderer_hpp

#include "objectedGraphicsLibrary.hpp"

namespace renderer {

void renderItems();
struct uniqueRenderItem {
    SDL_Texture* texture;
    ogl::texture text_texture{ogl::top_left};
};
uniqueRenderItem& getUniqueRenderItem(unsigned short id);

}

#endif /* renderer_hpp */
