#pragma once
#include "expression.hpp"

class ConstantIntegerType : public ValueType {
public:
    void print(Value* value, int depth) override;
    Value* parse(const Token*& curr_token) override;
};

class ConstantInteger : public Value {
public:
    ConstantInteger(ValueType* type) : Value(type) {}
    int value;
};
