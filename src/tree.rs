#[derive(Debug)]
pub enum Statement {
    ImportPackage(PackagePath),
    DefineType((GenericTemplate, TypeName, TypeBlock)),
    DefineFunction((GenericTemplate, FunctionName, FunctionParamters, FunctionReturnType, FunctionTypeSetting, FunctionBlock))
}

#[derive(Debug)]
pub struct PackagePath(Vec<Identifier>);

#[derive(Debug)]
pub struct GenericTemplate(Vec<Identifier>);
pub type TypeName = Identifier;

#[derive(Debug)]
pub struct TypeBlock(Vec<TypeCloumn>);

#[derive(Debug)]
pub struct TypeCloumn(Identifier, Identifier);

#[derive(Debug)]
pub enum FunctionName {
    Operator(Value),
    Normal(Identifier)
}

#[derive(Debug)]
pub struct FunctionParamters(Vec<Paramter>);
pub type FunctionReturnType = Identifier;

#[derive(Debug)]
pub struct FunctionTypeSetting(Vec<FunctionTypeColumn>);

#[derive(Debug)]
pub struct FunctionTypeColumn(Identifier, Identifier);

#[derive(Debug)]
pub struct FunctionBlock(Vec<FunctionStatement>);

#[derive(Debug)]
pub enum FunctionStatement {
    DefineVariable(Identifier, Value),
    DefineClosure((GenericTemplate, FunctionName, FunctionParamters, FunctionReturnType, FunctionTypeSetting, FunctionBlock)),
    DefineFact(Identifier, FactContent),
    CallFunction(Value, Vec<Value>)
}

#[derive(Debug)]
pub struct FactContent(Vec<FactCheck>);

#[derive(Debug)]
pub struct FactCheck(FactState, Option<FactCondition>);

#[derive(Debug)]
pub enum FactState {
    FactArrayState(ArrayState),
    FactSimpleState(ConstantValue),
    FactStructState(StructState)
}

#[derive(Debug)]
pub struct ArrayState(Vec<Paramter>);

#[derive(Debug)]
pub struct StructState(Identifier, Vec<Identifier>);
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
pub struct Array(Vec<Value>);

#[derive(Debug)]
pub struct Struct(Option<Identifier>, Vec<StructColumnPair>);

#[derive(Debug)]
pub struct StructColumnPair(Identifier, Vec<Value>);

pub type Identifier = String;

#[derive(Debug)]
pub struct Paramter(Identifier, ParamterType);

#[derive(Debug)]
pub enum ParamterType {
    Normal,
    Arguments
}
