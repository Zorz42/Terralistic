#include <filesystem>
#include "singleplayerSelector.hpp"
#include "game.hpp"
#include "worldCreator.hpp"
#include "choiceScreen.hpp"

#define TOP_HEIGHT (title.getHeight() + 2 * SPACING)
#define BOTTOM_HEIGHT (back_button.getHeight() + 2 * SPACING)

void World::render(int position, int mouse_x, int mouse_y) {
    int render_x = gfx::getWindowWidth() / 2 - 400 + SPACING, render_y = y - position, render_width = 800 - 2 * SPACING, render_height = 116 + 2 * SPACING;
    
    gfx::Color back_color = BLACK;
    back_color.a = 100;
    gfx::RectShape(render_x, render_y, render_width, render_height).render(back_color);
    
    icon.render(1, render_x + SPACING, render_y + SPACING);
    title.render(3, render_x + 2 * SPACING + icon.getTextureWidth(), render_y + SPACING * 1.5);
    
    play_button.x = render_x + 2 * SPACING + icon.getTextureWidth();
    play_button.y = render_y + render_height - play_button.getHeight() - SPACING;
    play_button.render(mouse_x, mouse_y);
    
    delete_button.x = render_x + 3 * SPACING + icon.getTextureWidth() + play_button.getWidth();
    delete_button.y = render_y + render_height - play_button.getHeight() - SPACING;
    delete_button.render(mouse_x, mouse_y);
    
    last_played.render(2, render_x + render_width - last_played.getTextureWidth() * 2 - SPACING, render_y + render_height - last_played.getTextureHeight() * 2 - SPACING);
}

void SingleplayerSelector::init() {
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
    
    bottom_rect.orientation = gfx::BOTTOM;
    bottom_rect.setHeight(BOTTOM_HEIGHT);
    bottom_rect.fill_color.a = TRANSPARENCY / 2;
    bottom_rect.shadow_intensity = SHADOW_INTENSITY;
    bottom_rect.blur_intensity = BLUR;
    
    refresh();
}

std::string getFormattedLastTimeModified(const std::string& path) {
    std::filesystem::file_time_type last_played_time = std::filesystem::last_write_time(path);
    auto sctp = std::chrono::time_point_cast<std::chrono::system_clock::duration>(last_played_time - std::filesystem::file_time_type::clock::now() + std::chrono::system_clock::now());
    std::time_t tt = std::chrono::system_clock::to_time_t(sctp);
    
    char formatted_time[100];
    std::strftime(formatted_time, sizeof(formatted_time), "%d %B %Y %H:%M", std::localtime(&tt));
    return formatted_time;
}

void SingleplayerSelector::refresh() {
    for(int i = 0; i < worlds.size(); i++)
        delete worlds[i];
    worlds.clear();
    
    position = 0;
    scroll_limit = 0;

    for(auto& p : std::filesystem::directory_iterator((sago::getDataHome() + "/Terralistic/Worlds/").c_str())) {
        std::string file_name = p.path().filename().string();
        std::string ending = ".world";
        if(file_name.size() > ending.size() && std::equal(ending.rbegin(), ending.rend(), file_name.rbegin())) {
            file_name.erase(file_name.end() - ending.size(), file_name.end());
            worlds.push_back(new World());
            worlds.back()->name = file_name;
        }
    }

    for(int i = 0; i < worlds.size(); i++) {
        worlds[i]->y = scroll_limit + TOP_HEIGHT;
        
        worlds[i]->icon.loadFromResources("world_icon.png");
        
        worlds[i]->title.loadFromText(worlds[i]->name);

        worlds[i]->play_button.loadFromResources("play_button.png");
        worlds[i]->play_button.scale = 3;
        worlds[i]->play_button.margin = 5;
        
        worlds[i]->delete_button.loadFromResources("delete_button.png");
        worlds[i]->delete_button.scale = 3;
        worlds[i]->delete_button.margin = 5;

        worlds[i]->last_played.setColor(GREY);
        
        worlds[i]->last_played.loadFromText("Last played: " + getFormattedLastTimeModified(sago::getDataHome() + "/Terralistic/Worlds/" + worlds[i]->name + ".world"));
        
        scroll_limit += 116 + SPACING * 3;
    }
}

bool SingleplayerSelector::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        else if(new_button.isHovered(getMouseX(), getMouseY())) {
            std::vector<std::string> worlds_names;
            for(int i = 0; i < worlds.size(); i++)
                worlds_names.push_back(worlds[i]->name);
            WorldCreator world_creator(worlds_names, menu_back, settings);
            switchToScene(world_creator);
            refresh();
        }
        else
            for(int i = 0; i < worlds.size(); i++) {
                if(worlds[i]->play_button.isHovered(getMouseX(), getMouseY())) {
                    startPrivateWorld(sago::getDataHome() + "/Terralistic/Worlds/" + worlds[i]->name + ".world", menu_back, settings, false);
                    refresh();
                }
                else if(worlds[i]->delete_button.isHovered(getMouseX(), getMouseY())) {
                    std::string result;
                    if(getKeyState(gfx::Key::SHIFT))
                        result = "Yes";
                    else {
                        ChoiceScreen choice_screen(menu_back, "Do you want to delete " + worlds[i]->name + "?", {"Yes", "No"}, &result);
                        switchToScene(choice_screen);
                    }

                    if(result == "Yes") {
                        std::filesystem::remove(sago::getDataHome() + "/Terralistic/Worlds/" + worlds[i]->name + ".world");
                        refresh();
                    }
                    break;
                }
            }
        return true;
    }
    return false;
}

void SingleplayerSelector::onMouseScroll(int distance) {
    position -= distance * 8;
    if(position < 0)
        position = 0;
    int scroll_limit_ = scroll_limit - gfx::getWindowHeight() + TOP_HEIGHT + BOTTOM_HEIGHT;
    if(position > scroll_limit_)
        position = scroll_limit_ > 0 ? scroll_limit_ : 0;
}

void SingleplayerSelector::render() {
    menu_back->setBackWidth(800);
    menu_back->renderBack();
    
    bool hoverable = getMouseY() > TOP_HEIGHT && getMouseY() < gfx::getWindowHeight() - BOTTOM_HEIGHT;

    for(int i = 0; i < worlds.size(); i++) {
        worlds[i]->play_button.disabled = !hoverable;
        worlds[i]->delete_button.disabled = !hoverable;
    }

    for(int i = 0; i < worlds.size(); i++)
        worlds[i]->render(position, getMouseX(), getMouseY());

    top_rect.setWidth(menu_back->getBackWidth());
    top_rect_visibility += ((position ? 1.f : 0.f) - top_rect_visibility) / 20;
    if(top_rect_visibility < 0.01f)
        top_rect_visibility = 0;
    if(top_rect_visibility > 0.99f)
        top_rect_visibility = 1;
    top_rect.fill_color.a = top_rect_visibility * TRANSPARENCY / 2;
    top_rect.blur_intensity = top_rect_visibility * BLUR;
    top_rect.shadow_intensity = top_rect_visibility * SHADOW_INTENSITY;
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
