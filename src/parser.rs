use crate::{token::{Token, TokenType}, tree::Statement};

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = vec![];
    let mut state = ParseState::None;
    for token in tokens {
        match token.ty {
            TokenType::Identifier => {},
            TokenType::NewLine => continue,
            _ => break
        }
    }

    statements
}

enum ParseState {
    Statement,
    ImportPackage,
    DefineType,
    DefineFunction,
    PackagePath,
    Identifier,
    GenericTemplate,
    TypeName,
    TypeBlock,
    TypeCloumn,
    FunctionName,
    FunctionParamters,
    FunctionReturnType,
    FunctionTypeSetting,
    FunctionBlock,
    Atom,
    Paramter,
    FunctionTypeColumn,
    FunctionStatement,
    DefineVariable,
    DefineClosure,
    DefineFact,
    CallFunction,
    FactCheck,
    Value,
    FactState,
    FactCondition,
    FactArrayState,
    FactSimpleState,
    FactStructState,
    ConstantValue,
    String,
    Char,
    Number,
    Array,
    Struct,
    StructColumnPair,
    None
}