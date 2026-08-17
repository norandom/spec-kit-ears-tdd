//! The guard language: a deliberately small boolean expression over vocabulary terms.
//!
//! ```text
//! or         := and ("or" and)*
//! and        := not ("and" not)*
//! not        := "not" not | atom
//! atom       := "(" or ")" | comparison | term
//! comparison := term op literal
//! op         := "==" | "!=" | "<" | "<=" | ">" | ">="
//! literal    := integer | 'string' | true | false
//! ```
//!
//! A bare term name means that boolean term holds. Comparisons are against literal constants only:
//! that restriction is what lets a bounded integer be split into a handful of regions at the
//! constants its guards mention, instead of enumerated across its whole declared range. Comparing
//! two terms against each other would need arithmetic the fragment does not have, and admitting it
//! would quietly invalidate the region abstraction everywhere else.
//!
//! Anything outside the grammar is rejected with a position rather than interpreted. A guard
//! language that silently accepts what it does not understand is worse than one that refuses, since
//! the misreading becomes a verdict.

use std::collections::BTreeSet;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Equal,
    NotEqual,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
}

impl Op {
    fn as_str(self) -> &'static str {
        match self {
            Op::Equal => "==",
            Op::NotEqual => "!=",
            Op::Less => "<",
            Op::LessOrEqual => "<=",
            Op::Greater => ">",
            Op::GreaterOrEqual => ">=",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Literal {
    Int(i64),
    Text(String),
    Bool(bool),
}

impl fmt::Display for Literal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Int(value) => write!(formatter, "{value}"),
            Literal::Text(value) => write!(formatter, "'{value}'"),
            Literal::Bool(value) => write!(formatter, "{value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Guard {
    /// A requirement with no condition. Distinct from a condition that happens to hold: it records
    /// that the requirement was never conditional in the first place.
    Always,
    Term(String),
    Compare {
        term: String,
        op: Op,
        value: Literal,
    },
    Not(Box<Guard>),
    And(Vec<Guard>),
    Or(Vec<Guard>),
}

impl Guard {
    /// Every term the guard mentions. This is what decomposition joins on, so it has to be exact:
    /// a term missed here puts a requirement in the wrong component and hides a real interaction.
    pub fn terms(&self) -> BTreeSet<String> {
        let mut found = BTreeSet::new();
        self.collect_terms(&mut found);
        found
    }

    fn collect_terms(&self, found: &mut BTreeSet<String>) {
        match self {
            Guard::Always => {}
            Guard::Term(name) => {
                found.insert(name.clone());
            }
            Guard::Compare { term, .. } => {
                found.insert(term.clone());
            }
            Guard::Not(inner) => inner.collect_terms(found),
            Guard::And(parts) | Guard::Or(parts) => {
                for part in parts {
                    part.collect_terms(found);
                }
            }
        }
    }

    /// Every literal a term is compared against, which is where an integer domain gets cut.
    pub fn comparisons(&self, into: &mut Vec<(String, Op, Literal)>) {
        match self {
            Guard::Always | Guard::Term(_) => {}
            Guard::Compare { term, op, value } => into.push((term.clone(), *op, value.clone())),
            Guard::Not(inner) => inner.comparisons(into),
            Guard::And(parts) | Guard::Or(parts) => {
                for part in parts {
                    part.comparisons(into);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    /// Byte offset into the guard text, so a message can point rather than describe.
    pub position: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} at position {}", self.message, self.position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    Term(String),
    Int(i64),
    Text(String),
    True,
    False,
    And,
    Or,
    Not,
    Op(Op),
    Open,
    Close,
}

struct Lexer<'a> {
    input: &'a str,
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            bytes: input.as_bytes(),
            position: 0,
        }
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            message: message.into(),
            position: self.position,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.bytes.len() && self.bytes[self.position].is_ascii_whitespace() {
            self.position += 1;
        }
    }

    fn tokenize(mut self) -> Result<Vec<(Token, usize)>, ParseError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            if self.position >= self.bytes.len() {
                return Ok(tokens);
            }
            let start = self.position;
            let byte = self.bytes[self.position];
            let token = match byte {
                b'(' => {
                    self.position += 1;
                    Token::Open
                }
                b')' => {
                    self.position += 1;
                    Token::Close
                }
                b'\'' | b'"' => self.text(byte)?,
                b'=' | b'!' | b'<' | b'>' => self.operator()?,
                b'-' | b'0'..=b'9' => self.number()?,
                _ if byte.is_ascii_alphabetic() => self.word(),
                _ => {
                    return Err(self.error(format!(
                        "unexpected character `{}`",
                        self.input[self.position..].chars().next().unwrap_or('?')
                    )))
                }
            };
            tokens.push((token, start));
        }
    }

