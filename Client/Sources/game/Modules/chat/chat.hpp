//
//  chat.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 13/06/2021.
//

#ifndef chat_hpp
#define chat_hpp

#ifdef _WIN32
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif

#endif

#include "clientNetworking.hpp"

class chat : public gfx::sceneModule, packetListener {
    gfx::textInput chat_box;
    networkingManager* manager;
public:
    chat(networkingManager* manager) : manager(manager), packetListener(manager) {}
    
    void init() override;
    void update() override;
    void render() override;
    void onKeyDown(gfx::key key) override;
    
    void onPacket(packets::packet packet) override;
};

#endif /* chat_hpp */
