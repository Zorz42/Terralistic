#pragma once
#include "parser.hpp"

enum class OperatorType { NONE, EQUALS, PLUS, MINUS, };

class Value;

class ValueType {
public:
    int id;
    virtual void print(Value* value, int depth) = 0;
    virtual Value* parse(const Token*& curr_token) = 0;
};

class Value {
public:
    Value(ValueType* type) : type(type) {}
    ValueType* const type;
    virtual ~Value() {}
};

class ExpressionType : public ProgramLineType {
    std::vector<ValueType*> value_types;
public:
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    void registerAValueType(ValueType* value_type);
    ValueType* getValueTypeByID(int id);
};

class Expression : public ProgramLine {
public:
    Expression(ProgramLineType* type) : ProgramLine(type) {}
    Value* value;
    OperatorType operator_type;
    Expression* next;
};
