#ifndef multiplayerSelector_hpp
#define multiplayerSelector_hpp

#include "graphics.hpp"

struct MultiplayerSelector : gfx::Scene {
    void init() override;
    void onKeyDown(gfx::Key key) override;
    void render() override;
    void stop() override;

private:
    gfx::Button back_button, join_button;
    gfx::Sprite server_ip_title, username_title;
    gfx::TextInput server_ip, username;
    bool can_connect = true;
};

#endif
