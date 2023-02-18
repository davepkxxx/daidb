use std::slice::Iter;

use super::{
    err::SqlErr, grammar::SqlGrammar, grammar_id::GrammarId, letter::SqlLetter, letter_id::LetterId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrammarPattern {
    Letter(LetterId),
    Grammar(GrammarId),
    And(Vec<Box<Self>>),
    Any(Box<Self>),
}

impl From<GrammarId> for GrammarPattern {
    fn from(value: GrammarId) -> Self {
        match value {
            GrammarId::CreateTableStmt => Self::And(vec![
                Box::new(Self::Letter(LetterId::Create)),
                Box::new(Self::Letter(LetterId::Table)),
                Box::new(Self::Grammar(GrammarId::IdExpr)),
                Box::new(Self::Letter(LetterId::ParenL)),
                Box::new(Self::Any(Box::new(Self::And(vec![
                    Box::new(Self::Grammar(GrammarId::ColumnClause)),
                    Box::new(Self::Letter(LetterId::Comma)),
                ])))),
                Box::new(Self::Grammar(GrammarId::ColumnClause)),
                Box::new(Self::Letter(LetterId::ParenR)),
            ]),
            GrammarId::ColumnClause => Self::And(vec![
                Box::new(Self::Grammar(GrammarId::IdExpr)),
                Box::new(Self::Grammar(GrammarId::IdExpr)),
            ]),
            GrammarId::IdExpr => Self::Letter(LetterId::Id),
            _ => panic!("Cannot convert {} to GrammarPattern", value.name()),
        }
    }
}

impl GrammarPattern {
    pub fn try_match(
        &self,
        iter: &mut Iter<SqlLetter>,
        n: usize,
    ) -> Result<(SqlGrammar, usize), SqlErr> {
        match self {
            Self::Letter(letter_id) => iter.next().map_or(
                Err(SqlErr::Miss(n, letter_id.name().to_string())),
                |letter| {
                    if letter.is(letter_id) {
                        Ok((
                            SqlGrammar::Letter(n, letter.end(), letter.clone()),
                            letter.end(),
                        ))
                    } else {
                        Err(SqlErr::Miss(n, letter_id.name().to_string()))
                    }
                },
            ),
            Self::Grammar(grammar_id) => Self::from(grammar_id.clone())
                .try_match(iter, n)
                .and_then(|(grammar, end)| Ok(((grammar_id.clone(), grammar).into(), end))),
            Self::And(patterns) => {
                let mut i = n;
                let mut grammars = vec![];
                for pattern in patterns {
                    match pattern.try_match(iter, i) {
                        Ok((grammar, end)) => {
                            grammars.push(grammar);
                            i = end;
                        }
                        Err(err) => return Err(err),
                    }
                }
                Ok(((GrammarId::Vec, SqlGrammar::Vec(n, i, grammars)).into(), i))
            }
            Self::Any(patterns) => {
                let mut grammars = vec![];
                let mut i = n;
                loop {
                    let mut cloned_iter = iter.clone();
                    match patterns.try_match(&mut cloned_iter, i) {
                        Ok((grammar, end)) => {
                            while i < end {
                                match iter.next() {
                                    Some(letter) => i = letter.end(),
                                    None => panic!("GrammarPattern::Any error"),
                                }
                            }
                            grammars.push(grammar);
                            i = end;
                        }
                        Err(_) => {
                            return Ok((
                                (GrammarId::Vec, SqlGrammar::Vec(n, i, grammars)).into(),
                                i,
                            ))
                        }
                    }
                }
            }
        }
    }
}
