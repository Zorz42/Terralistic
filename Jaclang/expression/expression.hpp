#pragma once
#include "parser.hpp"

enum class OperatorType { NONE, EQUALS, PLUS, MINUS, };

class Value;

class ValueType {
public:
    int id;
    virtual void print(Value* value, int depth) = 0;
    virtual Value* parse(const Token*& curr_token) = 0;
    virtual std::vector<Instruction*> toInstructions(Value* value) = 0;
};

class Value {
public:
    Value(ValueType* type) : type(type) {}
    ValueType* const type;
    virtual ~Value() {}
};


class AdditionInstructionType : public InstructionType {
public:
    void print(Instruction* instruction) override;
};

class AdditionInstruction : public Instruction {
public:
    AdditionInstruction(AdditionInstructionType* type) : Instruction(type) {}
};

class SubtractionInstructionType : public InstructionType {
public:
    void print(Instruction* instruction) override;
};

class SubtractionInstruction : public Instruction {
public:
    SubtractionInstruction(SubtractionInstructionType* type) : Instruction(type) {}
};


class ExpressionType : public ProgramLineType {
    std::vector<ValueType*> value_types;
    VirtualMachine* virtual_machine;
    
    AdditionInstructionType add_instruction;
    SubtractionInstructionType sub_instruction;
public:
    ExpressionType(VirtualMachine* virtual_machine);
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    std::vector<Instruction*> toInstructions(ProgramLine* line) override;
    void registerAValueType(ValueType* value_type);
    ValueType* getValueTypeByID(int id);
};

class Expression : public ProgramLine {
public:
    Expression(ProgramLineType* type) : ProgramLine(type) {}
    OperatorType operator_type = OperatorType::NONE;
    Value* value;
    Expression* next = nullptr;
};
