#pragma once
#include "menuBack.hpp"

class ServerAdder : public gfx::Scene {
    gfx::Button back_button, add_button;
    gfx::Sprite add_server_title;
    gfx::Sprite new_server_name;
    gfx::Sprite new_server_ip;
    gfx::TextInput server_ip_input;
    gfx::TextInput server_name_input;
    BackgroundRect* menu_back;
    bool can_add = true;
    void init() override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
public:
    explicit ServerAdder(BackgroundRect* menu_back) : menu_back(menu_back) {}
    std::string server_ip;
    std::string server_name;
};
