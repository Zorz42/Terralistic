//
//  multiplayerSelector.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#ifndef multiplayerSelector_hpp
#define multiplayerSelector_hpp

#include "graphics.hpp"

struct multiplayerSelector : gfx::Scene {
    void init() override;
    void onKeyDown(gfx::key key) override;
    void render() override;
    void stop() override;

private:
    gfx::Button back_button, join_button;
    gfx::Sprite server_ip_title, username_title;
    gfx::TextInput server_ip, username;
    bool can_connect = true;
};

#endif /* multiplayerSelector_hpp */
