#include <algorithm>
#include <filesystem>
#include "worldSelector.hpp"
#include "game.hpp"
#include "worldCreator.hpp"
#include "fileManager.hpp"
#include "choiceScreen.hpp"

#define TOP_HEIGHT (title.getHeight() + 2 * SPACING)
#define BOTTOM_HEIGHT (back_button.getHeight() + 2 * SPACING)
#define LINE_THICKNESS 2

void WorldSelector::world_to_select::render(int position_) {
    button.y = short(button_y - position_);
    button.render();
    delete_button.y = short(button_y - position_ + (button.getTranslatedRect().h - delete_button.getTranslatedRect().h) / 2);
    delete_button.render();
}

void WorldSelector::init() {
    title.scale = 3;
    title.renderText("Select a world to play!");
    title.y = SPACING;
    title.orientation = gfx::TOP;

    back_button.scale = 3;
    back_button.renderText("Back");
    back_button.y = -SPACING;
    back_button.orientation = gfx::BOTTOM;

    new_button.scale = 3;
    new_button.renderText("New");
    new_button.y = -SPACING;
    new_button.x = -SPACING;
    new_button.orientation = gfx::BOTTOM_RIGHT;

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
        world.button_y = scroll_limit + title.getTranslatedRect().h + 2 * SPACING;

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
            WorldCreator(worlds_names).run();
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
    bool hoverable = gfx::getMouseY() > TOP_HEIGHT && gfx::getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(world_to_select& world : worlds) {
        world.button.disabled = !hoverable;
        world.delete_button.disabled = !hoverable;
    }

    for(world_to_select& world : worlds)
        world.render(position);

    gfx::Rect(0, 0, gfx::getWindowWidth(), TOP_HEIGHT, BLACK).render();
    gfx::Rect(0, 0, gfx::getWindowWidth(), BOTTOM_HEIGHT, BLACK, gfx::BOTTOM_LEFT).render();
    if(position != 0)
        gfx::Rect(0, TOP_HEIGHT, gfx::getWindowWidth(), LINE_THICKNESS, GREY).render();
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position != scroll_limit_ && scroll_limit_ > 0)
        gfx::Rect(0, -BOTTOM_HEIGHT, gfx::getWindowWidth(), LINE_THICKNESS, GREY, gfx::BOTTOM_LEFT).render();

    title.render();
    back_button.render();
    new_button.render();
}
