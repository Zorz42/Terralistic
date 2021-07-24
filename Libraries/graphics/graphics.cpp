#include "graphics-internal.hpp"

static unsigned short min_window_width, min_window_height;
static sf::Clock global_clock;

void gfx::init(unsigned short window_width, unsigned short window_height) {
    window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    window->setVerticalSyncEnabled(true);
    render_target = window;
    setWindowSize(window_width, window_height);
}

void gfx::setWindowMinimumSize(unsigned short width, unsigned short height) {
    min_window_width = width;
    min_window_height = height;
}

void gfx::loadFont(const std::string& path, unsigned char size) {
    font_size = size;
    font.loadFromFile(resource_path + path);
}

void gfx::quit() {
    delete window;
}

unsigned short gfx::getWindowWidth() {
    return window->getSize().x / global_scale;
}

unsigned short gfx::getWindowHeight() {
    return window->getSize().y / global_scale;
}

unsigned short gfx::getMouseX() {
    return mouse_x;
}

unsigned short gfx::getMouseY() {
    return mouse_y;
}

void gfx::setRenderTarget(Image& tex) {
    render_target = tex.getSfmlTexture();
}

void gfx::resetRenderTarget() {
    ((sf::RenderTexture*)render_target)->display();
    render_target = window;
}

bool gfx::colliding(RectShape a, RectShape b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

unsigned int gfx::getTicks() {
    return global_clock.getElapsedTime().asMilliseconds();
}

float gfx::getDeltaTime() {
    return frame_length;
}

void gfx::clearWindow() {
    window->clear();
}

void gfx::updateWindow() {
    window->display();
}

void gfx::sleep(unsigned short ms) {
    sf::sleep(sf::milliseconds(ms));
}

void gfx::setScale(float scale) {
    global_scale = scale;
    setWindowSize(getWindowWidth() * global_scale, getWindowHeight() * global_scale);
}

void gfx::setWindowSize(unsigned short width, unsigned short height) {
    width *= global_scale;
    height *= global_scale;
    
    if(width < min_window_width * global_scale)
        width = min_window_width * global_scale;
    if(height < min_window_height * global_scale)
        height = min_window_height * global_scale;
    
    sf::FloatRect visibleArea(0, 0, (unsigned int)width / global_scale, (unsigned int)height / global_scale);
    window->setView(sf::View(visibleArea));
    window->setSize({(unsigned int)width, (unsigned int)height});
}
