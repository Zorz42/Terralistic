#include "camera.hpp"

void Camera::setX(int x_) {
    target_x = x_;
}

void Camera::setY(int y_) {
    target_y = y_;
}

int Camera::getX() {
    return x;
}

int Camera::getY() {
    return y;
}

int Camera::getTargetX() {
    return target_x;
}

int Camera::getTargetY() {
    return target_y;
}

void Camera::update(float frame_length) {
    x += (target_x - x) / 8;
    y += (target_y - y) / 8;
}

int Camera::getViewBeginX() {
    return (int)x - gfx::getWindowWidth() / 2;
}

int Camera::getViewEndX() {
    return (int)x + gfx::getWindowWidth() / 2;
}

int Camera::getViewBeginY() {
    return (int)y - gfx::getWindowHeight() / 2;
}

int Camera::getViewEndY() {
    return (int)y + gfx::getWindowHeight() / 2;
}

void Camera::jumpToTarget() {
    x = target_x;
    y = target_y;
}
