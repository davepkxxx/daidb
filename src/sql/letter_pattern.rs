use super::letter_id::LetterId;

#[derive(Debug, Clone, PartialEq, Eq)]

pub enum LetterPattern {
    Char(char),
    Chars(Vec<char>),
    Str(String),
    Range(char, char),
    And(Vec<Box<Self>>),
    Or(Vec<Box<Self>>),
    Any(Box<Self>),
    More(Box<Self>),
}

impl From<LetterId> for LetterPattern {
    fn from(value: LetterId) -> Self {
        match value {
            LetterId::Create => Self::Str("CREATE".to_string()),
            LetterId::Table => Self::Str("TABLE".to_string()),
            LetterId::Comma => Self::Char(','),
            LetterId::ParenL => Self::Char('('),
            LetterId::ParenR => Self::Char(')'),
            LetterId::Id => Self::And(vec![
                Box::new(Self::Or(vec![
                    Box::new(Self::Range('a', 'z')),
                    Box::new(Self::Range('A', 'Z')),
                ])),
                Box::new(Self::Any(Box::new(Self::Or(vec![
                    Box::new(Self::Range('a', 'z')),
                    Box::new(Self::Range('A', 'Z')),
                    Box::new(Self::Range('0', '9')),
                    Box::new(Self::Char('_')),
                ])))),
            ]),
            LetterId::Skip => Self::More(Box::new(Self::Chars(vec![' ', '\t', '\r', '\n']))),
        }
    }
}

impl LetterPattern {
    pub fn matches(&self, s: &str, n: usize) -> (bool, usize) {
        match self {
            Self::Char(pattern) => self.match_at(s, n, |c| {
                if c.to_lowercase().next() == pattern.to_lowercase().next() {
                    (true, n + 1)
                } else {
                    (false, n)
                }
            }),
            Self::Chars(patterns) => Self::Or(
                patterns
                    .iter()
                    .map(|pattern| Box::new(Self::Char(*pattern)))
                    .collect::<Vec<Box<Self>>>(),
            )
            .matches(s, n),
            Self::Str(patterns) => Self::And(
                patterns
                    .chars()
                    .map(|pattern| Box::new(Self::Char(pattern)))
                    .collect::<Vec<Box<Self>>>(),
            )
            .matches(s, n),
            Self::Range(start, end) => self.match_at(s, n, |c| {
                if &c >= start && &c <= end {
                    (true, n + 1)
                } else {
                    (false, n)
                }
            }),
            Self::And(patterns) => {
                let mut i = n;
                for pattern in patterns.iter() {
                    let (matched, end) = pattern.matches(s, i);
                    if matched {
                        i = end;
                    } else {
                        return (false, n);
                    }
                }
                (true, i)
            }
            Self::Or(patterns) => {
                for pattern in patterns.iter() {
                    let (matched, end) = pattern.matches(s, n);
                    if matched {
                        return (true, end);
                    }
                }
                (false, n)
            }
            Self::Any(pattern) => {
                let mut i = n;
                loop {
                    let (matched, end) = pattern.matches(s, i);
                    if matched {
                        i = end;
                    } else {
                        return (true, i);
                    }
                }
            }
            Self::More(pattern) => {
                let (mut matched, mut i) = pattern.matches(s, n);
                if matched {
                    while matched {
                        (matched, i) = pattern.matches(s, i);
                    }
                    (true, i)
                } else {
                    (false, n)
                }
            }
        }
    }

    fn match_at<F>(&self, sql: &str, n: usize, f: F) -> (bool, usize)
    where
        F: Fn(char) -> (bool, usize),
    {
        sql.chars().nth(n).map_or((false, n), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_char() {
        assert_eq!(
            LetterPattern::Char('V').matches("08c9b5eb10e54984ae9cb3d60b975635", 7),
            (false, 7)
        );
        assert_eq!(
            LetterPattern::Char('2').matches("7173fe47cb154e2f81928345206ed946", 14),
            (true, 15)
        );
    }

    #[test]
    fn test_match_chars() {
        let (mut matched, mut end) =
            LetterPattern::Chars(vec!['8', 'L', '0']).matches("OH$uISCX", 3);
        assert_eq!(
            matched, false,
            "match '%' '6' in 'OH$uISCX', match result, expected: false, actual: {}",
            matched
        );
        assert_eq!(
            end, 3,
            "match '%' '6' in 'OH$uISCX', end index, expected: 3, actual: {}",
            end
        );
        (matched, end) = LetterPattern::Chars(vec!['%', '6']).matches("EXtQ%6Ee", 4);
        assert_eq!(
            matched, true,
            "match '%' '6' in 'EXtQ%6Ee', match result, expected: true, actual: {}",
            matched
        );
        assert_eq!(
            end, 5,
            "match '%' '6' in 'EXtQ%6Ee', end index, expected: 5, actual: {}",
            end
        );
    }

    #[test]
    fn test_match_str() {
        assert_eq!(
            LetterPattern::Str("bea".to_string()).matches("lFMZtYWd", 1),
            (false, 1)
        );
        assert_eq!(
            LetterPattern::Str("UryCk".to_string()).matches("bViUryCk42t4a", 3),
            (true, 8)
        );
        assert_eq!(
            LetterPattern::Str("*ghb^sq".to_string()).matches("*gHB^sq7", 7),
            (false, 7)
        );
        assert_eq!(
            LetterPattern::Str("UDZB".to_string()).matches("Hn4UDzB7", 3),
            (true, 7)
        );
    }

    #[test]
    fn test_match_range() {
        // 0-9
        assert_eq!(
            LetterPattern::Range('0', '9').matches("114b1b588ce4492f94588f1b5063e2f0", 21),
            (false, 21)
        );
        assert_eq!(
            LetterPattern::Range('0', '9').matches("43eb282919914693ab923a3e0ec6af19", 18),
            (true, 19)
        );
        // a-z
        assert_eq!(
            LetterPattern::Range('a', 'z').matches("424f8459a1044258a3b5efb6d3c0eaae", 23),
            (false, 23)
        );
        assert_eq!(
            LetterPattern::Range('a', 'z').matches("c70373d04d1441f18239bd1595e08676", 14),
            (true, 15)
        );
        // A-Z
        assert_eq!(
            LetterPattern::Range('A', 'Z').matches("852CF46C5E1D46CBA773AE5659EA11C4", 8),
            (false, 8)
        );
        assert_eq!(
            LetterPattern::Range('A', 'Z').matches("53CEF0D9561140E4A42F2D3E0AF03658", 16),
            (true, 17)
        );
    }
}
