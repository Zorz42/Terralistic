#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "platform_folders.h"

void MultiplayerSelector::init() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setDefaultStr("username", "");
    config.setDefaultStr("server ip", "");
    
    back_button.scale = 3;
    back_button.loadFromText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;
    
    join_button.scale = 3;
    join_button.loadFromText("Join Server");
    join_button.y = -SPACING;
    join_button.orientation = gfx::BOTTOM;
    
    back_button.x = (-join_button.getWidth() - back_button.getWidth() + back_button.getWidth() - SPACING) / 2;
    join_button.x = (join_button.getWidth() + back_button.getWidth() - join_button.getWidth() + SPACING) / 2;
    
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
    
    server_ip_title.loadFromText("Server IP:");
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
    
    username_title.loadFromText("Username:");
    username_title.scale = 3;
    username_title.y = username.y - username.Sprite::getHeight() - SPACING;
    username_title.orientation = gfx::CENTER;
    
    server_ip.def_color.a = TRANSPARENCY;
    username.def_color.a = TRANSPARENCY;
    
    text_inputs = {&server_ip, &username};
}

bool MultiplayerSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT && back_button.isHovered(getMouseX(), getMouseY())) {
        returnFromScene();
        return true;
    } else if((key == gfx::Key::MOUSE_LEFT && join_button.isHovered(getMouseX(), getMouseY())) || (key == gfx::Key::ENTER && can_connect)) {
        Game game(menu_back, settings, username.getText(), server_ip.getText());
        game.start();
        return true;
    }
    return false;
}

void MultiplayerSelector::render() {
    menu_back->setBackWidth(username.getWidth() + 100);
    menu_back->renderBack();
    if(can_connect != (username.getText().size() >= 3 && !server_ip.getText().empty())) {
        can_connect = !can_connect;
        join_button.loadFromText("Join Server", {(unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100), (unsigned char)(can_connect ? 255 : 100)});
        join_button.disabled = !can_connect;
    }
    join_button.render(getMouseX(), getMouseY());
    back_button.render(getMouseX(), getMouseY());
    server_ip.render(getMouseX(), getMouseY());
    username.render(getMouseX(), getMouseY());
    server_ip_title.render();
    username_title.render();
}

void MultiplayerSelector::stop() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setStr("username", username.getText());
    config.setStr("server ip", server_ip.getText());
}
