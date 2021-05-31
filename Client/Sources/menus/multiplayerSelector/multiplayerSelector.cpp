//
//  multiplayerSelector.cpp
//  Terralistic
//
//  Created by Jakob Zorz on 12/01/2021.
//

#include <vector>
#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "config.hpp"

// this is a menu, where you select the server you want to play on

#define PADDING 20

void multiplayerSelector::init() {
    config.setDefault("username", "");
    config.setDefault("server ip", "");
    
    // the back button
    back_button.scale = 3;
    back_button.setTexture(gfx::renderText("Back", {255, 255, 255}));
    back_button.y = -PADDING;
    back_button.orientation = gfx::bottom;
    
    // "Join Server" button
    join_button.scale = 3;
    join_button.setTexture(gfx::renderText("Join Server", {255, 255, 255}));
    join_button.y = -PADDING;
    join_button.orientation = gfx::bottom;
    
    back_button.x = short((-join_button.getWidth() - back_button.getWidth() + back_button.getWidth() - PADDING) / 2);
    join_button.x = short((join_button.getWidth() + back_button.getWidth() - join_button.getWidth() + PADDING) / 2);
    
    // server ip text box
    server_ip.scale = 3;
    server_ip.orientation = gfx::center;
    server_ip.setText("");
    server_ip.y = 3 * PADDING;
    server_ip.active = true;
    server_ip.setText(config.get("server ip"));
    server_ip.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || c == '.')
            return c;
        return '\0';
    };
    
    // "Type the server IP:" text in the top
    server_ip_title.setTexture(gfx::renderText("Server IP:", {255, 255, 255}));
    server_ip_title.scale = 3;
    server_ip_title.y = server_ip.y - server_ip.sprite::getHeight() - PADDING;
    server_ip_title.orientation = gfx::center;
    
    // username text box
    username.scale = 3;
    username.orientation = gfx::center;
    username.setText("");
    username.y = server_ip_title.y - server_ip_title.getHeight() - 3 * PADDING;
    username.setText(config.get("username"));
    username.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_')
            return c;
        if(c == ' ')
            return '_';
        return '\0';
    };
    
    // "Type the server IP:" text in the top
    username_title.setTexture(gfx::renderText("Username:", {255, 255, 255}));
    username_title.scale = 3;
    username_title.y = username.y - username.sprite::getHeight() - PADDING;
    username_title.orientation = gfx::center;
    
    text_inputs = {&server_ip, &username};
}

void multiplayerSelector::onKeyDown(gfx::key key) {
    if(key == gfx::KEY_MOUSE_LEFT && back_button.isHovered())
        gfx::returnFromScene();
    else if((key == gfx::KEY_MOUSE_LEFT && join_button.isHovered()) || (key == gfx::KEY_ENTER && can_connect))
        gfx::runScene(new game(username.getText(), server_ip.getText()));
}

void multiplayerSelector::render() {
    if(can_connect != (username.getText().size() >= 3 && !server_ip.getText().empty())) {
        can_connect = !can_connect;
        join_button.setTexture(gfx::renderText("Join Server", {(unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100)}));
        join_button.disabled = !can_connect;
    }
    gfx::render(join_button);
    gfx::render(back_button);
    gfx::render(server_ip_title);
    gfx::render(server_ip);
    gfx::render(username);
    gfx::render(username_title);
}

void multiplayerSelector::stop() {
    config.set("username", username.getText());
    config.set("server ip", server_ip.getText());
}
