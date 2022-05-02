#pragma once
#include <SFML/Graphics.hpp>
#include "theme.hpp"

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

    class Timer {
        sf::Clock clock;
    public:
        float getTimeElapsed() const;
        void reset();
    };

    class Rect : public _CenteredObject {
        sf::RenderTexture* blur_texture = nullptr;
        using _CenteredObject::x;
        using _CenteredObject::y;
        int width, height;
        
        int target_x = 0, target_y = 0, target_width = 0, target_height = 0;
        Timer approach_timer, blur_timer;
        
        bool first_time = true;

        void updateBlurTextureSize();
        void updateBlurTexture();
        
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
        int length = 0;
    public:
        RectArray(int size);
        RectArray() = default;
        
        void setRect(int index, RectShape rect);
        void setColor(int index, Color color);
        void setTextureCoords(int index, RectShape texture_coordinates);
        void render(int size, const Texture* image=nullptr, int x=0, int y=0, bool blend_multiply=false);
        void resize(int size);
        ~RectArray();
    };
    
    class Texture {
    protected:
        friend void RectArray::render(int size, const Texture* image, int x, int y, bool blend_multiply);
        friend void setRenderTarget(Texture& texture);
        void freeTexture();
        sf::RenderTexture *sfml_render_texture = nullptr;
        Color color{255, 255, 255};
    public:
        void render(float scale, int x, int y, bool flipped=false) const;
        void render(float scale, int x, int y, RectShape src_rect, bool flipped=false) const;
        int getTextureWidth() const;
        int getTextureHeight() const;
        void clear();
        void createBlankImage(int width, int height);
        void loadFromText(const std::string& text, Color text_color=GFX_DEFAULT_TEXT_COLOR);
        void loadFromResources(const std::string& path);
        void loadFromFile(const std::string& path);
        void setColor(Color color_);


        ~Texture();
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
    public:
        bool flipped = false;
        float scale = 1;
        int getWidth() const override { return getTextureWidth() * scale; }
        int getHeight() const override { return getTextureHeight() * scale; }
        Sprite();
        void render() const;

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

    enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, ARROW_UP, ARROW_DOWN, UNKNOWN};

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
        std::vector<Key> getPassthroughKeys() const {return passthrough_keys;}
        void setPassthroughKeys(std::vector<Key> new_keys) {passthrough_keys = new_keys;};


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
        void onEvent(sf::Event event);
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
        void returnFromScene();
        const std::vector<SceneModule*>& getModules();
    };
    
    class GlobalUpdateFunction {
    public:
        virtual void update() = 0;
    };

    void addAGlobalUpdateFunction(GlobalUpdateFunction* global_update_function);

    void init(const std::string& resource_path_, int window_width_, int window_height_);
    void quit();

    void loadFont(const std::string& path, int size);
    
    std::string getResourcePath();

    void setMinimumWindowSize(int width, int height);
    void setWindowSize(int width, int height);
    int getWindowWidth();
    int getWindowHeight();

    void setRenderTarget(Texture& tex);
    void resetRenderTarget();
    
    void setGlobalScale(float scale);
    void setFpsLimit(int limit);
    void enableVsync(bool enabled);

    void loadIconFromFile(const std::string& path);

    inline bool blur_enabled = true;
};
