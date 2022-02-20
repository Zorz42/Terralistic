#include <thread>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "choiceScreen.hpp"
#include "content.hpp"

class WorldJoiningScreen : public gfx::Scene {
    BackgroundRect* menu_back;
    gfx::Sprite text;
    void init() override;
    void render() override;
    Game* game;
public:
    WorldJoiningScreen(BackgroundRect* menu_back, Game* game) : menu_back(menu_back), game(game) {}
};

void WorldJoiningScreen::init() {
    text.scale = 3;
    text.createBlankImage(1, 1);
    text.orientation = gfx::CENTER;
    text.loadFromText("Joining world");
}

void WorldJoiningScreen::render() {
    if(game->isInitialized() || game->interrupt)
        returnFromScene();
    
    menu_back->setBackWidth(text.getWidth() + 300);
    menu_back->renderBack();
    text.render();
}

Game::Game(BackgroundRect* background_rect, Settings* settings, const std::string& username, const std::string& ip_address, int port) :
    username(username),
    background_rect(background_rect),
    settings(settings),
    
    networking(ip_address, port, username),
    background(&camera, &resource_pack),
    blocks(&resource_pack, &networking, &camera),
    walls(&blocks, &resource_pack, &networking, &camera),
    particles(settings, &blocks, &camera),
    liquids(&blocks, &resource_pack, &networking, &camera),
    lights(settings, &blocks, &resource_pack, &camera),
    natural_light(&networking, &blocks, &lights),
    entities(&blocks, &networking),
    items(&resource_pack, &blocks, &entities, &networking, &camera),
    players(&networking, &blocks, &liquids, &resource_pack, &entities, &particles, &camera, username),
    block_selector(&networking, &blocks, &players, &camera),
    inventory(&networking, &resource_pack, &items, &recipes),
    chat(&networking),
    debug_menu(&players, &blocks),
    content(&blocks, &walls, &liquids, &items),
    player_health(&networking, &resource_pack)
{
    registerAModule(&networking);
    registerAModule(&resource_pack);
    registerAModule(&camera);
    registerAModule(&background);
    registerAModule(&walls);
    registerAModule(&blocks);
    registerAModule(&particles);
    registerAModule(&entities);
    registerAModule(&items);
    registerAModule(&players);
    registerAModule(&liquids);
    registerAModule(&lights);
    registerAModule(&natural_light);
    registerAModule(&block_selector);
    registerAModule(&inventory);
    registerAModule(&chat);
    registerAModule(&player_health);
    registerAModule(&debug_menu);
    
    content.loadContent(&blocks, &walls, &liquids, &items, &recipes, gfx::getResourcePath() + "resourcePack/");
}

void Game::initialize() {
    try {
        if(interrupt)
            throw Exception(interrupt_message);
        Scene::initialize();
        if(interrupt)
            throw Exception(interrupt_message);
    } catch (const std::exception& exception) {
        interrupt_message = exception.what();
        interrupt = true;
    }
}

void Game::start() {
    try {
        if(interrupt)
            throw Exception(interrupt_message);
        
        std::thread init_thread(&Game::initialize, this);
        WorldJoiningScreen(background_rect, this).run();
        init_thread.join();
        
        if(interrupt)
            throw Exception(interrupt_message);
        
        for(int i = 0; i < getModules().size(); i++)
            if(getModules()[i] != this)
                ((ClientModule*)getModules()[i])->loadTextures();
        std::thread parallel_update_thread(&Game::parallelUpdateLoop, this);
        run();
        parallel_update_thread.join();
        if(interrupt)
            throw Exception(interrupt_message);
    } catch (const std::exception& exception) {
        ChoiceScreen choice_screen(background_rect, exception.what(), {"Close"});
        switchToScene(choice_screen);
    }
}

void Game::parallelUpdateLoop() {
    try {
        gfx::Timer timer;
        while(isRunning()) {
            float frame_length = timer.getTimeElapsed();
            timer.reset();
            for(int i = 0; i < getModules().size(); i++)
                if(getModules()[i] != this)
                    ((ClientModule*)getModules()[i])->updateParallel(frame_length);
            if(timer.getTimeElapsed() < 16)
                gfx::sleep(16 - timer.getTimeElapsed());
        }
    } catch (const std::exception& exception) {
        interrupt_message = exception.what();
        interrupt = true;
    }
}

void Game::init() {
    for(int i = 0; i < getModules().size(); i++)
        if(getModules()[i] != this)
            ((ClientModule*)getModules()[i])->postInit();
}

void Game::render() {
    if(interrupt)
        throw Exception(interrupt_message);
}

void Game::renderBack() {
    cycleModules();
}

bool Game::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        PauseScreen pause_screen(this, settings);
        switchToScene(pause_screen);
        if(pause_screen.hasExitedToMenu())
            returnFromScene();
        return true;
    }
    return false;
}
