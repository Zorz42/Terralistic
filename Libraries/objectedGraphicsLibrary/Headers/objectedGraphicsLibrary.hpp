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
    [[maybe_unused]] void setX(short x_);
    void setY(short y_);
    
    inline virtual short getWidth() { return (short)width; };
    inline virtual short getHeight() { return (short)height; };
    
    void setOrientation(Uint8 objectType);
    
    SDL_Rect getRect();
    
protected:
    short x = 0, y = 0;
    unsigned short width = 0, height = 0;
    
    short getX();
    short getY();
    
    Uint8 orientation_x{}, orientation_y{};
};

}

namespace ogl {

enum objectType {top_left, top, top_right [[maybe_unused]], left, center, right, bottom_left [[maybe_unused]], bottom, bottom_right};

// texture that gets centered in window
class texture : public __ogl_private::centeredObject {
public:
    explicit texture(objectType type=center);
    ~texture();
    
    void setTexture(SDL_Texture* input_texture, int width_, int height_);
    void setTexture(SDL_Texture* input_texture);
    
    void render();
    
    void loadFromText(std::string text, SDL_Color text_color);
    void loadFromFile(std::string path);
    
    inline short getWidth() override { return short(width * scale); };
    inline short getHeight() override { return short(height * scale); };
    
    bool flipped{false};
    
    Uint8 scale = 1;
    bool free_texture = true;
    
protected:
    SDL_Texture* texture_ = nullptr;
    void freeTexture();
};

// rect that gets centered in window
class rect : public __ogl_private::centeredObject {
public:
    explicit rect(objectType type=center);
    
    inline void setWidth(short width_) { width = static_cast<unsigned short>(width_); }
    inline void setHeight(short height_) { height = static_cast<unsigned short>(height_); }
    
    void setColor(Uint8 r_, Uint8 g_, Uint8 b_);

    virtual void render();

    bool touchesPoint(int x, int y);
    bool fill = true;
    
    using __ogl_private::centeredObject::getX;
    using __ogl_private::centeredObject::getY;
    
protected:
    Uint8 r{}, g{}, b{};
};

}


#endif /* objectedGraphicsLibrary_hpp */
