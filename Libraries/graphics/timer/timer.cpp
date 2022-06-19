#include "timer.hpp"

gfx::Timer::Timer() {
    reset();
}

float gfx::Timer::getTimeElapsed() const {
    return std::chrono::duration_cast<std::chrono::microseconds>(std::chrono::steady_clock::now() - start_time).count() / 1000.f;
}

void gfx::Timer::reset() {
    start_time = std::chrono::steady_clock::now();
}
