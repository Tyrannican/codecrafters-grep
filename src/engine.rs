#[derive(Debug, Clone)]
pub enum RegexOperation {
    PositiveCharGroup { chars: Vec<char> },
    NegativeCharGroup { chars: Vec<char> },
    Digit,
    Alphanumeric,
    Literal { exact: String },
}

pub struct RegexEngine {
    pub input: String,
    pub operations: Vec<RegexOperation>,
    matches: Vec<bool>,
}

impl RegexEngine {
    pub fn new(input: String, pattern: String) -> Self {
        Self {
            input,
            operations: parse_pattern(&pattern),
            matches: vec![],
        }
    }

    pub fn matches(&mut self) {
        // TODO: Refactor
        for op in self.operations.clone() {
            match op {
                RegexOperation::Literal { exact } => self.literal(&exact),
                RegexOperation::Digit => self.digit(),
                RegexOperation::Alphanumeric => self.alphanumeric(),
                RegexOperation::PositiveCharGroup { chars } => self.positive_character_group(chars),
                RegexOperation::NegativeCharGroup { chars } => self.negative_character_group(chars),
            }
        }

        if self.matches.is_empty() || !self.matches.iter().all(|&m| m) {
            std::process::exit(1);
        }
    }

    fn literal(&mut self, pattern: &str) {
        self.matches.push(self.input.contains(pattern));
    }

    fn digit(&mut self) {
        for ch in self.input.chars() {
            if ch.is_numeric() {
                self.matches.push(true);
                break;
            }
        }
    }

    fn alphanumeric(&mut self) {
        for ch in self.input.chars() {
            if ch.is_alphanumeric() {
                self.matches.push(true);
                break;
            }
        }
    }

    fn positive_character_group(&mut self, chars: Vec<char>) {
        let mut matches = vec![];
        for char in self.input.chars() {
            if chars.contains(&char) {
                matches.push(true);
            }
        }

        if matches.is_empty() {
            matches.push(false);
        }
        let result = matches.into_iter().all(|c| c);
        self.matches.push(result);
    }

    fn negative_character_group(&mut self, chars: Vec<char>) {
        let mut matches = vec![];
        for char in self.input.chars() {
            if !chars.contains(&char) {
                matches.push(true);
            }
        }

        if matches.is_empty() {
            matches.push(false);
        }
        let result = matches.into_iter().all(|c| c);
        self.matches.push(result);
    }
}

fn parse_pattern(pattern: &str) -> Vec<RegexOperation> {
    let mut ops = vec![];
    let mut buf = String::new();
    let mut iter = pattern.chars();

    while let Some(next) = iter.next() {
        match next {
            '[' => {
                let mut pat = Vec::new();
                let mut pos = true;
                while let Some(ch) = iter.next() {
                    if ch == ']' {
                        break;
                    }

                    if ch == '^' {
                        pos = false;
                        continue;
                    }

                    pat.push(ch);
                }

                if pos {
                    ops.push(RegexOperation::PositiveCharGroup { chars: pat });
                } else {
                    ops.push(RegexOperation::NegativeCharGroup { chars: pat });
                }
            }
            '\\' => {
                let param = iter.next().unwrap();
                match param {
                    'd' => ops.push(RegexOperation::Digit),
                    'w' => ops.push(RegexOperation::Alphanumeric),
                    _ => unimplemented!("{param} not implemented"),
                }
            }
            _ => buf.push(next),
        }
    }

    if !buf.is_empty() {
        ops.push(RegexOperation::Literal { exact: buf });
    }

    ops
}
