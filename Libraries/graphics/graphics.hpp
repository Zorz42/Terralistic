#ifndef graphics_hpp
#define graphics_hpp

#include <string>
#include <vector>
#include <SFML/Graphics.hpp>
#include "theme.hpp"

namespace gfx {

    class Image;
    
    void init(unsigned short window_width_, unsigned short window_height_), quit();
    void loadFont(const std::string& path, unsigned char size);

    inline std::string resource_path;

    void setMinimumWindowSize(unsigned short width, unsigned short height);
    unsigned short getWindowWidth();
    unsigned short getWindowHeight();

    void setRenderTarget(Image& tex);
    void resetRenderTarget();

    unsigned int getTicks();
    void sleep(unsigned short ms);
    float getDeltaTime();

    void clearWindow();
    void updateWindow();

    void returnFromScene();

    void setScale(float scale);

    void setWindowSize(unsigned short width, unsigned short height);
        
    void drawVertices(const sf::VertexArray& array, const sf::Texture& texture);
    void drawVertices(const sf::VertexArray& array);

    enum ObjectType {TOP_LEFT, TOP, TOP_RIGHT, LEFT, CENTER, RIGHT, BOTTOM_LEFT, BOTTOM, BOTTOM_RIGHT};

    class Color {
    public:
        Color(unsigned char r, unsigned char g, unsigned char b, unsigned char a=255) : r(r), g(g), b(b), a(a) {}\
        operator sf::Color() const { return {r, g, b, a}; }
        unsigned char r, g, b, a;
    };

    class RectShape {
    public:
        short x, y;
        unsigned short w, h;
        RectShape(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0);
        void render(Color c, bool fill=true) const;
    };

    class _CenteredObject {
    public:
        _CenteredObject(short x, short y, ObjectType orientation = TOP_LEFT);
        ObjectType orientation;
        RectShape getTranslatedRect() const;
        virtual unsigned short getWidth() const { return 0; };
        virtual unsigned short getHeight() const { return 0; };
        short getTranslatedX(unsigned short width) const;
        short getTranslatedY(unsigned short height) const;
        short getTranslatedX() const;
        short getTranslatedY() const;

        short x, y;
    };

    class Rect : public _CenteredObject {
        sf::RenderTexture* shadow_texture = nullptr;
        sf::RenderTexture* blur_texture = nullptr;
        unsigned short prev_w, prev_h;
        unsigned char prev_shadow_intensity = 0;
        using _CenteredObject::x;
        using _CenteredObject::y;
        unsigned short width, height;
        
        short target_x = 0, target_y = 0;
        unsigned short target_width = 0, target_height = 0;
        
        bool first_time = true;
        
        void updateBlurTextureSize();
        
    public:
        explicit Rect(short x = 0, short y = 0, unsigned short w = 0, unsigned short h = 0, Color c = GFX_DEFAULT_RECT_COLOR, ObjectType orientation = TOP_LEFT);
        
        unsigned short getWidth() const override;
        void setWidth(unsigned short width_);
        
        unsigned short getHeight() const override;
        void setHeight(unsigned short height_);
        
        short getX() const;
        void setX(short x_);
        
        short getY() const;
        void setY(short y_);
        
        unsigned short smooth_factor = 1;
        Color c;
        float blur_intensity = 0;
        unsigned char shadow_intensity = 0;
        void render(bool fill=true);
        ~Rect();
        Color border_color = {0, 0, 0, 0};
    };
    
    class Image {
    protected:
        Image& operator=(const Image&);
        void freeTexture();
        sf::RenderTexture *sfml_render_texture = nullptr;
        Color color{255, 255, 255};
    public:
        void render(float scale, short x, short y, bool flipped=false) const;
        void render(float scale, short x, short y, RectShape src_rect, bool flipped=false) const;
        ~Image();
        bool free_texture = true;
        unsigned short getTextureWidth() const;
        unsigned short getTextureHeight() const;
        void clear();
        void createBlankImage(unsigned short width, unsigned short height);
        void renderText(const std::string& text, Color text_color=GFX_DEFAULT_TEXT_COLOR);
        void loadFromFile(const std::string& path);
        void setColor(Color color_);
        sf::RenderTexture* getSfmlTexture() const { return sfml_render_texture; }
    };


    class Sprite : public _CenteredObject, public Image {
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

        bool active = false, ignore_one_input = false;
        char (*textProcessing)(char c, int length) = nullptr;
        unsigned short width = 200;
        Color border_color = GFX_DEFAULT_TEXT_INPUT_BORDER_COLOR, text_color = GFX_DEFAULT_TEXT_COLOR;
        void setBlurIntensity(float blur_intensity);
        void setBorderColor(Color color);
    };
    
    enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, UNKNOWN};

    class GraphicalModule {
    public:
        bool _can_receive_events;
        
        virtual void init() {}
        virtual void update() {}
        virtual void render() {}
        virtual void stop() {}
        virtual void onKeyDown(Key key_) {}
        
        bool getKeyState(Key key_);
        
        virtual ~GraphicalModule() = default;

        std::vector<TextInput*> text_inputs;
        bool disable_events = false;
        unsigned short mouse_x, mouse_y;
    };

    class Scene : public GraphicalModule {
        void _operateEvent(sf::Event event);
    public:
        void enableAllEvents(bool enable);
        
        std::vector<GraphicalModule*> modules;
        
        virtual void onMouseScroll(int distance) {}
        
        void run();

        void onKeyDownCallback(Key key_);
        
        unsigned short mouse_x, mouse_y;
    };
    
};

#endif
