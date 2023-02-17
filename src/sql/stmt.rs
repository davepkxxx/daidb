use super::{clause::ColumnClause, expr::IdExpr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stmt {
    CreateTable(CreateTableStmt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateTableStmt {
    pub name: IdExpr,
    pub columns: Vec<ColumnClause>,
}
