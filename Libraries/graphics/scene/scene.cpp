#include "glfwAbstraction.hpp"
#include "scene.hpp"
#include "button.hpp"
#include <iostream>
#include <algorithm>
#include <iomanip>

static gfx::Scene* curr_scene = nullptr;
static bool key_states[(int)gfx::Key::UNKNOWN];
static bool absolute_key_states[(int)gfx::Key::UNKNOWN];

void gfx::addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function) {
    global_update_functions.push_back(global_update_function);
}

void gfx::Scene::initialize() {
    if(!initialized) {
        for(auto & module : modules)
            module->init();
        modules.insert(modules.begin(), this);
        init();
        initialized = true;
    }
}

void gfx::Scene::registerAModule(SceneModule* module) {
    modules.push_back(module);
}

const std::vector<gfx::SceneModule*>& gfx::Scene::getModules() {
    return modules;
}

int gfx::SceneModule::getMouseX() {
    return mouse_x;
}

int gfx::SceneModule::getMouseY() {
    return mouse_y;
}

int gfx::SceneModule::getMouseVel() {
    return mouse_vel;
}

bool gfx::SceneModule::getKeyState(Key key_) const {
    return enable_key_states && key_states[(int)key_];
}

bool gfx::SceneModule::getAbsoluteKeyState(Key key_) const {
    return enable_key_states && absolute_key_states[(int)key_];
}

