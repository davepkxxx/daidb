use std::vec;

use super::{
    err::SqlErr, grammar::SqlGrammar, grammar_id::GrammarId, letter::SqlLetter, letter_id::LetterId,
};

pub fn parse(sql: &str) -> Result<SqlGrammar, SqlErr> {
    parse_letters(sql).and_then(|letters| {
        GrammarId::CreateTableStmt.matches(
            &letters
                .into_iter()
                .filter(|letter| !letter.is(&LetterId::Skip))
                .collect(),
        )
    })
}

fn parse_letter(sql: &str, n: usize) -> Option<SqlLetter> {
    for id in LetterId::all().iter() {
        if let Some(letter) = id.matches(sql, n) {
            return Some(letter);
        }
    }
    None
}

fn parse_letters(sql: &str) -> Result<Vec<SqlLetter>, SqlErr> {
    let mut letters = vec![];
    let mut i: usize = 0;
    while i < sql.len() {
        match parse_letter(sql, i) {
            Some(letter) => {
                i = letter.end();
                letters.push(letter);
            }
            None => return Err(SqlErr::Syntax(i)),
        }
    }
    Ok(letters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_letters_create_table() {
        let s = r"
        CREATE TABLE Person(
            PersonID int,
            LastName varchar,
            FirstName varchar,
            Address varchar,
            City varchar
        )";
        match parse_letters(s) {
            Ok(letters) => {
                let assert_letter =
                    |i: usize, id: &LetterId, assert: &dyn Fn(&SqlLetter)| match letters.get(i) {
                        Some(letter) => {
                            assert!(letter.is(id), "{} no match", id.name());
                            assert(letter);
                        }
                        None => panic!("{} no match", id.name()),
                    };
                let assert_skip = |i: usize| assert_letter(i, &LetterId::Skip, &|_| {});
                let assert_letter_range = |i: usize, id: &LetterId, txt: &str| {
                    assert_letter(i, id, &|letter: &SqlLetter| {
                        assert_eq!(&s[letter.start()..letter.end()], txt, "{} no match", txt);
                    })
                };

                assert_eq!(letters.len(), 33);
                assert_skip(0);
                assert_letter_range(1, &LetterId::Create, "CREATE");
                assert_skip(2);
                assert_letter_range(3, &LetterId::Table, "TABLE");
                assert_skip(4);
                assert_letter_range(5, &LetterId::Id, "Person");
                assert_letter_range(6, &LetterId::ParenL, "(");
                assert_skip(7);
                assert_letter_range(8, &LetterId::Id, "PersonID");
                assert_skip(9);
                assert_letter_range(10, &LetterId::Id, "int");
                assert_letter_range(11, &LetterId::Comma, ",");
                assert_skip(12);
                assert_letter_range(13, &LetterId::Id, "LastName");
                assert_skip(14);
                assert_letter_range(15, &LetterId::Id, "varchar");
                assert_letter_range(16, &LetterId::Comma, ",");
                assert_skip(17);
                assert_letter_range(18, &LetterId::Id, "FirstName");
                assert_skip(19);
                assert_letter_range(20, &LetterId::Id, "varchar");
                assert_letter_range(21, &LetterId::Comma, ",");
                assert_skip(22);
                assert_letter_range(23, &LetterId::Id, "Address");
                assert_skip(24);
                assert_letter_range(25, &LetterId::Id, "varchar");
                assert_letter_range(26, &LetterId::Comma, ",");
                assert_skip(27);
                assert_letter_range(28, &LetterId::Id, "City");
                assert_skip(29);
                assert_letter_range(30, &LetterId::Id, "varchar");
                assert_skip(31);
                assert_letter_range(32, &LetterId::ParenR, ")");
            }
            Err(err) => panic!("Error: {}", err.msg(s)),
        }
    }

    #[test]
    fn test_parse_create_table() {
        let s = r"
        CREATE TABLE Person(
            PersonID int,
            LastName varchar,
            FirstName varchar,
            Address varchar,
            City varchar
        )";
        match parse(s) {
            Ok(grammar) => match grammar {
                SqlGrammar::CreateTableStmt(_, _, stmt) => {
                    assert_eq!(stmt.name.value, "Person");
                    assert_eq!(stmt.columns.len(), 5);
                    let mut iter = stmt.columns.iter();
                    match iter.next() {
                        Some(col) => {
                            assert_eq!(col.name.value, "PersonID");
                            assert_eq!(col.data_type.value, "int");
                        }
                        None => panic!("columns length, expected: 5, actual: 0"),
                    }
                    match iter.next() {
                        Some(col) => {
                            assert_eq!(col.name.value, "LastName");
                            assert_eq!(col.data_type.value, "varchar");
                        }
                        None => panic!("columns length, expected: 5, actual: 1"),
                    }
                    match iter.next() {
                        Some(col) => {
                            assert_eq!(col.name.value, "FirstName");
                            assert_eq!(col.data_type.value, "varchar");
                        }
                        None => panic!("columns length, expected: 5, actual: 2"),
                    }
                    match iter.next() {
                        Some(col) => {
                            assert_eq!(col.name.value, "Address");
                            assert_eq!(col.data_type.value, "varchar");
                        }
                        None => panic!("columns length, expected: 5, actual: 3"),
                    }
                    match iter.next() {
                        Some(col) => {
                            assert_eq!(col.name.value, "City");
                            assert_eq!(col.data_type.value, "varchar");
                        }
                        None => panic!("columns length, expected: 5, actual: 4"),
                    }
                }
                _ => panic!(
                    "root grammar, expected: {}, actual: {}",
                    GrammarId::CreateTableStmt.name(),
                    GrammarId::from(grammar).name()
                ),
            },
            Err(err) => panic!("Error: {}", err.msg(s)),
        }
    }
}
