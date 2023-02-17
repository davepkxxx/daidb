use super::{stmt::{Stmt, CreateTableStmt}, clause::ColumnClause, expr::IdExpr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxNode {
    Stmt(Stmt),
    CreateTableStmt(CreateTableStmt),
    ColumnClause(ColumnClause),
    IdExpr(IdExpr),
}
