//
//  renderMap.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 08/04/2021.
//

#ifndef renderMap_hpp
#define renderMap_hpp

#ifdef __WIN32__
#include "graphics.hpp"
#else
#include <Graphics/graphics.hpp>
#endif

#include "map.hpp"
#include "networkingModule.hpp"

class renderMap : public gfx::sceneModule, packetListener, public map {
public:
    renderMap(networkingManager* manager) : packetListener(manager) {}
    void onPacket(packets::packet packet) override;
};

#endif /* renderMap_hpp */
