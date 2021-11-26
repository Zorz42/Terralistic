#include <iostream>
#include "expression.hpp"
#include "error.hpp"

ExpressionType::ExpressionType(VirtualMachine* virtual_machine) : virtual_machine(virtual_machine) {
    virtual_machine->registerAnInstructionType(&add_instruction);
    virtual_machine->registerAnInstructionType(&sub_instruction);
}

void ExpressionType::print(ProgramLine* line, int depth) {
    Expression* expression = (Expression*)line;
    
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "expression:" << std::endl;
    depth++;
    
    while(true) {
        if(expression->operator_type != OperatorType::NONE)
            for(int i = 0; i < depth; i++)
                std::cout << '\t';
        switch(expression->operator_type) {
            case OperatorType::PLUS:
                std::cout << "+";
                break;
            case OperatorType::MINUS:
                std::cout << "-";
                break;
            default:;
        }
        
        if(expression->operator_type != OperatorType::NONE)
            std::cout << std::endl;
        
        expression->value->type->print(expression->value, depth);
        
        if(!expression->next)
            break;
        
        expression = expression->next;
    }
}

ProgramLine* ExpressionType::parse(const Token*& curr_token) {
    Expression *expression = new Expression(this), *curr_expression = expression;
    
    while(true) {
        Value* value = nullptr;
        for(ValueType* value_type : value_types) {
            value = value_type->parse(curr_token);
            if(value)
                break;
        }
        
        if(!value)
            throw Error("Expected value after operator.") ;
        
        curr_expression->value = value;
        
        OperatorType operator_type = OperatorType::NONE;
        
        switch(curr_token->type) {
            case TokenType::PLUS:
                operator_type = OperatorType::PLUS;
                break;
            case TokenType::MINUS:
                operator_type = OperatorType::MINUS;
                break;
            default:;
        }
        
        if(operator_type == OperatorType::NONE)
            return expression;
        
        curr_expression->next = new Expression(this);
        curr_expression = curr_expression->next;
        curr_expression->operator_type = operator_type;
        
        curr_token++;
    }
    
    return expression;
}

void ExpressionType::registerAValueType(ValueType* value_type) {
    value_type->id = (int)value_types.size();
    value_types.push_back(value_type);
}

ValueType* ExpressionType::getValueTypeByID(int id) {
    return value_types[id];
}

std::vector<Instruction*> ExpressionType::toInstructions(ProgramLine* line) {
    Expression* expression = (Expression*)line;
    std::vector<Instruction*> instructions;

    Instruction* new_instruction = nullptr;
    switch(expression->operator_type) {
        case OperatorType::PLUS:
            new_instruction = new AdditionInstruction(&add_instruction);
            break;
            
        case OperatorType::MINUS:
            new_instruction = new SubtractionInstruction(&sub_instruction);
            break;
            
        default:;
    }
    
    std::vector<Instruction*> value_instructions = expression->value->type->toInstructions(expression->value);
    instructions.insert(instructions.end(), value_instructions.begin(), value_instructions.end());
    
    if(new_instruction)
        instructions.push_back(new_instruction);
    
    if(expression->next) {
        std::vector<Instruction*> next_instructions = expression->type->toInstructions(expression->next);
        instructions.insert(instructions.end(), next_instructions.begin(), next_instructions.end());
    }
    
    return instructions;
}

void AdditionInstructionType::print(Instruction* instruction) {
    std::cout << "ADD" << std::endl;
}

void SubtractionInstructionType::print(Instruction* instruction) {
    std::cout << "SUB" << std::endl;
}
