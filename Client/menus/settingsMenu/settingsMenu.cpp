#include "settingsMenu.hpp"

void SettingsMenu::init() {
    for(auto i : settings->getSettings()) {
        switch(i->type) {
            case SettingType::CHOICE_SETTING: {
                ChoiceSetting* choice_setting = (ChoiceSetting*)i;
                RenderChoiceSetting* render_choice_setting = new RenderChoiceSetting(choice_setting);
                render_settings.push_back(render_choice_setting);
                break;
            }
            case SettingType::BOOLEAN_SETTING: {
                BooleanSetting* boolean_setting = (BooleanSetting*)i;
                RenderBooleanSetting* render_boolean_setting = new RenderBooleanSetting(boolean_setting);
                render_settings.push_back(render_boolean_setting);
                break;
            }
            case SettingType::SLIDER_SETTING: {
                SliderSetting* slider_setting = (SliderSetting*)i;
                RenderSliderSetting* render_slide_setting = new RenderSliderSetting(slider_setting);
                render_settings.push_back(render_slide_setting);
                break;
            }
        }
        
        required_width = std::max(render_settings.back()->getWidth(), required_width);
    }
    
    back_button.loadFromSurface(gfx::textToSurface("Back"));
    back_button.setScale(3);
    back_button.orientation = gfx::BOTTOM;
    back_button.y = -SPACING;
}

void SettingsMenu::stop() {
    for(auto & render_setting : render_settings)
        delete render_setting;
}

bool SettingsMenu::onKeyUp(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        if(back_button.isHovered(getMouseX(), getMouseY()))
            returnFromScene();
        for(auto & render_setting : render_settings)
            render_setting->onMouseButtonUp(getMouseX(), getMouseY());
        return true;
    }
    return false;
}

bool SettingsMenu::onKeyDown(gfx::Key key) {
    if(key == gfx::Key::MOUSE_LEFT) {
        for(auto & render_setting : render_settings)
            render_setting->onMouseButtonDown(getMouseX(), getMouseY());
        return true;
    }
    return false;
}

