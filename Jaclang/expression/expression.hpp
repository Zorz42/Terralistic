#pragma once
#include "parser.hpp"

enum class OperatorType { NONE, EQUALS, PLUS, MINUS, };

class Value {
public:
    Value(int type_id) : type_id(type_id) {}
    int type_id;
    virtual ~Value() {}
};

class ValueType {
public:
    int id;
    virtual void print(Value* value, int depth) = 0;
    virtual Value* parse(const Token*& curr_token) = 0;
};

class ExpressionType : public ProgramLineType {
    std::vector<ValueType*> value_types;
public:
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    void registerAValueType(ValueType* value_type);
    ValueType* getValueTypeByID(int id);
};

namespace ProgramLineTypes {
    inline ExpressionType expression;
};

class Expression : public ProgramLine {
public:
    Expression() : ProgramLine(ProgramLineTypes::expression.id) {}
    Value* value;
    OperatorType operator_type;
    Expression* next;
};
