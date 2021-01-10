enum Line {
    Definition(Definition),
    Expression(Expression),
}

enum Definition {
    Variable(VariableDefinition),
    Constant(ConstantDefinition),
    Word(WordDefinition),
}

enum Statement {
    IfThen(IfThenStatement),
    IfElseThen(IfElseThenStatement),
    DoLoop(DoLoopStatement),
}

enum ExpressionElement {
    Literal(Literal),
    Ident(Ident),
}

struct Expression {
    Vec<ExpressionElement> elements;
}