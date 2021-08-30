#include <cmath>
#include "graphics-internal.hpp"

static unsigned short min_window_width, min_window_height;
static sf::Clock global_clock;

static const char* blur_shader_code =
"uniform sampler2D source;"
"uniform vec2 offset;"
""
"void main() {"
"    vec2 textureCoordinates = gl_TexCoord[0].xy;"
"    vec4 color = vec4(0.0);"
"    color += texture2D(source, textureCoordinates - 10.0 * offset) * 0.0012;"
"    color += texture2D(source, textureCoordinates - 9.0 * offset) * 0.0015;"
"    color += texture2D(source, textureCoordinates - 8.0 * offset) * 0.0038;"
"    color += texture2D(source, textureCoordinates - 7.0 * offset) * 0.0087;"
"    color += texture2D(source, textureCoordinates - 6.0 * offset) * 0.0180;"
"    color += texture2D(source, textureCoordinates - 5.0 * offset) * 0.0332;"
"    color += texture2D(source, textureCoordinates - 4.0 * offset) * 0.0547;"
"    color += texture2D(source, textureCoordinates - 3.0 * offset) * 0.0807;"
"    color += texture2D(source, textureCoordinates - 2.0 * offset) * 0.1065;"
"    color += texture2D(source, textureCoordinates - offset) * 0.1258;"
"    color += texture2D(source, textureCoordinates) * 0.1330;"
"    color += texture2D(source, textureCoordinates + offset) * 0.1258;"
"    color += texture2D(source, textureCoordinates + 2.0 * offset) * 0.1065;"
"    color += texture2D(source, textureCoordinates + 3.0 * offset) * 0.0807;"
"    color += texture2D(source, textureCoordinates + 4.0 * offset) * 0.0547;"
"    color += texture2D(source, textureCoordinates + 5.0 * offset) * 0.0332;"
"    color += texture2D(source, textureCoordinates + 6.0 * offset) * 0.0180;"
"    color += texture2D(source, textureCoordinates - 7.0 * offset) * 0.0087;"
"    color += texture2D(source, textureCoordinates - 8.0 * offset) * 0.0038;"
"    color += texture2D(source, textureCoordinates - 9.0 * offset) * 0.0015;"
"    color += texture2D(source, textureCoordinates - 10.0 * offset) * 0.0012;"
"    gl_FragColor = color;"
"}";

static sf::RenderTexture window_texture;

void gfx::init(unsigned short window_width, unsigned short window_height) {
    window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    window->setVerticalSyncEnabled(true);
    window->setFramerateLimit(120);
    render_target = &window_texture;
    setWindowSize(window_width, window_height);

    bool result = blur_shader.loadFromMemory(blur_shader_code,  sf::Shader::Type::Fragment);
    assert(result);
    
    shadow_texture.create(700, 700);
    shadow_texture.clear({0, 0, 0, 0});
    sf::RectangleShape rect;
    rect.setPosition(200, 200);
    rect.setSize(sf::Vector2f(300, 300));
    rect.setFillColor({0, 0, 0});
    shadow_texture.draw(rect);
    blurTexture(shadow_texture, SHADOW_BLUR);
    rect.setFillColor({0, 0, 0, 0});
    shadow_texture.draw(rect, sf::BlendNone);
    shadow_texture.display();
    
    sf::Sprite part_sprite(shadow_texture.getTexture());
    
    shadow_part_left.create(200, 1);
    shadow_part_left.setRepeated(true);
    shadow_part_left.clear({0, 0, 0, 0});
    part_sprite.setTextureRect({0, 350, 200, 1});
    shadow_part_left.draw(part_sprite);
    shadow_part_left.display();
    
    shadow_part_right.create(200, 1);
    shadow_part_right.setRepeated(true);
    shadow_part_right.clear({0, 0, 0, 0});
    part_sprite.setTextureRect({500, 350, 200, 1});
    shadow_part_right.draw(part_sprite);
    shadow_part_right.display();
    
    shadow_part_up.create(1, 200);
    shadow_part_up.setRepeated(true);
    shadow_part_up.clear({0, 0, 0, 0});
    part_sprite.setTextureRect({350, 0, 1, 200});
    shadow_part_up.draw(part_sprite);
    shadow_part_up.display();
    
    shadow_part_down.create(1, 200);
    shadow_part_down.setRepeated(true);
    shadow_part_down.clear({0, 0, 0, 0});
    part_sprite.setTextureRect({350, 500, 1, 200});
    shadow_part_down.draw(part_sprite);
    shadow_part_down.display();
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
    output.generateMipmap(); // without that it doesnt work on smaller textures
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

void gfx::blurTexture(sf::RenderTexture& texture, float blur_intensity, int quality) {
    blur_intensity = std::pow(2, blur_intensity);
    quality = 2 << quality;
    
    blur_shader.setUniform("source", texture.getTexture());
    
    while(blur_intensity >= 1.f) {
        blur_shader.setUniform("offset", sf::Vector2f(blur_intensity / texture.getSize().x, 0));
        applyShader(blur_shader, texture);
        
        blur_shader.setUniform("offset", sf::Vector2f(0, blur_intensity / texture.getSize().y));
        applyShader(blur_shader, texture);
        
        if(blur_intensity < quality && blur_intensity != 1)
            blur_intensity = 1;
        else
            blur_intensity /= quality;
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
    window_texture.create(width / global_scale, height / global_scale);
}

void gfx::drawVertices(const sf::VertexArray& array, const sf::Texture& texture) {
    sf::RenderStates states;
    states.texture = &texture;
    render_target->draw(array, states);
}

void gfx::drawVertices(const sf::VertexArray& array) {
    render_target->draw(array);
}
