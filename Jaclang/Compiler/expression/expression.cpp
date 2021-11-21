#include <iostream>
#include "expression.hpp"
#include "error.hpp"

void ExpressionType::print(ProgramLine* line, int depth) {
    Expression* expression = (Expression*)line;
    
    for(int i = 0; i < depth; i++)
        std::cout << '\t';
    std::cout << "expression:" << std::endl;
    depth++;
    
    while(true) {
        for(int i = 0; i < depth; i++)
            std::cout << '\t';
        std::cout << expression->value << std::endl;
        
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
    if(curr_token->type != TokenType::CONSTANT_INTEGER)
        return nullptr;
    
    Expression *expression = new Expression, *curr_expression = expression;
    
    while(true) {
        if(curr_token->type != TokenType::CONSTANT_INTEGER)
            throw Error("Expected value after operator.");
        
        curr_expression->value = curr_token->const_int;
        curr_token++;
        
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
        
        curr_expression->next = new Expression;
        curr_expression = curr_expression->next;
        
        curr_token++;
    }
    
    return expression;
}
