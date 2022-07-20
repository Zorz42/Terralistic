#pragma once
#include "graphics.hpp"
#include "exception.hpp"

void loadOpa(gfx::Texture& texture, const std::string& path);
void loadOpaSkinTemplate(std::vector<unsigned char>& skin_template, const std::string& path);

EXCEPTION_TYPE(OpaFileError)