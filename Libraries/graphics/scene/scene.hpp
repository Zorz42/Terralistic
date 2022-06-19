#pragma once

namespace gfx {

class Scene;

class SceneModule {
    friend Scene;
    bool enable_key_states = true;
    int mouse_x, mouse_y;
public:
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
public:
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
};

};

#ifndef GRAPHICS_PUBLIC

namespace gfx {

};

#endif
