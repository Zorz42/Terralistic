#include <thread>
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

void gfx::sleep(float ms) {
    std::this_thread::sleep_for(std::chrono::microseconds(int(ms * 1000)));
}

gfx::ModTimer::ModTimer(int modulo) : modulo(modulo) {
    set(0);
}

long long gfx::ModTimer::getTimeElapsed() const {
    return (std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() - start_time) % modulo;
}

void gfx::ModTimer::set(long long val) {
    start_time = (long)std::chrono::duration_cast<std::chrono::milliseconds>(std::chrono::system_clock::now().time_since_epoch()).count() - val;
}
