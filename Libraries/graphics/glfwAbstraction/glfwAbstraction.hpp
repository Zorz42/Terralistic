#pragma once
#include "events.hpp"


#define GL_SILENCE_DEPRECATION
extern "C" {
#include <glad/glad.h>
}
#include <GLFW/glfw3.h>

namespace gfx {

void setMinimumWindowSize(int width, int height);
int getWindowWidth();
int getWindowHeight();

void enableVsync(bool enabled);

void setGlobalScale(float scale);


class _ScreenRefreshEvent {};
inline EventSender<_ScreenRefreshEvent> _screen_refresh_event_sender;

};

#ifndef GRAPHICS_PUBLIC

#include <string>
#include "transformation.hpp"

#define SHADER_VERTEX_BUFFER 0
#define SHADER_COLOR_BUFFER 1
#define SHADER_TEXTURE_COORD_BUFFER 2

namespace gfx {

void initGlfw(int window_width, int window_height, const std::string& window_title);
void quitGlfw();

inline GLFWwindow* glfw_window;

inline unsigned int shader_program;

inline int uniform_has_color_buffer, uniform_default_color, uniform_has_texture,
uniform_texture_sampler, uniform_transform_matrix, uniform_texture_transform_matrix;

inline unsigned int rect_vertex_buffer, rect_outline_vertex_buffer;

inline float global_scale_x = 1, global_scale_y = 1, system_scale_x = 1, system_scale_y = 1;

inline _Transformation window_normalization_transform, normalization_transform;

inline unsigned int window_texture, window_texture_back, default_framebuffer;

inline int window_width_min, window_height_min;

inline int window_resized_counter = 0;

unsigned int CompileShaders(const char* vertex_code, const char* fragment_code);

void keyCallback(GLFWwindow* window, int key, int scancode, int action, int mods);
void mouseButtonCallback(GLFWwindow* window, int button, int action, int mods);
void scrollCallback(GLFWwindow* window, double xoffset, double yoffset);
void characterCallback(GLFWwindow* window, unsigned int codepoint);

void updateWindow();

std::string getClipboard();
void setClipboard(const std::string& data);

inline bool is_window_focused = true;

};

#endif
