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
#include "networkingModule.hpp"

namespace itemEngineClient {

struct uniqueRenderItem {
    gfx::image texture;
    gfx::sprite text_texture;
};
uniqueRenderItem& getUniqueRenderItem(unsigned short id);

struct module : public gfx::sceneModule<game::scene>, networking::packetListener {
    using gfx::sceneModule<game::scene>::sceneModule;
    void init();
    void render();
    void stop();
    void onPacket(packets::packet packet);
};

}

#endif /* itemRenderer_hpp */
