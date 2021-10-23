#include <algorithm>
#include <filesystem>
#include "worldSelector.hpp"
#include "game.hpp"
#include "worldCreator.hpp"
#include "platform_folders.h"
#include "choiceScreen.hpp"

#define TOP_HEIGHT (title.getHeight() + 2 * SPACING)
#define BOTTOM_HEIGHT (back_button.getHeight() + 2 * SPACING)

void WorldToSelect::render(int position, unsigned short mouse_x, unsigned short mouse_y) {
    button.y = short(button_y - position);
    button.render(mouse_x, mouse_y);
    delete_button.y = short(button_y - position + (button.getTranslatedRect().h - delete_button.getTranslatedRect().h) / 2);
    delete_button.render(mouse_x, mouse_y);
}

void WorldSelector::init() {
    std::filesystem::create_directory(sago::getDataHome() + "/Terralistic/Worlds/");
    
    title.scale = 3;
    title.loadFromText("Select a world to play!");
    title.y = SPACING;
    title.orientation = gfx::TOP;

    back_button.scale = 3;
    back_button.loadFromText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;

    new_button.scale = 3;
    new_button.loadFromText("New");
    new_button.y = -SPACING;
    new_button.orientation = gfx::BOTTOM;
    
    top_rect.orientation = gfx::TOP;
    top_rect.setHeight(TOP_HEIGHT);
    top_rect.fill_color.a = TRANSPARENCY / 3;
    top_rect.shadow_intensity = SHADOW_INTENSITY / 2;
    top_rect.blur_intensity = BLUR - 2;
    
    bottom_rect.orientation = gfx::BOTTOM;
    bottom_rect.setHeight(BOTTOM_HEIGHT);
    bottom_rect.fill_color.a = TRANSPARENCY / 3;
    bottom_rect.shadow_intensity = SHADOW_INTENSITY / 2;
    bottom_rect.blur_intensity = BLUR - 2;
    
    refresh();
}

void WorldSelector::refresh() {
    position = 0;
    scroll_limit = 0;

    worlds.clear();
    worlds_names.clear();

    for(auto& p: std::filesystem::directory_iterator((sago::getDataHome() + "/Terralistic/Worlds/").c_str())) {
        std::string file_name = p.path().filename().string();
        std::string ending = ".world";
        if(file_name.size() > ending.size() && std::equal(ending.rbegin(), ending.rend(), file_name.rbegin())) {
            file_name.erase(file_name.end() - ending.size(), file_name.end());
            worlds.emplace_back(file_name);
        }
    }

    for(auto& world : worlds) {
        world.button.orientation = gfx::TOP;
        world.button.scale = 3;
        world.button.loadFromText(world.name);
        world.button_y = scroll_limit + TOP_HEIGHT;

        world.delete_button.orientation = gfx::TOP;
        world.delete_button.loadFromResources("x_button.png");
        world.delete_button.scale = 3;
        world.delete_button.x = short(world.button.getTranslatedRect().w / 2 + world.delete_button.getTranslatedRect().w / 2 + SPACING);

        scroll_limit += world.button.getTranslatedRect().h + SPACING * 2;

        worlds_names.push_back(world.name);
    }
}

bool WorldSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        else if(new_button.isHovered(getMouseX(), getMouseY())) {
            WorldCreator world_creator(worlds_names, menu_back);
            switchToScene(world_creator);
            refresh();
        }
        else
            for(auto & world : worlds) {
                if(world.button.isHovered(getMouseX(), getMouseY())) {
                    startPrivateWorld(sago::getDataHome() + "/Terralistic/Worlds/" + world.name + ".world", menu_back, false);
                    refresh();
                }
                else if(world.delete_button.isHovered(getMouseX(), getMouseY())) {
                    std::string result;
                    if(getKeyState(gfx::Key::SHIFT))
                        result = "Yes";
                    else {
                        ChoiceScreen choice_screen(menu_back, std::string("Do you want to delete ") + world.name + "?", {"Yes", "No"}, &result);
                        switchToScene(choice_screen);
                    }

                    if(result == "Yes") {
                        std::filesystem::remove(sago::getDataHome() + "/Terralistic/Worlds/" + world.name + ".world");
                        refresh();
                    }
                    break;
                }
            }
        return true;
    }
    return false;
}

void WorldSelector::onMouseScroll(int distance) {
    position -= distance * 8;
    if(position < 0)
        position = 0;
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position > scroll_limit_)
        position = scroll_limit_ > 0 ? scroll_limit_ : 0;
}

void WorldSelector::render() {
    menu_back->setBackWidth(800);
    menu_back->renderBack();
    
    bool hoverable = getMouseY() > TOP_HEIGHT && getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(WorldToSelect& world : worlds) {
        world.button.disabled = !hoverable;
        world.delete_button.disabled = !hoverable;
    }

    for(WorldToSelect& world : worlds)
        world.render(position, getMouseX(), getMouseY());

    top_rect.setWidth(menu_back->getBackWidth());
    top_rect_visibility += ((position ? 1.f : 0.f) - top_rect_visibility) / 20;
    if(top_rect_visibility < 0.01f)
        top_rect_visibility = 0;
    if(top_rect_visibility > 0.99f)
        top_rect_visibility = 1;
    top_rect.fill_color.a = top_rect_visibility * TRANSPARENCY / 2;
    top_rect.blur_intensity = top_rect_visibility * (BLUR - 1);
    top_rect.shadow_intensity = top_rect_visibility * SHADOW_INTENSITY / 2;
    if(top_rect_visibility)
        top_rect.render();
    
    bottom_rect.setWidth(menu_back->getBackWidth());
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(scroll_limit_ > 0)
        bottom_rect.render();

    title.render();
    back_button.render(getMouseX(), getMouseY());
    
    new_button.x = menu_back->getBackWidth() / 2 - SPACING - new_button.getWidth() / 2;
    new_button.render(getMouseX(), getMouseY());
}
