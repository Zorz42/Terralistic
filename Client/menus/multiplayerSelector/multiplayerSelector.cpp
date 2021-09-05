#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "configManager.hpp"
#include "platform_folders.h"

void MultiplayerSelector::init() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setDefaultStr("username", "");
    config.setDefaultStr("server ip", "");
    
    back_button.scale = 3;
    back_button.renderText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;
    
    join_button.scale = 3;
    join_button.renderText("Join Server");
    join_button.y = -SPACING;
    join_button.orientation = gfx::BOTTOM;
    
    back_button.x = short((-join_button.getWidth() - back_button.getWidth() + back_button.getWidth() - SPACING) / 2);
    join_button.x = short((join_button.getWidth() + back_button.getWidth() - join_button.getWidth() + SPACING) / 2);
    
    server_ip.scale = 3;
    server_ip.orientation = gfx::CENTER;
    server_ip.setText("");
    server_ip.y = 3 * SPACING;
    server_ip.active = true;
    server_ip.setText(config.getStr("server ip"));
    server_ip.textProcessing = [](char c, int length) {
        if((c >= '0' && c <= '9') || c == '.')
            return c;
        return '\0';
    };
    
    server_ip_title.renderText("Server IP:");
    server_ip_title.scale = 3;
    server_ip_title.y = server_ip.y - server_ip.Sprite::getHeight() - SPACING;
    server_ip_title.orientation = gfx::CENTER;
    
    username.scale = 3;
    username.orientation = gfx::CENTER;
    username.setText("");
    username.y = server_ip_title.y - server_ip_title.getHeight() - 3 * SPACING;
    username.setText(config.getStr("username"));
    username.textProcessing = [](char c, int length) {
        if((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_')
            return c;
        if(c == ' ')
            return '_';
        return '\0';
    };
    
    username_title.renderText("Username:");
    username_title.scale = 3;
    username_title.y = username.y - username.Sprite::getHeight() - SPACING;
    username_title.orientation = gfx::CENTER;
    
    server_ip.def_color.a = TRANSPARENCY;
    username.def_color.a = TRANSPARENCY;
    
    text_inputs = {&server_ip, &username};
}

void MultiplayerSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered(mouse_x, mouse_y))
        gfx::returnFromScene();
    else if((key == gfx::Key::MOUSE_LEFT && join_button.isHovered(mouse_x, mouse_y)) || (key == gfx::Key::ENTER && can_connect))
        game(menu_back, username.getText(), server_ip.getText()).run();
}

void MultiplayerSelector::render() {
    menu_back->setBackWidth(username.getWidth() + 100);
    menu_back->renderBack();
    if(can_connect != (username.getText().size() >= 3 && !server_ip.getText().empty())) {
        can_connect = !can_connect;
        join_button.renderText("Join Server", {(unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100)});
        join_button.disabled = !can_connect;
    }
    join_button.render(mouse_x, mouse_y);
    back_button.render(mouse_x, mouse_y);
    server_ip.render(mouse_x, mouse_y);
    username.render(mouse_x, mouse_y);
    server_ip_title.render();
    username_title.render();
}

void MultiplayerSelector::stop() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setStr("username", username.getText());
    config.setStr("server ip", server_ip.getText());
}
