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

impl From<(Range<usize>, &SqlLetter)> for SqlGrammar {
    fn from((range, value): (Range<usize>, &SqlLetter)) -> Self {
        let start = range.start;
        let end = range.end;
        match value {
            SqlLetter::Id(_, _, value) => Self::IdExpr(
                start,
                end,
                IdExpr {
                    value: value.to_string(),
                },
            ),
            _ => Self::Letter(start, end, value.clone()),
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

impl From<(Range<usize>, GrammarId, SqlGrammar)> for SqlGrammar {
    fn from((range, grammar_id, value): (Range<usize>, GrammarId, SqlGrammar)) -> Self {
        let start = range.start;
        let end = range.end;
        let grammars = match value {
            SqlGrammar::Vec(_, _, g) => g,
            _ => vec![value],
        };
        match grammar_id {
            GrammarId::Vec => {
                // let mut vec = vec![];
                // for grammar in grammars {
                //     match grammar {
                //         SqlGrammar::Vec(_, _, children) => vec.extend(children.clone()),
                //         _ => vec.push(grammar.clone()),
                //     }
                // }
                SqlGrammar::Vec(start, end, grammars)
            }
            GrammarId::CreateTableStmt => {
                for grammar in grammars.clone() {
                    println!("found {} in create table statement", GrammarId::from(grammar).name());
                }
                grammars
                .iter()
                .filter_map(|g| g.clone().into())
                .collect::<Vec<IdExpr>>()
                .get(0)
                .map_or_else(
                    || panic!("no table name in create table statement"),
                    |name| {
                        SqlGrammar::CreateTableStmt(
                            start,
                            end,
                            CreateTableStmt {
                                name: name.clone(),
                                columns: grammars.iter().filter_map(|g| g.clone().into()).collect(),
                            },
                        )
                    },
                )
            },
            GrammarId::ColumnClause => {
                let id_exprs: Vec<IdExpr> =
                    grammars.iter().filter_map(|g| g.clone().into()).collect();
                match (id_exprs.get(0), id_exprs.get(1)) {
                    (Some(name), Some(data_type)) => SqlGrammar::ColumnClause(
                        start,
                        end,
                        ColumnClause {
                            name: name.clone(),
                            data_type: data_type.clone(),
                        },
                    ),
                    (Some(_), None) => panic!("no data type in column clause"),
                    _ => panic!("no column name in column clause"),
                }
            }
            _ => panic!("cannot convert {} to SQL Grammar", grammar_id.name()),
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

    pub fn range(&self) -> Range<usize> {
        match self {
            Self::Letter(start, end, _) => *start..*end,
            Self::Vec(start, end, _) => *start..*end,
            Self::CreateTableStmt(start, end, _) => *start..*end,
            Self::ColumnClause(start, end, _) => *start..*end,
            Self::IdExpr(start, end, _) => *start..*end,
        }
    }

    pub fn start(&self) -> usize {
        self.range().start
    }

    pub fn end(&self) -> usize {
        self.range().end
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_from() {
//         assert!(
//             matches!(SqlGrammar::from(GrammarId::CreateTableStmt, 53..57),
//             SqlGrammar::CreateTable(start, end)
//             if start == 53 && end == 57)
//         );
//     }
// }
