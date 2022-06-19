#pragma once
#include "graphics.hpp"

#define GL_SILENCE_DEPRECATION
#include <glad/glad.h>
#include <GLFW/glfw3.h>

#define SHADER_VERTEX_BUFFER 0
#define SHADER_COLOR_BUFFER 1
#define SHADER_TEXTURE_COORD_BUFFER 2

namespace gfx {

inline std::vector<GlobalUpdateFunction*> global_update_functions;
inline float frame_length;

inline bool key_states[(int)gfx::Key::UNKNOWN];

inline int uniform_blur_transform_matrix,
uniform_blur_texture_transform_matrix, uniform_blur_texture_sampler,
uniform_blur_offset, uniform_blur_limit;

inline Scene* curr_scene = nullptr;

void blurRectangle(RectShape rect, int radius, unsigned int texture, unsigned int back_texture, float width, float height, _Transformation texture_transform);

inline Texture* font_texture = nullptr;
inline RectShape font_rects[256];

inline Texture* shadow_texture = nullptr;

inline bool updated_back_window_texture = false;
}
