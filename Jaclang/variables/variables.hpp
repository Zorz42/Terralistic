#pragma once
#include "expression.hpp"

class Variable {
public:
    Variable(const std::string& name, int id) : name(name), id(id) {}
    const std::string name;
    const int id;
};

class VariableManager {
    std::vector<Variable> variables;
public:
    Variable& getVariableByID(int id);
    Variable& getVariableByName(const std::string& name);
    int getVariableIdByName(const std::string& name);
    bool variableExists(const std::string& name);
    Variable& addVariable(const std::string& name);
};

class VariableDeclarationType : public ProgramLineType {
    VariableManager* variable_manager;
public:
    VariableDeclarationType(VariableManager* variable_manager) : variable_manager(variable_manager) {}
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    std::vector<Instruction*> toInstructions(ProgramLine* line) override;
};

class VariableSettingInstructionType : public InstructionType {
public:
    void print(Instruction* instruction) override;
};

class VariableSettingInstruction : public Instruction {
public:
    VariableSettingInstruction(VariableSettingInstructionType* type) : Instruction(type) {}
    int variable_id;
};

class VariableSettingType : public ProgramLineType {
    VariableManager* variable_manager;
    ExpressionType* expression_type;
    VariableSettingInstructionType variable_setting_instruction;
public:
    VariableSettingType(VariableManager* variable_manager, ExpressionType* expression_type) : variable_manager(variable_manager), expression_type(expression_type) {}
    void print(ProgramLine* line, int depth) override;
    ProgramLine* parse(const Token*& curr_token) override;
    std::vector<Instruction*> toInstructions(ProgramLine* line) override;
};

class VariableDeclaration : public ProgramLine {
public:
    VariableDeclaration(ProgramLineType* type) : ProgramLine(type) {}
    int variable_id;
};

class VariableSetting : public ProgramLine {
public:
    VariableSetting(ProgramLineType* type) : ProgramLine(type) {}
    int variable_id;
    Expression* value;
};

