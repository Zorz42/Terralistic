#include <cmath>
#include "graphics-internal.hpp"

static int min_window_width, min_window_height;
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

void gfx::init(const std::string& resource_path_, int window_width, int window_height) {
    if(window_width < 0 || window_height < 0)
        throw ValueException("Winow width and height must be positive.");
    
    resource_path = resource_path_;
    
    window = new sf::RenderWindow(sf::VideoMode(window_width, window_height), "Terralistic");
    render_target = &window_texture;
    setWindowSize(window_width, window_height);

    if(!blur_shader.loadFromMemory(blur_shader_code, sf::Shader::Type::Fragment))
        throw ShaderException();

    shadow_texture.create(700, 700);

    for (int i = 0; i < 2; i++) { // this is ugly but its the way i found working on linux
        sf::RenderTexture dummy;
        dummy.create(1, 1);

        shadow_texture.clear({0, 0, 0, 0});
        sf::RectangleShape rect2;
        rect2.setPosition(200, 200);
        rect2.setSize(sf::Vector2f(300, 300));
        rect2.setFillColor({0, 0, 0});
        shadow_texture.draw(rect2);
        shadow_texture.display();
        blurTexture(shadow_texture, GFX_SHADOW_BLUR);
        rect2.setFillColor({0, 0, 0, 0});
        shadow_texture.draw(rect2, sf::BlendNone);
        shadow_texture.display();
    }

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

void gfx::setMinimumWindowSize(int width, int height) {
    if(width < 0 || height < 0)
        throw ValueException("Window width and height must be positive.");
    min_window_width = width;
    min_window_height = height;
}

void gfx::loadFont(const std::string& path, int size) {
    if(size < 0)
        throw ValueException("Font size must be positive.");
    if(!font.loadFromFile(resource_path + path))
        throw LoadException(resource_path + path);
    font_size = size;
}

void gfx::quit() {
    delete window;
}

int gfx::getWindowWidth() {
    return window->getSize().x / global_scale;
}

int gfx::getWindowHeight() {
    return window->getSize().y / global_scale;
}

void gfx::setRenderTarget(Texture& texture) {
    render_target->display();
    render_target = texture.sfml_render_texture;
}

void gfx::resetRenderTarget() {
    render_target->display();
    render_target = &window_texture;
}

int gfx::getTicks() {
    return global_clock.getElapsedTime().asMilliseconds();
}

void applyShader(const sf::Shader& shader, sf::RenderTexture& output) {
    //output.generateMipmap(); // without that it doesnt work on smaller textures
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

void gfx::blurTexture(sf::RenderTexture& texture, float blur_intensity) {
    if(blur_intensity < 0)
        throw ValueException("Blur intensity must be positive.");
    
    blur_intensity = std::pow(2, blur_intensity);
    blur_shader.setUniform("source", texture.getTexture());
    
    while(blur_intensity >= 1.f) {
        blur_shader.setUniform("offset", sf::Vector2f(blur_intensity / texture.getSize().x, 0));
        applyShader(blur_shader, texture);
        
        blur_shader.setUniform("offset", sf::Vector2f(0, blur_intensity / texture.getSize().y));
        applyShader(blur_shader, texture);
        
        blur_intensity /= 2;
    }
}

void gfx::sleep(int ms) {
    if(ms < 0)
        throw ValueException("Milliseconds of sleep must be positive.");
    sf::sleep(sf::milliseconds(ms));
}

void gfx::setGlobalScale(float scale) {
    if(scale <= 0)
        throw ValueException("Scale must be positive.");
    global_scale = scale;
    setWindowSize(getWindowWidth(), getWindowHeight());
}

void gfx::setFpsLimit(int limit) {
    if(limit < 0)
        throw ValueException("Fps limit must be positive.");
    window->setFramerateLimit(limit);
}

void gfx::enableVsync(bool enabled) {
    window->setVerticalSyncEnabled(enabled);
}

void gfx::setWindowSize(int width, int height) {
    if(width < 0 || height < 0)
        throw ValueException("Window width and height must be positive.");
    
    width *= global_scale;
    height *= global_scale;
    
    if(width < min_window_width * global_scale)
        width = min_window_width * global_scale;
    if(height < min_window_height * global_scale)
        height = min_window_height * global_scale;
    
    sf::FloatRect visibleArea(0, 0, width / global_scale, height / global_scale);
    window->setView(sf::View(visibleArea));
    window->setSize({(unsigned int)width, (unsigned int)height});
    window_texture.create(width / global_scale, height / global_scale);
}

std::string gfx::getResourcePath() {
    return resource_path;
}

void gfx::loadIconFromFile(const std::string& path) {
    sf::Image icon;
    if(!icon.loadFromFile(path))
        throw LoadException(path);
    window->setIcon(icon.getSize().x, icon.getSize().y, icon.getPixelsPtr());
}
