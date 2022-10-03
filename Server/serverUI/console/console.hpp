#pragma once
#include "launcherModule.hpp"
#include "print.hpp"
#include "moduleManager.hpp"

class ChatLine {
public:
    std::string text;
    gfx::Sprite text_sprite;
    int y_to_be{};
};

class Console : LauncherModule, EventListener<PrintEvent>{
    bool enable_log = true;
    std::string log_file_name;
    std::vector<ChatLine*> chat_lines;
    gfx::TextInput input_box;
    std::vector<std::string> saved_lines = {""};
    int selected_saved_line = 0;
    void saveToLog(const std::string& line);
    void onEvent(PrintEvent& event) override;
    void stop() override;
    void init() override;
    void render() override;
    bool onKeyDown(gfx::Key key) override;
public:
    ModuleManager* module_manager;
    Console(std::string resource_path);
};
