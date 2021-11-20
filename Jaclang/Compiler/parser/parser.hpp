#pragma once
#include "lexer.hpp"

class ProgramLine {
public:
    
};

class Expression : public ProgramLine {
public:
    
};

class Parser {
public:
    std::vector<ProgramLine> parseTokens(const std::vector<Token>& tokens);
};
