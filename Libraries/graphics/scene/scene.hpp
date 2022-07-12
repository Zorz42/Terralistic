#pragma once
#include <vector>
#include "textInput.hpp"

namespace gfx {

class GlobalUpdateFunction {
public:
    virtual void update() = 0;
};

void addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function);

class Scene;

class SceneModule {
    friend Scene;
    bool enable_key_states = true;
    int mouse_x, mouse_y;
    std::string module_name;
    float render_time_sum = 0, update_time_sum = 0;
public:
    SceneModule(const std::string& module_name) : module_name(module_name) {}
    virtual void init() {}
    virtual void update(float frame_length) {}
    virtual void render() {}
    virtual void stop() {}
    virtual bool onKeyDown(Key key_) { return false; }
    virtual bool onKeyUp(Key key_) { return false; }
    bool getKeyState(Key key_) const;
    virtual void onMouseScroll(int distance) {}
    int getMouseX();
    int getMouseY();
    bool enabled = true;
    
    std::vector<TextInput*> text_inputs;
};

class Scene : public SceneModule {
    std::vector<SceneModule*> modules;
    void onKeyDownCallback(Key key_);
    void onKeyUpCallback(Key key_);
    bool running = true, initialized = false;
    float render_time;
    int frame_count = 0;
    Timer print_render_data_timer;
public:
    Scene(const std::string& module_name) : SceneModule(module_name) {}
    void initialize();
    bool isInitialized() const;
    bool isRunning() const;
    void run();
    void registerAModule(SceneModule* module);
    void switchToScene(Scene& scene);
    void cycleModules();
    void renderAll();
    void returnFromScene();
    const std::vector<SceneModule*>& getModules();
    void onMouseButtonEvent(gfx::Key key, bool pressed);
    void onKeyboardButtonEvent(gfx::Key key, bool pressed);
    void onTextEnteredEvent(char c);
    void onMouseWheelScrollEvent(int delta);
    float getRenderTime();
};

inline int fps_limit = 0;

};

#ifndef GRAPHICS_PUBLIC

namespace gfx {

inline std::vector<GlobalUpdateFunction*> global_update_functions;
inline float frame_length;

inline Scene* curr_scene = nullptr;

};

#endif
