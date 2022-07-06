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
    
    bool debug_menu_open = false;
    std::vector<DebugLine*> debug_lines;
    gfx::Rect back_rect;
    
    void init() override;
    void update(float frame_length) override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
public:
    void registerDebugLine(DebugLine* debug_line);
};
