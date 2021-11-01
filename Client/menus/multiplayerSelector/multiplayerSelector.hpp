#pragma once

#include "graphics.hpp"
#include "menuBack.hpp"
#include "settings.hpp"

class MultiplayerSelector : public gfx::Scene {
    void init() override;
    bool onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;
    gfx::Button back_button, join_button;
    gfx::Sprite server_ip_title, username_title;
    gfx::TextInput server_ip, username;
    bool can_connect = true;
    BackgroundRect* menu_back;
    Settings* settings;
public:
    explicit MultiplayerSelector(BackgroundRect* menu_back, Settings* settings) : menu_back(menu_back), settings(settings) {}
};
