#include <algorithm>
#include <filesystem>
#include "worldSelector.hpp"
#include "game.hpp"
#include "worldCreator.hpp"
#include "fileManager.hpp"
#include "choiceScreen.hpp"

#define TOP_HEIGHT (title.getHeight() + SPACING)
#define BOTTOM_HEIGHT (back_button.getHeight() + SPACING)
#define LINE_THICKNESS 2

void WorldToSelect::render(int position_) {
    button.y = short(button_y - position_);
    button.render();
    delete_button.y = short(button_y - position_ + (button.getTranslatedRect().h - delete_button.getTranslatedRect().h) / 2);
    delete_button.render();
}

void WorldSelector::init() {
    title.scale = 3;
    title.renderText("Select a world to play!");
    title.y = SPACING / 2;
    title.orientation = gfx::TOP;

    back_button.scale = 3;
    back_button.renderText("Back");
    back_button.y = -SPACING / 2;
    back_button.orientation = gfx::BOTTOM;

    new_button.scale = 3;
    new_button.renderText("New");
    new_button.y = -SPACING / 2;
    new_button.orientation = gfx::BOTTOM_RIGHT;
    
    top_rect.orientation = gfx::TOP;
    top_rect.h = TOP_HEIGHT;
    top_rect.c = BLACK;
    top_rect.c.a = TRANSPARENCY / 3;
    top_rect.shadow_intensity = SHADOW_INTENSITY / 2;
    top_rect.blur_intensity = BLUR - 2;
    
    bottom_rect.orientation = gfx::BOTTOM;
    bottom_rect.h = BOTTOM_HEIGHT;
    bottom_rect.c = BLACK;
    bottom_rect.c.a = TRANSPARENCY / 3;
    bottom_rect.shadow_intensity = SHADOW_INTENSITY / 2;
    bottom_rect.blur_intensity = BLUR - 2;
    
    refresh();
}

void WorldSelector::refresh() {
    position = 0;
    scroll_limit = 0;

    worlds.clear();
    worlds_names.clear();

    std::vector<std::string> ignored_dirs = {
            ".",
            "..",
            ".DS_Store",
    };

    for(auto& p: std::filesystem::directory_iterator(fileManager::getWorldsPath().c_str()))
        if(p.is_directory())
            worlds.emplace_back(p.path().filename().string());

    for(auto& world : worlds) {
        world.button.orientation = gfx::TOP;
        world.button.scale = 3;
        world.button.renderText(world.name);
        world.button_y = scroll_limit + TOP_HEIGHT;

        world.delete_button.orientation = gfx::TOP;
        world.delete_button.loadFromFile("x-button.png");
        world.delete_button.scale = 3;
        world.delete_button.x = short(world.button.getTranslatedRect().w / 2 + world.delete_button.getTranslatedRect().w / 2 + SPACING);

        scroll_limit += world.button.getTranslatedRect().h + SPACING;

        worlds_names.push_back(world.name);
    }
}

void WorldSelector::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered())
            gfx::returnFromScene();
        else if(new_button.isHovered()) {
            WorldCreator(worlds_names, menu_back).run();
            refresh();
        }
        else
            for(int i = 0; i < worlds.size(); i++) {
                if(worlds[i].button.isHovered()) {
                    startPrivateWorld(worlds[i].name);
                    refresh();
                }
                else if(worlds[i].delete_button.isHovered()) {
                    std::string result;
                    if(shift_pressed)
                        result = "Yes";
                    else
                        ChoiceScreen(std::string("Do you want to delete ") + worlds[i].name + "?", {"Yes", "No"}, &result).run();

                    if(result == "Yes")
                        std::filesystem::remove_all(fileManager::getWorldsPath() + worlds[i].name);
                    
                    refresh();
                    break;
                }
            }
    } else if(key == gfx::Key::SHIFT)
        shift_pressed = true;
}

void WorldSelector::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::SHIFT)
        shift_pressed = false;
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
    menu_back->setWidth(gfx::getWindowWidth() / 5 * 4);
    menu_back->render();
    
    bool hoverable = gfx::getMouseY() > TOP_HEIGHT && gfx::getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(WorldToSelect& world : worlds) {
        world.button.disabled = !hoverable;
        world.delete_button.disabled = !hoverable;
    }

    for(WorldToSelect& world : worlds)
        world.render(position);

    top_rect.w = menu_back->getWidth();
    top_rect_visibility += ((position ? 1.f : 0.f) - top_rect_visibility) / 20;
    if(top_rect_visibility < 0.01f)
        top_rect_visibility = 0;
    if(top_rect_visibility > 0.99f)
        top_rect_visibility = 1;
    top_rect.c.a = top_rect_visibility * TRANSPARENCY / 2;
    top_rect.blur_intensity = top_rect_visibility * (BLUR - 1);
    top_rect.shadow_intensity = top_rect_visibility * SHADOW_INTENSITY / 2;
    if(top_rect_visibility)
        top_rect.render();
    
    bottom_rect.w = menu_back->getWidth();
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(scroll_limit_ > 0)
        bottom_rect.render();

    title.render();
    back_button.render();
    
    new_button.x = -SPACING / 2 - gfx::getWindowWidth() / 10;
    new_button.render();
}
