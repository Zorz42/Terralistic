//
//  main.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#include <thread>
#include <iostream>

#include "print.hpp"
#include "worldSaver.hpp"
#include "main.hpp"
#include "networkingModule.hpp"
#include "playerHandler.hpp"
#include "fileSystem.hpp"
#include "terrainGenerator.hpp"
#include "clickEvents.hpp"

static bool running = true;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    running = false;
}

int main() {
    print::info("Starting server");
    
    print::info("Initializing modules");
    
    map::initItems();
    map::initBlocks();
    
    clickEvents::init();
    
    networkingManager networking_manager;
    
    map world_map(&networking_manager);
    world_map.createWorld(275, 75);
    
    playerHandler player_handler(&networking_manager, &world_map);
    
    if(worldSaver::worldExists("world")) {
        print::info("Loading world...");
        worldSaver::loadWorld("world", world_map);
    }
    else {
        print::info("Generating world...");
        terrainGenerator::generateTerrainDaemon(0, &world_map);
        worldSaver::saveWorld("world", world_map);
    }
    
    print::info("Post initializing modules...");
    
    world_map.setNaturalLight();
    
    for(unsigned short x = 0; x < world_map.getWorldWidth(); x++)
        for(unsigned short y = 0; y < world_map.getWorldHeight(); y++)
            world_map.getBlock(x, y).update();
    
    signal(SIGINT, inthand);
    networking_manager.startListening();
    
    print::info("Server has started!");
    long long a, b = ms_time();
    while(running) {
        a = ms_time();
        main_::frame_length = a - b;
        if(main_::frame_length < 50)
            std::this_thread::sleep_for(std::chrono::milliseconds(50 - main_::frame_length));
        b = a;
        
        world_map.updateItems(main_::frame_length);
        lookForItems(world_map);
        world_map.updatePlayersBreaking();
        
        for(player& player : players) {
            for(unsigned short x = player.x / 16 - player.sight_width / 2 - 20; x < player.x / 16 + player.sight_width / 2 + 20; x++)
                for(unsigned short y = player.y / 16 - player.sight_height / 2 - 20; y < player.y / 16 + player.sight_height / 2 + 20; y++)
                    if(world_map.getBlock(x, y).hasScheduledLightUpdate())
                        world_map.getBlock(x, y).lightUpdate();
        }
    }
    
    std::cout << std::endl;
    
    print::info("Stopping server");
    
    print::info("Saving world...");
    worldSaver::saveWorld("world", world_map);

    return 0;
}
