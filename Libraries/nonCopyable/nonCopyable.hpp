#pragma once

class NonCopyable {
public:
    NonCopyable(const NonCopyable&) = delete;
    NonCopyable() = default;
    NonCopyable& operator=(const NonCopyable&) = delete;
};
