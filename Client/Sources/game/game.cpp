//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include "game.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "generatingScreen.hpp"
#include "otherPlayers.hpp"
#include "textScreen.hpp"

#undef main

void game::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
    
    world_map = new renderMap(&networking_manager);
    world_map->createWorld(275, 75); // dimensions in chunks
    
    modules = {
        world_map,
        new players(&networking_manager, world_map),
        new pauseScreen(),
        new playerHandler(&networking_manager, &main_player, world_map, multiplayer),
    };
    
    if(multiplayer) {
        renderTextScreen("Connecting to server");
        if(!networking_manager.startListening(world_name)) {
            gfx::returnFromScene();
            return;
        }
        main_player.player_inventory.clear();
    } else if(worldSaver::worldExists(world_name)) {
        renderTextScreen("Loading world");
        worldSaver::loadWorld(world_name, *world_map);
    }
    else {
        main_player.player_inventory.clear();
        gfx::switchScene(new generatingScreen(0, world_map));
    }
}

void game::update() {
    static unsigned int count = gfx::getTicks() / 1000 - 1, fps_count = 0;
    fps_count++;
    if(gfx::getTicks() / 1000 > count) {
        count++;
        fps_text.setTexture(gfx::renderText(std::to_string(fps_count) + " fps", {0, 0, 0}));
        fps_count = 0;
    }
    
    if(!multiplayer)
        world_map->updateItems(gfx::getDeltaTime());
}

void game::render() {
    gfx::render(fps_text);
}

void game::stop() {
    if(multiplayer)
        networking_manager.sendPacket({packets::DISCONNECT});
    else {
        renderTextScreen("Saving world");
        worldSaver::saveWorld(world_name, *world_map);
    }
    
    networking_manager.stopListening();
}
