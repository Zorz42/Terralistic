#include "graphics-internal.hpp"

void gfx::Scene::initialize() {
    if(!initialized) {
        for(int i = 0; i < modules.size(); i++)
            modules[i]->init();
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

void gfx::Scene::onKeyDownCallback(Key key_) {
    if(!key_states[(int)key_]) {
        key_states[(int)key_] = true;
        for(int i = (int)modules.size() - 1; i >= 0; i--)
            if(modules[i]->enabled && modules[i]->onKeyDown(key_))
                break;
    }
}

void gfx::Scene::onKeyUpCallback(Key key_) {
    if(key_states[(int)key_]) {
        key_states[(int)key_] = false;
        for(int i = (int)modules.size() - 1; i >= 0; i--)
            if(modules[i]->enabled && modules[i]->onKeyUp(key_))
                break;
    }
}

bool gfx::SceneModule::getKeyState(Key key_) const {
    return enable_key_states && key_states[(int)key_];
}

gfx::Key translateKeyboardKey(int glfw_button) {
    switch (glfw_button) {
        case GLFW_KEY_A: return gfx::Key::A;
        case GLFW_KEY_B: return gfx::Key::B;
        case GLFW_KEY_C: return gfx::Key::C;
        case GLFW_KEY_D: return gfx::Key::D;
        case GLFW_KEY_E: return gfx::Key::E;
        case GLFW_KEY_F: return gfx::Key::F;
        case GLFW_KEY_G: return gfx::Key::G;
        case GLFW_KEY_H: return gfx::Key::H;
        case GLFW_KEY_I: return gfx::Key::I;
        case GLFW_KEY_J: return gfx::Key::J;
        case GLFW_KEY_K: return gfx::Key::K;
        case GLFW_KEY_L: return gfx::Key::L;
        case GLFW_KEY_M: return gfx::Key::M;
        case GLFW_KEY_N: return gfx::Key::N;
        case GLFW_KEY_O: return gfx::Key::O;
        case GLFW_KEY_P: return gfx::Key::P;
        case GLFW_KEY_Q: return gfx::Key::Q;
        case GLFW_KEY_R: return gfx::Key::R;
        case GLFW_KEY_S: return gfx::Key::S;
        case GLFW_KEY_T: return gfx::Key::T;
        case GLFW_KEY_U: return gfx::Key::U;
        case GLFW_KEY_V: return gfx::Key::V;
        case GLFW_KEY_W: return gfx::Key::W;
        case GLFW_KEY_X: return gfx::Key::X;
        case GLFW_KEY_Y: return gfx::Key::Y;
        case GLFW_KEY_Z: return gfx::Key::Z;
        case GLFW_KEY_0: return gfx::Key::NUM0;
        case GLFW_KEY_1: return gfx::Key::NUM1;
        case GLFW_KEY_2: return gfx::Key::NUM2;
        case GLFW_KEY_3: return gfx::Key::NUM3;
        case GLFW_KEY_4: return gfx::Key::NUM4;
        case GLFW_KEY_5: return gfx::Key::NUM5;
        case GLFW_KEY_6: return gfx::Key::NUM6;
        case GLFW_KEY_7: return gfx::Key::NUM7;
        case GLFW_KEY_8: return gfx::Key::NUM8;
        case GLFW_KEY_9: return gfx::Key::NUM9;
        case GLFW_KEY_SPACE: return gfx::Key::SPACE;
        case GLFW_KEY_ESCAPE: return gfx::Key::ESCAPE;
        case GLFW_KEY_ENTER: return gfx::Key::ENTER;
        case GLFW_KEY_LEFT_SHIFT: case GLFW_KEY_RIGHT_SHIFT: return gfx::Key::SHIFT;
        case GLFW_KEY_BACKSPACE: return gfx::Key::BACKSPACE;
        case GLFW_KEY_LEFT_CONTROL: case GLFW_KEY_RIGHT_CONTROL: return gfx::Key::CTRL;
        default: return gfx::Key::UNKNOWN;
    }
}

gfx::Key translateMouseKey(int glfw_button) {
    switch(glfw_button) {
        case GLFW_MOUSE_BUTTON_LEFT: return gfx::Key::MOUSE_LEFT;
        case GLFW_MOUSE_BUTTON_RIGHT: return gfx::Key::MOUSE_RIGHT;
        case GLFW_MOUSE_BUTTON_MIDDLE: return gfx::Key::MOUSE_MIDDLE;
        default: return gfx::Key::UNKNOWN;
    }
}

void gfx::Scene::onMouseButtonEvent(gfx::Key key, bool pressed) {
    if(pressed) {
        bool clicked_text_box = false;
        if (key == Key::MOUSE_LEFT) {
            for (SceneModule* module : modules)
                if(module->enabled)
                    for (TextInput* i : module->text_inputs) {
                        i->active = i->isHovered(getMouseX(), getMouseY());
                        if (i->active)
                            clicked_text_box = true;
                    }
        }
        if (key != Key::UNKNOWN && !clicked_text_box)
            onKeyDownCallback(key);
    }
    
    else {
        if (key != Key::UNKNOWN)
            onKeyUpCallback(key);
    }
}

void gfx::Scene::onKeyboardButtonEvent(gfx::Key key, bool pressed) {
    if(pressed) {
        if (key == Key::BACKSPACE) {
            for (SceneModule* module : modules)
                for (TextInput* i : module->text_inputs)
                    if (i->active && !i->getText().empty()) {
                        std::string str = i->getText();
                        str.pop_back();
                        i->setText(str);
                    }
        }
        
        bool is_textbox_active = false;
        for(auto & module : modules)
            if(module->enabled)
                for(auto & text_input : module->text_inputs)
                    if(text_input->active) {
                        is_textbox_active = true;
                        std::vector<Key> input_passthrough_keys = text_input->getPassthroughKeys();
                        if(std::count(input_passthrough_keys.begin(), input_passthrough_keys.end(), key))
                            module->onKeyDown(key);
                    }
        
        if(key != Key::UNKNOWN && (!is_textbox_active || key == Key::ENTER))
            onKeyDownCallback(key);
        
        if(is_textbox_active && key == Key::ESCAPE) {
            for(int i = 0; i < modules.size(); i++)
                if(modules[i]->enabled)
                    for(int i2 = 0; i2 < modules[i]->text_inputs.size(); i2++)
                        modules[i]->text_inputs[i2]->active = false;
        }
    }
        
    else {
        if (key != Key::UNKNOWN)
            onKeyUpCallback(key);
    }
}

void gfx::Scene::onTextEnteredEvent(char c) {
    if(c == '\b')
        return;

    for (SceneModule* module : modules)
        if(module->enabled)
            for (TextInput* i : module->text_inputs)
                if (i->active) {
                    char result = c;
                    if (!i->ignore_next_input) {
                        if (i->textProcessing)
                            result = i->textProcessing(result, (int)i->getText().size());
                        if (result)
                            i->setText(i->getText() + result);
                    }
                    i->ignore_next_input = false;
                }
}

void gfx::Scene::onMouseWheelScrollEvent(int delta) {
    onMouseScroll(delta);
}

void gfx::Scene::run() {
    initialize();
    
    while(running) {
        renderAll();
        
        glfwPollEvents();
        
        if(glfwWindowShouldClose(glfw_window) != 0)
            running = false;
    }
    
    for(int i = 0; i < modules.size(); i++)
        modules[i]->stop();
}

void gfx::Scene::renderAll() {
    curr_scene = this;
    Timer frame_timer;
    
    double mouse_x_normalized, mouse_y_normalized;
    glfwGetCursorPos(glfw_window, &mouse_x_normalized, &mouse_y_normalized);
    mouse_x = mouse_x_normalized;
    mouse_y = mouse_y_normalized;
    for(int i = 0; i < modules.size(); i++)
        if(modules[i]->enabled) {
            modules[i]->mouse_x = mouse_x;
            modules[i]->mouse_y = mouse_y;
        }
    
    glClear(GL_COLOR_BUFFER_BIT);
    
    resetRenderTarget();
    RectShape(0, 0, window_width, window_height).render({0, 0, 0});
    
    cycleModules();
    
    for(int i = 0; i < global_update_functions.size(); i++)
        global_update_functions[i]->update();
    
    updateWindow();
    
    frame_length = frame_timer.getTimeElapsed();
}

void gfx::Scene::cycleModules() {
    for(int i = 0; i < modules.size(); i++)
        modules[i]->update(frame_length);
    
    for(int i = 0; i < modules.size(); i++)
        if(modules[i]->enabled)
            modules[i]->render();
}

void gfx::Scene::returnFromScene() {
    running = false;
}

bool gfx::Scene::isInitialized() {
    return initialized;
}

bool gfx::Scene::isRunning() {
    return running;
}


void gfx::keyCallback(GLFWwindow* window, int key_, int scancode, int action, int mods) {
    gfx::Key key = translateKeyboardKey(key_);
    if(action == GLFW_PRESS || action == GLFW_RELEASE)
        curr_scene->onKeyboardButtonEvent(key, action == GLFW_PRESS);
}

void gfx::mouseButtonCallback(GLFWwindow* window, int button, int action, int mods) {
    gfx::Key key = translateMouseKey(button);
    if(action == GLFW_PRESS || action == GLFW_RELEASE)
        curr_scene->onMouseButtonEvent(key, action == GLFW_PRESS);
}

void gfx::scrollCallback(GLFWwindow* window, double xoffset, double yoffset) {
    curr_scene->onMouseWheelScrollEvent(yoffset);
}

void gfx::characterCallback(GLFWwindow* window, unsigned int codepoint) {
    curr_scene->onTextEnteredEvent(codepoint);
}

void gfx::Scene::switchToScene(Scene& scene) {
    for(int i = 0; i < modules.size(); i++)
        modules[i]->enable_key_states = false;
    scene.run();
    for(int i = 0; i < modules.size(); i++)
        modules[i]->enable_key_states = true;
}
