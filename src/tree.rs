#[derive(Debug)]
pub enum Statement {
    ImportPackage(PackagePath),
    DefineType(GenericTemplate, TypeName, TypeBlock),
    DefineFunction(GenericTemplate, FunctionName, FunctionParamters, FunctionReturnType, FunctionTypeSetting, FunctionBlock)
}

#[derive(Debug)]
pub struct PackagePath(pub Vec<Identifier>);

#[derive(Debug)]
pub struct GenericTemplate(pub Vec<Identifier>);
pub type TypeName = Identifier;

#[derive(Debug)]
pub struct TypeBlock(pub Vec<TypeCloumn>);

#[derive(Debug)]
pub struct TypeCloumn(pub Identifier, pub Identifier);

#[derive(Debug)]
pub enum FunctionName {
    Operator(Value),
    Normal(Identifier)
}

#[derive(Debug)]
pub struct FunctionParamters(pub Vec<Paramter>);
pub type FunctionReturnType = Identifier;

#[derive(Debug)]
pub struct FunctionTypeSetting(pub Vec<FunctionTypeColumn>);

#[derive(Debug)]
pub struct FunctionTypeColumn(pub Identifier, pub Identifier);

#[derive(Debug)]
pub struct FunctionBlock(pub Vec<FunctionStatement>);

#[derive(Debug)]
pub enum FunctionStatement {
    DefineVariable(Identifier, Value),
    DefineClosure(GenericTemplate, FunctionName, FunctionParamters, FunctionReturnType, FunctionTypeSetting, FunctionBlock),
    DefineFact(Identifier, FactContent),
    CallFunction(Value, Vec<Value>),
    LoopUse(Box<Loop>),
    Condition(IfCondition, Vec<IfCondition>, ElseCondition)
}

#[derive(Debug)]
pub struct FactContent(pub Vec<FactCheck>);

#[derive(Debug)]
pub struct FactCheck(pub FactState, pub Option<FactCondition>);

#[derive(Debug)]
pub enum FactState {
    FactArrayState(ArrayState),
    FactSimpleState(ConstantValue),
    FactStructState(StructState)
}

#[derive(Debug)]
pub struct ArrayState(pub Vec<Paramter>);

#[derive(Debug)]
pub struct StructState(pub Identifier, pub Vec<Identifier>);
pub type FactCondition = Value;

#[derive(Debug)]
pub enum ConstantValue {
    String(String),
    Char(String),
    Atom(String),
    Number(String)
}

#[derive(Debug)]
pub enum Value {
    Constant(ConstantValue),
    Identifier(Identifier),
    Array(Array),
    Struct(Struct)
}

#[derive(Debug)]
pub struct Array(pub Vec<Value>);

#[derive(Debug)]
pub struct Struct(pub Option<Identifier>, pub Vec<StructColumnPair>);

#[derive(Debug)]
pub struct StructColumnPair(pub Identifier, pub Vec<Value>);

pub type Identifier = String;

#[derive(Debug)]
pub struct Paramter(pub Identifier, pub ParamterType);

#[derive(Debug)]
pub enum ParamterType {
    Normal,
    Arguments
}

#[derive(Debug)]
pub enum Loop {
    For(ForLoopSetup, ForLoopCondition, ForLoopStep, LoopBlock),
    While(WhileLoopCondition, LoopBlock),
    Loop(LoopBlock),
    Foreach(ForeachParamters, ForeachLoopTarget, LoopBlock)
}

pub type ForLoopSetup = FunctionStatement;
pub type ForLoopCondition = Value;
pub type ForLoopStep = Value;
pub type WhileLoopCondition = Value;
pub type ForeachParamters = FunctionParamters;
pub type ForeachLoopTarget = Value;
pub type LoopBlock = FunctionBlock;

#[derive(Debug)]
pub struct IfCondition(pub Value, pub ConditionBlock);

#[derive(Debug)]
pub struct ElseCondition(pub ConditionBlock);

pub type ConditionBlock = FunctionBlock;