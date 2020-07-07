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
    inline void setX(short x_) { x = x_; };
    inline void setY(short y_) { y = y_; };
    
    inline short getWidth() { return width; };
    inline short getHeight() { return height; };
    
    bool centered_x, centered_y;
    
protected:
    short x = 0, y = 0;
    unsigned short width = 0, height = 0;
    
    short getX(short width_);
    short getY(short height_);
};

}

namespace ogl {

enum objectType {centered, absolute};

// texture that gets centered in window
class texture : public __ogl_private::centeredObject {
public:
    texture(objectType type=centered);
    ~texture();
    
    void setTexture(SDL_Texture* input_texture);
    inline SDL_Texture* getTexture() { return texture_; }
    
    void render();
    
    void loadFromText(std::string text, SDL_Color text_color);
    void loadFromFile(std::string path);
    
    Uint8 scale = 1;
    
    inline short getWidth() { return width * scale; };
    inline short getHeight() { return height * scale; };
    
protected:
    SDL_Texture* texture_ = nullptr;
    void freeTexture();
};

// rect that gets centered in window
class rect : public __ogl_private::centeredObject {
public:
    rect(objectType type=centered);
    
    inline void setWidth(short width_) { width = width_; }
    inline void setHeight(short height_) { height = height_; }
    
    void setColor(Uint8 r_, Uint8 g_, Uint8 b_);
    void render();
    SDL_Rect getRect();
    bool touchesPoint(int x, int y);
    
    bool fill = true;
    
protected:
    Uint8 r, g, b;
};

}


#endif /* objectedGraphicsLibrary_hpp */
