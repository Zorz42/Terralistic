#pragma once
#include "graphics.hpp"
#include "exception.hpp"

gfx::Surface readOpa(const std::string& path);

EXCEPTION_TYPE(OpaFileError)
