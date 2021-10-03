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
            case ServerState::STARTING:
                text.loadFromText("Starting server");
                break;
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

class HandshakeScreen : public gfx::Scene {
    gfx::Sprite text;
    std::thread handshake_thread;
    void init() override;
    void render() override;
    BackgroundRect* background_rect;
    Game* game;
public:
    HandshakeScreen(BackgroundRect* background_rect, Game* game) : background_rect(background_rect), game(game) {}
};

void HandshakeScreen::init() {
    handshake_thread = std::thread(&Game::handshakeWithServer, game);
    text.scale = 3;
    text.orientation = gfx::CENTER;
    text.loadFromText("Getting terrain");
    background_rect->setBackWidth(text.getWidth() + 300);
}

void HandshakeScreen::render() {
    if(game->isHandshakeDone()) {
        handshake_thread.join();
        returnFromScene();
    }
    background_rect->renderBack();
    text.render();
    
    sf::Packet packet;
}

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, bool structure_world) {
    Server private_server(gfx::getResourcePath(), world_name);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
    if(structure_world)
        private_server.seed = 1000;
  
    private_server.setPrivate(true);
    server_thread = std::thread(&Server::start, &private_server, port);
    
    WorldStartingScreen world_starting_screen(menu_back, &private_server);
    gfx::runScene(world_starting_screen);

    Game game(menu_back, "_", "127.0.0.1", port);
    gfx::runScene(game);
    private_server.stop();
    
    while(private_server.state == ServerState::RUNNING)
        gfx::sleep(1);
    
    WorldStartingScreen world_starting_screen2(menu_back, &private_server);
    gfx::runScene(world_starting_screen2);
    
    server_thread.join();
}

Game::Game(BackgroundRect* background_rect, const std::string& username, std::string ip_address, unsigned short port) : ip_address(std::move(ip_address)),
    port(port),
    username(username),
    background_rect(background_rect),
    client_blocks(&resource_pack, &networking, &blocks, &liquids, &lights),
    players(&networking, &blocks, &liquids, &client_blocks, &resource_pack, &entities, username),
    items(&resource_pack, &client_blocks, &entities, &networking),
    client_entities(&entities, &networking),
    entities(&blocks),
    block_selector(&networking, &blocks, &client_blocks, &players),
    inventory(&networking, &resource_pack),
    debug_menu(&players, &blocks, &client_blocks),
    chat(&networking),
    minimap(&blocks, &liquids, &lights, &client_blocks),
    liquids(&blocks),
    lights(&blocks)
{
    networking.packet_event.addListener(this);
}

void Game::init() {
    std::vector<std::string> active_resource_packs = {gfx::getResourcePath() + "resourcePack"};
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/activeMods.txt")) {
        std::ifstream active_mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt");
        std::string line;
        while(std::getline(active_mods_file, line))
            active_resource_packs.insert(active_resource_packs.begin(), sago::getDataHome() + "/Terralistic/Mods/" + line);
    }
    resource_pack.load(active_resource_packs);
    
    if(!networking.establishConnection(ip_address, port)) {
        ChoiceScreen choice_screen(background_rect, "Could not connect to the server!", {"Close"});
        switchToScene(choice_screen);
        returnFromScene();
    }
    
    HandshakeScreen handshake_screen(background_rect, this);
    switchToScene(handshake_screen);
    
    if(got_kicked) {
        ChoiceScreen choice_screen(background_rect, kick_reason, {"Close"});
        switchToScene(choice_screen);
        returnFromScene();
    }
    
    registerAModule(&players);
    registerAModule(&block_selector);
    registerAModule(&inventory);
#ifdef DEVELOPER_MODE
    registerAModule(&debug_menu);
#endif
    registerAModule(&chat);
    registerAModule(&minimap);
    
    client_entities.init();
    items.init();
    client_blocks.init();
}

void Game::handshakeWithServer() {
    sf::Packet join_packet;
    join_packet << username;
    networking.sendPacket(join_packet);
    
    while(true) {
        sf::Packet packet = networking.getPacket();
        WelcomePacketType type;
        packet >> type;
        if(type == WelcomePacketType::WELCOME)
            break;
        
        client_blocks.onWelcomePacket(packet, type);
        inventory.onWelcomePacket(packet, type);
        
    }
    
    /*sf::Packet packet = networking.getPacket();
    PacketType type;
    packet >> type;
    
    if(type == PacketType::KICK) {
        got_kicked = true;
        packet >> kick_reason;
        handshake_done = true;
        return;
    }
    
    int player_x, player_y;
    packet >> player_x >> player_y;
    unsigned short world_width, world_height;
    packet >> world_width >> world_height;
    unsigned int size;
    packet >> size;*/
    
    //blocks.create(world_width, world_height);
    liquids.create();
    lights.create();
    client_blocks.create();
    
    //std::vector<char> map_data = networking.getData(size);
    //char* iter = &map_data[0];
    //iter = blocks.loadFromSerial(iter);
    //iter = liquids.loadFromSerial(iter);
    //iter = inventory.loadFromSerial(iter);
    
    for(int x = 0; x < blocks.getWidth(); x++)
        for(unsigned short y = 0; y < blocks.getHeight() && blocks.getBlockInfo(x, y).transparent; y++)
            lights.setLightSource(x, y, MAX_LIGHT);
    
    networking.disableBlocking();
    
    handshake_done = true;
}

void Game::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::KICK: {
            event.packet >> kick_reason;
            got_kicked = true;
        }
        default:;
    }
}

void Game::update() {
    if(got_kicked) {
        ChoiceScreen choice_screen(background_rect, kick_reason, {"Close"});
        switchToScene(choice_screen);
        returnFromScene();
    }
    networking.checkForPackets();
    entities.updateAllEntities(getFrameLength());
    client_blocks.updateLights();
}

void Game::render() {
    float scale = (float)gfx::getWindowHeight() / resource_pack.getBackground().getTextureHeight();
    int position_x = -(client_blocks.view_x / 5) % int(resource_pack.getBackground().getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (resource_pack.getBackground().getTextureWidth() * scale) + 2; i++)
        resource_pack.getBackground().render(scale, position_x + i * resource_pack.getBackground().getTextureWidth() * scale, 0);
    client_blocks.renderBackBlocks();
    players.renderPlayers();
    items.renderItems();
    client_blocks.renderFrontBlocks();
}

void Game::stop() {
    networking.closeConnection();
}

void Game::renderBack() {
    for(SceneModule* module : getModules())
        module->update();
    for(SceneModule* module : getModules())
        module->render();
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
