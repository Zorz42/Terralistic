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
                text.renderText("Starting server");
                break;
            case ServerState::LOADING_WORLD:
                text.renderText("Loading world");
                break;
            case ServerState::GENERATING_WORLD:
                text.renderText("Generating world");
                break;
            case ServerState::STOPPING:
                text.renderText("Saving world");
                break;
            case ServerState::RUNNING:
            case ServerState::STOPPED:
                gfx::returnFromScene();
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
    Server private_server(gfx::resource_path, world_name);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
    if(structure_world)
        private_server.seed = 1000;
  
    private_server.setPrivate(true);
    server_thread = std::thread(&Server::start, &private_server, port);
    
    WorldStartingScreen(menu_back, &private_server).run();

    game(menu_back, "_", "127.0.0.1", port).run();
    private_server.stop();
    
    while(private_server.state == ServerState::RUNNING)
        gfx::sleep(1);
    
    WorldStartingScreen(menu_back, &private_server).run();
    
    server_thread.join();
}

game::game(BackgroundRect* background_rect, const std::string& username, std::string ip_address, unsigned short port) : ip_address(std::move(ip_address)),
    port(port),
    username(username),
    background_rect(background_rect),
    blocks(&resource_pack),
    players(&networking_manager, &blocks, &resource_pack, &entities, username),
    items(&resource_pack, &blocks, &entities),
    entities(&blocks),
    block_selector(&networking_manager, &blocks, &inventory, &players),
    inventory(&networking_manager, &resource_pack),
    debug_menu(&players, &blocks),
    chat(&networking_manager),
    minimap(&blocks)
{}

void game::init() {
    std::vector<std::string> active_resource_packs = {gfx::resource_path + "resourcePack"};
    if(std::filesystem::exists(sago::getDataHome() + "/Terralistic/activeMods.txt")) {
        std::ifstream active_mods_file(sago::getDataHome() + "/Terralistic/activeMods.txt");
        std::string line;
        while(std::getline(active_mods_file, line))
            active_resource_packs.insert(active_resource_packs.begin(), sago::getDataHome() + "/Terralistic/Mods/" + line);
    }
    resource_pack.load(active_resource_packs);
    
    if(!networking_manager.establishConnection(ip_address, port)) {
        ChoiceScreen(background_rect, "Could not connect to the server!", {"Close"}).run();
        gfx::returnFromScene();
    }
    
    std::thread handshake_thread(&game::handshakeWithServer, this);
    
    gfx::Sprite text;
    text.scale = 3;
    text.orientation = gfx::CENTER;
    text.renderText("Getting terrain");
    background_rect->setBackWidth(text.getWidth() + 300);
    
    while(!handshake_done) {
        gfx::clearWindow();
        
        background_rect->renderBack();
        text.render();
        
        gfx::updateWindow();
    }
    
    handshake_thread.join();
    
    modules = {
        &blocks,
        &players,
        &items,
        &block_selector,
        &inventory,
#ifdef DEVELOPER_MODE
        &debug_menu,
#endif
        &chat,
        &minimap,
    };
}

void game::handshakeWithServer() {
    sf::Packet join_packet;
    join_packet << username;
    networking_manager.sendPacket(join_packet);
    networking_manager.flushPackets();
    
    sf::Packet packet = networking_manager.getPacket();
    
    int player_x, player_y;
    packet >> player_x >> player_y;
    unsigned short world_width, world_height;
    packet >> world_width >> world_height;
    unsigned int size;
    packet >> size;
    
    std::vector<char> map_data = networking_manager.getData(size);
    
    blocks.create(world_width, world_height, map_data);
    
    networking_manager.disableBlocking();
    
    handshake_done = true;
}

void game::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::KICK: {
            std::string kick_message;
            event.packet >> kick_message;
            ChoiceScreen(background_rect, kick_message, {"Close"}).run();
            gfx::returnFromScene();
        }
        default:;
    }
}

void game::update() {
    networking_manager.checkForPackets();
    networking_manager.flushPackets();
    entities.updateAllEntities(gfx::getDeltaTime());
}

void game::render() {
    float scale = (float)gfx::getWindowHeight() / resource_pack.getBackground().getTextureHeight();
    int position_x = -(blocks.view_x / 5) % int(resource_pack.getBackground().getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (resource_pack.getBackground().getTextureWidth() * scale) + 2; i++)
        resource_pack.getBackground().render(scale, position_x + i * resource_pack.getBackground().getTextureWidth() * scale, 0);
    blocks.renderBackBlocks();
    players.renderPlayers();
    items.renderItems();
    blocks.renderFrontBlocks();
}

void game::stop() {
    networking_manager.closeConnection();
}

void game::renderBack() {
    update();
    for(GraphicalModule* module : modules)
        module->update();
    render();
    for(GraphicalModule* module : modules)
        module->render();
}

void game::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::ESCAPE) {
        enableAllEvents(false);
        PauseScreen pause_screen(this);
        pause_screen.run();
        if(pause_screen.hasExitedToMenu())
            gfx::returnFromScene();
    }
}
