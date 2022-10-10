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

class ModTimer {
    long long start_time;
public:
    ModTimer(int modulo);
    long long getTimeElapsed() const;
    void set(long long val);
    int modulo;
};

};
