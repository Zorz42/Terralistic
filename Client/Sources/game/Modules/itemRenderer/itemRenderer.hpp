//
//  itemRenderer.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 27/02/2021.
//

#ifndef itemRenderer_hpp
#define itemRenderer_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "game.hpp"
#include "networkingModule.hpp"


struct itemRenderer : public gfx::sceneModule<game>, networking::packetListener {
    struct uniqueRenderItem {
        gfx::image texture;
        gfx::sprite text_texture;
    };

    itemRenderer(game* scene, networking::networkingManager* manager) : gfx::sceneModule<game>(scene), networking::packetListener(manager) {}
    void init() override;
    void render() override;
    void stop() override;
    void onPacket(packets::packet packet) override;
    
    static uniqueRenderItem& getUniqueRenderItem(map::itemType type);
    
    static void loadItems();
};

#endif /* itemRenderer_hpp */
