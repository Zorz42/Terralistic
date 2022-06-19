#pragma once
#include <chrono>

namespace gfx {

void sleep(float ms);

class Timer {
    std::chrono::time_point<std::chrono::steady_clock> start_time;
public:
    Timer();
    float getTimeElapsed() const;
    void reset();
};

};
