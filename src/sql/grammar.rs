use std::ops::Range;

use super::{
    clause::ColumnClause, expr::IdExpr, grammar_id::GrammarId, letter::SqlLetter,
    stmt::CreateTableStmt,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlGrammar {
    Letter(usize, usize, SqlLetter),
    Vec(usize, usize, Vec<Self>),
    CreateTableStmt(usize, usize, CreateTableStmt),
    ColumnClause(usize, usize, ColumnClause),
    IdExpr(usize, usize, IdExpr),
}

impl From<SqlGrammar> for Range<usize> {
    fn from(value: SqlGrammar) -> Self {
        match value {
            SqlGrammar::Letter(start, end, _) => start..end,
            SqlGrammar::Vec(start, end, _) => start..end,
            SqlGrammar::CreateTableStmt(start, end, _) => start..end,
            SqlGrammar::ColumnClause(start, end, _) => start..end,
            SqlGrammar::IdExpr(start, end, _) => start..end,
        }
    }
}

impl From<SqlLetter> for SqlGrammar {
    fn from(value: SqlLetter) -> Self {
        let range: Range<usize> = value.clone().into();
        match value {
            SqlLetter::Id(_, _, text) => {
                Self::IdExpr(range.start, range.end, IdExpr { value: text })
            }
            _ => Self::Letter(range.start, range.end, value),
        }
    }
}

impl From<SqlGrammar> for Option<IdExpr> {
    fn from(grammar: SqlGrammar) -> Self {
        match grammar {
            SqlGrammar::IdExpr(_, _, value) => Some(value),
            _ => None,
        }
    }
}

impl From<SqlGrammar> for Option<ColumnClause> {
    fn from(grammar: SqlGrammar) -> Self {
        match grammar {
            SqlGrammar::ColumnClause(_, _, value) => Some(value),
            _ => None,
        }
    }
}

impl From<(GrammarId, SqlGrammar)> for SqlGrammar {
    fn from((grammar_id, value): (GrammarId, SqlGrammar)) -> Self {
        match grammar_id {
            GrammarId::Letter => value,
            GrammarId::Vec => {
                let range: Range<usize> = value.clone().into();
                match value {
                    SqlGrammar::Vec(start, end, grammars) => {
                        let mut newval = vec![];
                        for grammar in grammars {
                            match grammar {
                                SqlGrammar::Vec(_, _, children) => {
                                    newval.extend(children.into_iter())
                                }
                                _ => newval.push(grammar),
                            }
                        }
                        SqlGrammar::Vec(start, end, newval)
                    }
                    _ => SqlGrammar::Vec(range.start, range.end, vec![value]),
                }
            }
            GrammarId::CreateTableStmt => {
                let mut name_opt = None;
                let mut columns = vec![];
                let range: Range<usize> = value.clone().into();
                for grammar in value.collect() {
                    match grammar {
                        SqlGrammar::ColumnClause(_, _, col) => columns.push(col),
                        SqlGrammar::IdExpr(_, _, id_expr) => {
                            if name_opt.is_none() {
                                name_opt = Some(id_expr)
                            }
                        }
                        _ => {}
                    }
                }
                if let Some(name) = name_opt {
                    SqlGrammar::CreateTableStmt(
                        range.start,
                        range.end,
                        CreateTableStmt { name, columns },
                    )
                } else {
                    panic!("no table name in create table statement")
                }
            }
            GrammarId::ColumnClause => {
                let range: Range<usize> = value.clone().into();
                let mut id_iter = value
                    .collect()
                    .into_iter()
                    .filter_map(|g| g.into())
                    .collect::<Vec<IdExpr>>()
                    .into_iter();
                match (id_iter.nth(0), id_iter.nth(1)) {
                    (Some(name), Some(data_type)) => SqlGrammar::ColumnClause(
                        range.start,
                        range.end,
                        ColumnClause { name, data_type },
                    ),
                    (Some(_), None) => panic!("no data type in column clause"),
                    _ => panic!("no column name in column clause"),
                }
            }
            GrammarId::IdExpr => {
                if let SqlGrammar::Letter(_, _, letter) = value.clone() {
                    return letter.into();
                }
                panic!(
                    "no {} in {}",
                    grammar_id.name(),
                    GrammarId::from(value).name()
                )
            }
        }
    }
}

impl SqlGrammar {
    pub fn is(&self, id: &GrammarId) -> bool {
        match (self, id) {
            (Self::Letter(_, _, _), GrammarId::Letter) => true,
            (Self::Vec(_, _, _), GrammarId::Vec) => true,
            (Self::CreateTableStmt(_, _, _), GrammarId::CreateTableStmt) => true,
            (Self::ColumnClause(_, _, _), GrammarId::ColumnClause) => true,
            (Self::IdExpr(_, _, _), GrammarId::IdExpr) => true,
            _ => false,
        }
    }

    fn collect(&self) -> Vec<Self> {
        match self {
            SqlGrammar::Vec(_, _, grammars) => grammars.clone(),
            _ => vec![self.clone()],
        }
    }
}
