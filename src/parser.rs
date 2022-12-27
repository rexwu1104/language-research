
use crate::{token::{Token, TokenType}, tree::{Statement, PackagePath, GenericTemplate, TypeBlock, TypeCloumn}};

macro_rules! get_last {
    ($collection:ident) => {
        $collection.iter().last().unwrap()
    };
}

macro_rules! set_state {
    ($state:ident, $states:ident, $value:ident) => {
        $state = ParseState::$value;
        $states.push($state.clone());
    };
}

macro_rules! generate_statement {
    ($type:ident, $tokens:ident, $states:ident) => {
        match stringify!($type) {
            "import_package" => generate_import_package($tokens.clone(), &mut $states),
            "define_type" => generate_define_type($tokens.clone(), &mut $states),
            "define_function" => generate_define_function($tokens.clone(), &mut $states),
            _ => break
        }
    };
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut statements = vec![];
    let mut buffer = vec![];
    let mut state = ParseState::Statement;
    let mut states = vec![state.clone()];
    // println!("{tokens:?}");
    for token in tokens {
        // println!("{} {state:?}", token.raw);
        match (token.clone().ty, state.clone()) {
            (TokenType::Identifier, ParseState::Statement) => {
                match token.raw.as_str() {
                    "use" => { set_state!(state, states, ImportPackage); },
                    "type" => { set_state!(state, states, DefineType); },
                    "fn" => { set_state!(state, states, DefineFunction); },
                    _ => break
                }
            }, // statement start
            (TokenType::Identifier | TokenType::Symbol | TokenType::NewLine, ParseState::ImportPackage) => match token.ty {
                TokenType::Symbol => if token.raw != "." { break },
                TokenType::Identifier => buffer.push(token),
                TokenType::NewLine => {
                    statements.push(generate_statement!(import_package, buffer, states));
                    buffer.clear();
                    state = states.iter().last().unwrap().clone();
                },
                _ => break
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::DefineType) => match token.ty {
                TokenType::Identifier => {
                    buffer.push(token.clone());
                    set_state!(state, states, TypeBlock);
                },
                TokenType::Symbol => if token.raw != "<" { break } else {
                    buffer.push(token.clone());
                    set_state!(state, states, TypeGenericTemplate);
                },
                _ => break
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::TypeGenericTemplate) => match token.ty {
                TokenType::Symbol => if token.raw != ">" && token.raw != "," { break } else if token.raw == ">" {
                    println!("{token:#?}");
                    buffer.push(token.clone());
                    set_state!(state, states, TypeName);
                } else if get_last!(buffer).raw == "," { break },
                TokenType::Identifier => if get_last!(buffer).raw == "<" || get_last!(buffer).raw == "," {
                    println!("{token:#?}");
                    buffer.push(token.clone());
                } else { break },
                _ => break
            },
            (TokenType::Identifier, ParseState::TypeName) => {
                println!("hello");
                buffer.push(token.clone());
                set_state!(state, states, TypeBlock);
            }
            (TokenType::Symbol, ParseState::TypeBlock) => {
                if token.raw != "{" { break }
                set_state!(state, states, TypeCloumnName);
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::TypeCloumnName) => match token.ty {
                TokenType::Identifier => {
                    buffer.push(token.clone());
                    set_state!(state, states, TypeCloumnType);
                },
                TokenType::Symbol => if token.raw != "}" { break } else {
                    states.pop();
                    statements.push(generate_statement!(define_type, buffer, states));
                    buffer.clear();
                    state = states.iter().last().unwrap().clone();
                },
                _ => break
            },
            (TokenType::Symbol | TokenType::Identifier, ParseState::TypeCloumnType) => match token.ty {
                TokenType::Symbol => if token.raw != ":" || get_last!(buffer).raw == ":" { break } else { buffer.push(token.clone()) },
                TokenType::Identifier => {
                    buffer.push(token.clone());
                    set_state!(state, states, TypeCloumnName);
                },
                _ => break
            },
            (TokenType::Identifier | TokenType::Atom, ParseState::DefineFunction) => {
                buffer.push(token.clone());
                set_state!(state, states, FunctionParamters);
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::FunctionParamters) => match token.ty {
                TokenType::Identifier => buffer.push(token.clone()),
                TokenType::Symbol => if token.raw != "-" && token.raw != ">" { break }
                else if token.raw == "-" && get_last!(buffer).ty == TokenType::Identifier {
                    buffer.push(token.clone());
                } else if token.raw == ">" && get_last!(buffer).raw == "-" {
                    buffer.push(token.clone());
                    set_state!(state, states, FunctionReturnType);
                },
                _ => break
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::FunctionReturnType) => match token.ty {
                TokenType::Symbol => if token.raw != "{" { break } else {
                    buffer.push(token.clone());
                    set_state!(state, states, FunctionBlock);
                },
                TokenType::Identifier => {
                    buffer.push(token.clone());
                    states.push(state.clone());
                },
                _ => break
            },
            (TokenType::Identifier | TokenType::Symbol, ParseState::FunctionBlock) => match token.raw.as_str() {
                "fn" => { set_state!(state, states, DefineFunction); }, // closure
                "fact" => { set_state!(state, states, DefineFact); }, // fact
                "for" | "while" | "loop" => {}, // loop
                "if" => {}, // codition
                "}" => { statements.push(generate_statement!(define_function, buffer, states)) },
                _ => {} // variable or call
            },
            (TokenType::NewLine, _) => continue,
            _ => break
        }
    }

    statements
}

#[derive(Debug, Clone)]
enum ParseState {
    Statement,
    ImportPackage,
    DefineType,
    DefineFunction,
    TypeGenericTemplate,
    TypeName,
    TypeBlock,
    TypeCloumnName,
    TypeCloumnType,
    FunctionGenericTemplate,
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
    StructColumnPair
}

fn generate_import_package(tokens: Vec<Token>, states: &mut Vec<ParseState>) -> Statement {
    states.pop();
    Statement::ImportPackage(PackagePath(tokens.iter().map(|t| t.raw.clone()).collect()))
}

fn generate_define_type(mut tokens: Vec<Token>, states: &mut Vec<ParseState>) -> Statement {
    println!("{states:#?}");
    let mut columns = vec![];
    let mut generics = vec![];
    if tokens[0].raw == "<" { for token in tokens.clone() {
        if token.raw == ">" { tokens.remove(0); break }
        if token.raw == "<" { tokens.remove(0); continue }
        generics.push(token.raw.clone()); tokens.remove(0);
    } states.remove(1); states.remove(1); }

    let name = tokens[0].clone(); tokens.remove(0); states.remove(1);

    states.remove(1);
    for column in tokens.chunks(3) {
        if column[1].raw != ":" { break }
        states.remove(1); states.remove(1);
        columns.push(TypeCloumn(column[0].raw.clone(), column[2].raw.clone()));
    }

    Statement::DefineType(GenericTemplate(generics), name.raw.clone(), TypeBlock(columns))
}

fn generate_define_function(tokens: Vec<Token>, states: &mut Vec<ParseState>) -> Statement {
    // println!("{tokens:#?}\n{states:#?}");
    Statement::ImportPackage(PackagePath(vec![]))
}