void gfx::Scene::onEvent(_KeyPressEvent& event) {
    if(curr_scene != this)
        return;
    
    bool clicked_text_box = false;
    if(event.key == Key::MOUSE_LEFT) {
        for(SceneModule* module : modules)
            if(module->enabled)
                for (TextInput* i : module->text_inputs) {
                    i->active = i->isHovered(getMouseX(), getMouseY(), getMouseVel());
                    if (i->active)
                        clicked_text_box = true;
                }
    }
    
    if(event.key == Key::BACKSPACE) {
        for (SceneModule* module : modules)
            for (TextInput* i : module->text_inputs)
                if (i->active && !i->getText().empty()) {
                    std::string str = i->getText();
                    if(i->getCursorBegin() != i->getCursorEnd())
                        i->eraseSelected();
                    else if(i->getCursorBegin() != 0) {
                        str.erase(i->getCursorBegin() - 1, 1);
                        i->setCursor(i->getCursorBegin() - 1);
                        i->setText(str);
                    }
                }
    }
    
    if(event.key == Key::A && getAbsoluteKeyState(Key::CTRL)) {
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active)
                        text_input->setCursor(0, (int)text_input->getText().size());
    }
    
    if(event.key == Key::C && getAbsoluteKeyState(Key::CTRL)) {
         for(auto & module : modules)
             if(module->enabled)
                 for(auto & text_input : module->text_inputs)
                     if(text_input->active)
                         setClipboard(text_input->getText().substr(text_input->getCursorBegin(), text_input->getCursorEnd() - text_input->getCursorBegin()));
     }

     if(event.key == Key::X && getAbsoluteKeyState(Key::CTRL)) {
         for(auto & module : modules)
             if(module->enabled)
                 for(auto & text_input : module->text_inputs)
                     if(text_input->active) {
                         setClipboard(text_input->getText().substr(text_input->getCursorBegin(), text_input->getCursorEnd() - text_input->getCursorBegin()));
                         text_input->eraseSelected();
                     }
     }

    if(event.key == Key::V && getAbsoluteKeyState(Key::CTRL)) {
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active) {
                        text_input->eraseSelected();
                        std::string temp_str = text_input->getText();
                        std::string clipboard_str = getClipboard();
                        for(auto letter : clipboard_str) {
                            char result = text_input->textProcessing(letter, (int)text_input->getText().size());
                            if(result != '\0') {
                                temp_str.insert(text_input->getCursorBegin(), 1, result);
                                text_input->setCursor(text_input->getCursorBegin() + 1);
                            }
                        }
                        text_input->setText(temp_str);
                    }
    }

    if(event.key == Key::ARROW_LEFT) {
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active) {
                        if(getAbsoluteKeyState(Key::SHIFT)) {
                            if(text_input->getCursorBegin() == text_input->getCursorEnd()) {
                                text_input->setCursor(text_input->findLeftMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)), text_input->getCursorEnd());
                                text_input->setCursorEndActive(false);
                            } else {
                                if(text_input->getCursorEndActive())
                                    text_input->setCursor(text_input->getCursorBegin(), text_input->findLeftMove(text_input->getCursorEnd(), getKeyState(gfx::Key::CTRL)));
                                else
                                    text_input->setCursor(text_input->findLeftMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)), text_input->getCursorEnd());
                            }
                        } else {
                            if(text_input->getCursorBegin() == text_input->getCursorEnd())
                                text_input->setCursor(text_input->findLeftMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)));
                            else
                                text_input->setCursor(text_input->getCursorBegin());
                        }
                    }
    }

    if(event.key == Key::ARROW_RIGHT) {
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active) {
                        if(getAbsoluteKeyState(Key::SHIFT)) {
                            if(text_input->getCursorBegin() == text_input->getCursorEnd()) {
                                text_input->setCursor(text_input->getCursorBegin(), text_input->findRightMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)));
                                text_input->setCursorEndActive(true);
                            } else {
                                if(text_input->getCursorEndActive())
                                    text_input->setCursor(text_input->getCursorBegin(), text_input->findRightMove(text_input->getCursorEnd(), getKeyState(gfx::Key::CTRL)));
                                else
                                    text_input->setCursor(text_input->findRightMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)), text_input->getCursorEnd());
                            }
                        } else {
                            if (text_input->getCursorBegin() == text_input->getCursorEnd())
                                text_input->setCursor(text_input->findRightMove(text_input->getCursorBegin(), getKeyState(gfx::Key::CTRL)));
                            else
                                text_input->setCursor(text_input->getCursorEnd());
                        }
                    }
    }
    
    bool is_textbox_active = false;
    for(auto & module : modules)
        if(module->enabled)
            for(auto & text_input : module->text_inputs)
                if(text_input->active) {
                    is_textbox_active = true;
                    std::vector<Key> input_passthrough_keys = text_input->getPassthroughKeys();
                    if(std::count(input_passthrough_keys.begin(), input_passthrough_keys.end(), event.key))
                        module->onKeyDown(event.key);
                }
    
    if(event.key != Key::UNKNOWN && !clicked_text_box) {
        if(!absolute_key_states[(int)event.key])
            absolute_key_states[(int)event.key] = true;

        if(!is_textbox_active || event.key == Key::ENTER)
            if(!key_states[(int)event.key]) {
                key_states[(int)event.key] = true;
                for(int i = (int)modules.size() - 1; i >= 0; i--)
                    if(modules[i]->enabled && modules[i]->onKeyDown(event.key))
                        break;
            }
    }
    
    if(is_textbox_active && event.key == Key::ESCAPE) {
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    text_input->active = false;
    }
}

void gfx::Scene::onEvent(_KeyReleaseEvent& event) {
    if(curr_scene != this)
        return;
    
    if(event.key != Key::UNKNOWN) {
        if(absolute_key_states[(int)event.key])
            absolute_key_states[(int)event.key] = false;

        if(key_states[(int)event.key]) {
            key_states[(int)event.key] = false;
            for(int i = (int)modules.size() - 1; i >= 0; i--)
                if(modules[i]->enabled && modules[i]->onKeyUp(event.key))
                    break;
        }
    }
}

void gfx::Scene::onEvent(_CharInputEvent& event) {
    if(curr_scene != this)
        return;
    
    if(event.code == '\b')
        return;

    for(SceneModule* module : modules)
        if(module->enabled)
            for (TextInput* i : module->text_inputs)
                if(i->active) {
                    char result = event.code;
                    if(!i->ignore_next_input) {
                        if(i->textProcessing)
                            result = i->textProcessing(result, (int)i->getText().size());
                        if(result) {
                            i->eraseSelected();
                            std::string new_text = i->getText();
                            new_text.insert(new_text.begin() + i->getCursorBegin(), result);
                            i->setCursor(i->getCursorBegin() + 1, i->getCursorBegin() + 1);
                            i->setText(new_text);
                         }
                    }
                    i->ignore_next_input = false;
                }
}