    fn text(&mut self, quote: u8) -> Result<Token, ParseError> {
        self.position += 1;
        let start = self.position;
        while self.position < self.bytes.len() && self.bytes[self.position] != quote {
            self.position += 1;
        }
        if self.position >= self.bytes.len() {
            return Err(self.error("unterminated string literal"));
        }
        let value = self.input[start..self.position].to_string();
        self.position += 1;
        Ok(Token::Text(value))
    }

    fn operator(&mut self) -> Result<Token, ParseError> {
        let rest = &self.input[self.position..];
        for (text, op) in [
            ("==", Op::Equal),
            ("!=", Op::NotEqual),
            ("<=", Op::LessOrEqual),
            (">=", Op::GreaterOrEqual),
            ("<", Op::Less),
            (">", Op::Greater),
        ] {
            if rest.starts_with(text) {
                self.position += text.len();
                return Ok(Token::Op(op));
            }
        }
        // `=` alone is the mistake people actually make, so name the fix rather than the symbol.
        if rest.starts_with('=') {
            return Err(self.error("use `==` for equality"));
        }
        Err(self.error("unknown operator"))
    }

    fn number(&mut self) -> Result<Token, ParseError> {
        let start = self.position;
        if self.bytes[self.position] == b'-' {
            self.position += 1;
        }
        let digits_start = self.position;
        while self.position < self.bytes.len() && self.bytes[self.position].is_ascii_digit() {
            self.position += 1;
        }
        if self.position == digits_start {
            return Err(self.error("expected a number"));
        }
        self.input[start..self.position]
            .parse()
            .map(Token::Int)
            .map_err(|_| self.error("integer literal is out of range"))
    }

    /// Term identifiers are kebab-case, so `-` belongs to the word. It is never an operator here:
    /// the grammar has no arithmetic, and a negative literal starts with `-` followed by a digit,
    /// which a word cannot.
    fn word(&mut self) -> Token {
        let start = self.position;
        while self.position < self.bytes.len() {
            let byte = self.bytes[self.position];
            if byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_' || byte == b'.' {
                self.position += 1;
            } else {
                break;
            }
        }
        match &self.input[start..self.position] {
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            "true" => Token::True,
            "false" => Token::False,
            word => Token::Term(word.to_string()),
        }
    }
}

struct Parser {
    tokens: Vec<(Token, usize)>,
    index: usize,
}

impl Parser {
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index).map(|(token, _)| token)
    }

    fn position(&self) -> usize {
        self.tokens
            .get(self.index)
            .map(|(_, position)| *position)
            .unwrap_or_else(|| self.tokens.last().map(|(_, p)| *p).unwrap_or(0))
    }

    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            message: message.into(),
            position: self.position(),
        }
    }

    fn parse_or(&mut self) -> Result<Guard, ParseError> {
        let mut parts = vec![self.parse_and()?];
        while matches!(self.peek(), Some(Token::Or)) {
            self.index += 1;
            parts.push(self.parse_and()?);
        }
        Ok(if parts.len() == 1 {
            parts.pop().expect("checked length")
        } else {
            Guard::Or(parts)
        })
    }

    fn parse_and(&mut self) -> Result<Guard, ParseError> {
        let mut parts = vec![self.parse_not()?];
        while matches!(self.peek(), Some(Token::And)) {
            self.index += 1;
            parts.push(self.parse_not()?);
        }
        Ok(if parts.len() == 1 {
            parts.pop().expect("checked length")
        } else {
            Guard::And(parts)
        })
    }

    fn parse_not(&mut self) -> Result<Guard, ParseError> {
        if matches!(self.peek(), Some(Token::Not)) {
            self.index += 1;
            return Ok(Guard::Not(Box::new(self.parse_not()?)));
        }
        self.parse_atom()
    }

    fn parse_atom(&mut self) -> Result<Guard, ParseError> {
        match self.peek().cloned() {
            Some(Token::Open) => {
                self.index += 1;
                let inner = self.parse_or()?;
                if !matches!(self.peek(), Some(Token::Close)) {
                    return Err(self.error("expected `)`"));
                }
                self.index += 1;
                Ok(inner)
            }
            Some(Token::Term(name)) => {
                self.index += 1;
                let Some(Token::Op(op)) = self.peek().cloned() else {
                    return Ok(Guard::Term(name));
                };
                self.index += 1;
                let value = match self.peek().cloned() {
                    Some(Token::Int(value)) => Literal::Int(value),
                    Some(Token::Text(value)) => Literal::Text(value),
                    Some(Token::True) => Literal::Bool(true),
                    Some(Token::False) => Literal::Bool(false),
                    Some(Token::Term(other)) => {
                        return Err(self.error(format!(
                            "`{other}` is a term; comparisons are against literal constants only"
                        )))
                    }
                    _ => return Err(self.error("expected a literal after the operator")),
                };
                self.index += 1;
                Ok(Guard::Compare {
                    term: name,
                    op,
                    value,
                })
            }
            Some(Token::True) => {
                self.index += 1;
                Ok(Guard::Always)
            }
            Some(Token::False) => {
                self.index += 1;
                Ok(Guard::Not(Box::new(Guard::Always)))
            }
            Some(Token::Op(op)) => {
                Err(self.error(format!("`{}` needs a term on its left", op.as_str())))
            }
            Some(_) => Err(self.error("expected a term, a literal, or `(`")),
            None => Err(self.error("guard ended unexpectedly")),
        }
    }
}

