#pragma once
#include "parser.hpp"

enum class OperatorType { NONE, EQUALS, PLUS, MINUS, };

class Expression : public ProgramLine {
public:
    int value;
    OperatorType operator_type;
    Expression* next;
};

class ExpressionType : public ProgramLineType {
public:
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
};

namespace ProgramLineTypes {
    inline ExpressionType expression;
};
