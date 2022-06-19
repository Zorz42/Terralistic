#pragma once

#include <utility>
#include <vector>
#include <string>
#include <stdexcept>
#include <chrono>

#include "theme.hpp"

#define GRAPHICS_PUBLIC
#include "color.hpp"
#include "glfwAbstraction.hpp"
#include "rectShape.hpp"
#include "orientation.hpp"
#include "centeredObject.hpp"
#include "transformation.hpp"
#include "timer.hpp"
#include "texture.hpp"
#include "rectArray.hpp"
#include "rect.hpp"
#include "textureAtlas.hpp"
#include "sprite.hpp"
#include "button.hpp"
#include "textInput.hpp"
#include "scene.hpp"

namespace gfx {

enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, ARROW_UP, ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, UNKNOWN};

class TextInput : public Button {
    std::string text;
    Rect back_rect;
    std::vector<Key> passthrough_keys = {};
public:
    void render(int mouse_x, int mouse_y);
    TextInput();

    std::string getText() const { return text; }
    int getWidth() const override;
    void setText(const std::string& text);
    std::vector<Key> getPassthroughKeys() const { return passthrough_keys; }
    void setPassthroughKeys(std::vector<Key> new_keys) { passthrough_keys = std::move(new_keys); };

    bool active = false, ignore_next_input = false;
    char (*textProcessing)(char c, int length) = nullptr;
    int width = GFX_DEFAULT_TEXT_INPUT_WIDTH;
    Color text_color = GFX_DEFAULT_TEXT_COLOR;
    void setBlurIntensity(float blur_intensity);
    void setBorderColor(Color color);
};

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

class GlobalUpdateFunction {
public:
    virtual void update() = 0;
};

void addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function);

void init(int window_width_, int window_height_, const std::string& window_title);
void quit();

void loadFont(const unsigned char* data);

void resetRenderTarget();

inline bool blur_enabled = true;
inline int fps_limit = 0;

};
