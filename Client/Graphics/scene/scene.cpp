#include "scene.hpp"


//Scene
void Scene::onKeyDownCallback(key key_) {
    if(!disable_events_gl || disable_events)
        onKeyDown(key_);
    for(GraphicalModule module : modules)
        if(!disable_events_gl || module.disable_events)
            module.onKeyDown(key_);
}
void Scene::onKeyUpCallback(key key_) {
    if (!disable_events_gl || disable_events)
        onKeyUp(key_);
    for (GraphicalModule module : modules)
        if (!disable_events_gl || module.disable_events)
            module.onKeyUp(key_);
 }