//! Scanner/Tokenizer for SpecterScript
//!
//! Based on grammar.md specification with do/end blocks

use super::token::{Token, TokenKind, Span};

/// The lexer that tokenizes SpecterScript source code
pub struct Lexer<'src> {
    #[allow(dead_code)] // Kept for future error reporting enhancements
    source: &'src str,
    chars: Vec<char>,
    current: usize,
    start: usize,
    line: usize,
    column: usize,
    start_column: usize,
    emitted_eof: bool,
}

impl<'src> Lexer<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            current: 0,
            start: 0,
            line: 1,
            column: 1,
            start_column: 1,
            emitted_eof: false,
        }
    }

    /// Scan the next token
    pub fn scan_token(&mut self) -> Option<Token> {
        self.skip_whitespace_and_comments();

        if self.is_at_end() {
            if self.emitted_eof {
                return None;
            }
            self.emitted_eof = true;
            return Some(self.make_token(TokenKind::Eof));
        }

        self.start = self.current;
        self.start_column = self.column;

        let c = self.advance();

        let kind = match c {
            // Single-character tokens
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semicolon,
            '~' => TokenKind::Tilde,
            '?' => TokenKind::Question,
            
            // Could be compound
            '+' => {
                if self.match_char('=') {
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }
            '*' => {
                if self.match_char('=') {
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.match_char('=') {
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }
            '%' => TokenKind::Percent,
            
            '-' => {
                if self.match_char('>') {
                    TokenKind::Arrow
                } else if self.match_char('=') {
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                }
            }
            
            '=' => {
                if self.match_char('=') {
                    TokenKind::Equal
                } else if self.match_char('>') {
                    TokenKind::FatArrow
                } else {
                    TokenKind::Assign
                }
            }
            
            '!' => {
                if self.match_char('=') {
                    TokenKind::NotEqual
                } else {
                    TokenKind::Bang
                }
            }
            
            '<' => {
                if self.match_char('=') {
                    TokenKind::LessEqual
                } else if self.match_char('<') {
                    TokenKind::ShiftLeft
                } else if self.match_char('-') {
                    TokenKind::LeftArrow
                } else {
                    TokenKind::Less
                }
            }
            
            '>' => {
                if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else if self.match_char('>') {
                    TokenKind::ShiftRight
                } else {
                    TokenKind::Greater
                }
            }
            
            '^' => {
                if self.match_char('|') {
                    TokenKind::CaretPipe
                } else {
                    TokenKind::Caret
                }
            }
            
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Pipe,
            
            ':' => TokenKind::Colon,
            
            '.' => {
                if self.match_char('.') {
                    if self.match_char('<') {
                        TokenKind::DotDotLess
                    } else {
                        TokenKind::DotDot
                    }
                } else {
                    TokenKind::Dot
                }
            }
            
            '#' => TokenKind::Hash,

            // Newline
            '\n' => {
                self.line += 1;
                self.column = 1;
                TokenKind::Newline
            }

            // String literals
            '"' => self.scan_string('"'),
            '\'' => {
                // Check for block comment '''
                if self.peek() == '\'' && self.peek_next() == Some('\'') {
                    self.advance(); // second '
                    self.advance(); // third '
                    self.scan_block_comment()
                } else {
                    self.scan_string('\'')
                }
            }
            '`' => self.scan_raw_string(),

            // Numbers
            '0'..='9' => self.scan_number(c),

            // Identifiers and keywords
            c if c.is_alphabetic() || c == '_' => self.scan_identifier(c),

            _ => TokenKind::Error(format!("Unexpected character '{}'", c)),
        };

        Some(self.make_token(kind))
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }

            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '#' => {
                    // Line comment - skip to end
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn scan_block_comment(&mut self) -> TokenKind {
        // Already consumed opening '''
        while !self.is_at_end() {
            if self.peek() == '\'' && self.peek_next() == Some('\'') {
                if self.current + 2 < self.chars.len() && self.chars[self.current + 2] == '\'' {
                    self.advance(); // first '
                    self.advance(); // second '
                    self.advance(); // third '
                    // Comments are skipped, return next token
                    return self.scan_token().map(|t| t.kind).unwrap_or(TokenKind::Eof);
                }
            }
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 0;
            }
            self.advance();
        }
        TokenKind::Error("Unterminated block comment".into())
    }

    fn scan_string(&mut self, quote: char) -> TokenKind {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != quote {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            
            if c == '\\' && !self.is_at_end() {
                let escaped = self.advance();
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '"' => value.push('"'),
                    '\'' => value.push('\''),
                    '0' => value.push('\0'),
                    _ => {
                        return TokenKind::Error(format!("Invalid escape sequence '\\{}'", escaped));
                    }
                }
            } else {
                value.push(c);
            }
        }

        if self.is_at_end() {
            return TokenKind::Error("Unterminated string".into());
        }

        self.advance(); // Closing quote
        TokenKind::String(value)
    }

    fn scan_raw_string(&mut self) -> TokenKind {
        let mut value = String::new();
        
        while !self.is_at_end() && self.peek() != '`' {
            let c = self.advance();
            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
            value.push(c);
        }

        if self.is_at_end() {
            return TokenKind::Error("Unterminated raw string".into());
        }

        self.advance(); // Closing backtick
        TokenKind::String(value)
    }

    fn scan_number(&mut self, first: char) -> TokenKind {
        // Check for hex, binary, octal
        if first == '0' && !self.is_at_end() {
            match self.peek() {
                'x' | 'X' => {
                    self.advance();
                    return self.scan_hex();
                }
                'b' | 'B' => {
                    self.advance();
                    return self.scan_binary();
                }
                'o' | 'O' => {
                    self.advance();
                    return self.scan_octal();
                }
                _ => {}
            }
        }

        // Decimal integer or float
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        // Check for float
        if !self.is_at_end() && self.peek() == '.' {
            if let Some(next) = self.peek_next() {
                if next.is_ascii_digit() {
                    self.advance(); // Consume '.'
                    while !self.is_at_end() && self.peek().is_ascii_digit() {
                        self.advance();
                    }
                    
                    // Scientific notation
                    if !self.is_at_end() && (self.peek() == 'e' || self.peek() == 'E') {
                        self.advance();
                        if !self.is_at_end() && (self.peek() == '+' || self.peek() == '-') {
                            self.advance();
                        }
                        while !self.is_at_end() && self.peek().is_ascii_digit() {
                            self.advance();
                        }
                    }

                    let lexeme = self.current_lexeme();
                    return match lexeme.parse::<f64>() {
                        Ok(n) => TokenKind::Float(n),
                        Err(_) => TokenKind::Error(format!("Invalid float literal: {}", lexeme)),
                    };
                }
            }
        }

        let lexeme = self.current_lexeme();
        match lexeme.parse::<i64>() {
            Ok(n) => TokenKind::Integer(n),
            Err(_) => TokenKind::Error(format!("Invalid integer literal: {}", lexeme)),
        }
    }

    fn scan_hex(&mut self) -> TokenKind {
        let start = self.current;
        while !self.is_at_end() && self.peek().is_ascii_hexdigit() {
            self.advance();
        }
        
        if self.current == start {
            return TokenKind::Error("Expected hex digits after '0x'".into());
        }

        let hex_str: String = self.chars[start..self.current].iter().collect();
        match i64::from_str_radix(&hex_str, 16) {
            Ok(n) => TokenKind::Integer(n),
            Err(_) => TokenKind::Error(format!("Invalid hex literal: 0x{}", hex_str)),
        }
    }

    fn scan_binary(&mut self) -> TokenKind {
        let start = self.current;
        while !self.is_at_end() && (self.peek() == '0' || self.peek() == '1') {
            self.advance();
        }
        
        if self.current == start {
            return TokenKind::Error("Expected binary digits after '0b'".into());
        }

        let bin_str: String = self.chars[start..self.current].iter().collect();
        match i64::from_str_radix(&bin_str, 2) {
            Ok(n) => TokenKind::Integer(n),
            Err(_) => TokenKind::Error(format!("Invalid binary literal: 0b{}", bin_str)),
        }
    }

    fn scan_octal(&mut self) -> TokenKind {
        let start = self.current;
        while !self.is_at_end() && ('0'..='7').contains(&self.peek()) {
            self.advance();
        }
        
        if self.current == start {
            return TokenKind::Error("Expected octal digits after '0o'".into());
        }

        let oct_str: String = self.chars[start..self.current].iter().collect();
        match i64::from_str_radix(&oct_str, 8) {
            Ok(n) => TokenKind::Integer(n),
            Err(_) => TokenKind::Error(format!("Invalid octal literal: 0o{}", oct_str)),
        }
    }

    fn scan_identifier(&mut self, first: char) -> TokenKind {
        let mut ident = String::new();
        ident.push(first);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            ident.push(self.advance());
        }

        // Check for keywords
        if let Some(keyword) = TokenKind::keyword_from_str(&ident) {
            keyword
        } else {
            TokenKind::Identifier(ident)
        }
    }

    // Helper methods

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() { '\0' } else { self.chars[self.current] }
    }

    fn peek_next(&self) -> Option<char> {
        if self.current + 1 >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.current + 1])
        }
    }

    fn advance(&mut self) -> char {
        let c = self.chars[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    fn current_lexeme(&self) -> String {
        self.chars[self.start..self.current].iter().collect()
    }

    fn make_token(&self, kind: TokenKind) -> Token {
        let lexeme = self.current_lexeme();
        let span = Span::new(
            self.start,
            self.current - self.start,
            self.line,
            self.start_column,
        );
        Token::new(kind, span, lexeme)
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scan_token()?;
        if token.kind == TokenKind::Eof && self.emitted_eof {
            return None;
        }
        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "fb x = 42";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        
        assert!(matches!(tokens[0].kind, TokenKind::Fb));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(_)));
        assert!(matches!(tokens[2].kind, TokenKind::Assign));
        assert!(matches!(tokens[3].kind, TokenKind::Integer(42)));
    }

    #[test]
    fn test_keywords() {
        let source = "fn add(a, b) = a + b";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        
        assert!(matches!(tokens[0].kind, TokenKind::Fn));
    }

    #[test]
    fn test_booleans() {
        let source = "yes no nil";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
        
        assert!(matches!(tokens[0].kind, TokenKind::Yes));
        assert!(matches!(tokens[1].kind, TokenKind::No));
        assert!(matches!(tokens[2].kind, TokenKind::Nil));
    }
}
