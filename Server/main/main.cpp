//
//  main.cpp
//  Terralistic-server
//
//  Created by Jakob Zorz on 11/01/2021.
//

#define FILENAME main
#define NAMESPACE main
#include "essential.hpp"

#include "print.hpp"
#include "worldSaver.hpp"
#include "blockEngine.hpp"
#include "main.hpp"

volatile sig_atomic_t stop;

#define ms_time std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count
#define PORT 33770

void inthand(int signum) {
    stop = 1;
}

int main() {
    print::info("Starting server");
    
    print::info("Initializing modules");
    init::initModules();
    
    if(fileSystem::dirExists("world")) {
        print::info("Loading world...");
        worldSaver::loadWorld();
    }
    else {
        print::info("Generating world...");
        worldSaver::createWorld();
    }
    
    print::info("Post initializing modules...");
    
    signal(SIGINT, inthand);
    networking::spawnListener();
    
    print::info("Server has started!");
    long long a, b = ms_time();
    while(!stop) {
        a = ms_time();
        main_::frame_length = a - b;
        if(main_::frame_length < 50)
            std::this_thread::sleep_for(std::chrono::milliseconds(50 - main_::frame_length));
        b = a;
        
        itemEngine::updateItems(main_::frame_length);
        playerHandler::lookForItems();
        blockEngine::updatePlayersBreaking();
    }
    
    std::cout << std::endl;
    
    print::info("Stopping server");
    
    print::info("Saving world...");
    worldSaver::saveWorld();

    return 0;
}
