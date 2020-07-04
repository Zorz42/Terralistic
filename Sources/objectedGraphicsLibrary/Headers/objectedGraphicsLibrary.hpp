//
//  objectedGraphicsLibrary.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 04/07/2020.
//

#ifndef objectedGraphicsLibrary_hpp
#define objectedGraphicsLibrary_hpp

#include <iostream>
#include <SDL2/SDL.h>

namespace __ogl_private {

class centeredObject {
public:
    void setOffset(short x, short y);
    void setCenterRectBoundsOffset(short up, short down, short left, short right);
    void setPos(short x, short y);

protected:
    short center_rect_up_offset{0};
    short center_rect_down_offset{0};
    short center_rect_left_offset{0};
    short center_rect_right_offset{0};
    short x_offset{0};
    short y_offset{0};
    bool centered{false};
    
    short getX(short width);
    short getY(short height);
};

}

namespace ogl {

// texture that gets centered in window
class texture : public __ogl_private::centeredObject {
public:
    texture() = default;
    texture(SDL_Texture* initial_texture);
    ~texture();
    
    void setTexture(SDL_Texture* input_texture);
    inline SDL_Texture* getTexture() { return texture_; }
    
    void render();
    
    void loadFromText(std::string text, SDL_Color text_color);
    void loadFromFile(std::string path);
    
    inline void setScale(short scale_) { scale = scale_; };
    
    inline short getWidth() { return texture_width * scale; };
    inline short getHeight() { return texture_height * scale; };
    
private:
    SDL_Texture* texture_{nullptr};
    void freeTexture();
    short texture_height{0};
    short texture_width{0};
    short scale{1};
};

// rect that gets centered in window
class rect : public __ogl_private::centeredObject {
public:
    rect() = default;
    rect(SDL_Rect rect);
    
    void setRect(SDL_Rect rect);
    inline void setWidth(short width_) { width = width_; }
    inline void setHeight(short height_) { height = height_; }
    
    void render();
    
private:
    short width{0};
    short height{0};
};

}


#endif /* objectedGraphicsLibrary_hpp */
