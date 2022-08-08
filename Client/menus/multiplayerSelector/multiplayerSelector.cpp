#include "platform_folders.h"
#include "multiplayerSelector.hpp"
#include "game.hpp"
#include "choiceScreen.hpp"
#include "serverAdder.hpp"
#include "nameChooser.hpp"
#include "readOpa.hpp"
#include "resourcePath.hpp"

#define TOP_HEIGHT (title.getHeight() + 2 * SPACING)
#define BOTTOM_HEIGHT (back_button.h + 2 * SPACING)

void MenuServer::render(int position, int mouse_x, int mouse_y) {
    int render_x = gfx::getWindowWidth() / 2 - 400 + SPACING, render_y = y - position, render_width = 800 - 2 * SPACING, render_height = 116 + 2 * SPACING;
    
    gfx::Color back_color = BLACK;
    back_color.a = 100;
    gfx::RectShape(render_x, render_y, render_width, render_height).render(back_color);
    
    icon.render(1, render_x + SPACING, render_y + SPACING);
    name_texture.render(3, render_x + 2 * SPACING + icon.getTextureWidth(), render_y + SPACING * 1.5);
    
    join_button.x = render_x + 2 * SPACING + icon.getTextureWidth();
    join_button.y = render_y + render_height - join_button.h - SPACING;
    join_button.render(mouse_x, mouse_y);
    
    remove_button.x = render_x + 3 * SPACING + icon.getTextureWidth() + join_button.w;
    remove_button.y = render_y + render_height - join_button.h - SPACING;
    remove_button.render(mouse_x, mouse_y);
}

void MultiplayerSelector::refresh() {
    for(int i = 0; i < servers.size(); i++)
        delete servers[i];
    servers.clear();
    
    scroll_limit = 0;
    
    for(int i = 0; i < server_data.size(); i++) {
        servers.push_back(new MenuServer(ServerData("", "")));
        MenuServer* server = servers.back();
        
        server->y = scroll_limit + TOP_HEIGHT;
        
        server->icon.loadFromSurface(readOpa(resource_path + "world_icon.opa"));
        
        server->data.ip = server_data[i].ip;
        server->data.name = server_data[i].name;
        server->name_texture.loadFromSurface(gfx::textToSurface(server_data[i].name));

        server->join_button.loadFromSurface(readOpa(resource_path + "join_button.opa"));
        server->join_button.setScale(3);
        server->join_button.margin = 5;
        
        server->remove_button.loadFromSurface(readOpa(resource_path + "remove_button.opa"));
        server->remove_button.setScale(3);
        server->remove_button.margin = 5;
        
        scroll_limit += 116 + SPACING * 3;
    }
}

void MultiplayerSelector::init() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    config.setDefaultStr("servers", "");
    
    title.setScale(3);
    title.loadFromSurface(gfx::textToSurface("Select a server to join"));
    title.y = SPACING;
    title.orientation = gfx::TOP;

    back_button.setScale(3);
    back_button.loadFromSurface(gfx::textToSurface("Back"));
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;

    new_button.setScale(3);
    new_button.loadFromSurface(gfx::textToSurface("Add"));
    new_button.y = -SPACING;
    new_button.orientation = gfx::BOTTOM;
    
    top_rect.orientation = gfx::TOP;
    top_rect.setHeight(TOP_HEIGHT);
    
    bottom_rect.orientation = gfx::BOTTOM;
    bottom_rect.setHeight(BOTTOM_HEIGHT);
    bottom_rect.fill_color.a = TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = SHADOW_INTENSITY;
    bottom_rect.blur_radius = BLUR;
    
    std::string servers_str = config.getStr("servers"), curr_serv_ip, curr_serv_name;
    bool ip_name_switch = false;
    for(int i = 0; i < servers_str.size(); i++) {
        if(!ip_name_switch) {
            if (servers_str[i] == ' ') {
                ip_name_switch = true;
            } else
                curr_serv_name.push_back(servers_str[i]);
        }else{
            if (servers_str[i] == ' ') {
                server_data.emplace_back(curr_serv_name, curr_serv_ip);
                curr_serv_ip.clear();
                curr_serv_name.clear();
                ip_name_switch = false;
            } else
                curr_serv_ip.push_back(servers_str[i]);
        }
    }
    
    refresh();
}

bool MultiplayerSelector::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        else if(new_button.isHovered(getMouseX(), getMouseY())) {
            ServerAdder server_adder(menu_back);
            server_adder.run();
            if(!server_adder.server_ip.empty())
                server_data.emplace_back(server_adder.server_name, server_adder.server_ip);
            refresh();
        }
        else
            for(int i = 0; i < servers.size(); i++) {
                if(servers[i]->join_button.isHovered(getMouseX(), getMouseY())) {
                    NameChooser name_chooser(menu_back, settings, servers[i]->data.ip);
                    name_chooser.run();
                } else if(servers[i]->remove_button.isHovered(getMouseX(), getMouseY())) {
                    std::string result;
                    if(getKeyState(gfx::Key::SHIFT))
                        result = "Yes";
                    else {
                        ChoiceScreen choice_screen(menu_back, "Do you want to remove " + servers[i]->data.name + "?", {"Yes", "No"}, &result);
                        choice_screen.run();
                    }

                    if(result == "Yes") {
                        server_data.erase(server_data.begin() + i);
                        refresh();
                    }
                    break;
                }
            }
        return true;
    }
    return false;
}

void MultiplayerSelector::onMouseScroll(int distance) {
    position -= distance * 8;
    if(position < 0)
        position = 0;
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position > scroll_limit_)
        position = scroll_limit_ > 0 ? scroll_limit_ : 0;
}

void MultiplayerSelector::render() {
    menu_back->setBackWidth(800);
    menu_back->renderBack();
    
    bool hoverable = getMouseY() > TOP_HEIGHT && getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(int i = 0; i < servers.size(); i++) {
        servers[i]->join_button.disabled = !hoverable;
        servers[i]->remove_button.disabled = !hoverable;
    }

    for(int i = 0; i < servers.size(); i++)
        servers[i]->render(position, getMouseX(), getMouseY());

    top_rect.setWidth(menu_back->getBackWidth());
    top_rect_visibility += ((position ? 1.f : 0.f) - top_rect_visibility) / 20;
    if(top_rect_visibility < 0.01f)
        top_rect_visibility = 0;
    if(top_rect_visibility > 0.99f)
        top_rect_visibility = 1;
    top_rect.fill_color.a = top_rect_visibility * TRANSPARENCY / 2;
    top_rect.blur_radius = top_rect_visibility * BLUR;
    top_rect.shadow_intensity = top_rect_visibility * SHADOW_INTENSITY;
    if(top_rect_visibility)
        top_rect.render();
    
    bottom_rect.setWidth(menu_back->getBackWidth());
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(scroll_limit_ > 0)
        bottom_rect.render();

    title.render();
    back_button.render(getMouseX(), getMouseY());
    
    new_button.x = menu_back->getBackWidth() / 2 - SPACING - new_button.w / 2;
    new_button.render(getMouseX(), getMouseY());
}

void MultiplayerSelector::stop() {
    ConfigFile config(sago::getDataHome() + "/Terralistic/servers.txt");
    std::string servers_str;
    for(int i = 0; i < server_data.size(); i++) {
        servers_str += server_data[i].name;
        servers_str.push_back(' ');
        servers_str += server_data[i].ip;
        servers_str.push_back(' ');
    }
    config.setStr("servers", servers_str);
    for(int i = 0; i < servers.size(); i++)
        delete servers[i];
}
