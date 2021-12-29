#pragma once
#include "menuBack.hpp"

class ServerAdder : public gfx::Scene {
    gfx::Button back_button, add_button;
    gfx::Sprite add_server_title;
    gfx::TextInput server_ip_input;
    BackgroundRect* menu_back;
    bool can_create = true;
    void init() override;
    bool onKeyUp(gfx::Key key) override;
    void render() override;
public:
    ServerAdder(BackgroundRect* menu_back) : menu_back(menu_back) {}
    std::string server_ip;
};
