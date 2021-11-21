#pragma once
#include "lexer.hpp"

class ProgramLine {
public:
    ProgramLine(int type_id) : type_id(type_id) {}
    const int type_id;
    virtual ~ProgramLine() {}
};

class ProgramLineType {
public:
    int id;
    virtual void print(ProgramLine* line, int depth) = 0;
    virtual ProgramLine* parse(const Token*& curr_token) = 0;
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
