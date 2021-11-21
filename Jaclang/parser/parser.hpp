#pragma once
#include "lexer.hpp"

class ProgramLine;

class ProgramLineType {
public:
    int id;
    virtual void print(ProgramLine* line, int depth) = 0;
    virtual ProgramLine* parse(const Token*& curr_token) = 0;
};

class ProgramLine {
public:
    ProgramLine(ProgramLineType* type) : type(type) {}
    ProgramLineType* const type;
    virtual ~ProgramLine() {}
};

class Parser {
    std::vector<ProgramLine*> program_lines;
    std::vector<ProgramLineType*> program_line_types;
public:
    void parseTokens(const std::vector<Token>& tokens);
    const std::vector<ProgramLine*> getProgramLines();
    ProgramLineType* getProgramLineTypeByID(int id);
    void registerAProgramLineType(ProgramLineType* program_line_type);
};
