#include "graphics-internal.hpp"

void gfx::init(unsigned short window_width, unsigned short window_height) {
    sfml_window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    render_target = sfml_window;
}

void gfx::setWindowMinimumSize(unsigned short width, unsigned short height) {
    // TODO: set limit
}

void gfx::loadFont(const std::string& path, unsigned char size) {
    font_size = size;
    sfml_font.loadFromFile(resource_path + path);
}

void gfx::quit() {
    delete sfml_window;
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
}

void gfx::resetRenderTarget() {
    ((sf::RenderTexture*)render_target)->display();
    render_target = sfml_window;
}

bool gfx::colliding(RectShape a, RectShape b) {
    return a.y + a.h > b.y && a.y < b.y + b.h && a.x + a.w > b.x && a.x < b.x + b.w;
}

unsigned int gfx::getTicks() {
    return clock.getElapsedTime().asMilliseconds();
}

float gfx::getDeltaTime() {
    return frame_length;
}

void gfx::clearWindow() {
    sfml_window->clear();
}

void gfx::updateWindow() {
    sfml_window->display();
}

void gfx::sleep(unsigned short ms) {
    sf::sleep(sf::milliseconds(ms));
}
