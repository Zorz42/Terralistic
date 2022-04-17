#pragma once
#include "clientPlayers.hpp"

class DebugMenu : public ClientModule, EventListener<ClientPacketEvent>{
    class DebugLine {
        std::string prev_text;
        gfx::Texture texture;
    public:
        std::string text = " ";
        void render(int x, int y);
        int getWidth();
        int getHeight();
        void update();
    };
    
    ClientNetworking* networking;
    ClientBlocks* blocks;
    ClientPlayers* players;
    
    void onEvent(ClientPacketEvent& event) override;
    
    bool debug_menu_open = false;
    int fps_count = 0;
    int server_tps = 0;
    int packet_count = 0;
    std::vector<DebugLine*> debug_lines;
    gfx::Rect back_rect;
    gfx::Timer timer;
    
    DebugLine fps_line, server_tps_line, coords_line, packets_line;
    
    void init() override;
    void update(float frame_length) override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;
public:
    DebugMenu(ClientNetworking* networking, ClientBlocks* blocks, ClientPlayers* players) : networking(networking), blocks(blocks), players(players) {}
};