pub fn parse(text: &str) -> Result<Guard, ParseError> {
    if text.trim().is_empty() {
        return Ok(Guard::Always);
    }
    let tokens = Lexer::new(text).tokenize()?;
    if tokens.is_empty() {
        return Ok(Guard::Always);
    }
    let mut parser = Parser { tokens, index: 0 };
    let guard = parser.parse_or()?;
    if parser.index != parser.tokens.len() {
        return Err(parser.error("unexpected trailing input"));
    }
    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parsed(text: &str) -> Guard {
        parse(text).unwrap_or_else(|error| panic!("{text:?} should parse: {error}"))
    }

    #[test]
    fn a_bare_term_is_a_boolean_test() {
        assert_eq!(
            parsed("integrity-verified"),
            Guard::Term("integrity-verified".into())
        );
    }

    #[test]
    fn kebab_case_terms_are_not_subtraction() {
        let guard = parsed("operating-mode == 'maintenance'");
        assert_eq!(guard.terms(), ["operating-mode".to_string()].into());
    }

    #[test]
    fn precedence_binds_and_tighter_than_or() {
        // a or (b and c), not (a or b) and c
        assert_eq!(
            parsed("a or b and c"),
            Guard::Or(vec![
                Guard::Term("a".into()),
                Guard::And(vec![Guard::Term("b".into()), Guard::Term("c".into())]),
            ])
        );
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(
            parsed("(a or b) and c"),
            Guard::And(vec![
                Guard::Or(vec![Guard::Term("a".into()), Guard::Term("b".into())]),
                Guard::Term("c".into()),
            ])
        );
    }

    #[test]
    fn an_empty_guard_is_unconditional() {
        assert_eq!(parsed(""), Guard::Always);
        assert_eq!(parsed("   "), Guard::Always);
    }

    #[test]
    fn negative_integers_parse() {
        assert_eq!(
            parsed("depth >= -5"),
            Guard::Compare {
                term: "depth".into(),
                op: Op::GreaterOrEqual,
                value: Literal::Int(-5)
            }
        );
    }

    #[test]
    fn terms_are_collected_from_every_branch() {
        let guard = parsed("not a and (b or c == 1)");
        assert_eq!(
            guard.terms(),
            ["a".to_string(), "b".to_string(), "c".to_string()].into()
        );
    }

    #[test]
    fn comparing_two_terms_is_rejected() {
        // Admitting this would need arithmetic, and would silently invalidate the region
        // abstraction that every integer domain depends on.
        let error = parse("queue-depth < timeout-ms").unwrap_err();
        assert!(error.message.contains("literal constants only"), "{error}");
    }

    #[test]
    fn a_single_equals_names_the_fix() {
        let error = parse("mode = 'on'").unwrap_err();
        assert!(error.message.contains("`==`"), "{error}");
    }

    #[test]
    fn unknown_syntax_is_refused_with_a_position() {
        let error = parse("a & b").unwrap_err();
        assert_eq!(error.position, 2, "{error}");
    }

    #[test]
    fn trailing_input_is_refused() {
        assert!(parse("a b").is_err());
    }
}
