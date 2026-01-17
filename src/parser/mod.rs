pub mod ast;
mod expr;
mod stmt;
mod types;
use crate::error::{NebulaError, NebulaResult};
use crate::lexer::{Token, TokenKind};
pub use ast::*;
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
    pub fn parse_program(&mut self) -> NebulaResult<Program> {
        let mut items = Vec::new();
        self.skip_newlines();
        while !self.is_at_end() {
            items.push(self.parse_item()?);
            self.skip_newlines();
        }
        Ok(Program { items })
    }
    fn parse_item(&mut self) -> NebulaResult<Item> {
        self.skip_newlines();
        match &self.peek().kind {
            TokenKind::Function | TokenKind::Async => self.parse_function().map(Item::Function),
            TokenKind::Struct => self.parse_struct().map(Item::Struct),
            TokenKind::Enum => self.parse_enum().map(Item::Enum),
            TokenKind::Type => self.parse_type_alias().map(Item::TypeAlias),
            TokenKind::Mod => self.parse_module().map(Item::Module),
            TokenKind::Use => self.parse_use().map(Item::Use),
            _ => {
                let stmt = self.parse_statement()?;
                Ok(Item::Statement(stmt))
            }
        }
    }
    fn parse_function(&mut self) -> NebulaResult<Function> {
        let is_async = self.match_token(&TokenKind::Async);
        let start_span = self.expect(TokenKind::Function)?.span;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RightParen)?;
        let return_type = None;
        let body = if self.match_token(&TokenKind::Assign) {
            FunctionBody::Expression(self.parse_expression()?)
        } else {
            self.expect(TokenKind::Do)?;
            let stmts = self.parse_block_until_end()?;
            self.expect(TokenKind::End)?;
            FunctionBody::Block(stmts)
        };
        Ok(Function {
            name,
            params,
            return_type,
            body,
            is_async,
            span: start_span,
        })
    }
    fn parse_params(&mut self) -> NebulaResult<Vec<Param>> {
        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                let variadic =
                    self.match_token(&TokenKind::DotDot) && self.match_token(&TokenKind::Dot);
                let name = self.expect_identifier()?;
                let ty = if self.match_token(&TokenKind::Colon) {
                    Some(self.parse_type()?)
                } else {
                    None
                };
                let default = if self.match_token(&TokenKind::Assign) {
                    Some(self.parse_expression()?)
                } else {
                    None
                };
                params.push(Param {
                    name,
                    ty,
                    default,
                    variadic,
                });
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        Ok(params)
    }
    fn parse_struct(&mut self) -> NebulaResult<Struct> {
        let start_span = self.expect(TokenKind::Struct)?.span;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut fields = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            let field_name = self.expect_identifier()?;
            self.expect(TokenKind::Colon)?;
            let field_type = self.parse_type()?;
            fields.push(Field {
                name: field_name,
                ty: field_type,
            });
            self.match_token(&TokenKind::Comma);
            self.skip_newlines();
        }
        self.expect(TokenKind::RightBrace)?;
        Ok(Struct {
            name,
            fields,
            span: start_span,
        })
    }
    fn parse_enum(&mut self) -> NebulaResult<Enum> {
        let start_span = self.expect(TokenKind::Enum)?.span;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut variants = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(&TokenKind::RightBrace) {
                break;
            }
            variants.push(self.expect_identifier()?);
            self.match_token(&TokenKind::Comma);
            self.skip_newlines();
        }
        self.expect(TokenKind::RightBrace)?;
        Ok(Enum {
            name,
            variants,
            span: start_span,
        })
    }
    fn parse_type_alias(&mut self) -> NebulaResult<TypeAlias> {
        let start_span = self.expect(TokenKind::Type)?.span;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::Assign)?;
        let ty = self.parse_type()?;
        Ok(TypeAlias {
            name,
            ty,
            span: start_span,
        })
    }
    fn parse_module(&mut self) -> NebulaResult<Module> {
        let start_span = self.expect(TokenKind::Mod)?.span;
        let name = self.expect_identifier()?;
        Ok(Module {
            name,
            span: start_span,
        })
    }
    fn parse_use(&mut self) -> NebulaResult<Use> {
        let start_span = self.expect(TokenKind::Use)?.span;
        let path = self.expect_identifier()?;
        let alias = if self.match_token(&TokenKind::As) {
            Some(self.expect_identifier()?)
        } else {
            None
        };
        Ok(Use {
            path,
            alias,
            span: start_span,
        })
    }
    fn parse_block_until_end(&mut self) -> NebulaResult<Vec<Stmt>> {
        let mut statements = Vec::new();
        self.skip_newlines();
        while !self.check(&TokenKind::End)
            && !self.check(&TokenKind::Elsif)
            && !self.check(&TokenKind::Else)
            && !self.check(&TokenKind::Catch)
            && !self.check(&TokenKind::Finally)
            && !self.is_at_end()
        {
            statements.push(self.parse_statement()?);
            self.skip_newlines();
        }
        Ok(statements)
    }
    pub fn parse_statement(&mut self) -> NebulaResult<Stmt> {
        self.skip_newlines();
        match &self.peek().kind {
            TokenKind::Perm => self.parse_const(),
            TokenKind::Give => self.parse_return(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::For => self.parse_for(),
            TokenKind::Each => self.parse_each(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Try => self.parse_try(),
            TokenKind::Arrow => self.parse_return(),
            TokenKind::Break => {
                self.advance();
                Ok(Stmt::Break)
            }
            TokenKind::Continue => {
                self.advance();
                Ok(Stmt::Continue)
            }
            _ => {
                let expr = self.parse_expression()?;
                if self.match_token(&TokenKind::Assign) {
                    let value = self.parse_expression()?;
                    Ok(Stmt::Assignment {
                        target: expr,
                        value,
                    })
                } else if let Some(op) = self.match_compound_assign() {
                    let value = self.parse_expression()?;
                    Ok(Stmt::CompoundAssignment {
                        target: expr,
                        op,
                        value,
                    })
                } else {
                    Ok(Stmt::Expression(expr))
                }
            }
        }
    }
    fn match_compound_assign(&mut self) -> Option<CompoundOp> {
        match &self.peek().kind {
            TokenKind::PlusAssign => {
                self.advance();
                Some(CompoundOp::Add)
            }
            TokenKind::MinusAssign => {
                self.advance();
                Some(CompoundOp::Sub)
            }
            TokenKind::StarAssign => {
                self.advance();
                Some(CompoundOp::Mul)
            }
            TokenKind::SlashAssign => {
                self.advance();
                Some(CompoundOp::Div)
            }
            _ => None,
        }
    }
    fn parse_const(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::Perm)?;
        let name = self.expect_identifier()?;
        let ty = if self.match_token(&TokenKind::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign)?;
        let value = self.parse_expression()?;
        Ok(Stmt::Const { name, ty, value })
    }
    fn parse_if(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::If)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Do)?;
        let then_block = self.parse_block_until_end()?;
        let mut elif_branches = Vec::new();
        while self.match_token(&TokenKind::Elsif) {
            let elif_cond = self.parse_expression()?;
            self.expect(TokenKind::Do)?;
            let elif_body = self.parse_block_until_end()?;
            elif_branches.push((elif_cond, elif_body));
        }
        let else_block = if self.match_token(&TokenKind::Else) {
            Some(self.parse_block_until_end()?)
        } else {
            None
        };
        self.expect(TokenKind::End)?;
        Ok(Stmt::If {
            condition,
            then_block,
            elif_branches,
            else_block,
        })
    }
    fn parse_while(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::While)?;
        let condition = self.parse_expression()?;
        self.expect(TokenKind::Do)?;
        let body = self.parse_block_until_end()?;
        self.expect(TokenKind::End)?;
        Ok(Stmt::While { condition, body })
    }
    fn parse_for(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::For)?;
        let var = self.expect_identifier()?;
        self.expect(TokenKind::Assign)?;
        let start = self.parse_expression()?;
        self.expect(TokenKind::Comma)?;
        let end = self.parse_expression()?;
        let step = if self.match_token(&TokenKind::Comma) {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenKind::Do)?;
        let body = self.parse_block_until_end()?;
        self.expect(TokenKind::End)?;
        Ok(Stmt::For {
            var,
            start,
            end,
            step,
            body,
        })
    }
    fn parse_each(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::Each)?;
        let var = self.expect_identifier()?;
        self.expect(TokenKind::In)?;
        let iterator = self.parse_expression()?;
        self.expect(TokenKind::Do)?;
        let body = self.parse_block_until_end()?;
        self.expect(TokenKind::End)?;
        Ok(Stmt::Each {
            var,
            iterator,
            body,
        })
    }
    fn parse_match(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::Match)?;
        let value = self.parse_expression()?;
        self.expect(TokenKind::Do)?;
        self.skip_newlines();
        let mut arms = Vec::new();
        while !self.check(&TokenKind::End) && !self.is_at_end() {
            let pattern = self.parse_pattern()?;
            self.expect(TokenKind::FatArrow)?;
            let body = self.parse_expression()?;
            arms.push(MatchArm { pattern, body });
            self.skip_newlines();
        }
        self.expect(TokenKind::End)?;
        Ok(Stmt::Match { value, arms })
    }
    fn parse_pattern(&mut self) -> NebulaResult<Pattern> {
        match &self.peek().kind {
            TokenKind::Identifier(name) if name == "_" => {
                self.advance();
                Ok(Pattern::Wildcard)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Pattern::Binding(name))
            }
            TokenKind::Integer(n) => {
                let value = *n;
                self.advance();
                Ok(Pattern::Literal(Literal::Integer(value)))
            }
            TokenKind::Float(n) => {
                let value = *n;
                self.advance();
                Ok(Pattern::Literal(Literal::Float(value)))
            }
            TokenKind::String(s) => {
                let value = s.clone();
                self.advance();
                Ok(Pattern::Literal(Literal::String(value)))
            }
            TokenKind::On => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(true)))
            }
            TokenKind::Off => {
                self.advance();
                Ok(Pattern::Literal(Literal::Bool(false)))
            }
            _ => Err(NebulaError::Parse {
                message: "Expected pattern".to_string(),
                span: self.peek().span,
            }),
        }
    }
    fn parse_try(&mut self) -> NebulaResult<Stmt> {
        self.expect(TokenKind::Try)?;
        self.expect(TokenKind::Do)?;
        let try_block = self.parse_block_until_end()?;
        let (catch_var, catch_block) = if self.match_token(&TokenKind::Catch) {
            let var = self.expect_identifier()?;
            self.expect(TokenKind::Do)?;
            let block = self.parse_block_until_end()?;
            (Some(var), Some(block))
        } else {
            (None, None)
        };
        let finally_block = if self.match_token(&TokenKind::Finally) {
            self.expect(TokenKind::Do)?;
            Some(self.parse_block_until_end()?)
        } else {
            None
        };
        self.expect(TokenKind::End)?;
        Ok(Stmt::Try {
            try_block,
            catch_var,
            catch_block,
            finally_block,
        })
    }
    fn parse_return(&mut self) -> NebulaResult<Stmt> {
        if self.check(&TokenKind::Arrow) {
            self.advance();
        } else {
            self.expect(TokenKind::Give)?;
        }
        let value =
            if self.check(&TokenKind::Newline) || self.check(&TokenKind::End) || self.is_at_end() {
                None
            } else {
                Some(self.parse_expression()?)
            };
        Ok(Stmt::Return(value))
    }
    pub fn parse_expression(&mut self) -> NebulaResult<Expr> {
        self.parse_ternary()
    }
    fn parse_ternary(&mut self) -> NebulaResult<Expr> {
        let expr = self.parse_or()?;
        if self.match_token(&TokenKind::Question) {
            let then_expr = self.parse_expression()?;
            self.expect(TokenKind::Colon)?;
            let else_expr = self.parse_expression()?;
            return Ok(Expr::Ternary {
                condition: Box::new(expr),
                then_expr: Box::new(then_expr),
                else_expr: Box::new(else_expr),
            });
        }
        Ok(expr)
    }
    fn parse_or(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_and()?;
        while self.check(&TokenKind::Pipe) && !self.check_next(&TokenKind::Pipe) {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_and(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_not()?;
        while self.check(&TokenKind::Ampersand) {
            self.advance();
            let right = self.parse_not()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_not(&mut self) -> NebulaResult<Expr> {
        if self.match_token(&TokenKind::Bang) {
            let operand = self.parse_not()?;
            Ok(Expr::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            })
        } else {
            self.parse_comparison()
        }
    }
    fn parse_comparison(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_bitor()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Equal => BinaryOp::Eq,
                TokenKind::NotEqual => BinaryOp::Ne,
                TokenKind::Less => BinaryOp::Lt,
                TokenKind::Greater => BinaryOp::Gt,
                TokenKind::LessEqual => BinaryOp::Le,
                TokenKind::GreaterEqual => BinaryOp::Ge,
                _ => break,
            };
            self.advance();
            let right = self.parse_bitor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_bitor(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_bitxor()?;
        while self.match_token(&TokenKind::Pipe) {
            let right = self.parse_bitxor()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitOr,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_bitxor(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_bitand()?;
        while self.match_token(&TokenKind::CaretPipe) {
            let right = self.parse_bitand()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitXor,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_bitand(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_shift()?;
        while self.match_token(&TokenKind::Ampersand) {
            let right = self.parse_shift()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::BitAnd,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_shift(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_range()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::ShiftLeft => BinaryOp::Shl,
                TokenKind::ShiftRight => BinaryOp::Shr,
                _ => break,
            };
            self.advance();
            let right = self.parse_range()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_range(&mut self) -> NebulaResult<Expr> {
        let left = self.parse_additive()?;
        if self.match_token(&TokenKind::DotDot) {
            let right = self.parse_additive()?;
            return Ok(Expr::Range {
                start: Box::new(left),
                end: Box::new(right),
                inclusive: true,
            });
        }
        if self.match_token(&TokenKind::DotDotLess) {
            let right = self.parse_additive()?;
            return Ok(Expr::Range {
                start: Box::new(left),
                end: Box::new(right),
                inclusive: false,
            });
        }
        Ok(left)
    }
    fn parse_additive(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_multiplicative()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Plus => BinaryOp::Add,
                TokenKind::Minus => BinaryOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplicative()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_multiplicative(&mut self) -> NebulaResult<Expr> {
        let mut left = self.parse_power()?;
        loop {
            let op = match &self.peek().kind {
                TokenKind::Star => BinaryOp::Mul,
                TokenKind::Slash => BinaryOp::Div,
                TokenKind::Percent => BinaryOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        Ok(left)
    }
    fn parse_power(&mut self) -> NebulaResult<Expr> {
        let left = self.parse_unary()?;
        if self.match_token(&TokenKind::Caret) {
            let right = self.parse_power()?;
            return Ok(Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Pow,
                right: Box::new(right),
            });
        }
        Ok(left)
    }
    fn parse_unary(&mut self) -> NebulaResult<Expr> {
        match &self.peek().kind {
            TokenKind::Minus => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::Neg,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Tilde => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Unary {
                    op: UnaryOp::BitNot,
                    operand: Box::new(operand),
                })
            }
            TokenKind::Hash => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Length(Box::new(operand)))
            }
            TokenKind::Ampersand => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Borrow(Box::new(operand)))
            }
            TokenKind::LeftArrow => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Receive(Box::new(operand)))
            }
            TokenKind::Await => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(Expr::Await(Box::new(operand)))
            }
            TokenKind::Spawn => {
                self.advance();
                let operand = self.parse_postfix()?;
                Ok(Expr::Spawn(Box::new(operand)))
            }
            _ => self.parse_postfix(),
        }
    }
    fn parse_postfix(&mut self) -> NebulaResult<Expr> {
        let mut expr = self.parse_primary()?;
        loop {
            match &self.peek().kind {
                TokenKind::LeftParen => {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(TokenKind::RightParen)?;
                    expr = Expr::Call {
                        callee: Box::new(expr),
                        args,
                    };
                }
                TokenKind::LeftBracket => {
                    self.advance();
                    let start = if self.check(&TokenKind::Colon) {
                        None
                    } else {
                        Some(Box::new(self.parse_expression()?))
                    };
                    if self.match_token(&TokenKind::Colon) {
                        let end = if self.check(&TokenKind::RightBracket) {
                            None
                        } else {
                            Some(Box::new(self.parse_expression()?))
                        };
                        self.expect(TokenKind::RightBracket)?;
                        expr = Expr::Slice {
                            array: Box::new(expr),
                            start,
                            end,
                        };
                    } else {
                        self.expect(TokenKind::RightBracket)?;
                        if let Some(index) = start {
                            expr = Expr::Index {
                                array: Box::new(expr),
                                index,
                            };
                        }
                    }
                }
                TokenKind::Dot => {
                    self.advance();
                    let field = self.expect_identifier()?;
                    expr = Expr::Field {
                        object: Box::new(expr),
                        field,
                    };
                }
                TokenKind::Colon if self.is_next_identifier() => {
                    self.advance();
                    let method = self.expect_identifier()?;
                    self.expect(TokenKind::LeftParen)?;
                    let args = self.parse_args()?;
                    self.expect(TokenKind::RightParen)?;
                    expr = Expr::MethodCall {
                        receiver: Box::new(expr),
                        method,
                        args,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }
    fn check_next(&self, kind: &TokenKind) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current + 1].kind)
                == std::mem::discriminant(kind)
        }
    }
    fn is_next_identifier(&self) -> bool {
        if self.current + 1 >= self.tokens.len() {
            false
        } else {
            matches!(
                &self.tokens[self.current + 1].kind,
                TokenKind::Identifier(_)
            )
        }
    }
    fn parse_args(&mut self) -> NebulaResult<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                args.push(self.parse_expression()?);
                if !self.match_token(&TokenKind::Comma) {
                    break;
                }
            }
        }
        Ok(args)
    }
    fn parse_primary(&mut self) -> NebulaResult<Expr> {
        match self.peek().kind.clone() {
            TokenKind::Integer(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Integer(n)))
            }
            TokenKind::Float(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(n)))
            }
            TokenKind::String(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String(s)))
            }
            TokenKind::On => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true)))
            }
            TokenKind::Off => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false)))
            }
            TokenKind::Empty => {
                self.advance();
                Ok(Expr::Nil)
            }
            TokenKind::Identifier(name) => {
                self.advance();
                if self.check(&TokenKind::LeftParen)
                    && name
                        .chars()
                        .next()
                        .map(|c| c.is_uppercase())
                        .unwrap_or(false)
                {
                    self.advance();
                    let args = self.parse_args()?;
                    self.expect(TokenKind::RightParen)?;
                    return Ok(Expr::StructInit { name, args });
                }
                Ok(Expr::Variable(name))
            }
            TokenKind::Lst => {
                self.advance();
                self.expect(TokenKind::LeftParen)?;
                let elements = self.parse_args()?;
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::List(elements))
            }
            TokenKind::Map => {
                self.advance();
                self.expect(TokenKind::LeftParen)?;
                let mut pairs = Vec::new();
                if !self.check(&TokenKind::RightParen) {
                    loop {
                        let key = self.parse_expression()?;
                        self.expect(TokenKind::Colon)?;
                        let value = self.parse_expression()?;
                        pairs.push((key, value));
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Map(pairs))
            }
            TokenKind::Tup => {
                self.advance();
                self.expect(TokenKind::LeftParen)?;
                let elements = self.parse_args()?;
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Tuple(elements))
            }
            TokenKind::Err => {
                self.advance();
                self.expect(TokenKind::LeftParen)?;
                let msg = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Error(Box::new(msg)))
            }
            TokenKind::Assert => {
                self.advance();
                self.expect(TokenKind::LeftParen)?;
                let condition = self.parse_expression()?;
                let message = if self.match_token(&TokenKind::Comma) {
                    Some(Box::new(self.parse_expression()?))
                } else {
                    None
                };
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Assert {
                    condition: Box::new(condition),
                    message,
                })
            }
            TokenKind::Nb | TokenKind::Wrd | TokenKind::Int | TokenKind::Fl => {
                let ty = self.parse_type()?;
                self.expect(TokenKind::LeftParen)?;
                let value = self.parse_expression()?;
                self.expect(TokenKind::RightParen)?;
                Ok(Expr::Cast {
                    ty,
                    value: Box::new(value),
                })
            }
            TokenKind::LeftParen => {
                self.advance();
                let first = self.parse_expression()?;
                if self.match_token(&TokenKind::Comma) {
                    let mut elements = vec![first];
                    loop {
                        elements.push(self.parse_expression()?);
                        if !self.match_token(&TokenKind::Comma) {
                            break;
                        }
                    }
                    self.expect(TokenKind::RightParen)?;
                    if self.match_token(&TokenKind::FatArrow) {
                        let params: Result<Vec<_>, _> = elements
                            .iter()
                            .map(|e| {
                                if let Expr::Variable(name) = e {
                                    Ok(name.clone())
                                } else {
                                    Err(NebulaError::Parse {
                                        message: "Lambda parameters must be identifiers"
                                            .to_string(),
                                        span: self.peek().span,
                                    })
                                }
                            })
                            .collect();
                        let body = self.parse_expression()?;
                        return Ok(Expr::Lambda {
                            params: params?,
                            body: Box::new(body),
                        });
                    }
                    return Ok(Expr::Tuple(elements));
                }
                self.expect(TokenKind::RightParen)?;
                if self.match_token(&TokenKind::FatArrow) {
                    if let Expr::Variable(name) = first {
                        let body = self.parse_expression()?;
                        return Ok(Expr::Lambda {
                            params: vec![name],
                            body: Box::new(body),
                        });
                    }
                }
                Ok(first)
            }
            _ => Err(NebulaError::Parse {
                message: format!("Unexpected token: {:?}", self.peek().kind),
                span: self.peek().span,
            }),
        }
    }
    pub fn parse_type(&mut self) -> NebulaResult<Type> {
        let base_type = match &self.peek().kind {
            TokenKind::Nb => {
                self.advance();
                Type::Nb
            }
            TokenKind::Int => {
                self.advance();
                Type::Int
            }
            TokenKind::Fl => {
                self.advance();
                Type::Fl
            }
            TokenKind::Wrd => {
                self.advance();
                Type::Wrd
            }
            TokenKind::By => {
                self.advance();
                Type::By
            }
            TokenKind::Chr => {
                self.advance();
                Type::Chr
            }
            TokenKind::Any => {
                self.advance();
                Type::Any
            }
            TokenKind::Void => {
                self.advance();
                Type::Void
            }
            TokenKind::Empty => {
                self.advance();
                Type::Nil
            }
            TokenKind::Lst => {
                self.advance();
                Type::Lst(None)
            }
            TokenKind::Map => {
                self.advance();
                Type::Map(None, None)
            }
            TokenKind::Tup => {
                self.advance();
                Type::Tup(vec![])
            }
            TokenKind::Set => {
                self.advance();
                Type::Set(None)
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Type::Named(name)
            }
            _ => {
                return Err(NebulaError::Parse {
                    message: format!("Expected type, got {:?}", self.peek().kind),
                    span: self.peek().span,
                })
            }
        };
        if self.match_token(&TokenKind::Question) {
            return Ok(Type::Optional(Box::new(base_type)));
        }
        Ok(base_type)
    }
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap_or_else(|| {
            self.tokens
                .last()
                .expect("Token stream should not be empty")
        })
    }
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || self.peek().kind == TokenKind::Eof
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current.saturating_sub(1)]
    }
    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
        }
    }
    fn match_token(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn expect(&mut self, kind: TokenKind) -> NebulaResult<&Token> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            Err(NebulaError::Parse {
                message: format!("Expected {:?}, got {:?}", kind, self.peek().kind),
                span: self.peek().span,
            })
        }
    }
    fn expect_identifier(&mut self) -> NebulaResult<String> {
        match &self.peek().kind {
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(NebulaError::Parse {
                message: format!("Expected identifier, got {:?}", self.peek().kind),
                span: self.peek().span,
            }),
        }
    }
    fn skip_newlines(&mut self) {
        while self.check(&TokenKind::Newline) {
            self.advance();
        }
    }
}
