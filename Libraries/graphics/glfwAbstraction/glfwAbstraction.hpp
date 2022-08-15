#pragma once
#include "events.hpp"


#define GL_SILENCE_DEPRECATION
extern "C" {
#include <glad/glad.h>
}
#include <GLFW/glfw3.h>
#define SYSTEM_SCALE 0

namespace gfx {

void setMinimumWindowSize(int width, int height);
int getWindowWidth();
int getWindowHeight();

void enableVsync(bool enabled);

void setGlobalScale(float scale);

enum class BlendMode { BLEND_ALPHA, BLEND_MULTIPLY };

void setBlendMode(BlendMode blend_mode);

enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, ARROW_UP, ARROW_DOWN, ARROW_LEFT, ARROW_RIGHT, UNKNOWN};

class _ScreenRefreshEvent {};

class _CharInputEvent {
public:
    _CharInputEvent(char code) : code(code) {}
    char code;
};

class _KeyPressEvent {
public:
    _KeyPressEvent(Key key) : key(key) {}
    Key key;
};

class _KeyReleaseEvent {
public:
    _KeyReleaseEvent(Key key) : key(key) {}
    Key key;
};

class _ScrollEvent {
public:
    _ScrollEvent(int delta) : delta(delta) {}
    int delta;
};

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

inline unsigned int shader_program;

inline int uniform_has_color_buffer, uniform_default_color, uniform_has_texture,
uniform_texture_sampler, uniform_transform_matrix, uniform_texture_transform_matrix;

inline unsigned int rect_vertex_buffer, rect_outline_vertex_buffer;

inline _Transformation normalization_transform;

inline unsigned int window_texture, window_texture_back;

unsigned int compileShaders(const char* vertex_code, const char* fragment_code);

inline EventSender<_ScreenRefreshEvent> screen_refresh_event_sender;
inline EventSender<_CharInputEvent> char_input_event_sender;
inline EventSender<_KeyPressEvent> key_press_event_sender;
inline EventSender<_KeyReleaseEvent> key_release_event_sender;
inline EventSender<_ScrollEvent> scroll_event_sender;

void updateWindow();

std::string getClipboard();
void setClipboard(const std::string& data);

inline bool is_window_focused = true;

int getMouseX();
int getMouseY();

bool isWindowClosed();

};

#endif
