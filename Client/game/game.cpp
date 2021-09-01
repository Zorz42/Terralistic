#include <thread>
#include <cassert>
#include <filesystem>

#include "game.hpp"
#include "pauseScreen.hpp"
#include "fileManager.hpp"
#include "choiceScreen.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"
#include "server.hpp"
#include "blockSelector.hpp"
#include "compress.hpp"

#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

class WorldStartingScreen : public gfx::Scene {
    BackgroundRect* menu_back;
    gfx::Sprite text;
    void init() override;
    void render() override;
    Server* server;
    ServerState prev_server_state = ServerState::NEUTRAL;
public:
    WorldStartingScreen(BackgroundRect* menu_back, Server* server) : menu_back(menu_back), server(server) {}
};

void WorldStartingScreen::init() {
    text.scale = 3;
    text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    text.createBlankImage(1, 1);
    text.orientation = gfx::CENTER;
}

void WorldStartingScreen::render() {
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
                /*loading_bar.w += (private_server.getGeneratingCurrent() * LOADING_RECT_WIDTH / private_server.getGeneratingTotal() - loading_bar.w) / 3;
                loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
                gfx::clearWindow();
                generating_text.render();
                loading_bar_back.render();
                loading_bar.render();
                gfx::updateWindow();*/
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
    menu_back->renderBack();
    text.render();
}

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back) {
    Server private_server(fileManager::getWorldsPath(), gfx::resource_path, fileManager::getWorldsPath() + world_name);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
  
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

void game::init() {
    resource_pack.load("resourcePack");
    
    if(!networking_manager.establishConnection(ip_address, port)) {
        ChoiceScreen(menu_back, "Could not connect to the server!", {"Close"}).run();
        gfx::returnFromScene();
    }
    
    std::thread handshake_thread(&game::handshakeWithServer, this);
    
    gfx::Sprite text;
    text.scale = 3;
    text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    text.orientation = gfx::CENTER;
    text.renderText("Getting terrain");
    menu_back->setBackWidth(text.getWidth() + 300);
    
    while(!handshake_done) {
        gfx::clearWindow();
        
        menu_back->renderBack();
        text.render();
        
        gfx::updateWindow();
    }
    
    handshake_thread.join();
    
    ClientInventory* inventory_handler = new ClientInventory(&networking_manager, &resource_pack);
    items = new ClientItems(&resource_pack, blocks);
    
    modules = {
        blocks,
        player_handler,
        items,
        new BlockSelector(&networking_manager, blocks, inventory_handler, player_handler),
        inventory_handler,
#ifdef DEVELOPER_MODE
        new DebugMenu(player_handler, blocks),
#endif
        new Chat(&networking_manager),
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
    
    map_data = decompress(map_data);
    
    blocks = new ClientBlocks(&networking_manager, &resource_pack, world_width, world_height, map_data);
    
    networking_manager.disableBlocking();
    
    player_handler = new ClientPlayers(&networking_manager, blocks, &resource_pack, player_x, player_y, username);
    
    handshake_done = true;
}

void game::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::KICK: {
            std::string kick_message;
            event.packet >> kick_message;
            ChoiceScreen(menu_back, kick_message, {"Close"}).run();
            gfx::returnFromScene();
        }
        default:;
    }
}

void game::update() {
    networking_manager.checkForPackets();
    networking_manager.flushPackets();
}

void game::render() {
    float scale = (float)gfx::getWindowHeight() / resource_pack.getBackground().getTextureHeight();
    int position_x = -(blocks->view_x / 5) % int(resource_pack.getBackground().getTextureWidth() * scale);
    for(int i = 0; i < gfx::getWindowWidth() / (resource_pack.getBackground().getTextureWidth() * scale) + 2; i++)
        resource_pack.getBackground().render(scale, position_x + i * resource_pack.getBackground().getTextureWidth() * scale, 0);
    blocks->renderBackBlocks();
    player_handler->renderPlayers();
    items->renderItems();
    blocks->renderFrontBlocks();
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
