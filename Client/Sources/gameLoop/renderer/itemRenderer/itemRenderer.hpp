//
//  itemRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef itemRenderer_hpp
#define itemRenderer_hpp

namespace itemRenderer {

void render();

struct uniqueRenderItem {
    SDL_Texture* texture;
    ogl::texture text_texture{ogl::top_left};
};
uniqueRenderItem& getUniqueRenderItem(unsigned short id);

}

#endif /* itemRenderer_hpp */
