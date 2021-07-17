#include "graphics-internal.hpp"

void gfx::init(unsigned short window_width, unsigned short window_height) {
    sfml_window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    render_target = sfml_window;
    
    // initialize basic sdl module
    int result = SDL_Init(SDL_INIT_EVERYTHING);
    SDL_assert(result >= 0);

    // initialize image loading part of sdl
    result = IMG_Init(IMG_INIT_PNG);
    SDL_assert(result & IMG_INIT_PNG);
    
    // initialize font rendering part of sdl
    result = TTF_Init();
    SDL_assert(result != -1);
    
    // create actual window
    window = SDL_CreateWindow("Terralistic", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, window_width, window_height, SDL_WINDOW_RESIZABLE);
    SDL_assert(window);

    // create renderer for GPU accelerated
    renderer = SDL_CreateRenderer(gfx::window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    SDL_assert(renderer);
    
    SDL_SetRenderDrawBlendMode(renderer, SDL_BLENDMODE_BLEND);
    SDL_DisplayMode dm = {SDL_PIXELFORMAT_UNKNOWN, 0, 0, 0, nullptr};
    SDL_SetWindowDisplayMode(window, &dm);
}

void gfx::setWindowMinimumSize(unsigned short width, unsigned short height) {
    SDL_SetWindowMinimumSize(window, width, height);
}

void gfx::loadFont(const std::string& path, unsigned char size) {
    font_size = size;
    
    font = TTF_OpenFont((resource_path + path).c_str(), font_size);
    SDL_assert(font);
    
    sfml_font.loadFromFile(resource_path + path);
}

void gfx::quit() {
    delete sfml_window;
    
    SDL_DestroyWindow(window);
    SDL_Quit();
}

unsigned short gfx::getWindowWidth() {
    return sfml_window->getSize().x;
}

unsigned short gfx::getWindowHeight() {
    return sfml_window->getSize().y;
}

unsigned short gfx::getMouseX() {
    return mouse_x;
}

unsigned short gfx::getMouseY() {
    return mouse_y;
}

void gfx::setRenderTarget(Image& tex) {
    render_target = tex.getSfmlTexture();
    SDL_SetRenderTarget(renderer, (SDL_Texture*)tex.getTexture());
}

void gfx::resetRenderTarget() {
    render_target = sfml_window;
    SDL_SetRenderTarget(renderer, nullptr);
}

bool gfx::colliding(RectShape a, RectShape b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

unsigned int gfx::getTicks() {
    return SDL_GetTicks();
}

float gfx::getDeltaTime() {
    return frame_length;
}

void gfx::clearWindow() {
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0);
    SDL_RenderClear(renderer);
    sfml_window->clear();
}

void gfx::updateWindow() {
    SDL_RenderPresent(renderer);
    sfml_window->display();
}

void gfx::sleep(unsigned short ms) {
    SDL_Delay(ms);
}
