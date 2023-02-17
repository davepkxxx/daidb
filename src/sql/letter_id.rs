use super::{letter::SqlLetter, letter_pattern::LetterPattern};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterId {
    Create,
    Table,
    Comma,
    ParenL,
    ParenR,
    Id,
    Skip,
}

impl LetterId {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Create,
            Self::Table,
            Self::Comma,
            Self::ParenL,
            Self::ParenR,
            Self::Id,
            Self::Skip,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Create => "CREATE",
            Self::Table => "TABLE",
            Self::Comma => "COMMA",
            Self::ParenL => "LEFT PAREN",
            Self::ParenR => "RIGHT PAREN",
            Self::Id => "ID",
            Self::Skip => "SKIP",
        }
    }

    pub fn matches(&self, s: &str, start: usize) -> Option<SqlLetter> {
        let (matched, end) = LetterPattern::from(self.clone()).matches(s, start);
        if matched {
            Option::Some(SqlLetter::from((self.clone(), start..end, &s[start..end])))
        } else {
            Option::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches() {
        // CREATE
        assert!(matches!(
            LetterId::Create.matches("BxbU0wVK", 0),
            Option::None
        ));
        assert!(matches!(
            LetterId::Create.matches("40b4868f81734907becreate479d2e911ffabe", 18),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Create(start, end, value)
                    if *start == 18 && *end == 24 && value == "create")
        ));
        assert!(matches!(
            LetterId::Create.matches("sHQq#jUFCREATE7F0nRk1K", 8),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Create(start, end, value)
                    if *start == 8 && *end == 14 && value == "CREATE")
        ));
        // TABLE
        assert!(matches!(
            LetterId::Table.matches("c56bcc83515c405883080b0fd456bdbe", 12),
            Option::None
        ));
        assert!(matches!(
            LetterId::Table.matches("ed270table09032974f8aa390f1523e5e935f", 5),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Table(start, end, value)
                    if *start == 5 && *end == 10 && value == "table")
        ));
        assert!(matches!(
            LetterId::Table.matches("e252TABLEad73b68b4ef2a49f9aa30e6d4bf2", 4),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Table(start, end, value)
                    if *start == 4 && *end == 9 && value == "TABLE")
        ));
        // ID
        assert!(matches!(LetterId::Id.matches("_Person", 0), Option::None));
        assert!(matches!(
            LetterId::Id.matches("Person_", 0),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Id(start, end, value)
                    if *start == 0 && *end == 7 && value == "Person_")
        ));
        // SKIP
        assert!(matches!(
            LetterId::Skip.matches("e463d062", 0),
            Option::None
        ));
        assert!(matches!(
            LetterId::Skip.matches(" \t\r\n7c764d2d8ae1", 0),
            Option::Some(letter)
                if matches!(&letter, SqlLetter::Skip(start, end, value)
                    if *start == 0 && *end == 4 && value == " \t\r\n")
        ));
    }
}
