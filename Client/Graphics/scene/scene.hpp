#pragma once
#include "../graphics-internal.hpp"
#include <vector>
enum key { KEY_MOUSE_LEFT, KEY_MOUSE_RIGHT, KEY_MOUSE_MIDDLE, KEY_A, KEY_B, KEY_C, KEY_D, KEY_E, KEY_F, KEY_G, KEY_H, KEY_I, KEY_J, KEY_K, KEY_L, KEY_M, KEY_N, KEY_O, KEY_P, KEY_Q, KEY_R, KEY_S, KEY_T, KEY_U, KEY_V, KEY_W, KEY_X, KEY_Y, KEY_Z, KEY_0, KEY_1, KEY_2, KEY_3, KEY_4, KEY_5, KEY_6, KEY_7, KEY_8, KEY_9, KEY_SPACE, KEY_ESCAPE, KEY_ENTER, KEY_SHIFT, KEY_UNKNOWN };
class GraphicalModule {
public:
	~GraphicalModule();
    virtual void init();
    virtual void update();
    virtual void render();
    virtual void stop();
    virtual void onKeyDown(key key_);
    virtual void onKeyUp(key key_);

    std::vector<TextInput*> text_inputs;
    bool disable_events = false;
};

class GraphicalModule : Scene {
public:
    std::vector<GraphicalModule> modules;

    void run();

    void onKeyDownCallback(key key_);
    void onKeyUpCallback(key key_);
};