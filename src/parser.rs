use crate::ast::*;
use crate::errors::CompileError;
use crate::lexer::Lexer;
use crate::token::{Token, TokenKind};
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
enum Precedence {
    Lowest,
    LogicalOr,   // ||
    LogicalAnd,  // &&
    Equals,      // == !=
    LessGreater, // < > <= >=
    Range,       // .. ..=
    Sum,         // + -
    Product,     // * / %
}

lazy_static::lazy_static! {
    static ref PRECEDENCES: HashMap<TokenKind, Precedence> = {
        let mut m = HashMap::new();
        m.insert(TokenKind::PipePipe, Precedence::LogicalOr);  // ||
        m.insert(TokenKind::AmpAmp, Precedence::LogicalAnd);   // &&
        m.insert(TokenKind::Eq, Precedence::Equals);
        m.insert(TokenKind::NotEq, Precedence::Equals);
        m.insert(TokenKind::LAngle, Precedence::LessGreater);
        m.insert(TokenKind::RAngle, Precedence::LessGreater);
        m.insert(TokenKind::LtEq, Precedence::LessGreater);
        m.insert(TokenKind::GtEq, Precedence::LessGreater);
        m.insert(TokenKind::DotDot, Precedence::Range);  // ..
        m.insert(TokenKind::DotDotEq, Precedence::Range);  // ..=
        m.insert(TokenKind::Plus, Precedence::Sum);
        m.insert(TokenKind::Minus, Precedence::Sum);
        m.insert(TokenKind::Star, Precedence::Product);
        m.insert(TokenKind::Slash, Precedence::Product);
        m.insert(TokenKind::Percent, Precedence::Product);
        m
    };
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();
        Self { lexer, current, peek }
    }

    pub fn parse_program(&mut self) -> Result<Program, CompileError> {
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::Eof {
            statements.push(self.parse_statement()?);
        }
        Ok(Program { statements })
    }

    fn parse_statement(&mut self) -> Result<Statement, CompileError> {
        let stmt = match self.current_token().kind {
            TokenKind::Use => self.parse_use_statement().map(Statement::Use),
            TokenKind::Struct => self.parse_struct_definition().map(Statement::Struct),
            TokenKind::Enum => self.parse_enum_definition().map(Statement::Enum),
            TokenKind::Impl => self.parse_impl_block().map(Statement::ImplBlock),
            TokenKind::Trait => self.parse_trait_definition().map(Statement::Trait),
            TokenKind::Component => self.parse_component_definition().map(Statement::Component),
            TokenKind::At => {
                // Annotations can be on functions or components
                // Try parsing as function first, which handles @server/@client fn
                // Components are marked with "component" keyword, not @client
                self.parse_function_definition().map(Statement::Function)
            }
            TokenKind::Fn | TokenKind::Server | TokenKind::Client | TokenKind::Async => self.parse_function_definition().map(Statement::Function),
            TokenKind::Let => self.parse_let_statement().map(Statement::Let),
            TokenKind::Const => self.parse_const_declaration().map(Statement::Const),
            TokenKind::Return => self.parse_return_statement().map(Statement::Return),
            TokenKind::If => self.parse_if_statement().map(Statement::If),
            TokenKind::While => self.parse_while_statement().map(Statement::While),
            TokenKind::For => {
                // Look ahead to distinguish between for-in and C-style for loop
                // for item in collection { } vs for (init; cond; update) { }
                if self.peek_token().kind == TokenKind::LParen {
                    self.parse_for_statement().map(Statement::For)
                } else {
                    self.parse_for_in_statement().map(Statement::ForIn)
                }
            },
            TokenKind::Identifier => {
                // Parse as expression first, then check if it's actually an assignment
                // This handles both simple assignments (x = 5) and complex ones (obj.field = 5, arr[0] = 5)
                let expr = self.parse_expression(Precedence::Lowest)?;

                // Check if this is followed by an assignment operator
                if self.current_token().kind == TokenKind::Assign {
                    self.next_token(); // consume =
                    let value = self.parse_expression(Precedence::Lowest)?;
                    let stmt = Statement::Assignment(AssignmentStatement {
                        target: expr,
                        value,
                    });
                    // Don't return early - let the semicolon be consumed below
                    Ok(stmt)
                } else {
                    // Otherwise it's just an expression statement
                    Ok(Statement::Expression(expr))
                }
            }
            _ => self.parse_expression_statement().map(Statement::Expression),
        }?;
        self.consume_if_matches(&TokenKind::Semicolon);
        Ok(stmt)
    }

    fn parse_use_statement(&mut self) -> Result<UseStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Use)?;
        let mut path = vec![self.parse_identifier()?];
        while self.consume_if_matches(&TokenKind::DoubleColon) {
            if self.current_token().kind == TokenKind::LBrace { break; }
            path.push(self.parse_identifier()?);
        }
        let mut imports = Vec::new();
        if self.consume_if_matches(&TokenKind::LBrace) {
            while self.current_token().kind != TokenKind::RBrace {
                imports.push(self.parse_identifier()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RBrace)?;
        }
        Ok(UseStatement { path, imports })
    }
    
    fn parse_type_params(&mut self) -> Result<Vec<TypeParam>, CompileError> {
        if !self.consume_if_matches(&TokenKind::LAngle) {
            return Ok(Vec::new());
        }

        let mut type_params = Vec::new();
        while self.current_token().kind != TokenKind::RAngle {
            let name = self.parse_identifier()?;

            // Parse optional trait bounds: T: Display or T: Display + Clone
            let mut bounds = Vec::new();
            if self.consume_if_matches(&TokenKind::Colon) {
                // Parse first bound
                bounds.push(self.parse_identifier()?);

                // Parse additional bounds with + separator
                while self.consume_if_matches(&TokenKind::Plus) {
                    bounds.push(self.parse_identifier()?);
                }
            }

            type_params.push(TypeParam { name, bounds });

            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }
        self.expect_and_consume(&TokenKind::RAngle)?;
        Ok(type_params)
    }

    fn parse_struct_definition(&mut self) -> Result<StructDefinition, CompileError> {
        self.expect_and_consume(&TokenKind::Struct)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut fields = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            let field_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let field_type = self.parse_type_expression()?;
            fields.push((field_name, field_type));
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(StructDefinition { name, lifetime_params: Vec::new(), type_params, fields, derives: Vec::new() })
    }

    fn parse_enum_definition(&mut self) -> Result<EnumDefinition, CompileError> {
        self.expect_and_consume(&TokenKind::Enum)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut variants = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            let variant_name = self.parse_identifier()?;

            // Check if this variant has associated data
            let fields = if self.consume_if_matches(&TokenKind::LBrace) {
                // Struct-style variant: Name { field1: Type, field2: Type }
                let mut variant_fields = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    let field_name = self.parse_identifier()?;
                    self.expect_and_consume(&TokenKind::Colon)?;
                    let field_type = self.parse_type_expression()?;
                    variant_fields.push((field_name, field_type));
                    if !self.consume_if_matches(&TokenKind::Comma) { break; }
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Some(variant_fields)
            } else if self.consume_if_matches(&TokenKind::LParen) {
                // Tuple-style variant: Name(Type1, Type2)
                let mut variant_fields = Vec::new();
                let mut index = 0;
                while self.current_token().kind != TokenKind::RParen {
                    let field_type = self.parse_type_expression()?;
                    // For tuple variants, use numeric field names
                    variant_fields.push((
                        Identifier { value: index.to_string() },
                        field_type,
                    ));
                    index += 1;
                    if !self.consume_if_matches(&TokenKind::Comma) { break; }
                }
                self.expect_and_consume(&TokenKind::RParen)?;
                Some(variant_fields)
            } else {
                // Simple variant with no data: Name
                None
            };

            variants.push(EnumVariant {
                name: variant_name,
                fields,
            });

            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(EnumDefinition { name, lifetime_params: Vec::new(), type_params, variants, derives: Vec::new() })
    }

    fn parse_impl_block(&mut self) -> Result<ImplBlock, CompileError> {
        // impl<T> TypeName { methods... } or impl<T> Trait for TypeName { methods... }
        self.expect_and_consume(&TokenKind::Impl)?;

        // Parse optional type parameters
        let type_params = self.parse_type_params()?;

        let first_name = self.parse_identifier()?;

        // Skip optional type arguments on the first name (e.g., Box<T>)
        // We store type_params separately, so we can ignore these for now
        if self.current_token().kind == TokenKind::LAngle {
            self.parse_type_params()?; // Consume and ignore type arguments
        }

        // Check if this is "impl Trait for Type" or just "impl Type"
        let (trait_name, type_name) = if self.consume_if_matches(&TokenKind::For) {
            // This is a trait implementation
            let type_name = self.parse_identifier()?;

            // Skip optional type arguments on the type name
            if self.current_token().kind == TokenKind::LAngle {
                self.parse_type_params()?; // Consume and ignore type arguments
            }

            (Some(first_name), type_name)
        } else {
            // This is an inherent implementation
            (None, first_name)
        };

        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut methods = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            // Parse method: fn method_name(...) -> ReturnType { body }
            self.expect_and_consume(&TokenKind::Fn)?;
            let method_name = self.parse_identifier()?;

            // Parse parameter list
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut parameters = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                let param_name = self.parse_identifier()?;
                self.expect_and_consume(&TokenKind::Colon)?;
                let param_type = self.parse_type_expression()?;
                parameters.push(FunctionParameter {
                    name: param_name,
                    type_annotation: param_type,
                });
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect_and_consume(&TokenKind::RParen)?;

            // Parse optional return type (-> Type)
            let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                Some(self.parse_type_expression()?)
            } else {
                None
            };

            // Parse method body (block statement)
            self.expect_and_consume(&TokenKind::LBrace)?;
            let mut statements = Vec::new();
            while self.current_token().kind != TokenKind::RBrace {
                statements.push(self.parse_statement()?);
            }
            self.expect_and_consume(&TokenKind::RBrace)?;

            methods.push(ImplMethod {
                name: method_name,
                parameters,
                return_type,
                body: BlockStatement { statements },
            });
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(ImplBlock { trait_name, lifetime_params: Vec::new(), type_params, type_name, methods })
    }

    fn parse_trait_definition(&mut self) -> Result<TraitDefinition, CompileError> {
        // trait TraitName<T> { method signatures... }
        self.expect_and_consume(&TokenKind::Trait)?;
        let name = self.parse_identifier()?;
        let type_params = self.parse_type_params()?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut methods = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            // Parse method signature: fn method_name(...) -> ReturnType;
            self.expect_and_consume(&TokenKind::Fn)?;
            let method_name = self.parse_identifier()?;

            // Parse parameter list
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut parameters = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                let param_name = self.parse_identifier()?;
                self.expect_and_consume(&TokenKind::Colon)?;
                let param_type = self.parse_type_expression()?;
                parameters.push(FunctionParameter {
                    name: param_name,
                    type_annotation: param_type,
                });
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }
            self.expect_and_consume(&TokenKind::RParen)?;

            // Parse optional return type (-> Type)
            let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                Some(self.parse_type_expression()?)
            } else {
                None
            };

            // Consume semicolon (trait methods are signatures only)
            self.consume_if_matches(&TokenKind::Semicolon);

            methods.push(TraitMethod {
                name: method_name,
                parameters,
                return_type,
            });
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(TraitDefinition { name, lifetime_params: Vec::new(), type_params, methods })
    }

    fn parse_component_definition(&mut self) -> Result<ComponentDefinition, CompileError> {
        // Check for optional @client annotation (components are client-only by default)
        let has_at = self.consume_if_matches(&TokenKind::At);
        let is_client = if has_at {
            self.consume_if_matches(&TokenKind::Client)
        } else {
            true  // Components are client-side by default
        };

        self.expect_and_consume(&TokenKind::Component)?;
        let name = self.parse_identifier()?;
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            let param_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let param_type = self.parse_type_expression()?;
            parameters.push(FunctionParameter {
                name: param_name,
                type_annotation: param_type,
            });
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse component body as a block of statements
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ComponentDefinition {
            name,
            parameters,
            is_client,
            body: BlockStatement { statements },
        })
    }

    fn parse_function_definition(&mut self) -> Result<FunctionDefinition, CompileError> {
        // Check for optional @ symbol (for @server or @client)
        let has_at = self.consume_if_matches(&TokenKind::At);

        // Check for server/client annotations (with or without @)
        let is_server = if has_at {
            self.consume_if_matches(&TokenKind::Server)
        } else {
            self.consume_if_matches(&TokenKind::Server)
        };

        let is_client = if has_at && !is_server {
            self.consume_if_matches(&TokenKind::Client)
        } else if !has_at {
            self.consume_if_matches(&TokenKind::Client)
        } else {
            false
        };

        let is_async = self.consume_if_matches(&TokenKind::Async);

        // Expect fn keyword
        self.expect_and_consume(&TokenKind::Fn)?;

        // Parse function name
        let name = self.parse_identifier()?;

        // Parse optional type parameters
        let type_params = self.parse_type_params()?;

        // Parse parameter list
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut parameters = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            let param_name = self.parse_identifier()?;
            self.expect_and_consume(&TokenKind::Colon)?;
            let param_type = self.parse_type_expression()?;
            parameters.push(FunctionParameter {
                name: param_name,
                type_annotation: param_type,
            });
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;

        // Parse optional return type (-> Type)
        let _return_type = if self.consume_if_matches(&TokenKind::Arrow) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        // Parse function body (block statement)
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(FunctionDefinition {
            name,
            lifetime_params: Vec::new(),
            type_params,
            parameters,
            is_server,
            is_client,
            is_async,
            body: BlockStatement { statements },
        })
    }

    fn parse_type_expression(&mut self) -> Result<TypeExpression, CompileError> {
        // Check if this is a function type: fn(T1, T2) -> R or fn()
        if self.consume_if_matches(&TokenKind::Fn) {
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut param_types = Vec::new();
            while self.current_token().kind != TokenKind::RParen {
                param_types.push(self.parse_type_expression()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RParen)?;

            // Arrow and return type are optional - if not present, default to unit type ()
            let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                Box::new(self.parse_type_expression()?)
            } else {
                // Default to unit type () when no return type specified
                Box::new(TypeExpression::Tuple(Vec::new()))
            };

            return Ok(TypeExpression::Function(param_types, return_type));
        }

        // Check if this is a slice type [T] or sized array [T; N]
        if self.consume_if_matches(&TokenKind::LBracket) {
            let inner_type = self.parse_type_expression()?;

            // Check for sized array [T; N]
            if self.consume_if_matches(&TokenKind::Semicolon) {
                // Parse the size (must be a number literal)
                let size_token = self.current_token().clone();
                if let TokenKind::Integer(val) = size_token.kind {
                    if val < 0 {
                        return Err(CompileError::ParserError {
                            message: "Array size must be non-negative".to_string(),
                            line: size_token.line,
                            column: size_token.column,
                        });
                    }
                    let size = val as usize;
                    self.next_token();
                    self.expect_and_consume(&TokenKind::RBracket)?;
                    return Ok(TypeExpression::SizedArray(Box::new(inner_type), size));
                } else {
                    return Err(CompileError::ParserError {
                        message: "Expected integer literal for array size".to_string(),
                        line: size_token.line,
                        column: size_token.column,
                    });
                }
            }

            // Otherwise it's a slice [T]
            self.expect_and_consume(&TokenKind::RBracket)?;
            return Ok(TypeExpression::Slice(Box::new(inner_type)));
        }

        // Check if this is a reference type (&T or &mut T)
        if self.consume_if_matches(&TokenKind::Ampersand) {
            // Check for &mut T (mutable reference)
            if self.consume_if_matches(&TokenKind::Mut) {
                let inner_type = self.parse_type_expression()?;
                return Ok(TypeExpression::MutableReference(Box::new(inner_type)));
            }
            // Otherwise it's &T (immutable reference)
            let inner_type = self.parse_type_expression()?;
            return Ok(TypeExpression::Reference(Box::new(inner_type)));
        }

        // Check if this is a tuple type: (i32, String, bool)
        if self.consume_if_matches(&TokenKind::LParen) {
            let mut types = Vec::new();

            // Handle empty tuple: ()
            if self.current_token().kind == TokenKind::RParen {
                self.expect_and_consume(&TokenKind::RParen)?;
                return Ok(TypeExpression::Tuple(types));
            }

            // Parse tuple elements
            loop {
                types.push(self.parse_type_expression()?);

                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }

                // Allow trailing comma
                if self.current_token().kind == TokenKind::RParen {
                    break;
                }
            }

            self.expect_and_consume(&TokenKind::RParen)?;
            return Ok(TypeExpression::Tuple(types));
        }

        let name = self.parse_identifier()?;
        if self.consume_if_matches(&TokenKind::LAngle) {
            let mut args = Vec::new();
            while self.current_token().kind != TokenKind::RAngle {
                args.push(self.parse_type_expression()?);
                if !self.consume_if_matches(&TokenKind::Comma) { break; }
            }
            self.expect_and_consume(&TokenKind::RAngle)?;
            Ok(TypeExpression::Generic(name, args))
        } else {
            Ok(TypeExpression::Named(name))
        }
    }

    fn parse_let_statement(&mut self) -> Result<LetStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Let)?;

        // Check for optional mut keyword: let mut x = ...
        let mutable = self.consume_if_matches(&TokenKind::Mut);

        // Parse pattern (identifier or tuple)
        let pattern = self.parse_let_pattern()?;

        // Parse optional type annotation: let x: Type = value
        let type_annotation = if self.consume_if_matches(&TokenKind::Colon) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        self.expect_and_consume(&TokenKind::Assign)?;
        let value = self.parse_expression(Precedence::Lowest)?;
        Ok(LetStatement { pattern, mutable, type_annotation, value })
    }

    fn parse_const_declaration(&mut self) -> Result<ConstDeclaration, CompileError> {
        self.expect_and_consume(&TokenKind::Const)?;

        // Parse constant name (must be an identifier)
        let name = self.parse_identifier()?;

        // Parse optional type annotation: const MAX_SIZE: i32 = 100
        let type_annotation = if self.consume_if_matches(&TokenKind::Colon) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        self.expect_and_consume(&TokenKind::Assign)?;
        let value = self.parse_expression(Precedence::Lowest)?;
        Ok(ConstDeclaration { name, type_annotation, value })
    }

    fn parse_let_pattern(&mut self) -> Result<Pattern, CompileError> {
        if self.current_token().kind == TokenKind::LParen {
            // Tuple pattern: (a, b, c)
            self.expect_and_consume(&TokenKind::LParen)?;
            let mut patterns = vec![];

            // Parse first pattern
            if self.current_token().kind != TokenKind::RParen {
                patterns.push(self.parse_let_pattern()?);

                // Parse remaining patterns
                while self.consume_if_matches(&TokenKind::Comma) {
                    if self.current_token().kind == TokenKind::RParen {
                        break;  // Trailing comma
                    }
                    patterns.push(self.parse_let_pattern()?);
                }
            }

            self.expect_and_consume(&TokenKind::RParen)?;
            Ok(Pattern::Tuple(patterns))
        } else {
            // Simple identifier pattern
            let ident = self.parse_identifier()?;
            Ok(Pattern::Identifier(ident))
        }
    }

    fn parse_return_statement(&mut self) -> Result<ReturnStatement, CompileError> {
        self.expect_and_consume(&TokenKind::Return)?;

        // Check if this is a bare return (followed by semicolon or closing brace)
        let value = if self.current_token().kind == TokenKind::Semicolon ||
                       self.current_token().kind == TokenKind::RBrace {
            // Bare return - return unit/void value represented as integer literal 0
            Expression::IntegerLiteral(0)
        } else {
            self.parse_expression(Precedence::Lowest)?
        };

        Ok(ReturnStatement { value })
    }

    fn parse_if_statement(&mut self) -> Result<IfStatement, CompileError> {
        self.expect_and_consume(&TokenKind::If)?;
        // Disable struct literal parsing in if conditions to avoid ambiguity with the then block
        // Example: `if x { }` should parse `x` as condition and `{ }` as block, not `x {}` as struct literal
        let condition = self.parse_expression_no_struct_literals()?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse then branch
        let mut then_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            then_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;
        let then_branch = BlockStatement { statements: then_statements };

        // Parse optional else branch
        let else_branch = if self.consume_if_matches(&TokenKind::Else) {
            if self.current_token().kind == TokenKind::If {
                // else if
                Some(Box::new(self.parse_if_statement().map(Statement::If)?))
            } else {
                // else block
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut else_statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    else_statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                // Wrap the parsed else block statements in a proper Block expression
                Some(Box::new(Statement::Expression(Expression::Block(
                    BlockStatement { statements: else_statements }
                ))))
            }
        } else {
            None
        };

        Ok(IfStatement {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<WhileStatement, CompileError> {
        self.expect_and_consume(&TokenKind::While)?;
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse loop body
        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(WhileStatement {
            condition,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_for_statement(&mut self) -> Result<ForStatement, CompileError> {
        self.expect_and_consume(&TokenKind::For)?;
        self.expect_and_consume(&TokenKind::LParen)?;

        // Parse init (optional)
        let init = if self.current_token().kind == TokenKind::Semicolon {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        // Consume semicolon after init (if there was one, or standalone)
        if !self.consume_if_matches(&TokenKind::Semicolon) {
            // parse_statement might have consumed it already
        }

        // Parse condition
        let condition = self.parse_expression(Precedence::Lowest)?;
        self.expect_and_consume(&TokenKind::Semicolon)?;

        // Parse update (optional)
        let update = if self.current_token().kind == TokenKind::RParen {
            None
        } else {
            Some(Box::new(self.parse_statement()?))
        };

        self.expect_and_consume(&TokenKind::RParen)?;
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse loop body
        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ForStatement {
            init,
            condition,
            update,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_for_in_statement(&mut self) -> Result<ForInStatement, CompileError> {
        // Parse: for item in collection { body }
        self.expect_and_consume(&TokenKind::For)?;

        // Parse loop variable
        let variable = self.parse_identifier()?;

        // Expect 'in' keyword
        self.expect_and_consume(&TokenKind::In)?;

        // Parse iterator expression
        // Disable struct literal parsing to avoid ambiguity with the loop body
        // Example: `for x in list { }` should parse `list` as iterator and `{ }` as body, not `list {}` as struct literal
        let iterator = self.parse_expression_no_struct_literals()?;

        // Parse loop body
        self.expect_and_consume(&TokenKind::LBrace)?;

        let mut body_statements = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            body_statements.push(self.parse_statement()?);
        }
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(ForInStatement {
            variable,
            iterator,
            body: BlockStatement { statements: body_statements },
        })
    }

    fn parse_expression_statement(&mut self) -> Result<Expression, CompileError> {
        self.parse_expression(Precedence::Lowest)
    }
    
    #[allow(dead_code)] // For future macro support
    fn parse_macro_invocation(&mut self) -> Result<Statement, CompileError> {
        let macro_token = self.current_token().clone();
        self.next_token();
        let mut input_tokens = Vec::new();
        let mut brace_level = 0;
        let mut paren_level = 0;
        loop {
            let token = self.current_token().clone();
            input_tokens.push(token.clone());
            self.next_token();
            match token.kind {
                TokenKind::LParen => paren_level += 1,
                TokenKind::RParen => paren_level -= 1,
                TokenKind::LBrace => brace_level += 1,
                TokenKind::RBrace => brace_level -= 1,
                TokenKind::Eof => return Err(self.error("Unmatched braces in macro invocation")),
                _ => {}
            }
            if paren_level == 0 && brace_level == 0 { break; }
        }
        Ok(Statement::MacroInvocation(MacroInvocation {
            name: Identifier { value: macro_token.lexeme },
            input_tokens,
        }))
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, CompileError> {
        self.parse_expression_internal(precedence, true)
    }

    fn parse_expression_no_struct_literals(&mut self) -> Result<Expression, CompileError> {
        self.parse_expression_internal(Precedence::Lowest, false)
    }

    fn parse_expression_internal(&mut self, precedence: Precedence, allow_struct_literals: bool) -> Result<Expression, CompileError> {
        let mut left_expr = self.parse_prefix_internal(allow_struct_literals)?;
        while self.current_token().kind != TokenKind::Semicolon && precedence < self.current_precedence() {
            left_expr = self.parse_infix(left_expr, allow_struct_literals)?;
        }
        Ok(left_expr)
    }

    fn parse_prefix(&mut self) -> Result<Expression, CompileError> {
        self.parse_prefix_internal(true)
    }

    fn parse_prefix_internal(&mut self, allow_struct_literals: bool) -> Result<Expression, CompileError> {
        let token = self.current_token().clone();

        // Step 1: Parse the base/atomic expression
        let mut expr = match &token.kind {
            TokenKind::Identifier => {
                self.next_token();
                let ident = Identifier { value: token.lexeme };

                // Check if this is a struct literal (Identifier { field: value, ... })
                if allow_struct_literals && self.current_token().kind == TokenKind::LBrace && self.is_struct_literal_ahead() {
                    return self.parse_struct_literal(ident);
                }

                Expression::Identifier(ident)
            },
            TokenKind::Integer(val) => { self.next_token(); Expression::IntegerLiteral(*val) },
            TokenKind::Float(val) => { self.next_token(); Expression::FloatLiteral(val.clone()) },
            TokenKind::String(val) => { self.next_token(); Expression::StringLiteral(val.clone()) },
            TokenKind::Bool(val) => { self.next_token(); Expression::BoolLiteral(*val) },
            TokenKind::Minus | TokenKind::Bang => {
                // Parse prefix expressions: -x or !x
                let operator = token.clone();
                self.next_token();
                let right = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Expression::Prefix(PrefixExpression {
                    operator,
                    right: Box::new(right),
                })
            },
            TokenKind::DotDotDot => {
                // Parse spread operator: ...expr (used in array literals)
                self.next_token();
                let expression = self.parse_expression(Precedence::Lowest)?;
                Expression::Spread(SpreadExpression {
                    expression: Box::new(expression),
                })
            },
            TokenKind::Ampersand => {
                // Parse borrow expression: &x or &mut x
                self.next_token();
                // Check for &mut x (mutable borrow)
                if self.consume_if_matches(&TokenKind::Mut) {
                    let expression = self.parse_expression(Precedence::Product)?;
                    Expression::MutableBorrow(MutableBorrowExpression {
                        expression: Box::new(expression),
                    })
                } else {
                    // Otherwise it's &x (immutable borrow)
                    let expression = self.parse_expression(Precedence::Product)?;
                    Expression::Borrow(BorrowExpression {
                        expression: Box::new(expression),
                    })
                }
            },
            TokenKind::Star => {
                // Parse dereference expression: *x
                self.next_token();
                let expression = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Expression::Dereference(DereferenceExpression {
                    expression: Box::new(expression),
                })
            },
            TokenKind::Await => {
                // Parse await expression: await future_expr
                self.next_token();
                let expression = self.parse_expression(Precedence::Product)?; // High precedence for prefix ops
                Expression::Await(AwaitExpression {
                    expression: Box::new(expression),
                })
            },
            TokenKind::LParen => self.parse_lambda_or_grouped()?,
            TokenKind::LAngle => self.parse_jsx_element()?,
            TokenKind::Pipe => self.parse_lambda_with_pipes()?,
            TokenKind::PipePipe => {
                // Handle || { } closures (no parameters)
                self.next_token(); // consume ||

                // Optional => before body
                self.consume_if_matches(&TokenKind::FatArrow);

                // Parse body expression
                let body = self.parse_expression(Precedence::Lowest)?;

                Expression::Lambda(LambdaExpression {
                    parameters: vec![], // No parameters
                    return_type: None,
                    body: Box::new(body),
                    captures: vec![], // Will be analyzed later
                })
            }
            TokenKind::LBracket => self.parse_array_literal()?,
            TokenKind::Match => self.parse_match_expression()?,
            TokenKind::If => {
                // Parse if-expression: if cond { then_expr } else { else_expr }
                self.next_token(); // consume if
                let condition = Box::new(self.parse_expression(Precedence::Lowest)?);

                // Parse then block
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut then_statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    then_statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                let then_expr = Box::new(Expression::Block(BlockStatement { statements: then_statements }));

                // Parse optional else block
                let else_expr = if self.consume_if_matches(&TokenKind::Else) {
                    if self.current_token().kind == TokenKind::If {
                        // else if - parse as nested if-expression
                        Some(Box::new(self.parse_prefix()?))
                    } else {
                        // else block
                        self.expect_and_consume(&TokenKind::LBrace)?;
                        let mut else_statements = Vec::new();
                        while self.current_token().kind != TokenKind::RBrace {
                            else_statements.push(self.parse_statement()?);
                        }
                        self.expect_and_consume(&TokenKind::RBrace)?;
                        Some(Box::new(Expression::Block(BlockStatement { statements: else_statements })))
                    }
                } else {
                    None
                };

                Expression::IfExpression(IfExpression {
                    condition,
                    then_expr,
                    else_expr,
                })
            },
            TokenKind::LBrace | TokenKind::JsxOpenBrace => {
                // Parse block as expression: { statements... }
                // Accept both LBrace and JsxOpenBrace since in JSX mode { is tokenized as JsxOpenBrace
                self.next_token(); // consume {
                let mut statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace && self.current_token().kind != TokenKind::JsxCloseBrace {
                    statements.push(self.parse_statement()?);
                }
                // Accept either RBrace or JsxCloseBrace
                if !self.consume_if_matches(&TokenKind::RBrace) {
                    self.expect_and_consume(&TokenKind::JsxCloseBrace)?;
                }
                Expression::Block(BlockStatement { statements })
            },
            TokenKind::CssMacro => self.parse_css_macro()?,
            _ => return Err(self.error(&format!("No prefix parse function for {:?}", token.kind))),
        };

        // Step 2: Apply postfix operations (function call, field access, namespace resolution, array indexing, try operator)
        loop {
            match self.current_token().kind {
                TokenKind::LParen => {
                    expr = self.parse_function_call(expr, None)?;
                }
                TokenKind::Dot => {
                    self.next_token(); // consume the dot
                    let field = self.parse_identifier()?;
                    expr = Expression::FieldAccess(FieldAccessExpression {
                        object: Box::new(expr),
                        field,
                    });
                }
                TokenKind::DoubleColon => {
                    self.next_token(); // consume the ::

                    // Check for turbofish syntax: ::<T>
                    if self.current_token().kind == TokenKind::LAngle {
                        self.next_token(); // consume <
                        let mut type_params = Vec::new();

                        // Parse type parameters
                        while self.current_token().kind != TokenKind::RAngle {
                            type_params.push(self.parse_type_expression()?);
                            if !self.consume_if_matches(&TokenKind::Comma) { break; }
                        }
                        self.expect_and_consume(&TokenKind::RAngle)?;

                        // After turbofish, we must have a function call
                        expr = self.parse_function_call(expr, Some(type_params))?;
                    } else {
                        // Handle namespace resolution: console::log()
                        let next_ident = self.parse_identifier()?;

                        // Build the full namespaced identifier
                        if let Expression::Identifier(base_ident) = expr {
                            let namespaced_name = Identifier {
                                value: format!("{}::{}", base_ident.value, next_ident.value)
                            };
                            expr = Expression::Identifier(namespaced_name);
                        } else {
                            // If the left side is not an identifier, treat it as field access for now
                            expr = Expression::FieldAccess(FieldAccessExpression {
                                object: Box::new(expr),
                                field: next_ident,
                            });
                        }
                    }
                }
                TokenKind::LBracket => {
                    self.next_token(); // consume the [
                    let start = self.parse_expression(Precedence::Lowest)?;

                    // Check if this is a slice (arr[start..end]) or regular index access (arr[index])
                    if self.current_token().kind == TokenKind::DotDot || self.current_token().kind == TokenKind::DotDotEq {
                        // This is a slice: arr[start..end] or arr[start..=end]
                        let inclusive = self.current_token().kind == TokenKind::DotDotEq;
                        self.next_token(); // consume .. or ..=
                        let end = self.parse_expression(Precedence::Lowest)?;
                        self.expect_and_consume(&TokenKind::RBracket)?;

                        // Create a Range expression and use it as the index
                        let range_expr = Expression::Range(RangeExpression {
                            start: Some(Box::new(start)),
                            end: Some(Box::new(end)),
                            inclusive,
                        });

                        expr = Expression::IndexAccess(IndexExpression {
                            array: Box::new(expr),
                            index: Box::new(range_expr),
                        });
                    } else {
                        // Regular index access: arr[index]
                        self.expect_and_consume(&TokenKind::RBracket)?;
                        expr = Expression::IndexAccess(IndexExpression {
                            array: Box::new(expr),
                            index: Box::new(start),
                        });
                    }
                }
                TokenKind::As => {
                    self.next_token(); // consume 'as'

                    // Parse the target type
                    let target_type = self.parse_type_expression()?;

                    expr = Expression::TypeCast(TypeCastExpression {
                        expression: Box::new(expr),
                        target_type,
                    });
                }
                TokenKind::Question => {
                    self.next_token(); // consume the ?

                    // Distinguish between try operator (x?) and ternary (x ? y : z)
                    // Try operator: ? is followed by semicolon, comma, closing brace/paren, or end of statement
                    // Ternary: ? is followed by an expression
                    match self.current_token().kind {
                        TokenKind::Semicolon | TokenKind::Comma | TokenKind::RBrace |
                        TokenKind::RParen | TokenKind::RBracket | TokenKind::Eof => {
                            // Try operator
                            expr = Expression::TryOperator(TryOperatorExpression {
                                expression: Box::new(expr),
                            });
                        }
                        _ => {
                            // Ternary operator - parse true branch
                            let true_expr = Box::new(self.parse_expression(Precedence::Lowest)?);

                            // Expect colon
                            self.expect_and_consume(&TokenKind::Colon)?;

                            // Parse false branch
                            let false_expr = Box::new(self.parse_expression(Precedence::Lowest)?);

                            expr = Expression::Ternary(TernaryExpression {
                                condition: Box::new(expr),
                                true_expr,
                                false_expr,
                            });
                        }
                    }
                }
                TokenKind::Bang => {
                    // Handle macro invocation: vec![], println!(), etc.
                    // expr should be an Identifier at this point
                    if let Expression::Identifier(macro_name) = expr {
                        expr = self.parse_macro_call(macro_name)?;
                    } else {
                        return Err(self.error("Macro invocation must follow an identifier"));
                    }
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_function_call(&mut self, function: Expression, type_params: Option<Vec<TypeExpression>>) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LParen)?;
        let mut arguments = Vec::new();
        while self.current_token().kind != TokenKind::RParen {
            arguments.push(self.parse_expression(Precedence::Lowest)?);
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::RParen)?;
        Ok(Expression::FunctionCall(FunctionCall {
            function: Box::new(function),
            arguments,
            type_params,
        }))
    }

    fn parse_macro_call(&mut self, name: Identifier) -> Result<Expression, CompileError> {
        // Consume the ! token
        self.expect_and_consume(&TokenKind::Bang)?;

        // Macros can use (), [], or {} delimiters
        let (open_delimiter, close_delimiter) = match self.current_token().kind {
            TokenKind::LParen => (TokenKind::LParen, TokenKind::RParen),
            TokenKind::LBracket => (TokenKind::LBracket, TokenKind::RBracket),
            TokenKind::LBrace => (TokenKind::LBrace, TokenKind::RBrace),
            _ => return Err(self.error("Expected (, [, or { after macro name!")),
        };

        self.expect_and_consume(&open_delimiter)?;
        let mut arguments = Vec::new();

        while self.current_token().kind != close_delimiter {
            arguments.push(self.parse_expression(Precedence::Lowest)?);
            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }

        self.expect_and_consume(&close_delimiter)?;

        Ok(Expression::MacroCall(MacroCall {
            name,
            arguments,
        }))
    }

    fn parse_lambda_or_grouped(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LParen)?;

        // Try to determine if this is a lambda, tuple, or grouped expression
        if self.current_token().kind == TokenKind::RParen {
            // Empty parameter list for lambda: () =>
            self.expect_and_consume(&TokenKind::RParen)?;
            if self.consume_if_matches(&TokenKind::FatArrow) {
                let body = self.parse_expression(Precedence::Lowest)?;
                return Ok(Expression::Lambda(LambdaExpression {
                    parameters: vec![],
                    return_type: None,
                    body: Box::new(body),
                    captures: vec![],  // Will be analyzed later
                }));
            }
            // Just empty parens, not lambda - error
            return Err(self.error("Unexpected empty parentheses"));
        }

        // Check if this might be a typed lambda parameter list: (x: Type, y: Type) =>
        // Lookahead: if we see identifier followed by colon, it's typed params
        if let TokenKind::Identifier = self.current_token().kind {
            if let TokenKind::Colon = self.peek_token().kind {
                // Parse typed parameters
                let mut parameters = Vec::new();
                loop {
                    let name = self.parse_identifier()?;

                    // Parse type annotation
                    let type_annotation = if self.consume_if_matches(&TokenKind::Colon) {
                        Some(self.parse_type_expression()?)
                    } else {
                        None
                    };

                    parameters.push(LambdaParameter { name, type_annotation });

                    if !self.consume_if_matches(&TokenKind::Comma) {
                        break;
                    }
                }

                self.expect_and_consume(&TokenKind::RParen)?;

                // Optional return type: (x: i32) -> i32 =>
                let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
                    Some(self.parse_type_expression()?)
                } else {
                    None
                };

                self.expect_and_consume(&TokenKind::FatArrow)?;
                let body = self.parse_expression(Precedence::Lowest)?;

                return Ok(Expression::Lambda(LambdaExpression {
                    parameters,
                    return_type,
                    body: Box::new(body),
                    captures: vec![],
                }));
            }
        }

        // Parse first expression
        let first_expr = self.parse_expression(Precedence::Lowest)?;

        // Check if this is a tuple (has comma after first element)
        if self.consume_if_matches(&TokenKind::Comma) {
            // This is a tuple: (expr, ...)
            let mut elements = vec![first_expr];

            // Parse remaining elements
            while self.current_token().kind != TokenKind::RParen {
                elements.push(self.parse_expression(Precedence::Lowest)?);
                if !self.consume_if_matches(&TokenKind::Comma) {
                    break;
                }
            }

            self.expect_and_consume(&TokenKind::RParen)?;
            return Ok(Expression::TupleLiteral(TupleLiteral { elements }));
        }

        // No comma, so it's either grouped expression or lambda
        self.expect_and_consume(&TokenKind::RParen)?;

        // Check if this is actually a lambda with single param: (x) => body
        if self.consume_if_matches(&TokenKind::FatArrow) {
            if let Expression::Identifier(param_name) = first_expr {
                let body = self.parse_expression(Precedence::Lowest)?;
                return Ok(Expression::Lambda(LambdaExpression {
                    parameters: vec![LambdaParameter { name: param_name, type_annotation: None }],
                    return_type: None,
                    body: Box::new(body),
                    captures: vec![],  // Will be analyzed later
                }));
            }
        }

        // Just a grouped expression
        Ok(first_expr)
    }

    fn parse_lambda_with_pipes(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::Pipe)?;
        let mut parameters = Vec::new();

        // Parse parameters with optional type annotations: |x: i32, y: i32|
        while self.current_token().kind != TokenKind::Pipe {
            let name = self.parse_identifier()?;

            // Check for optional type annotation: |x: i32|
            let type_annotation = if self.consume_if_matches(&TokenKind::Colon) {
                Some(self.parse_type_expression()?)
            } else {
                None
            };

            parameters.push(LambdaParameter { name, type_annotation });

            if !self.consume_if_matches(&TokenKind::Comma) { break; }
        }
        self.expect_and_consume(&TokenKind::Pipe)?;

        // Check for optional return type: -> i32
        let return_type = if self.consume_if_matches(&TokenKind::Arrow) {
            Some(self.parse_type_expression()?)
        } else {
            None
        };

        // Check for => or just use the next expression
        self.consume_if_matches(&TokenKind::FatArrow);

        let body = self.parse_expression(Precedence::Lowest)?;
        Ok(Expression::Lambda(LambdaExpression {
            parameters,
            return_type,
            body: Box::new(body),
            captures: vec![],  // Will be analyzed later
        }))
    }

    fn parse_array_literal(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LBracket)?;
        let mut elements = Vec::new();

        // Handle empty array []
        if self.current_token().kind == TokenKind::RBracket {
            self.expect_and_consume(&TokenKind::RBracket)?;
            return Ok(Expression::ArrayLiteral(ArrayLiteral { elements }));
        }

        // Parse comma-separated elements
        while self.current_token().kind != TokenKind::RBracket {
            elements.push(self.parse_expression(Precedence::Lowest)?);
            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }

        self.expect_and_consume(&TokenKind::RBracket)?;
        Ok(Expression::ArrayLiteral(ArrayLiteral { elements }))
    }

    fn parse_struct_literal(&mut self, name: Identifier) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::LBrace)?;
        let mut fields = Vec::new();

        // Handle empty struct literal {}
        if self.current_token().kind == TokenKind::RBrace {
            self.expect_and_consume(&TokenKind::RBrace)?;
            return Ok(Expression::StructLiteral(StructLiteral { name, fields }));
        }

        // Parse comma-separated field: value pairs
        while self.current_token().kind != TokenKind::RBrace {
            let field_name = self.parse_identifier()?;

            // Check for field shorthand: if followed by comma or }, use field_name as both key and value
            if self.current_token().kind == TokenKind::Comma || self.current_token().kind == TokenKind::RBrace {
                // Field shorthand: `username,` is equivalent to `username: username,`
                let field_value = Expression::Identifier(field_name.clone());
                fields.push((field_name, field_value));
            } else {
                // Regular field: value syntax
                self.expect_and_consume(&TokenKind::Colon)?;
                let field_value = self.parse_expression(Precedence::Lowest)?;
                fields.push((field_name, field_value));
            }

            if !self.consume_if_matches(&TokenKind::Comma) {
                break;
            }
        }

        self.expect_and_consume(&TokenKind::RBrace)?;
        Ok(Expression::StructLiteral(StructLiteral { name, fields }))
    }

    // Parse an expression without treating { as struct literal
    // Used for match scrutinee where { starts the match arms
    fn parse_simple_expression(&mut self) -> Result<Expression, CompileError> {
        let token = self.current_token().clone();
        match &token.kind {
            TokenKind::Identifier => {
                self.next_token();
                let ident = Identifier { value: token.lexeme };
                let mut expr = Expression::Identifier(ident);

                // Check for postfix operations but NOT struct literals
                loop {
                    match self.current_token().kind {
                        TokenKind::LParen => {
                            expr = self.parse_function_call(expr, None)?;
                        }
                        TokenKind::Dot => {
                            self.next_token();
                            let field = self.parse_identifier()?;
                            expr = Expression::FieldAccess(FieldAccessExpression {
                                object: Box::new(expr),
                                field,
                            });
                        }
                        TokenKind::DoubleColon => {
                            self.next_token();
                            let next_ident = self.parse_identifier()?;
                            if let Expression::Identifier(base_ident) = expr {
                                expr = Expression::Identifier(Identifier {
                                    value: format!("{}::{}", base_ident.value, next_ident.value)
                                });
                            }
                        }
                        TokenKind::LBracket => {
                            self.next_token();
                            let index = self.parse_expression(Precedence::Lowest)?;
                            self.expect_and_consume(&TokenKind::RBracket)?;
                            expr = Expression::IndexAccess(IndexExpression {
                                array: Box::new(expr),
                                index: Box::new(index),
                            });
                        }
                        _ => break,
                    }
                }
                Ok(expr)
            }
            TokenKind::Integer(val) => {
                self.next_token();
                Ok(Expression::IntegerLiteral(*val))
            }
            TokenKind::String(val) => {
                self.next_token();
                Ok(Expression::StringLiteral(val.clone()))
            }
            TokenKind::LParen => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Lowest)?;
                self.expect_and_consume(&TokenKind::RParen)?;
                Ok(expr)
            }
            _ => self.parse_expression(Precedence::Lowest),
        }
    }

    fn parse_match_expression(&mut self) -> Result<Expression, CompileError> {
        self.expect_and_consume(&TokenKind::Match)?;

        // Parse the scrutinee (the value being matched)
        // We need to parse just the expression without allowing struct literals
        // because the { after the scrutinee starts the match arms, not a struct literal
        let scrutinee = Box::new(self.parse_simple_expression()?);

        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse match arms
        let mut arms = Vec::new();
        while self.current_token().kind != TokenKind::RBrace {
            // Parse patterns - support OR patterns: 3 | 4 | 5 => ...
            let mut patterns = vec![self.parse_pattern()?];

            // Collect additional patterns separated by |
            while self.current_token().kind == TokenKind::Pipe {
                self.next_token(); // consume |
                patterns.push(self.parse_pattern()?);
            }

            self.expect_and_consume(&TokenKind::FatArrow)?;

            // Parse body - can be a block { ... } or an expression
            let body = if self.current_token().kind == TokenKind::LBrace {
                // Parse block as expression
                self.expect_and_consume(&TokenKind::LBrace)?;
                let mut statements = Vec::new();
                while self.current_token().kind != TokenKind::RBrace {
                    statements.push(self.parse_statement()?);
                }
                self.expect_and_consume(&TokenKind::RBrace)?;
                Box::new(Expression::Block(BlockStatement { statements }))
            } else {
                Box::new(self.parse_expression(Precedence::Lowest)?)
            };

            arms.push(MatchArm { patterns, body });

            // Optionally consume comma between arms
            self.consume_if_matches(&TokenKind::Comma);
        }

        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(Expression::Match(MatchExpression { scrutinee, arms }))
    }

    fn parse_pattern(&mut self) -> Result<Pattern, CompileError> {
        let token = self.current_token().clone();

        match token.kind {
            // Wildcard pattern: _
            TokenKind::Identifier if token.lexeme == "_" => {
                self.next_token();
                Ok(Pattern::Wildcard)
            }
            // Identifier (variable binding or enum variant)
            TokenKind::Identifier => {
                let first_ident = self.parse_identifier()?;

                // Check for :: (enum variant like Result::Ok)
                if self.consume_if_matches(&TokenKind::DoubleColon) {
                    let variant_name = self.parse_identifier()?;

                    // Check for associated fields
                    let fields = if self.consume_if_matches(&TokenKind::LParen) {
                        let mut field_patterns = Vec::new();
                        while self.current_token().kind != TokenKind::RParen {
                            field_patterns.push(self.parse_pattern()?);
                            if !self.consume_if_matches(&TokenKind::Comma) { break; }
                        }
                        self.expect_and_consume(&TokenKind::RParen)?;
                        Some(field_patterns)
                    } else {
                        None
                    };

                    // Combine enum name and variant name
                    let full_name = Identifier {
                        value: format!("{}::{}", first_ident.value, variant_name.value)
                    };

                    Ok(Pattern::EnumVariant {
                        name: full_name,
                        fields,
                    })
                } else if self.current_token().kind == TokenKind::LParen {
                    // Enum variant without namespace (like Ok(...) or Err(...))
                    self.consume_if_matches(&TokenKind::LParen);
                    let mut field_patterns = Vec::new();
                    while self.current_token().kind != TokenKind::RParen {
                        field_patterns.push(self.parse_pattern()?);
                        if !self.consume_if_matches(&TokenKind::Comma) { break; }
                    }
                    self.expect_and_consume(&TokenKind::RParen)?;

                    Ok(Pattern::EnumVariant {
                        name: first_ident,
                        fields: Some(field_patterns),
                    })
                } else {
                    // Simple identifier binding
                    Ok(Pattern::Identifier(first_ident))
                }
            }
            // Literal patterns
            TokenKind::Integer(_) | TokenKind::Float(_) | TokenKind::String(_) |
            TokenKind::Bool(_) | TokenKind::True | TokenKind::False => {
                let literal_expr = self.parse_expression(Precedence::Lowest)?;
                Ok(Pattern::Literal(literal_expr))
            }
            _ => Err(self.error(&format!("Expected pattern, found {:?}", token.kind)))
        }
    }

    fn parse_infix(&mut self, left: Expression, allow_struct_literals: bool) -> Result<Expression, CompileError> {
        let operator = self.current_token().clone();

        // Handle range operators specially
        if operator.kind == TokenKind::DotDot || operator.kind == TokenKind::DotDotEq {
            let inclusive = operator.kind == TokenKind::DotDotEq;
            self.next_token();
            let precedence = self.current_precedence();
            let right = self.parse_expression_internal(precedence, allow_struct_literals)?;
            return Ok(Expression::Range(RangeExpression {
                start: Some(Box::new(left)),
                end: Some(Box::new(right)),
                inclusive,
            }));
        }

        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression_internal(precedence, allow_struct_literals)?;
        Ok(Expression::Infix(InfixExpression { left: Box::new(left), operator, right: Box::new(right) }))
    }

    fn parse_jsx_element(&mut self) -> Result<Expression, CompileError> {
        // Check if we need to enter JSX mode for the root element
        let was_jsx_mode = self.lexer.is_jsx_mode();

        let opening_tag = self.parse_jsx_opening_tag_with_mode_check(was_jsx_mode)?;

        // Note: For self-closing tags, exit_jsx_mode() is already called in
        // parse_jsx_opening_tag_with_mode_check() BEFORE consuming the final token,
        // to prevent lexer from generating JSX text tokens in the lookahead buffer

        let children = if opening_tag.self_closing { vec![] } else { self.parse_jsx_children()? };
        let closing_tag = if opening_tag.self_closing { None } else {
            Some(self.parse_jsx_closing_tag_with_mode_check(was_jsx_mode)?)
        };
        if !opening_tag.self_closing && opening_tag.name.value != closing_tag.as_ref().unwrap().value {
            return Err(self.error("Mismatched closing tag"));
        }

        Ok(Expression::JsxElement(JsxElement { opening_tag, children, closing_tag }))
    }

    fn parse_jsx_opening_tag_with_mode_check(&mut self, was_jsx_mode: bool) -> Result<JsxOpeningTag, CompileError> {
        self.expect_and_consume(&TokenKind::LAngle)?;

        // CRITICAL FIX: Enter JSX mode BEFORE parsing tag name
        // This ensures peek is fetched in JSX mode when we parse the identifier
        // Without this, {expr} children are tokenized as JsxText instead of separate tokens
        // ALSO: Always push baseline for nested JSX (e.g. JSX inside ternary/expressions)
        if !was_jsx_mode {
            self.lexer.enter_jsx_mode();
        } else {
            // Already in JSX mode - track nested JSX element
            // This pushes a new baseline for JSX inside expressions: {cond ? (<div>...</div>) : ...}
            self.lexer.enter_nested_jsx();
        }

        let name = self.parse_identifier()?;

        let mut attributes = vec![];
        while self.current_token().kind != TokenKind::RAngle &&
              self.current_token().kind != TokenKind::Slash &&
              self.current_token().kind != TokenKind::JsxSelfClose {
            attributes.push(self.parse_jsx_attribute()?);
        }

        // Check for self-closing tag />
        // CRITICAL: Exit JSX mode BEFORE consuming any tokens for self-closing tags
        // to prevent lexer from generating JSX text tokens in the lookahead buffer
        let is_self_closing = self.current_token().kind == TokenKind::JsxSelfClose ||
                              self.current_token().kind == TokenKind::Slash;

        let self_closing = if is_self_closing {
            // Exit JSX mode FIRST, before consuming any tokens
            self.lexer.exit_jsx_mode();

            if self.consume_if_matches(&TokenKind::JsxSelfClose) {
                // Self-closing tag with />
                true
            } else if self.consume_if_matches(&TokenKind::Slash) {
                // Self-closing with separate / and >
                self.expect_and_consume(&TokenKind::RAngle)?;
                true
            } else {
                // Should not reach here
                false
            }
        } else {
            // Regular opening tag - consume >
            self.expect_and_consume(&TokenKind::RAngle)?;
            false
        };
        Ok(JsxOpeningTag { name, attributes, self_closing })
    }

    fn parse_jsx_attribute(&mut self) -> Result<JsxAttribute, CompileError> {
        let name = self.parse_identifier()?;

        // Check if this is a boolean attribute (no = sign)
        // Boolean attributes like `disabled`, `readonly`, `checked` don't have values
        if self.current_token().kind != TokenKind::Assign {
            // Boolean attribute - defaults to true
            let value = Expression::BoolLiteral(true);
            return Ok(JsxAttribute { name, value });
        }

        self.expect_and_consume(&TokenKind::Assign)?;

        // Check if value is wrapped in curly braces for expression interpolation
        if self.consume_if_matches(&TokenKind::JsxOpenBrace) || self.consume_if_matches(&TokenKind::LBrace) {
            let value = self.parse_expression(Precedence::Lowest)?;
            if !self.consume_if_matches(&TokenKind::JsxCloseBrace) {
                self.expect_and_consume(&TokenKind::RBrace)?;
            }
            Ok(JsxAttribute { name, value })
        } else {
            // Parse just the prefix (string literal, identifier, etc.) without infix operators
            // This prevents treating > or / as operators
            let value = self.parse_prefix()?;
            Ok(JsxAttribute { name, value })
        }
    }

    fn parse_jsx_children(&mut self) -> Result<Vec<JsxChild>, CompileError> {
        let mut children = Vec::new();

        loop {
            // Check if we've reached the closing tag
            if self.current_token().kind == TokenKind::LAngle && self.peek_token().kind == TokenKind::Slash {
                break;
            }

            if self.current_token().kind == TokenKind::Eof {
                return Err(self.error("Unclosed JSX element"));
            }

            // Check for JSX text - any content that's not a tag or expression
            // This handles string literals and potentially other tokens as text
            match &self.current_token().kind {
                TokenKind::JsxText(text) => {
                    if !text.is_empty() {
                        children.push(JsxChild::Text(text.clone()));
                    }
                    self.next_token();
                    continue;
                }
                TokenKind::String(text) => {
                    // String literals in JSX are treated as text
                    children.push(JsxChild::Text(text.clone()));
                    self.next_token();
                    continue;
                }
                TokenKind::Illegal(ch) => {
                    // Illegal tokens are Unicode characters (like emojis) that were tokenized
                    // before JSX mode was entered. Treat them as JSX text content.
                    children.push(JsxChild::Text(ch.to_string()));
                    self.next_token();
                    continue;
                }
                // CRITICAL FIX: Keywords that appear as JSX text content
                // When JSX mode is entered too late, keywords get tokenized as keyword tokens instead of JsxText
                // We handle them here by treating them as text content
                TokenKind::In | TokenKind::If | TokenKind::For | TokenKind::Let | TokenKind::Return |
                TokenKind::Match | TokenKind::While | TokenKind::Else | TokenKind::As | TokenKind::Fn |
                TokenKind::Struct | TokenKind::Enum | TokenKind::Impl | TokenKind::Trait |
                TokenKind::Component | TokenKind::Extern | TokenKind::Server | TokenKind::Client |
                TokenKind::Async | TokenKind::Await | TokenKind::Use | TokenKind::Mut |
                TokenKind::True | TokenKind::False |
                TokenKind::Identifier | TokenKind::Bang | TokenKind::Comma => {
                    // Various tokens in JSX children position are text content
                    // This handles words, punctuation, etc.
                    let text = self.current_token().lexeme.clone();
                    children.push(JsxChild::Text(text));
                    self.next_token();
                    continue;
                }
                _ => {}
            }

            // Check for {expr} interpolation with JSX-aware braces
            let is_jsx_open_brace = self.current_token().kind == TokenKind::JsxOpenBrace;
            let is_lbrace = self.current_token().kind == TokenKind::LBrace;
            if is_jsx_open_brace || is_lbrace {
                self.next_token(); // Consume the brace
                let expr = self.parse_expression(Precedence::Lowest)?;
                if !self.consume_if_matches(&TokenKind::JsxCloseBrace) {
                    self.expect_and_consume(&TokenKind::RBrace)?;
                }
                children.push(JsxChild::Expression(Box::new(expr)));
                continue;
            }

            // Check for nested JSX element (starts with <, but not </)
            if self.current_token().kind == TokenKind::LAngle {
                // Double-check it's not a closing tag
                if self.peek_token().kind == TokenKind::Slash {
                    break; // This is actually the closing tag, exit
                }
                // This is a nested element
                let child_expr = self.parse_jsx_element()?;
                if let Expression::JsxElement(el) = child_expr {
                    children.push(JsxChild::Element(Box::new(el)));
                }
                continue;
            }

            // If we get here and haven't matched anything, break
            break;
        }

        Ok(children)
    }

    fn parse_jsx_closing_tag_with_mode_check(&mut self, was_jsx_mode: bool) -> Result<Identifier, CompileError> {
        // Enter closing tag mode to prevent lexer from reading JSX text during closing tag parsing
        self.lexer.enter_closing_tag_mode();

        self.expect_and_consume(&TokenKind::LAngle)?;
        self.expect_and_consume(&TokenKind::Slash)?;
        let name = self.parse_identifier()?;

        // CRITICAL: Exit BOTH closing tag mode AND JSX mode BEFORE consuming `>`
        // This ensures the next token (created during consume) has the correct lexer state
        // including the correct baseline brace depth for at_baseline calculation
        self.lexer.exit_closing_tag_mode();
        self.lexer.exit_jsx_mode();
        self.expect_and_consume(&TokenKind::RAngle)?;

        Ok(name)
    }

    fn parse_identifier(&mut self) -> Result<Identifier, CompileError> {
        let token = self.current_token();
        if let TokenKind::Identifier = &token.kind {
            let ident = Identifier { value: token.lexeme.clone() };
            self.next_token();
            Ok(ident)
        } else {
            Err(self.error(&format!("Expected Identifier, found {:?}", token.kind)))
        }
    }

    fn current_token(&self) -> &Token { &self.current }
    fn peek_token(&self) -> &Token { &self.peek }
    fn current_precedence(&self) -> Precedence { PRECEDENCES.get(&self.current_token().kind).cloned().unwrap_or(Precedence::Lowest) }
    fn next_token(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lexer.next_token();
    }

    fn expect_and_consume(&mut self, expected: &TokenKind) -> Result<(), CompileError> {
        if &self.current_token().kind == expected {
            self.next_token();
            Ok(())
        } else {
            Err(self.error(&format!("Expected {:?}, found {:?}", expected, self.current_token().kind)))
        }
    }

    fn consume_if_matches(&mut self, kind: &TokenKind) -> bool {
        if &self.current_token().kind == kind {
            self.next_token();
            true
        } else {
            false
        }
    }

    // Check if the tokens ahead look like a struct literal
    // A struct literal after { must have either:
    // - } (empty struct)
    // - Identifier (field name)
    // Keywords like if/while/for/let/return indicate a block, not a struct literal
    fn is_struct_literal_ahead(&self) -> bool {
        // Current token should be {, peek token tells us what's inside
        match self.peek_token().kind {
            TokenKind::RBrace => true,  // Empty struct literal: Name {}
            TokenKind::Identifier => true,  // Field name: Name { field: value }
            _ => false,  // Keywords or other tokens: not a struct literal
        }
    }

    fn error(&self, message: &str) -> CompileError {
        let t = self.current_token();
        CompileError::ParserError { message: message.to_string(), line: t.line, column: t.column }
    }

    // CSS Parsing Methods (Phase 7.5 Sprint 1 Task 1.3)

    /// Parse css! { ... } macro
    fn parse_css_macro(&mut self) -> Result<Expression, CompileError> {
        // CRITICAL: Enter CSS mode BEFORE consuming any tokens
        // This ensures that when we consume css! and advance, the NEW peek is fetched in CSS mode
        // Current = css! (normal), Peek = { (normal)
        // After entering CSS mode and consuming css!: Current = { (normal), Peek = .button (CSS mode!)
        self.lexer.enter_css_mode();

        // Expect css! token
        self.expect_and_consume(&TokenKind::CssMacro)?;

        // Consume the opening brace
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse CSS rules
        // Now current should be the first CSS selector
        let mut rules = Vec::new();
        while self.current_token().kind != TokenKind::RBrace && self.current_token().kind != TokenKind::Eof {
            rules.push(self.parse_css_rule()?);
        }

        // Expect closing brace (CSS mode will auto-exit when depth reaches 0)
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(Expression::CssMacro(CssExpression { rules }))
    }

    /// Parse a CSS rule: .button { property: value; } or with nesting
    fn parse_css_rule(&mut self) -> Result<CssRule, CompileError> {
        // Parse selector
        let selector = self.parse_css_selector()?;

        // Expect opening brace for declarations/nested rules
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse declarations and nested rules
        let mut declarations = Vec::new();
        let mut nested_rules = Vec::new();
        let mut media_queries = Vec::new();

        while self.current_token().kind != TokenKind::RBrace && self.current_token().kind != TokenKind::Eof {
            // Check if this is a nested rule, media query, or a declaration
            if self.current_token().kind == TokenKind::CssMedia {
                // Parse media query: @media (condition) { ... }
                media_queries.push(self.parse_css_media_query()?);
            } else if self.is_nested_rule_start() {
                // Parse nested rule recursively
                nested_rules.push(self.parse_css_rule()?);
            } else {
                // Parse declaration
                declarations.push(self.parse_css_declaration()?);

                // Optional semicolon after declaration
                self.consume_if_matches(&TokenKind::Semicolon);
            }
        }

        // Expect closing brace
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(CssRule {
            selector,
            declarations,
            nested_rules,
            media_queries,
        })
    }

    /// Check if the current token indicates the start of a nested CSS rule
    fn is_nested_rule_start(&self) -> bool {
        match &self.current_token().kind {
            TokenKind::CssSelector(_) => true,  // .class, #id, div, &:hover, etc.
            TokenKind::Ampersand => true,        // & for nesting
            _ => false,
        }
    }

    /// Parse CSS selector: .button, #id, div, :hover, etc.
    fn parse_css_selector(&mut self) -> Result<CssSelector, CompileError> {
        let token = self.current_token().clone();

        match &token.kind {
            TokenKind::CssSelector(selector_str) => {
                self.next_token();

                // Parse the selector string to determine type
                if selector_str.starts_with('.') {
                    // Class selector: .button
                    Ok(CssSelector::Class(selector_str[1..].to_string()))
                } else if selector_str.starts_with('#') {
                    // ID selector: #main
                    Ok(CssSelector::Id(selector_str[1..].to_string()))
                } else if selector_str.starts_with("::") {
                    // Pseudo-element: ::before, ::after
                    Ok(CssSelector::PseudoElement(selector_str[2..].to_string()))
                } else if selector_str.starts_with(':') {
                    // Pseudo-class: :hover, :focus
                    Ok(CssSelector::PseudoClass(selector_str[1..].to_string()))
                } else if selector_str.starts_with('&') {
                    // Nested selector: & (Sprint 2)
                    Ok(CssSelector::Nested(selector_str.clone()))
                } else if selector_str.contains(' ') {
                    // Nested/descendant selector: ".card .title"
                    Ok(CssSelector::Nested(selector_str.clone()))
                } else if selector_str.contains(':') {
                    // Compound selector with pseudo: .button:hover
                    self.parse_compound_selector_from_string(selector_str)
                } else if selector_str.matches('.').count() > 1 {
                    // Compound selector with multiple classes: .button.primary
                    self.parse_compound_selector_from_string(selector_str)
                } else {
                    // Element selector: div, button, span
                    Ok(CssSelector::Element(selector_str.clone()))
                }
            }
            TokenKind::Colon => {
                // Check for :: (pseudo-element) vs : (pseudo-class)
                self.next_token(); // consume first :

                // Check if next token is also a colon (::)
                let is_pseudo_element = matches!(self.current_token().kind, TokenKind::Colon);
                if is_pseudo_element {
                    self.next_token(); // consume second :
                }

                // Now parse the name
                let ident_token = self.current_token().clone();
                match &ident_token.kind {
                    TokenKind::CssProperty(name) => {
                        self.next_token();
                        if is_pseudo_element {
                            Ok(CssSelector::PseudoElement(name.clone()))
                        } else {
                            Ok(CssSelector::PseudoClass(name.clone()))
                        }
                    }
                    TokenKind::CssSelector(name) => {
                        self.next_token();
                        if is_pseudo_element {
                            Ok(CssSelector::PseudoElement(name.clone()))
                        } else {
                            Ok(CssSelector::PseudoClass(name.clone()))
                        }
                    }
                    TokenKind::Identifier => {
                        let name = ident_token.lexeme.clone();
                        self.next_token();
                        if is_pseudo_element {
                            Ok(CssSelector::PseudoElement(name))
                        } else {
                            Ok(CssSelector::PseudoClass(name))
                        }
                    }
                    _ => Err(self.error(&format!("Expected identifier after ':', found {:?}", ident_token.kind)))
                }
            }
            _ => Err(self.error(&format!("Expected CSS selector, found {:?}", token.kind)))
        }
    }

    /// Parse CSS declaration: property: value;
    fn parse_css_declaration(&mut self) -> Result<CssDeclaration, CompileError> {
        // Parse property name
        let property_token = self.current_token().clone();
        let property = match &property_token.kind {
            TokenKind::CssProperty(prop) => {
                self.next_token();
                prop.clone()
            }
            _ => return Err(self.error(&format!("Expected CSS property, found {:?}", property_token.kind)))
        };

        // Expect colon
        self.expect_and_consume(&TokenKind::Colon)?;

        // Parse value
        let value = self.parse_css_value()?;

        Ok(CssDeclaration { property, value })
    }

    /// Parse CSS value: blue, 12px, "Arial", etc.
    fn parse_css_value(&mut self) -> Result<CssValue, CompileError> {
        let token = self.current_token().clone();

        match &token.kind {
            TokenKind::CssValue(value_str) => {
                self.next_token();

                // For Sprint 1, we'll use Raw for all values
                // In Sprint 2, we'll parse into specific types (Color, Length, etc.)
                Ok(CssValue::Raw(value_str.clone()))
            }
            TokenKind::String(s) => {
                // String literal: "Arial", "url(...)"
                self.next_token();
                Ok(CssValue::String(s.clone()))
            }
            TokenKind::Integer(i) => {
                // Numeric value: 0, 1, 2
                self.next_token();
                Ok(CssValue::Number(*i as f64))
            }
            TokenKind::Float(f) => {
                // Floating point: 1.5, 0.5
                self.next_token();
                Ok(CssValue::Number(f.parse().unwrap_or(0.0)))
            }
            _ => Err(self.error(&format!("Expected CSS value, found {:?}", token.kind)))
        }
    }

    /// Parse CSS media query: @media (min-width: 768px) { ... }
    fn parse_css_media_query(&mut self) -> Result<CssMediaQuery, CompileError> {
        use crate::ast::CssMediaQuery;

        // Expect @media token
        self.expect_and_consume(&TokenKind::CssMedia)?;

        // Expect opening parenthesis
        self.expect_and_consume(&TokenKind::LParen)?;

        // Read the condition as a string until we hit the closing paren
        // Now that lexer properly tokenizes inside parens, we can read tokens normally
        let mut condition = String::from("(");
        let mut paren_depth = 1;

        while paren_depth > 0 && self.current_token().kind != TokenKind::Eof {
            let token = self.current_token().clone();

            match &token.kind {
                TokenKind::LParen => {
                    condition.push('(');
                    paren_depth += 1;
                }
                TokenKind::RParen => {
                    paren_depth -= 1;
                    if paren_depth > 0 {
                        condition.push(')');
                    }
                }
                _ => {
                    // Add token lexeme with space
                    if !condition.ends_with('(') {
                        condition.push(' ');
                    }
                    condition.push_str(&token.lexeme);
                }
            }

            self.next_token();
        }

        condition.push(')');

        // Expect opening brace for media query block
        self.expect_and_consume(&TokenKind::LBrace)?;

        // Parse declarations within the media query
        let mut declarations = Vec::new();

        while self.current_token().kind != TokenKind::RBrace && self.current_token().kind != TokenKind::Eof {
            declarations.push(self.parse_css_declaration()?);
            self.consume_if_matches(&TokenKind::Semicolon);
        }

        // Expect closing brace
        self.expect_and_consume(&TokenKind::RBrace)?;

        Ok(CssMediaQuery {
            condition,
            declarations,
        })
    }

    /// Parse a compound selector from a string like ".button:hover" or ".button.primary"
    /// Returns a Compound variant containing the individual selectors
    fn parse_compound_selector_from_string(&self, selector_str: &str) -> Result<CssSelector, CompileError> {
        let mut selectors = Vec::new();
        let mut current = String::new();

        let chars: Vec<char> = selector_str.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            match ch {
                '.' => {
                    // New class selector
                    if !current.is_empty() {
                        selectors.push(self.selector_from_string(&current)?);
                    }
                    current = String::from(".");
                    i += 1;
                }
                ':' => {
                    // Check if next char is also : (::)
                    if !current.is_empty() {
                        selectors.push(self.selector_from_string(&current)?);
                    }
                    if i + 1 < chars.len() && chars[i + 1] == ':' {
                        current = String::from("::");
                        i += 2; // skip both colons
                    } else {
                        current = String::from(":");
                        i += 1;
                    }
                }
                _ => {
                    current.push(ch);
                    i += 1;
                }
            }
        }

        // Don't forget the last selector
        if !current.is_empty() {
            selectors.push(self.selector_from_string(&current)?);
        }

        if selectors.len() > 1 {
            Ok(CssSelector::Compound(selectors))
        } else if selectors.len() == 1 {
            Ok(selectors.into_iter().next().unwrap())
        } else {
            Err(self.error("Empty compound selector"))
        }
    }

    /// Convert a selector string fragment to a CssSelector
    fn selector_from_string(&self, s: &str) -> Result<CssSelector, CompileError> {
        if s.starts_with('.') {
            Ok(CssSelector::Class(s[1..].to_string()))
        } else if s.starts_with('#') {
            Ok(CssSelector::Id(s[1..].to_string()))
        } else if s.starts_with("::") {
            Ok(CssSelector::PseudoElement(s[2..].to_string()))
        } else if s.starts_with(':') {
            Ok(CssSelector::PseudoClass(s[1..].to_string()))
        } else {
            Ok(CssSelector::Element(s.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_expr(source: &str) -> Result<Expression, CompileError> {
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        parser.parse_expression(Precedence::Lowest)
    }

    #[test]
    fn test_jsx_empty_element() {
        let expr = parse_expr("<div></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "div");
                assert_eq!(jsx.children.len(), 0);
                assert_eq!(jsx.closing_tag.as_ref().unwrap().value, "div");
                assert!(!jsx.opening_tag.self_closing);
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_self_closing() {
        let expr = parse_expr("<img />").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "img");
                assert_eq!(jsx.children.len(), 0);
                assert!(jsx.closing_tag.is_none());
                assert!(jsx.opening_tag.self_closing);
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_text() {
        let expr = parse_expr("<div>Hello World</div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                // Text may be split into multiple tokens
                assert!(jsx.children.len() >= 1);
                // Verify we have text children
                for child in &jsx.children {
                    match child {
                        JsxChild::Text(_) => {},
                        _ => panic!("Expected text child"),
                    }
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_single_attribute() {
        let expr = parse_expr(r#"<div class="container"></div>"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.attributes.len(), 1);
                let attr = &jsx.opening_tag.attributes[0];
                assert_eq!(attr.name.value, "class");
                match &attr.value {
                    Expression::StringLiteral(s) => assert_eq!(s, "container"),
                    _ => panic!("Expected string literal"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_multiple_attributes() {
        let expr = parse_expr(r#"<div class="container" id="app"></div>"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.attributes.len(), 2);
                assert_eq!(jsx.opening_tag.attributes[0].name.value, "class");
                assert_eq!(jsx.opening_tag.attributes[1].name.value, "id");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_self_closing_with_attribute() {
        let expr = parse_expr(r#"<img src="photo.jpg" />"#).unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert!(jsx.opening_tag.self_closing);
                assert_eq!(jsx.opening_tag.attributes.len(), 1);
                assert_eq!(jsx.opening_tag.attributes[0].name.value, "src");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_nested_element() {
        let expr = parse_expr("<div><span>Hello</span></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.children.len(), 1);
                match &jsx.children[0] {
                    JsxChild::Element(child) => {
                        assert_eq!(child.opening_tag.name.value, "span");
                        assert_eq!(child.children.len(), 1);
                    }
                    _ => panic!("Expected element child"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_with_expression() {
        let expr = parse_expr("<div>Hello {name}!</div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                // Should have at least text, expression, text
                assert!(jsx.children.len() >= 3);
                // Find the expression child
                let has_expr = jsx.children.iter().any(|child| {
                    matches!(child, JsxChild::Expression(e) if matches!(e.as_ref(), Expression::Identifier(id) if id.value == "name"))
                });
                assert!(has_expr, "Should contain expression with identifier 'name'");
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_multiple_children() {
        let expr = parse_expr("<div><span>A</span><span>B</span></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.children.len(), 2);
                match (&jsx.children[0], &jsx.children[1]) {
                    (JsxChild::Element(a), JsxChild::Element(b)) => {
                        assert_eq!(a.opening_tag.name.value, "span");
                        assert_eq!(b.opening_tag.name.value, "span");
                    }
                    _ => panic!("Expected element children"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    #[test]
    fn test_jsx_mismatched_tags() {
        let result = parse_expr("<div></span>");
        assert!(result.is_err());
        if let Err(CompileError::ParserError { message, .. }) = result {
            assert!(message.contains("Mismatched"));
        }
    }

    #[test]
    fn test_jsx_deeply_nested() {
        let expr = parse_expr("<div><section><article>Text</article></section></div>").unwrap();
        match expr {
            Expression::JsxElement(jsx) => {
                assert_eq!(jsx.opening_tag.name.value, "div");
                match &jsx.children[0] {
                    JsxChild::Element(section) => {
                        assert_eq!(section.opening_tag.name.value, "section");
                        match &section.children[0] {
                            JsxChild::Element(article) => {
                                assert_eq!(article.opening_tag.name.value, "article");
                            }
                            _ => panic!("Expected element"),
                        }
                    }
                    _ => panic!("Expected element"),
                }
            }
            _ => panic!("Expected JsxElement"),
        }
    }

    // CSS Parser Tests (Phase 7.5 Sprint 1)

    #[test]
    fn test_css_lexer_tokens_debug() {
        // Debug test to see what tokens the lexer generates
        let source = r#"css! {
            .button {
                background: blue;
            }
        }"#;
        let mut lexer = Lexer::new(source.to_string());

        // Get tokens
        let t1 = lexer.next_token();
        eprintln!("t1: {:?}", t1.kind);
        assert_eq!(t1.kind, TokenKind::CssMacro);

        let t2 = lexer.next_token();
        eprintln!("t2: {:?}", t2.kind);
        assert_eq!(t2.kind, TokenKind::LBrace);

        // Enter CSS mode
        lexer.enter_css_mode();

        let t3 = lexer.next_token();
        eprintln!("t3: {:?}", t3.kind);

        let t4 = lexer.next_token();
        eprintln!("t4: {:?}", t4.kind);

        let t5 = lexer.next_token();
        eprintln!("t5: {:?}", t5.kind);
    }

    #[test]
    fn test_css_macro_simple() {
        // Test parsing just the CSS macro expression
        let source = r#"css! {
            .button {
                background: blue;
            }
        }"#;
        let mut lexer = Lexer::new(source.to_string());
        let mut parser = Parser::new(&mut lexer);
        let expr = parser.parse_expression(Precedence::Lowest);
        if let Err(e) = &expr {
            eprintln!("Parser error: {:?}", e);
        }
        assert!(expr.is_ok(), "Should parse CSS macro successfully");

        // Verify it's a CssMacro expression
        match expr.unwrap() {
            Expression::CssMacro(css_expr) => {
                assert_eq!(css_expr.rules.len(), 1, "Should have 1 CSS rule");
            }
            _ => panic!("Expected CssMacro expression"),
        }
    }

    #[test]
    fn test_css_selector_types() {
        // Test different selector types
        let test_cases = vec![
            (".button", "class"),
            ("#main", "id"),
            ("div", "element"),
            (":hover", "pseudo"),
        ];

        for (selector, selector_type) in test_cases {
            let source = format!(r#"
                let styles = css! {{
                    {} {{
                        color: red;
                    }}
                }};
            "#, selector);

            let mut lexer = Lexer::new(source);
            let mut parser = Parser::new(&mut lexer);
            let program = parser.parse_program();
            if let Err(e) = &program {
                eprintln!("Failed to parse {} selector '{}': {:?}", selector_type, selector, e);
            }
            assert!(program.is_ok(), "Should parse {} selector", selector_type);
        }
    }
}