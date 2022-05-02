#pragma once

#define GL_SILENCE_DEPRECATION
#include "theme.hpp"
#include <vector>
#include <string>
#include <stdexcept>
#include <chrono>
#include <GL/glew.h>
#include <GLFW/glfw3.h>

namespace gfx {

class Color {
public:
    unsigned char r = 0, g = 0, b = 0, a = 255;
};

class RectShape {
public:
    RectShape(int x = 0, int y = 0, int w = 0, int h = 0);
    int x, y, w, h;
    void render(Color color) const;
    void renderOutline(Color color) const;
};

class Orientation {
public:
    float x, y;
};

inline const Orientation TOP_LEFT =     {0 , 0 };
inline const Orientation TOP =          {.5, 0 };
inline const Orientation TOP_RIGHT =    {1 , 0 };
inline const Orientation LEFT =         {0 , .5};
inline const Orientation CENTER =       {.5, .5};
inline const Orientation RIGHT =        {1 , .5};
inline const Orientation BOTTOM_LEFT =  {0 , 1 };
inline const Orientation BOTTOM =       {.5, 1 };
inline const Orientation BOTTOM_RIGHT = {1 , 1 };

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

class Transformation {
    float matrix[3][3] = {
        {1.f, 0.f, 0.f},
        {0.f, 1.f, 0.f},
        {0.f, 0.f, 1.f},
    };
    void applyMatrix(float applied_matrix[3][3]);
public:
    void translate(float x, float y);
    void stretch(float x, float y);
    void reset();
    float* getArray();
    Transformation operator*(const Transformation& a);
};

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
    GLuint gl_texture = -1;
    int width = 0, height = 0;
    Transformation texture_normalization_transform;
public:
    void render(float scale, int x, int y, bool flipped=false, Color color={255, 255, 255}) const;
    void render(float scale, int x, int y, RectShape src_rect, bool flipped=false, Color color={255, 255, 255}) const;
    
    int getTextureWidth() const;
    int getTextureHeight() const;
    void createBlankImage(int width, int height);
    void loadFromData(const unsigned char* data, int width, int height);
    void loadFromText(const std::string& text, Color color={255, 255, 255});
    GLuint getGlTexture();

    void setRenderTarget();
    
    ~Texture();
};

class RectArray {
    int length = 0;
    std::vector<float> vertex_array;
    std::vector<float> color_array;
    std::vector<float> texture_pos_array;
    GLuint vertex_buffer, color_buffer, texture_pos_buffer;
    const Texture* image;
    bool update_vertex = true, update_color = true;
    
    void setVertex(int index, int x, int y);
    void setVertexColor(int index, Color color);
    void setVertexTextureCoord(int index, int x, int y);
    
public:
    RectArray(Texture* image_=nullptr);
    
    void setRect(int index, RectShape rect);
    void setColor(int index, Color color);
    void setColor(int index, Color color1, Color color2, Color color3, Color color4);
    void setTextureCoords(int index, RectShape texture_coordinates);
    void render(int x=0, int y=0, bool blend_multiply=false);
    void resize(int size);
    ~RectArray();
};

class Rect : public _CenteredObject {
    using _CenteredObject::x;
    using _CenteredObject::y;
    int width, height;
    
    int target_x = 0, target_y = 0, target_width = 0, target_height = 0;
    Timer approach_timer, blur_timer;
    
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
    void setPassthroughKeys(std::vector<Key> new_keys) { passthrough_keys = new_keys; };

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
    bool isInitialized();
    bool isRunning();
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

void setMinimumWindowSize(int width, int height);
void setWindowSize(int width, int height);
int getWindowWidth();
int getWindowHeight();

void resetRenderTarget();

void setGlobalScale(float scale);
void setFpsLimit(int limit);
void enableVsync(bool enabled);

void loadIconFromFile(const std::string& path);

inline bool blur_enabled = true;


};
