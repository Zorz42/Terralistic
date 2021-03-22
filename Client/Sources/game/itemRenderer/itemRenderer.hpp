//
//  itemRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef itemRenderer_hpp
#define itemRenderer_hpp

#include <Graphics/graphics.hpp>
#include "game.hpp"

namespace itemRenderer {

struct uniqueRenderItem {
    gfx::image texture;
    gfx::sprite text_texture;
};
uniqueRenderItem& getUniqueRenderItem(unsigned short id);

struct module : public gfx::sceneModule<game::scene> {
    using gfx::sceneModule<game::scene>::sceneModule;
    void render();
};

}

#endif /* itemRenderer_hpp */
