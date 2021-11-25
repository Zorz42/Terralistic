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
        expression->value->type->print(expression->value, depth);
        
        if(expression->operator_type == OperatorType::NONE)
            break;

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
        std::cout << std::endl;
        
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
        
        switch(curr_token->type) {
            case TokenType::PLUS:
                curr_expression->operator_type = OperatorType::PLUS;
                break;
            case TokenType::MINUS:
                curr_expression->operator_type = OperatorType::MINUS;
                break;
            default:
                curr_expression->operator_type = OperatorType::NONE;
                break;
        }
        
        if(curr_expression->operator_type == OperatorType::NONE) {
            curr_expression->next = nullptr;
            return expression;
        }
        
        curr_expression->next = new Expression(this);
        curr_expression = curr_expression->next;
        
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
    return {};
}

void AdditionInstructionType::print(Instruction* instruction) {
    std::cout << "ADD" << std::endl;
}

void SubtractionInstructionType::print(Instruction* instruction) {
    std::cout << "SUB" << std::endl;
}
