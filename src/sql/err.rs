pub enum SqlErr {
    Miss(usize, String),
    Syntax(usize),
}

impl SqlErr {
    pub fn msg(&self, s: &str) -> String {
        match self {
            Self::Miss(n, name) => Self::loc_str(&format!("Missing '{}'", name), s, *n),
            Self::Syntax(n) => Self::loc_str("Syntax error", s, *n),
        }
    }

    fn loc_str(msg: &str, s: &str, n: usize) -> String {
        let (line, col) = Self::loc(s, n);
        format!("{} at {}, {}", msg, line, col)
    }

    fn loc(s: &str, n: usize) -> (usize, usize) {
        let mut line = 1;
        let mut col = 1;
        for (i, c) in s.chars().enumerate() {
            if i == n {
                break;
            } else if c == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }
}
