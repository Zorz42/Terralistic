#pragma once

#include <string>
#include <vector>
#include <SFML/Graphics.hpp>
#include "theme.hpp"

namespace gfx {
    struct Color {
        unsigned char r = 0, g = 0, b = 0, a = 255;
    };

    class RectShape {
    public:
        RectShape(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0);
        short x, y;
        unsigned short w, h;
        void render(Color color) const;
        void renderOutline(Color color) const;
    };
    
    class PixelGrid {
        unsigned char* array;
        unsigned short width, height;
    public:
        PixelGrid(unsigned short width, unsigned short height);
        unsigned short getWidth() const;
        unsigned short getHeight() const;
        void setPixel(unsigned short x, unsigned short y, Color color);
        unsigned char* getArray() const;
        ~PixelGrid();
    };
    
    struct Orientation {
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
        explicit _CenteredObject(short x = 0, short y = 0, Orientation orientation = TOP_LEFT);
        Orientation orientation;
        RectShape getTranslatedRect() const;
        virtual unsigned short getWidth() const { return 0; };
        virtual unsigned short getHeight() const { return 0; };
        short getTranslatedX() const;
        short getTranslatedY() const;

        short x, y;
    };

    class Rect : public _CenteredObject {
        sf::RenderTexture* blur_texture = nullptr;
        using _CenteredObject::x;
        using _CenteredObject::y;
        unsigned short width, height;
        
        short target_x = 0, target_y = 0;
        unsigned short target_width = 0, target_height = 0;
        
        bool first_time = true;

        void updateBlurTextureSize();
        
    public:
        unsigned short getWidth() const override;
        void setWidth(unsigned short width_);
        
        unsigned short getHeight() const override;
        void setHeight(unsigned short height_);
        
        short getX() const;
        void setX(short x_);
        
        short getY() const;
        void setY(short y_);
        
        unsigned short smooth_factor = 1;
        float blur_intensity = 0;
        unsigned char shadow_intensity = 0;
        Color fill_color = {0, 0, 0, 0};
        Color border_color = {0, 0, 0, 0};
        void render();
        ~Rect();
    };
    
    class Texture;
    
    class RectArray {
        sf::Vertex* vertex_array = nullptr;
        sf::VertexBuffer vertex_buffer;
    public:
        RectArray(unsigned short size);
        RectArray() = default;
        
        void setRect(unsigned short index, RectShape rect);
        void setColor(unsigned short index, Color color);
        void setTextureCoords(unsigned short index, RectShape texture_coordinates);
        void render(int size, const Texture* image=nullptr);
        void resize(int size);
        ~RectArray();
    };
    
    class Texture {
    protected:
        friend void RectArray::render(int size, const Texture* image);
        friend void setRenderTarget(Texture& texture);
        void freeTexture();
        sf::RenderTexture *sfml_render_texture = nullptr;
        Color color{255, 255, 255};
    public:
        void render(float scale, short x, short y, bool flipped=false) const;
        void render(float scale, short x, short y, RectShape src_rect, bool flipped=false) const;
        unsigned short getTextureWidth() const;
        unsigned short getTextureHeight() const;
        void clear();
        void createBlankImage(unsigned short width, unsigned short height);
        void loadFromText(const std::string& text, Color text_color=GFX_DEFAULT_TEXT_COLOR);
        void loadFromResources(const std::string& path);
        void loadFromFile(const std::string& path);
        void loadFromPixelGrid(const PixelGrid& pixel_grid);
        void setColor(Color color_);
        
        ~Texture();
    };

    class Sprite : public _CenteredObject, public Texture {
    public:
        bool flipped = false;
        float scale = 1;
        unsigned short getWidth() const override { return getTextureWidth() * scale; }
        unsigned short getHeight() const override { return getTextureHeight() * scale; }
        Sprite();
        void render() const;

    };

    class Button : public Sprite {
    public:
        unsigned short margin = GFX_DEFAULT_BUTTON_MARGIN;

        unsigned short getWidth() const override;
        unsigned short getHeight() const override;

        Color def_color = GFX_DEFAULT_BUTTON_COLOR, hover_color = GFX_DEFAULT_HOVERED_BUTTON_COLOR;
        bool isHovered(unsigned short mouse_x, unsigned short mouse_y) const;
        bool disabled = false;
        unsigned char hover_progress = 0;
        void render(unsigned short mouse_x, unsigned short mouse_y);
    };

    class TextInput : public Button {
        std::string text;
        Rect back_rect;
    public:
        void render(unsigned short mouse_x, unsigned short mouse_y);
        TextInput();

        std::string getText() const { return text; }
        unsigned short getWidth() const override;
        void setText(const std::string& text);

        bool active = false, ignore_next_input = false;
        char (*textProcessing)(char c, int length) = nullptr;
        unsigned short width = GFX_DEFAULT_TEXT_INPUT_WIDTH;
        Color text_color = GFX_DEFAULT_TEXT_COLOR;
        void setBlurIntensity(float blur_intensity);
        void setBorderColor(Color color);
    };
    
    enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, UNKNOWN};

    class Scene;
    
    class SceneModule {
        friend Scene;
        bool enable_key_states = true;
        short mouse_x, mouse_y;
    public:
        virtual void init() {}
        virtual void update(float frame_length) {}
        virtual void render() {}
        virtual void stop() {}
        virtual bool onKeyDown(Key key_) { return false; }
        bool getKeyState(Key key_) const;
        virtual void onMouseScroll(int distance) {}
        short getMouseX();
        short getMouseY();
        
        std::vector<TextInput*> text_inputs;
    };

    class Scene : public SceneModule {
        void onEvent(sf::Event event);
        float frame_length;
        std::vector<SceneModule*> modules;
        void onKeyDownCallback(Key key_);
        bool running = true;
    public:
        void run();
        void registerAModule(SceneModule* module);
        void switchToScene(Scene& scene);
        void cycleModules();
        void returnFromScene();
    };
    
    void init(const std::string& resource_path_, unsigned short window_width_, unsigned short window_height_);
    void quit();
    void loadFont(const std::string& path, unsigned char size);
    
    std::string getResourcePath();

    void setMinimumWindowSize(unsigned short width, unsigned short height);
    unsigned short getWindowWidth();
    unsigned short getWindowHeight();

    void setRenderTarget(Texture& tex);
    void resetRenderTarget();

    unsigned int getTicks();
    void sleep(unsigned short ms);
    
    void setGlobalScale(float scale);

    void setWindowSize(unsigned short width, unsigned short height);
};
