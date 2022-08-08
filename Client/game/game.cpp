#include <thread>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "choiceScreen.hpp"
#include "content.hpp"
#include "resourcePath.hpp"

class WorldJoiningScreen : public gfx::Scene {
    BackgroundRect* menu_back;
    gfx::Sprite text;
    void init() override;
    void render() override;
    Game* game;
public:
    WorldJoiningScreen(BackgroundRect* menu_back, Game* game) : gfx::Scene("WorldJoiningScreen"), menu_back(menu_back), game(game) {}
};

void WorldJoiningScreen::init() {
    text.setScale(3);
    text.orientation = gfx::CENTER;
    text.loadFromSurface(gfx::textToSurface("Joining world"));
}

void WorldJoiningScreen::render() {
    if(game->isInitialized() || game->interrupt)
        returnFromScene();
    
    menu_back->setBackWidth(text.getWidth() + 300);
    menu_back->renderBack();
    text.render();
}

Game::Game(BackgroundRect* background_rect, Settings* settings, const std::string& username, const std::string& ip_address, int port) :
    gfx::Scene("Game"),
    username(username),
    background_rect(background_rect),
    settings(settings),
    
    debug_menu(),
    networking(&debug_menu, ip_address, port, username),
    camera(&debug_menu),
    resource_pack(),
    background(&camera, &resource_pack),
    blocks(&debug_menu, &resource_pack, &networking, &camera),
    walls(&debug_menu, &blocks, &resource_pack, &networking, &camera),
    particles(settings, &blocks, &camera),
    liquids(&debug_menu, &blocks, &resource_pack, &networking, &camera),
    lights(&debug_menu, settings, &blocks, &resource_pack, &camera),
    natural_light(&networking, &blocks, &lights),
    entities(&blocks, &networking),
    items(&resource_pack, &blocks, &entities, &networking, &camera),
    players(&networking, &blocks, &liquids, &resource_pack, &entities, &particles, &camera, username),
    block_selector(&networking, &blocks, &players, &camera),
    inventory(&networking, &resource_pack, &items, &recipes, &players, &blocks),
    chat(&networking, &players),
    player_health(&networking, &resource_pack, &players),
    respawn_screen(&networking, &players),
    content(&blocks, &walls, &liquids, &items)
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
    registerAModule(&respawn_screen);
    registerAModule(&debug_menu);
    
    content.loadContent(&blocks, &walls, &liquids, &items, &recipes, resource_path + "resourcePack/");
    
    debug_menu.registerDebugLine(&fps_debug_line);
    debug_menu.registerDebugLine(&frame_length_line);
}

void Game::initializeGame() {
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

void Game::preInit() {
    std::thread init_thread(&Game::initializeGame, this);
    WorldJoiningScreen(background_rect, this).run();
    init_thread.join();
    
    if(interrupt)
        throw Exception(interrupt_message);
    
    for(auto i : getModules())
        if(i != this)
            ((ClientModule*)i)->loadTextures();
}

void Game::start() {
    try {
        if(interrupt)
            throw Exception(interrupt_message);
        
        run();
        
        if(interrupt)
            throw Exception(interrupt_message);
    } catch(const std::exception& exception) {
        ChoiceScreen(background_rect, exception.what(), {"Close"}).run();
    }
}

#define PARALLEL_UPDATES_PER_SECOND 10

void Game::parallelUpdateLoop() {
    try {
#ifdef __APPLE__
                pthread_setname_np("Client Parallel");
#endif

#ifdef __linux__
                pthread_setname_np(pthread_self(), "Client Parallel");
#endif
        
        gfx::Timer timer;
        while(isRunning()) {
            float frame_length = timer.getTimeElapsed();
            timer.reset();
            for(auto i : getModules())
                if(i != this && i->enabled && isRunning())
                    ((ClientModule*)i)->updateParallel(frame_length);
            if(timer.getTimeElapsed() < 1000 / PARALLEL_UPDATES_PER_SECOND && isRunning())
                gfx::sleep(1000 / PARALLEL_UPDATES_PER_SECOND - timer.getTimeElapsed());
        }
    } catch(const std::exception& exception) {
        interrupt_message = exception.what();
        interrupt = true;
    }
}

void Game::init() {
    for(auto i : getModules())
        if(i != this)
            ((ClientModule*)i)->postInit();
    parallel_update_thread = std::thread(&Game::parallelUpdateLoop, this);
    ms_timer_counter = ms_timer.getTimeElapsed();
}

void Game::update(float frame_length) {
    if(interrupt)
        throw Exception(interrupt_message);
    
    while(ms_timer_counter < (int)ms_timer.getTimeElapsed()) {
        ms_timer_counter++;
        for(SceneModule* module : getModules())
            if(module != this)
                ((ClientModule*)module)->updatePerMs();
    }
    
    fps_count++;
    frame_length_sum += getRenderTime();
    if(line_refresh_counter.getTimeElapsed() >= 1000) {
        frame_length_line.text = std::to_string(frame_length_sum / fps_count) + "ms per render";
        frame_length_sum = 0;
        
        fps_debug_line.text = std::to_string(fps_count) + " FPS";
        fps_count = 0;
        
        line_refresh_counter.reset();
    }
}

void Game::renderBack() {
    cycleModules();
}

bool Game::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        PauseScreen pause_screen(this, settings);
        pause_screen.run();
        if(pause_screen.hasExitedToMenu())
            returnFromScene();
        
        // reloads resources
        if(pause_screen.changed_mods) {
            resource_pack.loadPaths();
            for(auto i : getModules())
                if(i != this)
                    ((ClientModule*)i)->loadTextures();
            for(int i = 0; i < blocks.getWidth() / CHUNK_SIZE; i++)
                for (int j = 0; j < blocks.getHeight() / CHUNK_SIZE; j++)
                    blocks.getRenderBlockChunk(i, j)->has_update = true;

            for(auto entity : entities.getEntities())
                if(entity->type == EntityType::PLAYER) {
                    auto* player = (ClientPlayer*)entity;
                    if(player == players.getMainPlayer())
                        player->has_created_texture = false;
                }
        }
        return true;
    }
    return false;
}

void Game::stop() {
    parallel_update_thread.join();
}
