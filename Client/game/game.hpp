#pragma once
#include "clientItems.hpp"
#include "menuBack.hpp"
#include "clientEntities.hpp"
#include "clientInventory.hpp"
#include "blockSelector.hpp"
#include "debugMenu.hpp"
#include "chat.hpp"
#include "clientLiquids.hpp"
#include "clientLights.hpp"
#include "health.hpp"
#include "content.hpp"
#include "naturalLight.hpp"
#include "camera.hpp"
#include "background.hpp"

void startPrivateWorld(const std::string& world_name, BackgroundRect* menu_back, Settings* settings, bool structure_world);

class Game : public gfx::Scene, public BackgroundRect {
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    
    std::string username;
    
    ClientNetworking networking;
    ResourcePack resource_pack;
    Camera camera;
    Background background;
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
    Chat chat;
    DebugMenu debug_menu;
    Health player_health;
    Recipes recipes;
    GameContent content;

    Settings* settings;
    BackgroundRect* background_rect;
    bool handshake_done = false;
    std::string kick_reason;
public:
    Game(BackgroundRect* background_rect, Settings* settings, const std::string& username, const std::string& ip_address, int port=33770);
    
    void renderBack() override;
    void setBackWidth(int width) override { }
    int getBackWidth() override { return 0; }
    bool isHandshakeDone();
    
    bool interrupt = false;
    std::string interrupt_message;
    
    void initialize();
    void start();
};
