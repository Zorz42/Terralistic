#pragma once

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
#include "minimap.hpp"
#include "clientBlocks.hpp"
#include "clientLiquids.hpp"
#include "clientLights.hpp"
#include "naturalLight.hpp"
#include "particles.hpp"
#include "settings.hpp"

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, Settings* settings, bool structure_world);

class Game : gfx::Scene, EventListener<GameErrorEvent>, public BackgroundRect {
    void onEvent(GameErrorEvent& event) override;
    
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    
    std::string username;
    
    ClientNetworking networking;
    ResourcePack resource_pack;
    ClientBlocks blocks;
    Particles particles;
    ClientLiquids liquids;
    ClientLights lights;
    NaturalLight natural_light;
    ClientEntities entities;
    ClientItems items;
    ClientPlayers players;
    BlockSelector block_selector;
    ClientInventory inventory;
    Minimap minimap;
    Chat chat;
    DebugMenu debug_menu;
    
    Recipes recipes;
    
    Settings* settings;
    BackgroundRect* background_rect;
    bool handshake_done = false;
    std::string kick_reason;
    
    std::vector<ClientModule*> modules;
    
    void registerAModule(ClientModule* module);
public:
    Game(BackgroundRect* background_rect, Settings* settings, const std::string& username, const std::string& ip_address, unsigned short port=33770);
    
    void renderBack() override;
    void setBackWidth(unsigned short width) override { }
    unsigned short getBackWidth() override { return 0; }
    bool isHandshakeDone();
    
    void start();
};