void SettingsMenu::render() {
    background->setBackWidth(required_width + 6 * SPACING);
    background->renderBack();
    
    int y = 0;
    for(auto & render_setting : render_settings) {
        int width = required_width, height = render_setting->getHeight();
        y += SPACING;
        
        gfx::Color c = BLACK;
        c.a = TRANSPARENCY;
        gfx::RectShape(gfx::getWindowWidth() / 2 - width / 2, y, width, height).render(c);
        
        render_setting->render(y, required_width, getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
        
        y += height;
    }
    
    
    back_button.render(getMouseX(), getMouseY(), getKeyState(gfx::Key::MOUSE_LEFT));
}

RenderChoiceSetting::RenderChoiceSetting(ChoiceSetting* setting) : setting(setting) {
    choice_text.loadFromSurface(gfx::textToSurface(setting->ident));
    choice_text.setScale(3);
    choice_text.orientation = gfx::TOP;
    select_rect.fill_color = DARK_GREY;
    select_rect.smooth_factor = 2;
    select_rect.orientation = gfx::TOP;
    for(const auto & choice : setting->choices) {
        gfx::Button* button = new gfx::Button;
        button->loadFromSurface(gfx::textToSurface(choice));
        button->setScale(2);
        button->orientation = gfx::TOP;
        button->setMargin(5);
        choice_buttons.push_back(button);
    }
}

void RenderChoiceSetting::render(int y, int width, int mouse_x, int mouse_y, bool is_mouse_pressed) {
    int x = width / 2 - SPACING;
    for(int i = (int)choice_buttons.size() - 1; i >= 0; i--) {
        gfx::Button* button = choice_buttons[i];
        x -= button->getWidth() / 2;
        button->x = x;
        x -= button->getWidth() / 2 + 10;
        button->y = y + getHeight() / 2 - button->getHeight() / 2;
    }
    
    gfx::Button* selected_button = choice_buttons[setting->getSelectedChoice()];
    select_rect.x = selected_button->x;
    select_rect.y = selected_button->y;
    select_rect.w = selected_button->getWidth();
    select_rect.h = selected_button->getHeight();
    select_rect.render();
    
    choice_text.y = y + getHeight() / 2 - choice_text.getHeight() / 2;
    choice_text.x = -width / 2 + choice_text.getWidth() / 2 + SPACING;
    
    choice_text.render();
    
    for(int i = 0; i < (int)choice_buttons.size(); i++)
        choice_buttons[i]->render(mouse_x, mouse_y, is_mouse_pressed);
}

int RenderChoiceSetting::getHeight() {
    int height = 0;
    height = std::max(height, choice_text.getHeight() + 2 * SPACING);
    return height;
}

int RenderChoiceSetting::getWidth() {
    int width = choice_text.getWidth() + 3 * SPACING;
    for(auto & choice_button : choice_buttons)
        width += choice_button->getWidth() + 10;
    return width;
}

void RenderChoiceSetting::onMouseButtonUp(int x, int y) {
    for(int i = 0; i < choice_buttons.size(); i++)
        if(choice_buttons[i]->isHovered(x, y))
            setting->setSelectedChoice(i);
}

RenderBooleanSetting::RenderBooleanSetting(BooleanSetting* setting) : setting(setting) {
    text.loadFromSurface(gfx::textToSurface(setting->ident));
    text.setScale(3);
    text.orientation = gfx::TOP;
    
    toggle_button.setScale(2);
    toggle_button.orientation = gfx::TOP;
    toggle_button.setMargin(5);
    
    updateButtonText();
}

void RenderBooleanSetting::render(int y, int width, int mouse_x, int mouse_y, bool is_mouse_pressed) {
    text.y = y + getHeight() / 2 - text.getHeight() / 2;
    text.x = -width / 2 + text.getWidth() / 2 + SPACING;
    
    toggle_button.y = y + getHeight() / 2 - toggle_button.getHeight() / 2;
    toggle_button.x = width / 2 - toggle_button.getWidth() / 2 - SPACING;
    toggle_button.render(mouse_x, mouse_y, is_mouse_pressed);
    
    text.render();
}

void RenderBooleanSetting::updateButtonText() {
    if(setting->getValue()) {
        toggle_button.def_color = {44, 159, 44};
        toggle_button.hover_color = {44, 199, 44};
        toggle_button.loadFromSurface(gfx::textToSurface("On"));
    } else {
        toggle_button.def_color = GFX_DEFAULT_BUTTON_COLOR;
        toggle_button.hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR;
        toggle_button.loadFromSurface(gfx::textToSurface("Off"));
    }
}

int RenderBooleanSetting::getHeight() {
    int height = 0;
    height = std::max(height, text.getHeight() + 2 * SPACING);
    height = std::max(height, toggle_button.getHeight() + 2 * SPACING);
    return height;
}

int RenderBooleanSetting::getWidth() {
    int width = text.getWidth() + 3 * SPACING;
    return width;
}

void RenderBooleanSetting::onMouseButtonUp(int x, int y) {
    if(toggle_button.isHovered(x, y)) {
        setting->setValue(!setting->getValue());
        updateButtonText();
    }
}

RenderSliderSetting::RenderSliderSetting(SliderSetting* setting) : setting(setting) {
    choice_text.loadFromSurface(gfx::textToSurface(setting->ident));
    choice_text.setScale(3);
    choice_text.orientation = gfx::TOP;
    
    select_rect.fill_color = DARK_GREY;
    select_rect.smooth_factor = 2;
    select_rect.orientation = gfx::TOP;
    
    for(const auto & choice : setting->choices) {
        gfx::Button* button = new gfx::Button;
        button->loadFromSurface(gfx::textToSurface(choice));
        button->setScale(2);
        button->orientation = gfx::TOP;
        button->setMargin(5);
        choice_buttons.push_back(button);
    }
    
    gfx::Button dummy_button;
    dummy_button.loadFromSurface(gfx::textToSurface("dummy"));
    dummy_button.setScale(2);
    dummy_button.setMargin(5);
    
    slider_rect.w = ((setting->max - setting->min) / setting->step + 1) * 10;
    slider_rect.h = dummy_button.getHeight();
    slider_rect.orientation = gfx::TOP;
    
    slider_text.setScale(2);
    slider_text.orientation = gfx::TOP;
    slider_text.setColor(WHITE);
    
    updateSliderText();
}

void RenderSliderSetting::render(int y, int width, int mouse_x, int mouse_y, bool is_mouse_pressed) {
    int x = width / 2 - SPACING;
    
    slider_rect.x = x - slider_rect.w / 2;
    slider_rect.y = y + getHeight() / 2 - slider_rect.h / 2;
    x -= slider_rect.w + 10;
    
    for(int i = (int)choice_buttons.size() - 1; i >= 0; i--) {
        gfx::Button* button = choice_buttons[i];
        x -= button->getWidth() / 2;
        button->x = x;
        x -= button->getWidth() / 2 + 10;
        button->y = y + getHeight() / 2 - button->getHeight() / 2;
    }
    
    slider_hovered = slider_rect.getTranslatedRect().x < mouse_x && slider_rect.getTranslatedRect().x + slider_rect.w > mouse_x && slider_rect.getTranslatedRect().y < mouse_y && slider_rect.getTranslatedRect().y + slider_rect.h > mouse_y;
    
    if(setting->getSelectedChoice() < choice_buttons.size()) {
        gfx::Button* selected_button = choice_buttons[setting->getSelectedChoice()];
        select_rect.x = selected_button->x;
        select_rect.y = selected_button->y;
        select_rect.w = selected_button->getWidth();
        select_rect.h = selected_button->getHeight();
        slider_rect.border_color = TRANSPARENT;
    } else {
        select_rect.w = 10;
        select_rect.h = slider_rect.h;
        select_rect.y = slider_rect.y;
        select_rect.x = slider_rect.x - slider_rect.w / 2 + 10 * (setting->getSelectedChoice() - (int)choice_buttons.size()) + select_rect.w / 2;
        slider_rect.border_color = DARK_GREY;
    }
    
    slider_rect.fill_color = slider_hovered ? GREY : BLACK;
    slider_rect.render();
    
    if(holding_slider) {
        int selected_choice = (mouse_x - slider_rect.getTranslatedRect().x) / 10 + (int)choice_buttons.size();
        selected_choice = std::max(selected_choice, (int)choice_buttons.size());
        selected_choice = std::min(selected_choice, (setting->max - setting->min) / setting->step + (int)choice_buttons.size());
        if(setting->getSelectedChoice() != selected_choice) {
            setting->setSelectedChoice(selected_choice);
            updateSliderText();
        }
    }
    
    select_rect.render();
    
    choice_text.y = y + getHeight() / 2 - choice_text.getHeight() / 2;
    choice_text.x = -width / 2 + choice_text.getWidth() / 2 + SPACING;
    
    choice_text.render();
    
    for(int i = 0; i < (int)choice_buttons.size(); i++)
        choice_buttons[i]->render(mouse_x, mouse_y, is_mouse_pressed);
    
    slider_text.y = slider_rect.y + slider_rect.h / 2 - slider_text.getHeight() / 2;
    slider_text.x = slider_rect.x;
    slider_text.render();
}

int RenderSliderSetting::getHeight() {
    int height = 0;
    height = std::max(height, choice_text.getHeight() + 2 * SPACING);
    return height;
}

int RenderSliderSetting::getWidth() {
    int width = choice_text.getWidth() + 3 * SPACING + slider_rect.w + 10;
    for(auto & choice_button : choice_buttons)
        width += choice_button->getWidth() + 10;
    return width;
}

void RenderSliderSetting::onMouseButtonUp(int x, int y) {
    holding_slider = false;
    for(int i = 0; i < choice_buttons.size(); i++)
        if(choice_buttons[i]->isHovered(x, y)) {
            setting->setSelectedChoice(i);
            updateSliderText();
        }
}

void RenderSliderSetting::onMouseButtonDown(int x, int y) {
    if(slider_hovered)
        holding_slider = true;
    
    for(int i = 0; i < choice_buttons.size(); i++)
        if(choice_buttons[i]->isHovered(x, y))
            setting->setSelectedChoice(i);
}

void RenderSliderSetting::updateSliderText() {
    if(setting->getSelectedChoice() < choice_buttons.size())
        slider_text.loadFromSurface(gfx::textToSurface(setting->default_slider_text));
    else
        slider_text.loadFromSurface(gfx::textToSurface(std::to_string(setting->getSliderValue()) + " " + setting->slider_text));
}
