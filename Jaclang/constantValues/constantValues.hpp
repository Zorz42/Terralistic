#pragma once
#include "expression.hpp"

class IntegerLoadInstructionType : public InstructionType {
public:
    void print(Instruction* instruction) override;
};

class IntegerLoadInstruction : public Instruction {
public:
    IntegerLoadInstruction(IntegerLoadInstructionType* type) : Instruction(type) {}
    int value;
};

class ConstantIntegerType : public ValueType {
    IntegerLoadInstructionType int_load_instruction;
public:
    void print(Value* value, int depth) override;
    Value* parse(const Token*& curr_token) override;
    std::vector<Instruction*> toInstructions(Value* value) override;
};

class ConstantInteger : public Value {
public:
    ConstantInteger(ValueType* type) : Value(type) {}
    int value;
};
