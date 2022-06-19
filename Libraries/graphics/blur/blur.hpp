#pragma once

namespace gfx {

inline bool blur_enabled = true;

};

#ifndef GRAPHICS_PUBLIC

#include "rectShape.hpp"
#include "transformation.hpp"

namespace gfx {

void initBlur();

inline unsigned int blur_shader_program;

inline int uniform_blur_transform_matrix,
uniform_blur_texture_transform_matrix, uniform_blur_texture_sampler,
uniform_blur_offset, uniform_blur_limit;

void blurRectangle(RectShape rect, int radius, unsigned int texture, unsigned int back_texture, float width, float height, _Transformation texture_transform);

};

#endif