void gfx::Scene::onEvent(_ScrollEvent& event) {
    if(curr_scene != this)
        return;
    
    for(auto module : modules)
        module->onMouseScroll(event.delta);
}

void gfx::Scene::run() {
    curr_scene = this;
    screen_refresh_event_sender.addListener(this);
    key_press_event_sender.addListener(this);
    key_release_event_sender.addListener(this);
    scroll_event_sender.addListener(this);
    char_input_event_sender.addListener(this);
    preInit();
    initialize();
    
    while(running) {
        curr_scene = this;
        
        renderAll();
        
        if(fps_limit || !is_window_focused) {
            float ms_per_frame = is_window_focused ? 1000.f / fps_limit : 1000;
            if(frame_length < ms_per_frame)
                gfx::sleep(ms_per_frame - frame_length);
        }
            
        glfwPollEvents();
        
        if(isWindowClosed())
            running = false;
    }

    if(initialized) {
        stop();
        for(int i = 0; i < modules.size(); i++)
            if(modules[i] != this)
                modules[i]->stop();
    }
    
    screen_refresh_event_sender.removeListener(this);
    key_press_event_sender.removeListener(this);
    key_release_event_sender.removeListener(this);
    scroll_event_sender.removeListener(this);
    char_input_event_sender.removeListener(this);
}

void gfx::Scene::onEvent(_ScreenRefreshEvent& event) {
    if(this == curr_scene)
        renderAll();
}

void gfx::Scene::renderAll() {
    Timer frame_timer;
   
    mouse_vel = std::min((std::abs(mouse_x - gfx::getMouseX()) + std::abs(mouse_y - gfx::getMouseY())) / frame_length * 1000, 10000.f);
    mouse_x = gfx::getMouseX();
    mouse_y = gfx::getMouseY();
    for(int i = 0; i < modules.size(); i++)
        if(modules[i]->enabled) {
            modules[i]->mouse_x = mouse_x;
            modules[i]->mouse_y = mouse_y;
            modules[i]->mouse_vel = mouse_vel;
        }
    
    cycleModules();
    
    for(int i = 0; i < global_update_functions.size(); i++)
        global_update_functions[i]->update();
    
    render_time = frame_timer.getTimeElapsed();
    
    updateWindow();
    
    frame_length = frame_timer.getTimeElapsed();
}

//#define ENABLE_DEBUG_PRINT

void gfx::Scene::cycleModules() {
    for(auto & module : modules)
        module->enable_key_states = curr_scene == this;
    
    for(int i = 0; i < modules.size(); i++) {
        Timer timer;
        modules[i]->update(frame_length);
        modules[i]->update_time_sum += timer.getTimeElapsed();
    }
    
    for(int i = 0; i < modules.size(); i++)
        if(modules[i]->enabled) {
            Timer timer;
            modules[i]->render();
            modules[i]->render_time_sum += timer.getTimeElapsed();
        }
    
    frame_count++;
    render_time_sum += render_time;
    if(print_render_data_timer.getTimeElapsed() > 1000) {
        print_render_data_timer.reset();
        
#ifdef ENABLE_DEBUG_PRINT
        std::cout << "---> Render Times for: " << module_name << std::endl;
#endif
        
        for(int i = 0; i < modules.size(); i++) {
#ifdef ENABLE_DEBUG_PRINT
            if(modules[i]->enabled)
                std::cout << std::fixed << std::setprecision(3) << modules[i]->module_name << std::setw(30 - (int)modules[i]->module_name.length()) << " Update: " << modules[i]->update_time_sum / frame_count << " Render: " << modules[i]->render_time_sum / frame_count << std::endl;
            else
                std::cout << std::fixed << std::setprecision(3) << modules[i]->module_name << " (Disabled)" << std::endl;
#endif
                
            modules[i]->update_time_sum = 0;
            modules[i]->render_time_sum = 0;
        }
        
        frame_count = 0;
    }
}

void gfx::Scene::returnFromScene() {
    running = false;
}

bool gfx::Scene::isInitialized() const {
    return initialized;
}

bool gfx::Scene::isRunning() const {
    return running;
}

float gfx::Scene::getRenderTime() {
    return render_time;
}
