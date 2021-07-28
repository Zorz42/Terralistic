#include <cmath>
#include "graphics-internal.hpp"

static unsigned short min_window_width, min_window_height;
static sf::Clock global_clock;

static std::string blur_shader_code =
"uniform sampler2D source;"
"uniform vec2 offsetFactor;"
""
"void main() {"
"    vec2 textureCoordinates = gl_TexCoord[0].xy;"
"    vec4 color = vec4(0.0);"
"    color += texture2D(source, textureCoordinates - 4.0 * offsetFactor) * 0.0162162162;"
"    color += texture2D(source, textureCoordinates - 3.0 * offsetFactor) * 0.0540540541;"
"    color += texture2D(source, textureCoordinates - 2.0 * offsetFactor) * 0.1216216216;"
"    color += texture2D(source, textureCoordinates - offsetFactor) * 0.1945945946;"
"    color += texture2D(source, textureCoordinates) * 0.2270270270;"
"    color += texture2D(source, textureCoordinates + offsetFactor) * 0.1945945946;"
"    color += texture2D(source, textureCoordinates + 2.0 * offsetFactor) * 0.1216216216;"
"    color += texture2D(source, textureCoordinates + 3.0 * offsetFactor) * 0.0540540541;"
"    color += texture2D(source, textureCoordinates + 4.0 * offsetFactor) * 0.0162162162;"
"    gl_FragColor = color;"
"}"
;
static sf::RenderTexture window_texture;
static sf::RenderTexture blurred_texture;

void gfx::init(unsigned short window_width, unsigned short window_height) {
    window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    window_texture.create(window_width, window_height);
    blurred_texture.create(window_width, window_height);
    window->setVerticalSyncEnabled(true);
    window->setFramerateLimit(360);
    render_target = &window_texture;
    setWindowSize(window_width, window_height);
    
    bool result = blur_shader.loadFromMemory(blur_shader_code,  sf::Shader::Type::Fragment);
    assert(result);
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
    render_target->display();
    render_target = &window_texture;
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
    window_texture.clear(GFX_BACKGROUND_COLOR);
}

void gfx::blurRegion(RectShape region, float blur_intensity) {
    sf::Sprite sprite;
    sprite.setTexture(window_texture.getTexture());
    sprite.setTextureRect({region.x, region.y, region.w, region.h});
    sprite.setPosition(region.x, region.y);
    
    blur_shader.setUniform("source", window_texture.getTexture());
    
    float blur_intensity_shader = std::pow(2, blur_intensity);
    for(float i = 0;; i += 2.f) {
        blur_shader.setUniform("offsetFactor", sf::Vector2f(blur_intensity_shader / getWindowWidth() / global_scale, blur_intensity_shader / getWindowHeight() / global_scale));
        window_texture.draw(sprite, &blur_shader);
        
        blur_shader.setUniform("offsetFactor", sf::Vector2f(blur_intensity_shader / -getWindowWidth() / global_scale, blur_intensity_shader / getWindowHeight() / global_scale));
        window_texture.draw(sprite, &blur_shader);
        
        if(i > blur_intensity)
            break;
        
        blur_intensity_shader /= 4;
    }
}

void gfx::updateWindow() {
    window_texture.display();
    window->draw(sf::Sprite(window_texture.getTexture()));
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
    window_texture.create(width, height);
    blurred_texture.create(width, height);
}
