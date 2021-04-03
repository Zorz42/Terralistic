//
//  game.cpp
//  Terralistic
//
//  Created by Jakob Zorz on ???.
//

#include "core.hpp"

#include "game.hpp"
#include "playerHandler.hpp"
#include "blockSelector.hpp"
#include "worldSaver.hpp"
#include "pauseScreen.hpp"
#include "generatingScreen.hpp"
#include "otherPlayers.hpp"
#include "itemEngineClient.hpp"
#include "blockRenderer.hpp"
#include "textScreen.hpp"

#undef main

void game::init() {
    fps_text.scale = 3;
    fps_text.x = 10;
    fps_text.y = 10;
    fps_text.orientation = gfx::top_left;
    
    blockEngine::prepare();
    
    modules = {
        new blockRenderer(this, &networking_manager),
        new itemEngineClient::module(this, &networking_manager),
        new players(this, &networking_manager),
        new pauseScreen(this),
        new playerHandler::module(this, &networking_manager),
        new blockSelector(this),
    };
    
    if(multiplayer) {
        renderTextScreen("Connecting to server");
        if(!networking_manager.startListening(world_name)) {
            gfx::returnFromScene();
            return;
        }
        playerHandler::player_inventory.clear();
    } else if(fileSystem::fileExists(fileSystem::getWorldsPath() + world_name + ".world")) {
        renderTextScreen("Loading world");
        worldSaver::loadWorld(world_name);
    }
    else {
        playerHandler::player_inventory.clear();
        gfx::switchScene(new generatingScreen(0));
    }
    blockEngine::prepareWorld();
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
        itemEngine::updateItems(gfx::getDeltaTime());
}

void game::render() {
    gfx::render(fps_text);
}

void game::stop() {
    if(multiplayer)
        networking_manager.sendPacket({packets::DISCONNECT});
    else {
        renderTextScreen("Saving world");
        worldSaver::saveWorld(world_name);
    }
    
    networking_manager.stopListening();
}
