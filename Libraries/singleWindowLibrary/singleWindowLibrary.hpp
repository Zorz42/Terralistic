//
//  singleWindowLibrary.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#ifndef swl_hpp
#define swl_hpp

#include <iostream>
#include <SDL2_ttf/SDL_ttf.h>

namespace swl {

struct rect {
    short x, y;
    unsigned short w, h;
};

void init();

void clear();
void update();

void quit();

void render(SDL_Texture* texture, swl::rect destination_rectangle, swl::rect source_rectangle, bool flipped=false);
void render(SDL_Texture* texture, swl::rect destination_rectangle, bool flipped=false);
void render(SDL_Texture* texture, bool flipped=false);
void render(swl::rect& rect, bool fill=true);

SDL_Texture* loadTextureFromFile(std::string path, unsigned short* w=nullptr, unsigned short* h=nullptr);
SDL_Texture* loadTextureFromText(const std::string& text, SDL_Color text_color, unsigned short* w=nullptr, unsigned short* h=nullptr);

bool handleBasicEvents(SDL_Event &event, bool* running);
inline unsigned short window_height, window_width;

void setDrawColor(Uint8 r, Uint8 g, Uint8 b, Uint8 a=255);

void loadFont(std::string path, unsigned char size);

bool colliding(swl::rect a, swl::rect b);

inline unsigned short mouse_x, mouse_y;

void setWindowMinimumSize(unsigned short width, unsigned short height);

void setRenderTarget(SDL_Texture* texture);
void resetRenderTarget();

SDL_Texture* createBlankTexture(unsigned short width, unsigned short height);

inline std::string resourcePath;

}

namespace swl_private {

inline SDL_Window* window = nullptr;
inline SDL_Renderer* renderer = nullptr;
inline TTF_Font *font = nullptr;

}

#endif /* swl_hpp */
