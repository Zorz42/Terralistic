#ifndef game_hpp
#define game_hpp

#include "graphics.hpp"
#include "clientPlayers.hpp"
#include "clientItems.hpp"
#include "menuBack.hpp"
#include "menuBack.hpp"
#include "clientEntities.hpp"
#include "clientInventory.hpp"
#include "blockSelector.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"
#include "Minimap.hpp"
#include "clientBlocks.hpp"
#include "clientLiquids.hpp"
#include "clientLights.hpp"

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, bool structure_world);

class Game : public gfx::Scene, EventListener<ClientPacketEvent>, public BackgroundRect {
    void onEvent(ClientPacketEvent& event) override;
    void init() override;
    void update() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    const std::string ip_address;
    const unsigned short port;
    std::string username;
    NetworkingManager networking;
    ResourcePack resource_pack;
    ClientEntities client_entities;
    ClientPlayers players;
    ClientItems items;
    ClientInventory inventory;
    BlockSelector block_selector;
    DebugMenu debug_menu;
    Chat chat;
    Minimap minimap;
    ClientBlocks blocks;
    ClientLiquids liquids;
    ClientLights lights;
    BackgroundRect* background_rect;
    bool handshake_done = false, got_kicked = false;
    std::string kick_reason;
    
public:
    Game(BackgroundRect* background_rect, const std::string& username, std::string ip_address, unsigned short port=33770);
    
    void renderBack() override;
    void setBackWidth(unsigned short width) override { }
    unsigned short getBackWidth() override { return 0; }
    bool isHandshakeDone();
    void handshakeWithServer();
};

#endif
