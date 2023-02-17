use std::ops::Range;

use super::letter_id::LetterId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlLetter {
    Create(usize, usize, String),
    Table(usize, usize, String),
    Comma(usize, usize, String),
    ParenL(usize, usize, String),
    ParenR(usize, usize, String),
    Id(usize, usize, String),
    Skip(usize, usize, String),
}

impl From<(LetterId, Range<usize>, &str)> for SqlLetter {
    fn from((id, range, value): (LetterId, Range<usize>, &str)) -> Self {
        let start = range.start;
        let end = range.end;
        let s = value.to_string();
        match id {
            LetterId::Create => SqlLetter::Create(start, end, s),
            LetterId::Table => SqlLetter::Table(start, end, s),
            LetterId::Comma => SqlLetter::Comma(start, end, s),
            LetterId::ParenL => SqlLetter::ParenL(start, end, s),
            LetterId::ParenR => SqlLetter::ParenR(start, end, s),
            LetterId::Id => SqlLetter::Id(start, end, s),
            LetterId::Skip => SqlLetter::Skip(start, end, s),
        }
    }
}

impl SqlLetter {
    pub fn is(&self, id: &LetterId) -> bool {
        match (self, id) {
            (Self::Create(_, _, _), LetterId::Create) => true,
            (Self::Table(_, _, _), LetterId::Table) => true,
            (Self::Comma(_, _, _), LetterId::Comma) => true,
            (Self::ParenL(_, _, _), LetterId::ParenL) => true,
            (Self::ParenR(_, _, _), LetterId::ParenR) => true,
            (Self::Id(_, _, _), LetterId::Id) => true,
            (Self::Skip(_, _, _), LetterId::Skip) => true,
            _ => false,
        }
    }

    pub fn range(&self) -> Range<usize> {
        match self {
            Self::Create(start, end, _) => *start..*end,
            Self::Table(start, end, _) => *start..*end,
            Self::Comma(start, end, _) => *start..*end,
            Self::ParenL(start, end, _) => *start..*end,
            Self::ParenR(start, end, _) => *start..*end,
            Self::Id(start, end, _) => *start..*end,
            Self::Skip(start, end, _) => *start..*end,
        }
    }

    pub fn start(&self) -> usize {
        self.range().start
    }

    pub fn end(&self) -> usize {
        self.range().end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        assert!(
            matches!(SqlLetter::from((LetterId::Create, 53..57, "CREATE")),
            SqlLetter::Create(start, end, value) if start == 53 && end == 57 && value == "CREATE")
        );
        assert!(matches!(SqlLetter::from((LetterId::Table, 0..24, "TABLE")),
            SqlLetter::Table(start, end, value) if start == 0 && end == 24 && value == "TABLE"));
        assert!(matches!(SqlLetter::from((LetterId::Comma, 48..55, ",")),
            SqlLetter::Comma(start, end, value) if start == 48 && end == 55 && value == ","));
        assert!(matches!(SqlLetter::from((LetterId::ParenL, 25..86, "(")),
            SqlLetter::ParenL(start, end, value) if start == 25 && end == 86 && value == "("));
        assert!(matches!(SqlLetter::from((LetterId::ParenR, 71..84, ")")),
            SqlLetter::ParenR(start, end, value) if start == 71 && end == 84 && value == ")"));
        assert!(matches!(SqlLetter::from((LetterId::Skip, 42..85, " ")),
            SqlLetter::Skip(start, end, value) if start == 42 && end == 85 && value == " "));
    }
}
