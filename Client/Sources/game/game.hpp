//
//  game.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 06/07/2020.
//

#ifndef game_hpp
#define game_hpp

#ifdef __WIN32__
#include <utility>
#include "graphics.hpp"
#else

#ifdef DEVELOPER_MODE
#include <Graphics_Debug/graphics.hpp>
#else
#include <Graphics/graphics.hpp>
#endif


#endif

#include "playerHandler.hpp"
#include "clientNetworking.hpp"

void startPrivateWorld(const std::string& world_name);

class game : public gfx::scene {
public:
    const std::string ip_address;
    gfx::sprite fps_text;
    networkingManager networking_manager;
    map *world_map;
    mainPlayer main_player;
    
    game(std::string ip_address) : ip_address(std::move(ip_address)) {}
    void init() override;
    void stop() override;
    void render() override;
    void update() override;
};

#endif /* game_hpp */
