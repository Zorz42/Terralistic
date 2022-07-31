#include "camera.hpp"
#include "blocks.hpp"

void Camera::init() {
    debug_menu->registerDebugLine(&coords_debug_line);
}

void Camera::setX(int x_) {
    target_x = x_;
}

void Camera::setY(int y_) {
    target_y = y_;
}

int Camera::getX() const {
    return x;
}

int Camera::getY() const {
    return y;
}

int Camera::getTargetX() const {
    return target_x;
}

int Camera::getTargetY() const {
    return target_y;
}

void Camera::update(float frame_length) {
    while(timer_counter < timer.getTimeElapsed()) {
        timer_counter++;
        x += (target_x - x) / 100;
        y += (target_y - y) / 100;
    }
    
    coords_debug_line.text = std::string("X: ") + std::to_string(int(x / (BLOCK_WIDTH * 2))) + ", Y: " + std::to_string(int(y / (BLOCK_WIDTH * 2)));
}

int Camera::getViewBeginX() const {
    return (int)x - gfx::getWindowWidth() / 2;
}

int Camera::getViewEndX() const {
    return (int)x + gfx::getWindowWidth() / 2;
}

int Camera::getViewBeginY() const {
    return (int)y - gfx::getWindowHeight() / 2;
}

int Camera::getViewEndY() const {
    return (int)y + gfx::getWindowHeight() / 2;
}

void Camera::jumpToTarget() {
    x = target_x;
    y = target_y;
}
