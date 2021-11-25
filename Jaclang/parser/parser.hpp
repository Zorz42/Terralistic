#pragma once
#include "lexer.hpp"
#include "virtualMachine.hpp"

class ProgramLine;

class ProgramLineType {
public:
    virtual void print(ProgramLine* line, int depth) = 0;
    virtual ProgramLine* parse(const Token*& curr_token) = 0;
    virtual std::vector<Instruction*> toInstructions(ProgramLine* line) = 0;
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
    void registerAProgramLineType(ProgramLineType* program_line_type);
};
