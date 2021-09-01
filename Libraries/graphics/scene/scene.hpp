#ifndef scene_hpp
#define scene_hpp

#include "ui.hpp"
#include <vector>

namespace gfx {

enum class Key {MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, NUM0, NUM1, NUM2, NUM3, NUM4, NUM5, NUM6, NUM7, NUM8, NUM9, SPACE, ESCAPE, ENTER, SHIFT, BACKSPACE, CTRL, UNKNOWN};

class GraphicalModule {
public:
    bool _can_receive_events;
    
    virtual void init() {}
    virtual void update() {}
    virtual void render() {}
    virtual void stop() {}
    virtual void onKeyDown(Key key_) {}
    
    bool getKeyState(Key key_);
    
    virtual ~GraphicalModule() = default;

    std::vector<TextInput*> text_inputs;
    bool disable_events = false;
    unsigned short mouse_x, mouse_y;
};

class Scene : public GraphicalModule {
    void _operateEvent(sf::Event event);
public:
    void enableAllEvents(bool enable);
    
    std::vector<GraphicalModule*> modules;
    
    virtual void onMouseScroll(int distance) {}
    
    void run();

    void onKeyDownCallback(Key key_);
    
    unsigned short mouse_x, mouse_y;
};

}

#endif
