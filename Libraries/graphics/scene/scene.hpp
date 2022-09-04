#pragma once
#include <vector>
#include "textInput.hpp"
#include "glfwAbstraction.hpp"
#include "nonCopyable.hpp"

namespace gfx {

class GlobalUpdateFunction {
public:
    virtual void update() = 0;
};

void addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function);

class SceneModule : public NonCopyable {
    friend class Scene;
    bool enable_key_states = true;
    int mouse_x = 0, mouse_y = 0, mouse_vel = 0;
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
    bool getAbsoluteKeyState(Key key_) const;
    virtual void onMouseScroll(int distance) {}
    int getMouseX();
    int getMouseY();
    int getMouseVel();
    bool enabled = true;
    
    std::vector<TextInput*> text_inputs;
};

class Scene : public SceneModule, EventListener<_ScreenRefreshEvent>, EventListener<_KeyPressEvent>, EventListener<_KeyReleaseEvent>, EventListener<_ScrollEvent>, EventListener<_CharInputEvent> {
    std::vector<SceneModule*> modules;
    bool running = true, initialized = false;
    float render_time = 0;
    int frame_count = 0;
    Timer print_render_data_timer;
    void onEvent(_ScreenRefreshEvent& event) override;
    void onEvent(_KeyPressEvent& event) override;
    void onEvent(_KeyReleaseEvent& event) override;
    void onEvent(_ScrollEvent& event) override;
    void onEvent(_CharInputEvent& event) override;
public:
    Scene(const std::string& module_name) : SceneModule(module_name) {}
    virtual void preInit() {}
    
    void initialize();
    bool isInitialized() const;
    bool isRunning() const;
    void run();
    void registerAModule(SceneModule* module);
    void cycleModules();
    void renderAll();
    void returnFromScene();
    const std::vector<SceneModule*>& getModules();
    float getRenderTime();
};

inline int fps_limit = 0;

};

#ifndef GRAPHICS_PUBLIC

namespace gfx {

inline std::vector<GlobalUpdateFunction*> global_update_functions;
inline float frame_length;

};

#endif
