#pragma once
#include "clientModule.hpp"

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

class DebugMenu : public ClientModule {
    
    //void onEvent(ClientPacketEvent& event) override;
    
    bool debug_menu_open = false;
    //bool received_ping_answer = true;
    //gfx::Timer ping_timer;
    //int fps_count = 0;
    //int server_tps = 0;
    //int packet_count = 0;
    std::vector<DebugLine*> debug_lines;
    gfx::Rect back_rect;
    //gfx::Timer timer;
    
    //DebugLine fps_line, server_tps_line, ping_line, coords_line, packets_line;
    
    void init() override;
    void update(float frame_length) override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
    void stop() override;
public:
    void registerDebugLine(DebugLine* debug_line);
};
