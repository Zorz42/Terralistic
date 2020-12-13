//
//  singleWindowLibrary.hpp
//  Terralistic
//
//  Created by Jakob Zorz on 24/06/2020.
//

#ifndef core_hpp
#define core_hpp

#include <iostream>
#include <SDL2_ttf/SDL_ttf.h>

int swl_main();

namespace swl {
    
void clear();
void update();

void quit();

void popupError(std::string message);

void render(SDL_Texture* texture, SDL_Rect destination_rectangle, SDL_Rect source_rectangle, bool flipped=false);
void render(SDL_Texture* texture, SDL_Rect destination_rectangle, bool flipped=false);
void render(SDL_Texture* texture, bool flipped=false);
void render(SDL_Rect& rect, bool fill=true);

SDL_Texture* loadTextureFromFile(std::string path, int* w=nullptr, int* h=nullptr);
SDL_Texture* loadTextureFromText(std::string text, SDL_Color text_color, int *w=nullptr, int *h=nullptr);

bool handleBasicEvents(SDL_Event &event, bool* running);
inline int window_height, window_width;

void setDrawColor(Uint8 r, Uint8 g, Uint8 b, Uint8 a=255);

void loadFont(std::string path, int size);

bool colliding(SDL_Rect a, SDL_Rect b);

inline int mouse_x, mouse_y;

void setWindowMinimumSize(int width, int height);
}

namespace __swl_private {

inline SDL_Window* window = nullptr;
inline SDL_Renderer* renderer = nullptr;
inline TTF_Font *font = nullptr;

inline std::string resourcePath;
void setResourcePath(std::string executable_path);
}

#endif /* core_hpp */
