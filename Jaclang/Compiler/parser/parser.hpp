#pragma once
#include "lexer.hpp"

class ProgramLine {
public:
    int type_id;
    virtual ~ProgramLine() {}
};

class ProgramLineType {
public:
    int id;
    virtual void print(ProgramLine* line) = 0;
    virtual ProgramLine* parse(const Token*& curr_token) = 0;
};

enum class OperatorType { NONE, EQUALS, PLUS, MINUS, };

class Expression : public ProgramLine {
public:
    int first_value;
    std::vector<std::pair<OperatorType, int>> values;
};

class ExpressionType : public ProgramLineType {
public:
    void print(ProgramLine* line) override;
    ProgramLine* parse(const Token*& curr_token) override;
};

namespace ProgramLineTypes {
    inline ExpressionType expression;
};

class Parser {
    std::vector<ProgramLine*> program_lines;
    std::vector<ProgramLineType*> program_line_types;
public:
    Parser();
    void parseTokens(const std::vector<Token>& tokens);
    const std::vector<ProgramLine*> getProgramLines();
    ProgramLineType* getProgramLineTypeByID(int id);
    void registerAProgramLineType(ProgramLineType* program_line_type);
};
