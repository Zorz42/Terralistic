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

#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

class ServerStart : public gfx::Scene {
    MenuBack* menu_back;
    gfx::Sprite text;
    void init() override;
    void render() override;
    Server* server;
    ServerState prev_server_state = ServerState::NEUTRAL;
public:
    ServerStart(MenuBack* menu_back, Server* server) : menu_back(menu_back), server(server) {}
};

void ServerStart::init() {
    text.scale = 3;
    text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    text.createBlankImage(1, 1);
    text.orientation = gfx::CENTER;
}

void ServerStart::render() {
    if(server->state != prev_server_state) {
        prev_server_state = server->state;
        if(server->state == ServerState::RUNNING || server->state == ServerState::STOPPED) {
            gfx::returnFromScene();
            return;
        }
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
            default:;
        }
        menu_back->setWidth(text.getWidth() + 300);
    }
    menu_back->render();
    text.render();
}

void startPrivateWorld(const std::string& world_name, MenuBack* menu_back) {
  
    Server private_server(fileManager::getWorldsPath(), gfx::resource_path, fileManager::getWorldsPath() + world_name);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
  
    private_server.setPrivate(true);
    server_thread = std::thread(&Server::start, &private_server, port);
    
    ServerStart(menu_back, &private_server).run();

    game(menu_back, "_", "127.0.0.1", port).run();
    private_server.stop();
    
    while(private_server.state == ServerState::RUNNING)
        gfx::sleep(1);
    
    ServerStart(menu_back, &private_server).run();
    
    server_thread.join();
}

void game::init() {
    resource_pack.load("resourcePack");
    
    blocks = new ClientBlocks(&networking_manager, &resource_pack);
    blocks->createWorld(4400, 1200);
    
    player_handler = new ClientPlayers(&networking_manager, blocks, &resource_pack, username);
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
        new PauseScreen(),
    };

    //renderTextScreen("Connecting to server");
    if(!networking_manager.establishConnection(ip_address, port)) {
        ChoiceScreen(menu_back, "Could not connect to the server!", {"Close"}).run();
        gfx::returnFromScene();
    }
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
