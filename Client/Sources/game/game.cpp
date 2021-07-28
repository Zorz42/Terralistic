#include <thread>
#include <cassert>
#include <filesystem>
#include <thread>
#include <utility>
#include "game.hpp"
#include "pauseScreen.hpp"
#include "textScreen.hpp"
#include "fileManager.hpp"
#include "choiceScreen.hpp"
#include "DebugMenu.hpp"
#include "chat.hpp"
#include "server.hpp"
#include "blockSelector.hpp"

#define FROM_PORT 49152
#define TO_PORT 65535

static std::thread server_thread;

#define TEXT_SCALE 3

#define LOADING_RECT_HEIGHT 20
#define LOADING_RECT_WIDTH (gfx::getWindowWidth() / 5 * 4)
#define LOADING_RECT_ELEVATION 50

void startPrivateWorld(const std::string& world_name) {
    gfx::Sprite generating_text;
    gfx::Rect loading_bar_back{0, -LOADING_RECT_ELEVATION, (unsigned short)(LOADING_RECT_WIDTH), LOADING_RECT_HEIGHT, GREY, gfx::BOTTOM},
    loading_bar{0, -LOADING_RECT_ELEVATION, 0, LOADING_RECT_HEIGHT, WHITE, gfx::BOTTOM};

    generating_text.scale = TEXT_SCALE;
    generating_text.y = (LOADING_RECT_HEIGHT - LOADING_RECT_ELEVATION) / 2;
    generating_text.renderText("Generating world");
    generating_text.orientation = gfx::CENTER;

    std::filesystem::create_directory(fileManager::getWorldsPath() + world_name);

    Server private_server(fileManager::getWorldsPath() + world_name, gfx::resource_path);
    unsigned short port = rand() % (TO_PORT - FROM_PORT) + TO_PORT;
    server_thread = std::thread(&Server::start, &private_server, port);

    while(private_server.state != ServerState::RUNNING)
        switch (private_server.state) {
            case ServerState::NEUTRAL:
                gfx::sleep(1);
                break;
            case ServerState::STARTING:
                renderTextScreen("Starting server");
                break;
            case ServerState::LOADING_WORLD:
                renderTextScreen("Loading world");
                break;
            case ServerState::GENERATING_WORLD:
                loading_bar.w += (private_server.getGeneratingCurrent() * LOADING_RECT_WIDTH / private_server.getGeneratingTotal() - loading_bar.w) / 3;
                loading_bar.x = -short(loading_bar_back.w - loading_bar.w) / 2;
                gfx::clearWindow();
                generating_text.render();
                loading_bar_back.render();
                loading_bar.render();
                gfx::updateWindow();
                break;
            default:
                assert(false);
                break;
        }

    private_server.setPrivate(true);

    game("_", "127.0.0.1", port).run();
    private_server.stop();
    while(private_server.state != ServerState::STOPPING)
        renderTextScreen("Saving world");
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

    renderTextScreen("Connecting to server");
    if(!networking_manager.establishConnection(ip_address, port)) {
        ChoiceScreen("Could not connect to the server!", {"Close"}).run();
        gfx::returnFromScene();
    }
}

void game::onEvent(ClientPacketEvent& event) {
    switch(event.packet_type) {
        case PacketType::KICK: {
            std::string kick_message;
            event.packet >> kick_message;
            ChoiceScreen(kick_message, {"Close"}).run();
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
