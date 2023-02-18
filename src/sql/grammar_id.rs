use super::{err::SqlErr, grammar::SqlGrammar, grammar_pattern::GrammarPattern, letter::SqlLetter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrammarId {
    Letter,
    Vec,
    CreateTableStmt,
    ColumnClause,
    IdExpr,
}

impl From<SqlGrammar> for GrammarId {
    fn from(value: SqlGrammar) -> Self {
        match value {
            SqlGrammar::Letter(_, _, _) => Self::Letter,
            SqlGrammar::Vec(_, _, _) => Self::Vec,
            SqlGrammar::CreateTableStmt(_, _, _) => Self::CreateTableStmt,
            SqlGrammar::ColumnClause(_, _, _) => Self::ColumnClause,
            SqlGrammar::IdExpr(_, _, _) => Self::IdExpr,
        }
    }
}

impl GrammarId {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Letter => "Letter",
            Self::Vec => "Vector",
            Self::CreateTableStmt => "Create Table Statement",
            Self::ColumnClause => "Column Clause",
            Self::IdExpr => "Identity Expression",
        }
    }

    pub fn matches(&self, letters: &Vec<SqlLetter>) -> Result<SqlGrammar, SqlErr> {
        GrammarPattern::from(self.clone())
            .try_match(&mut letters.iter(), 0)
            .and_then(|(grammar, end)| {
                if end < letters.last().map_or(0, |letter| letter.end()) {
                    Err(SqlErr::Syntax(end))
                } else {
                    Ok((self.clone(), grammar).into())
                }
            })
    }
}
