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
                    Ok((0..end, self.clone(), grammar).into())
                }
            })
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_matches() {
//         // CREATE
//         assert!(matches!(
//             GrammarId::Create.matches("BxbU0wVK", 0),
//             Option::None
//         ));
//         assert!(matches!(
//             GrammarId::Create.matches("40b4868f81734907becreate479d2e911ffabe", 18),
//             Option::Some(letter) if matches!(letter, SqlLetter::Create(start, end) if start == 18 && end == 24)
//         ));
//         assert!(matches!(
//             GrammarId::Create.matches("sHQq#jUFCREATE7F0nRk1K", 8),
//             Option::Some(letter) if matches!(letter, SqlLetter::Create(start, end) if start == 8 && end == 14)
//         ));
//         // TABLE
//         assert!(matches!(
//             GrammarId::Table.matches("c56bcc83515c405883080b0fd456bdbe", 12),
//             Option::None
//         ));
//         assert!(matches!(
//             GrammarId::Table.matches("ed270table09032974f8aa390f1523e5e935f", 5),
//             Option::Some(letter) if matches!(letter, SqlLetter::Table(start, end) if start == 5 && end == 10)
//         ));
//         assert!(matches!(
//             GrammarId::Table.matches("e252TABLEad73b68b4ef2a49f9aa30e6d4bf2", 4),
//             Option::Some(letter) if matches!(letter, SqlLetter::Table(start, end) if start == 4 && end == 9)
//         ));
//         // ID
//         assert!(matches!(
//             GrammarId::Id.matches("_Person", 0),
//             Option::None
//         ));
//         assert!(matches!(
//             GrammarId::Id.matches("Person_", 0),
//             Option::Some(letter) if matches!(letter, SqlLetter::Id(start, end) if start == 0 && end == 7)
//         ));
//         // SKIP
//         assert!(matches!(
//             GrammarId::Skip.matches("e463d062", 0),
//             Option::None
//         ));
//         assert!(matches!(
//             GrammarId::Skip.matches(" \t\r\n7c764d2d8ae1", 0),
//             Option::Some(letter) if matches!(letter, SqlLetter::Skip(start, end) if start == 0 && end == 4)
//         ));
//     }
// }
