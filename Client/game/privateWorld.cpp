#include <thread>
#include "game.hpp"
#include "server.hpp"
#include "choiceScreen.hpp"

#define FROM_PORT 49152
#define TO_PORT 65535

class WorldStartingScreen : public gfx::Scene {
    BackgroundRect* menu_back;
    gfx::Sprite text;
    gfx::Rect loading_bar, loading_bar_back;
    void init() override;
    void render() override;
    Server* server;
    ServerState prev_server_state = ServerState::NEUTRAL;
public:
    WorldStartingScreen(BackgroundRect* menu_back, Server* server) : menu_back(menu_back), server(server) {}
};

void WorldStartingScreen::init() {
    text.scale = 3;
    text.createBlankImage(1, 1);
    text.orientation = gfx::CENTER;
    
    loading_bar_back.orientation = gfx::CENTER;
    loading_bar_back.fill_color.a = TRANSPARENCY;
    loading_bar_back.setHeight(50);
    loading_bar_back.setY(100);
    
    loading_bar.fill_color = WHITE;
    loading_bar.fill_color.a = TRANSPARENCY;
    loading_bar.setHeight(50);
}

void WorldStartingScreen::render() {
    menu_back->renderBack();
    if(server->state != prev_server_state) {
        prev_server_state = server->state;
        switch(server->state) {
            case ServerState::LOADING_WORLD:
                text.loadFromText("Loading world");
                break;
            case ServerState::GENERATING_WORLD:
                text.loadFromText("Generating world");
                break;
            case ServerState::STOPPING:
                text.loadFromText("Saving world");
                break;
            case ServerState::CRASHED:
            case ServerState::STOPPED:
            case ServerState::RUNNING:
                returnFromScene();
                break;
            default:;
        }
        menu_back->setBackWidth(text.getWidth() + 300);
    }
    
    if(server->state == ServerState::GENERATING_WORLD) {
        loading_bar_back.setWidth(text.getWidth() + 200);
        loading_bar.setX(loading_bar_back.getTranslatedX());
        loading_bar.setY(loading_bar_back.getTranslatedY());
        loading_bar.setWidth(server->getGeneratingCurrent() * (text.getWidth() + 200) / server->getGeneratingTotal());
        loading_bar_back.render();
        loading_bar.render();
    }
    text.render();
}

void startServer(Server* server, Game* game) {
    try {
        server->start();
    } catch(const std::exception& exception) {
        server->state = ServerState::CRASHED;
        game->interrupt_message = exception.what();
        game->interrupt = true;
    }
}

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, Settings* settings, bool structure_world) {
    int port = rand() % (TO_PORT - FROM_PORT) + FROM_PORT;
    Server private_server(gfx::getResourcePath(), world_name, port);
    Game game(menu_back, settings, "_", "127.0.0.1", port);
    if(structure_world)
        private_server.seed = 1000;
  
    private_server.setPrivate(true);
    std::thread server_thread = std::thread(startServer, &private_server, &game);
    
    WorldStartingScreen(menu_back, &private_server).run();
    
    game.start();
    
    private_server.stop();
    
    while(private_server.state == ServerState::RUNNING)
        gfx::sleep(1);
    
    WorldStartingScreen(menu_back, &private_server).run();
    
    if(game.interrupt) {
        ChoiceScreen choice_screen(menu_back, game.interrupt_message, {"Close"});
        choice_screen.run();
    }
    
    server_thread.join();
}
