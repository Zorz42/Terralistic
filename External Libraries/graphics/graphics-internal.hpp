#pragma once
#include "graphics.hpp"

#define SHADER_VERTEX_BUFFER 0
#define SHADER_COLOR_BUFFER 1
#define SHADER_TEXTURE_COORD_BUFFER 2

namespace gfx {

inline GLFWwindow* glfw_window;
inline GLuint vertex_array_id;
inline GLuint shader_program, blur_shader_program;
inline int window_width, window_height;
inline float window_width_reciprocal, window_height_reciprocal;

inline std::vector<GlobalUpdateFunction*> global_update_functions;
inline float frame_length;
inline float global_scale_x = 1, global_scale_y = 1, system_scale_x = 1, system_scale_y = 1;

inline bool key_states[(int)gfx::Key::UNKNOWN];

inline GLuint uniform_has_color_buffer, uniform_default_color, uniform_has_texture,
uniform_texture_sampler, uniform_transform_matrix, uniform_texture_transform_matrix,
uniform_back_texture_sampler, uniform_blend_multiply, uniform_blur_transform_matrix,
uniform_blur_texture_transform_matrix, uniform_blur_texture_sampler,
uniform_blur_offset, uniform_blur_limit;
inline Transformation window_normalization_transform, normalization_transform;
inline GLuint window_texture, window_texture_back, default_framebuffer;
inline float global_scale = 0;

inline GLuint rect_vertex_buffer, rect_outline_vertex_buffer;

inline Scene* curr_scene = nullptr;
inline int window_resized_counter = 0;

void blurRectangle(RectShape rect, int radius);

void updateWindow();

inline Texture font_texture;
inline RectShape font_rects[256];

void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
void mouseButtonCallback(GLFWwindow* window, int button, int action, int mods);
void scrollCallback(GLFWwindow* window, double xoffset, double yoffset);
void characterCallback(GLFWwindow* window, unsigned int codepoint);

//inline sf::RenderTexture shadow_texture, shadow_part_left, shadow_part_right, shadow_part_up, shadow_part_down;
}
