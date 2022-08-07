#pragma once
#include "launcherModule.hpp"
#include "print.hpp"

class ChatLine {
public:
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
};

class Console : LauncherModule, EventListener<PrintEvent>{
    std::vector<ChatLine*> chat_lines;
    gfx::TextInput input_box;
    std::vector<std::string> saved_lines = {""};
    int selected_saved_line = 0;
    void moduleConfig(std::string command);
    void onEvent(PrintEvent& event) override;
    void stop() override;
    void init() override;
    void update(float frame_length) override;
    bool onKeyDown(gfx::Key key) override;
public:
    std::vector<SceneModule*> module_vector;
    Console();
};