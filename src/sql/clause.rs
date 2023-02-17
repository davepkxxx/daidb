use super::expr::IdExpr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnClause {
    pub name: IdExpr,
    pub data_type: IdExpr,
}
