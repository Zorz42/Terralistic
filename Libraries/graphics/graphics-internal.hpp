#pragma once
#include "graphics.hpp"

namespace gfx {

    inline sf::RenderWindow *window = nullptr;
    inline sf::RenderTexture window_texture;
    inline sf::Font font;

    inline int font_size;
    inline sf::RenderTexture *render_target = nullptr;

    inline float global_scale = 1;

    inline sf::Shader blur_shader;

    void blurTexture(sf::RenderTexture& texture, float blur_intensity);

    inline sf::RenderTexture shadow_texture, shadow_part_left, shadow_part_right, shadow_part_up, shadow_part_down;
    
    inline std::string resource_path;

    inline bool key_states[(int)gfx::Key::UNKNOWN];

    inline std::vector<GlobalUpdateFunction*> global_update_functions;
}
