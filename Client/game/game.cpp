#include <thread>
#include <cassert>
#include <filesystem>
#include <fstream>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "platform_folders.h"
#include "choiceScreen.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"
#include "server.hpp"
#include "blockSelector.hpp"
#include "compress.hpp"

#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;

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
            case ServerState::RUNNING:
            case ServerState::STOPPED:
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

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, bool structure_world) {
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
    Server private_server(gfx::getResourcePath(), world_name, port);
    if(structure_world)
        private_server.seed = 1000;
  
    private_server.setPrivate(true);
    server_thread = std::thread(&Server::start, &private_server);
    
    WorldStartingScreen(menu_back, &private_server).run();

    Game(menu_back, "_", "127.0.0.1", port).run();
    
    private_server.stop();
    
    while(private_server.state == ServerState::RUNNING)
        gfx::sleep(1);
    
    WorldStartingScreen(menu_back, &private_server).run();
    
    server_thread.join();
}

Game::Game(BackgroundRect* background_rect, const std::string& username, const std::string& ip_address, unsigned short port) :
    username(username),
    background_rect(background_rect),
    
    networking(ip_address, port, username),
    blocks(&resource_pack, &networking, &lights),
    particles(&blocks),
    liquids(&blocks, &resource_pack, &networking),
    lights(&blocks, &resource_pack),
    natural_light(&networking, &blocks, &lights),
    entities(&blocks, &networking),
    items(&resource_pack, &blocks, &entities, &networking),
    players(&networking, &blocks, &liquids, &resource_pack, &entities, &particles, username),
    block_selector(&networking, &blocks, &players),
    inventory(&networking, &resource_pack),
    minimap(&blocks, &liquids, &lights, &natural_light),
    chat(&networking),
    debug_menu(&players, &blocks)
{
    registerAModule(&networking);
    registerAModule(&resource_pack);
    registerAModule(&blocks);
    registerAModule(&particles);
    registerAModule(&players);
    registerAModule(&liquids);
    registerAModule(&lights);
    registerAModule(&natural_light);
    registerAModule(&entities);
    registerAModule(&items);
    registerAModule(&block_selector);
    registerAModule(&inventory);
    registerAModule(&minimap);
    registerAModule(&chat);
#ifdef DEVELOPER_MODE
    registerAModule(&debug_menu);
#endif
}

void Game::init() {
    for(ClientModule* module : modules)
        module->postInit();
}

void Game::render() {
    float scale = (float)gfx::getWindowHeight() / resource_pack.getBackground().getTextureHeight();
    int position_x = -(blocks.view_x / 5) % int(resource_pack.getBackground().getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (resource_pack.getBackground().getTextureWidth() * scale) + 2; i++)
        resource_pack.getBackground().render(scale, position_x + i * resource_pack.getBackground().getTextureWidth() * scale, 0);
}

void Game::renderBack() {
    cycleModules();
}

bool Game::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        PauseScreen pause_screen(this);
        switchToScene(pause_screen);
        if(pause_screen.hasExitedToMenu())
            returnFromScene();
        return true;
    }
    return false;
}

bool Game::isHandshakeDone() {
    return handshake_done;
}

void Game::registerAModule(ClientModule* module) {
    Scene::registerAModule(module);
    module->game_error_event.addListener(this);
    modules.push_back(module);
}

void Game::onEvent(GameErrorEvent& event) {
    ChoiceScreen choice_screen(background_rect, event.message, {"Close"});
    switchToScene(choice_screen);
    returnFromScene();
}

void Game::stop() {
    for(ClientModule* module : modules)
        module->game_error_event.removeListener(this);
}
