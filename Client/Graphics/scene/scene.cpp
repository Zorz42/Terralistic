#include "scene.hpp"

Scene::onKeyDownCallback(key key_) {
    if (!disable_events_gl || disable_events)
        onKeyDown(key_);
    for (sceneModule* module : modules)
        if (!disable_events_gl || module->disable_events)
            module->onKeyDown(key_);
}