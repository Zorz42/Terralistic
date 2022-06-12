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

class _CenteredObject {
public:
    explicit _CenteredObject(int x = 0, int y = 0, Orientation orientation = TOP_LEFT);
    Orientation orientation;
    RectShape getTranslatedRect() const;
    virtual int getWidth() const { return 0; };
    virtual int getHeight() const { return 0; };
    int getTranslatedX() const;
    int getTranslatedY() const;

    int x, y;
};

void sleep(float ms);

class Timer {
    std::chrono::time_point<std::chrono::steady_clock> start_time;
public:
    Timer();
    float getTimeElapsed() const;
    void reset();
};

class Texture {
protected:
    void freeTexture();
    unsigned int gl_texture = -1;
    int width = 0, height = 0;
    _Transformation texture_normalization_transform;
public:
    void render(float scale, int x, int y, bool flipped=false, Color color={255, 255, 255}) const;
    void render(float scale, int x, int y, RectShape src_rect, bool flipped=false, Color color={255, 255, 255}) const;
    
    int getTextureWidth() const;
    int getTextureHeight() const;
    void createBlankImage(int width, int height);
    void loadFromData(const unsigned char* data, int width, int height);
    void loadFromText(const std::string& text, Color color={255, 255, 255});

    void setRenderTarget();
    const _Transformation& getNormalizationTransform() const;
    unsigned int getGlTexture() const;
    
    ~Texture();
};

class RectArray {
    int length = 0;
    std::vector<float> vertex_array;
    std::vector<float> color_array;
    std::vector<float> texture_pos_array;
    unsigned int vertex_buffer = -1, color_buffer, texture_pos_buffer;
    bool update_vertex = true, update_color = true, update_texture_vertex = true;
    
    void setVertex(int index, int x, int y);
    void setVertexColor(int index, Color color);
    void setVertexTextureCoord(int index, int x, int y);
    
public:
    void setRect(int index, RectShape rect);
    void setColor(int index, Color color);
    void setColor(int index, Color color1, Color color2, Color color3, Color color4);
    void setTextureCoords(int index, RectShape texture_coordinates);
    void render(const Texture* image=nullptr, int x=0, int y=0, bool blend_multiply=false);
    void resize(int size);
    ~RectArray();
};

class Rect : public _CenteredObject {
    using _CenteredObject::x;
    using _CenteredObject::y;
    int width, height;
    
    int target_x = 0, target_y = 0, target_width = 0, target_height = 0;
    Timer approach_timer;
    
    bool first_time = true;
public:
    int getWidth() const override;
    void setWidth(int width_);
    
    int getHeight() const override;
    void setHeight(int height_);
    
    int getX() const;
    void setX(int x_);
    
    int getY() const;
    void setY(int y_);
    
    int getTargetX() const;
    int getTargetY() const;
    
    void jumpToTarget();
    
    int smooth_factor = 1;
    float blur_radius = 0;
    unsigned char shadow_intensity = 0;
    
    Color fill_color = {0, 0, 0, 0};
    Color border_color = {0, 0, 0, 0};
    
    void render();
};

class TextureAtlas {
    Texture texture;
    std::vector<RectShape> rects;
public:
    const Texture& getTexture() { return texture; }
    void create(const std::vector<Texture*>& textures);
    RectShape getRect(int id);
};

class Sprite : public _CenteredObject, public Texture {
    Color color{255, 255, 255};
    RectShape src_rect;
public:
    Sprite();
    
    bool flipped = false;
    float scale = 1;
    int getWidth() const override;
    int getHeight() const override;
    void setColor(Color color_);
    void render() const;
    void setSrcRect(RectShape src_rect);
    void createBlankImage(int width, int height);
    void loadFromData(const unsigned char* data, int width, int height);
    void loadFromText(const std::string &text, Color color={255, 255, 255});
};

class Button : public Sprite {
    gfx::Timer timer;
public:
    int margin = GFX_DEFAULT_BUTTON_MARGIN;

    int getWidth() const override;
    int getHeight() const override;

    Color def_color = GFX_DEFAULT_BUTTON_COLOR, def_border_color = GFX_DEFAULT_BUTTON_BORDER_COLOR, hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR, border_hover_color = GFX_DEFAULT_HOVERED_BUTTON_BORDER_COLOR;
    bool isHovered(int mouse_x, int mouse_y) const;
    bool disabled = false;
    float hover_progress = 0;
    void render(int mouse_x, int mouse_y);
};

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

void init(int window_width_, int window_height_);
void quit();

void loadFont(const unsigned char* data);

void resetRenderTarget();

inline bool blur_enabled = true;
inline int fps_limit = 0;

};
