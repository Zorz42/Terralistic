#include <cmath>
#include "graphics-internal.hpp"

static unsigned short min_window_width, min_window_height;
static sf::Clock global_clock;

// higher number = faster, but worse quality
#define SHADER_QUALITY 4

static std::string blur_shader_code =
"uniform sampler2D source;"
"uniform vec2 offset;"
"uniform vec2 bottom_left, top_right;"
""
"void main() {"
"   vec2 textureCoordinates = gl_TexCoord[0].xy;"
"   vec2 inside_box = step(bottom_left, textureCoordinates) - step(top_right, textureCoordinates); "
"   vec4 color = vec4(0.0);"
"   color += texture2D(source, textureCoordinates - 4.0 * offset) * 0.0162162162;"
"   color += texture2D(source, textureCoordinates - 3.0 * offset) * 0.0540540541;"
"   color += texture2D(source, textureCoordinates - 2.0 * offset) * 0.1216216216;"
"   color += texture2D(source, textureCoordinates - offset) * 0.1945945946;"
"   color += texture2D(source, textureCoordinates) * 0.2270270270;"
"   color += texture2D(source, textureCoordinates + offset) * 0.1945945946;"
"   color += texture2D(source, textureCoordinates + 2.0 * offset) * 0.1216216216;"
"   color += texture2D(source, textureCoordinates + 3.0 * offset) * 0.0540540541;"
"   color += texture2D(source, textureCoordinates + 4.0 * offset) * 0.0162162162;"
"   gl_FragColor = mix(texture2D(source, textureCoordinates), color, inside_box.x * inside_box.y);"
"}"
;
static sf::RenderTexture window_texture, blurred_texture;

void gfx::init(unsigned short window_width, unsigned short window_height) {
    window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
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

void applyShader(const sf::Shader& shader, sf::RenderTexture& output) {
    sf::Vector2f outputSize = static_cast<sf::Vector2f>(output.getSize());

    sf::VertexArray vertices(sf::TrianglesStrip, 4);
    vertices[0] = sf::Vertex(sf::Vector2f(0, 0),                       sf::Vector2f(0, 1));
    vertices[1] = sf::Vertex(sf::Vector2f(outputSize.x, 0),            sf::Vector2f(1, 1));
    vertices[2] = sf::Vertex(sf::Vector2f(0, outputSize.y),            sf::Vector2f(0, 0));
    vertices[3] = sf::Vertex(sf::Vector2f(outputSize.x, outputSize.y), sf::Vector2f(1, 0));

    sf::RenderStates states;
    states.shader    = &shader;
    states.blendMode = sf::BlendNone;

    output.draw(vertices, states);
}

void gfx::blurRegion(RectShape region, float blur_intensity) {
    blurred_texture.clear(TRANSPARENT);
    blurred_texture.draw(sf::Sprite(window_texture.getTexture()));
    
    blur_shader.setUniform("bottom_left", sf::Vector2f(float(region.x) / getWindowWidth(), 1.f - float(region.y + region.h) / getWindowHeight()));
    blur_shader.setUniform("top_right", sf::Vector2f(float(region.x + region.w) / getWindowWidth(), 1.f - float(region.y) / getWindowWidth()));
    blur_shader.setUniform("source", blurred_texture.getTexture());
    
    blur_intensity = std::pow(2, blur_intensity);
    
    while(blur_intensity >= 1.f / SHADER_QUALITY) {
        blur_shader.setUniform("offset", sf::Vector2f(blur_intensity / region.w / global_scale, 0));
        applyShader(blur_shader, blurred_texture);
        
        blur_shader.setUniform("offset", sf::Vector2f(0, blur_intensity / region.h / global_scale));
        applyShader(blur_shader, blurred_texture);
        
        blur_intensity /= SHADER_QUALITY;
    }
    
    blurred_texture.display();
    sf::Sprite blurred_sprite;
    blurred_sprite.setTexture(blurred_texture.getTexture());
    blurred_sprite.setTextureRect({region.x, region.y, region.w, region.h});
    blurred_sprite.setPosition(region.x, region.y);
    window_texture.draw(blurred_sprite);
    
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
    window_texture.create(width / global_scale, height / global_scale);
    blurred_texture.create(width / global_scale, height / global_scale);
}
