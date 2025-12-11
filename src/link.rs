// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

//! PSY-Q Linker Script File Parser
//!
//! **n.b.!** This interface is not stable and should be considered unstable.
//!
//! ## Overview
//!
//! The `link.rs` module provides a parser for PSY-Q linker script files (`.LNK` files). These scripts control how the
//! `psylink.exe` program links object files together, specifying memory layouts, symbol definitions, section placement,
//! and overlay configurations.
//!
//! ## Purpose
//!
//! PSY-Q linker scripts define:
//! - **Memory organization**: Where code and data are placed in memory
//! - **Section grouping**: How logical sections map to physical memory regions
//! - **Symbol management**: External references, definitions, and aliases
//! - **Overlay management**: How code overlays are structured for memory-constrained systems
//! - **Library inclusion**: Which object files and libraries to link
//!
//! These scripts were essential for PlayStation 1 development where memory constraints required precise control over code placement and overlays.
//!
//! ## Linker Script Syntax
//!
//! ### Integer Constants
//!
//! Three formats supported:
//! ```asm
//! 1234        # Decimal
//! $1234       # Hexadecimal (0x1234)
//! %1010       # Binary (10)
//! ```
//!
//! ### Commands
//!
//! #### Memory Organization
//!
//! ```asm
//! org $80010000           ; Set origin address
//! workspace $801F0000     ; Set workspace address
//! ```
//!
//! #### File Inclusion
//!
//! ```asm
//! include "main.obj"      ; Include object file
//! inclib "libgpu.lib"     ; Include library file
//! ```
//!
//! #### Symbol Management
//!
//! ```asm
//! ENTRY_POINT = $80010000     ; Symbol assignment
//! ENTRY_POINT equ $80010000   ; Alternative syntax
//!
//! regs pc=ENTRY_POINT         ; Set register value
//!
//! global symbol1, symbol2      ; Declare global symbols
//! xdef exported1, exported2    ; Define exported symbols
//! xref imported1, imported2    ; Declare imported symbols
//! ```
//!
//! #### Section Management
//!
//! ```asm
//! ; Define group with attributes
//! text group org($80010000), size($8000)
//!
//! ; Define section in group
//! .text section text, word
//!
//! ; Alternative section syntax
//! section .text
//! section .data, text        ; Section with group
//! ```
//!
//! #### Overlays
//!
//! ```asm
//! ; Create overlay group
//! overlay group over(main)
//!
//! ; Section in overlay
//! ovl1 section overlay, file("overlay1.obj")
//! ```
//!
//! #### Aliases and Units
//!
//! ```asm
//! _start alias ENTRY_POINT    ; Create symbol alias
//! unit 1                      ; Set unit number
//! public on                   ; Enable public symbols
//! public off                  ; Disable public symbols
//! ```
//!
//! ### Attributes
//!
//! Attributes modify group/section behavior:
//!
//! ```asm
//! bss                    ; Uninitialized data section
//! org($80010000)        ; Set base address
//! obj($80020000)        ; Object-specific address
//! obj()                 ; Object-relative addressing
//! over(groupname)       ; Part of overlay group
//! word                  ; Word-aligned
//! file("path.obj")      ; Associated source file
//! size($8000)          ; Maximum size constraint
//! ```
//!
//! ### Comments
//!
//! ```asm
//! ; Full line comment
//! org $80010000   ; End-of-line comment
//! ```
//!
//! ## Usage Examples
//!
//! ### Parsing a Complete Script
//!
//! ```rust
//! use psyx::link;
//!
//! let script = r#"
//! ; Memory layout
//! org $80010000
//!
//! ; Main program group
//! text group org($80010000), size($8000)
//!
//! ; Code section
//! .text section text, word
//!
//! ; Include main code
//! include "main.obj"
//!
//! ; Library functions
//! inclib "libgpu.lib"
//! "#;
//!
//! for line in script.lines() {
//!     let mut line_str = line;
//!     match link::parse_line(&mut line_str) {
//!         Ok((Some(command), comment)) => {
//!             println!("Command: {:?}", command);
//!             if let Some(c) = comment {
//!                 println!("Comment: {}", c.comment);
//!             }
//!         }
//!         Ok((None, Some(comment))) => {
//!             println!("Comment: {}", comment.comment);
//!         }
//!         Err(e) => {
//!             eprintln!("Parse error: {:?}", e);
//!         }
//!         _ => {}
//!     }
//! }
//! ```
//!
//! ### Parsing Individual Commands
//!
//! ```rust
//! use psyx::link;
//!
//! let mut input = "org $80010000";
//! let result = link::parse_line(&mut input);
//! match result {
//!     Ok((Some(link::Command::Origin { address }), _)) => {
//!         println!("Origin set to: 0x{:x}", address);
//!     }
//!     _ => {}
//! }
//! ```
//!
//! ### Building a Linker Configuration
//!
//! ```no_run
//! use psyx::link;
//!
//! # fn main() -> Result<(), std::io::Error> {
//!     let mut commands = Vec::new();
//!
//!     // Parse entire script
//!     let script = std::fs::read_to_string("game.lnk")?;
//!     for line in script.lines() {
//!         let mut line_str = line;
//!         if let Ok((Some(cmd), _)) = link::parse_line(&mut line_str) {
//!             commands.push(cmd);
//!         }
//!     }
//!
//!     // Process commands
//!     for cmd in commands {
//!         match cmd {
//!             link::Command::Origin { address } => {
//!                 println!("Set origin: 0x{:x}", address);
//!             }
//!             link::Command::Include { filename } => {
//!                 println!("Include: {}", filename);
//!             }
//!             // Handle other commands...
//!             _ => {}
//!         }
//!     }
//! #    Ok(())
//! # }
//! ```
//!
//! ## Common Linker Script Patterns
//!
//! ### Basic Memory Layout
//!
//! ```asm
//! ; PlayStation 1 typical layout
//! org $80010000                   ; Code starts at 1MB mark
//!
//! text group org($80010000)       ; Text segment
//! .text section text, word
//!
//! data group org($80080000)       ; Data segment
//! .data section data
//!
//! bss group org($800C0000), bss   ; Uninitialized data
//! .bss section bss
//! ```
//!
//! ### Overlay System
//!
//! ```asm
//! ; Main program
//! org $80010000
//! main group org($80010000)
//! .text section main
//!
//! ; Overlay region
//! overlay group org($80100000), size($10000)
//!
//! ; Individual overlays (loaded on demand)
//! level1 section overlay, over(main), file("level1.obj")
//! level2 section overlay, over(main), file("level2.obj")
//! ```
//!
//! ### Symbol Management
//!
//! ```asm
//! ; Export entry point
//! xdef _start, main
//!
//! ; Import library functions
//! xref InitGeom, SetDefDrawEnv
//!
//! ; Register setup
//! regs pc=_start
//! regs sp=$801FFF00
//!
//! ; Symbol assignments
//! SCREEN_WIDTH = 320
//! SCREEN_HEIGHT = 240
//! ```
//!
//! ## Error Handling
//!
//! The parser uses `winnow`'s error handling with context:
//!
//! ```rust
//! use psyx::link;
//! use winnow::error::ContextError;
//!
//! let mut input = "...";
//!
//! match link::parse_line(&mut input) {
//!     Ok(result) => { /* ... */ },
//!     Err(e) => {
//!         eprintln!("Parse error at: {}", e);
//!         // Error includes context about what was expected
//!     }
//! }
//! ```
//!
//! Common error scenarios:
//! - **Invalid integer format**: `org xyz` (not a number)
//! - **Missing quotes**: `include file.obj` (needs quotes)
//! - **Invalid attribute syntax**: `org(80010000)` (missing $)
//! - **Unknown command**: Misspelled keywords

use std::fmt;
use std::fmt::Debug;

use winnow::ascii::digit1;
use winnow::ascii::hex_digit1;
use winnow::ascii::space0;
use winnow::ascii::space1;
use winnow::ascii::Caseless;
use winnow::combinator::alt;
use winnow::combinator::cut_err;
use winnow::combinator::delimited;
use winnow::combinator::fail;
use winnow::combinator::opt;
use winnow::combinator::preceded;
use winnow::combinator::separated;
use winnow::combinator::seq;
use winnow::error::ContextError;
use winnow::error::ErrMode;
use winnow::error::StrContext;
use winnow::error::StrContextValue;
use winnow::stream::Stream;
use winnow::token::take_while;
use winnow::ModalResult;
use winnow::Parser;

#[derive(Debug, PartialEq)]
pub enum Attribute {
    BSS,
    Origin { address: u64 },
    Obj { address: Option<u64> },
    Over { group: String },
    Word,
    File { filename: String },
    Size { maxsize: u64 },
}

/// An expression in a linker script
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// Integer constant: `1234`, `$ABCD`, `%1010`
    Constant(u64),

    /// Symbol reference: `BUFFER_START`, `_end`
    Symbol(String),

    /// Binary operation: `a + b`, `x & $FF`
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },

    /// Unary operation: `-x`, `~flags`
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },

    /// Parenthesized expression: `(a + b)`
    Parens(Box<Expression>),

    /// Function call: `sectstart(text)`, `sectbase(1)`
    Function { name: String, arg: Box<Expression> },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Constant(n) => write!(f, "${:x}", n),
            Expression::Symbol(s) => write!(f, "{}", s),
            Expression::Binary { left, op, right } => {
                write!(f, "({} {} {})", left, op, right)
            }
            Expression::Unary { op, operand } => write!(f, "({}{})", op, operand),
            Expression::Parens(expr) => write!(f, "({})", expr),
            Expression::Function { name, arg } => write!(f, "{}({})", name, arg),
        }
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // Arithmetic
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %

    // Bitwise
    And, // &
    Or,  // |
    Xor, // ^
    Shl, // <<
    Shr, // >>

    // Comparison
    Eq, // ==
    Ne, // !=
    Lt, // <
    Le, // <=
    Gt, // >
    Ge, // >=

    // Logical
    LogAnd, // &&
    LogOr,  // ||
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::And => "&",
            BinaryOp::Or => "|",
            BinaryOp::Xor => "^",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Le => "<=",
            BinaryOp::Gt => ">",
            BinaryOp::Ge => ">=",
            BinaryOp::LogAnd => "&&",
            BinaryOp::LogOr => "||",
        };
        write!(f, "{}", s)
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,    // -
    Not,    // ~
    LogNot, // !
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "~",
            UnaryOp::LogNot => "!",
        };
        write!(f, "{}", s)
    }
}

/// Operator precedence (higher = binds tighter)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Precedence(u8);

impl Precedence {
    const LOWEST: Self = Self(0);
    const LOGICAL_OR: Self = Self(1); // ||
    const LOGICAL_AND: Self = Self(2); // &&
    const BITWISE_OR: Self = Self(3); // |
    const BITWISE_XOR: Self = Self(4); // ^
    const BITWISE_AND: Self = Self(5); // &
    const EQUALITY: Self = Self(6); // == !=
    const COMPARISON: Self = Self(7); // < <= > >=
    const SHIFT: Self = Self(8); // << >>
    const ADDITIVE: Self = Self(9); // + -
    const MULTIPLICATIVE: Self = Self(10); // * / %
                                           // n.b.! currently unused
                                           // const UNARY: Self = Self(11); // - ~ !
                                           // const CALL: Self = Self(12); // function()
}

impl BinaryOp {
    fn precedence(self) -> Precedence {
        match self {
            BinaryOp::LogOr => Precedence::LOGICAL_OR,
            BinaryOp::LogAnd => Precedence::LOGICAL_AND,
            BinaryOp::Or => Precedence::BITWISE_OR,
            BinaryOp::Xor => Precedence::BITWISE_XOR,
            BinaryOp::And => Precedence::BITWISE_AND,
            BinaryOp::Eq | BinaryOp::Ne => Precedence::EQUALITY,
            BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => Precedence::COMPARISON,
            BinaryOp::Shl | BinaryOp::Shr => Precedence::SHIFT,
            BinaryOp::Add | BinaryOp::Sub => Precedence::ADDITIVE,
            BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => Precedence::MULTIPLICATIVE,
        }
    }

    fn is_left_associative(self) -> bool {
        true // All binary ops in linker scripts are left-associative
    }
}

#[derive(Debug, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Long,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    /// Include an object file
    Include {
        filename: String,
    },

    /// Include a library file
    IncLib {
        filename: String,
    },

    /// Specify the origin address
    Origin {
        address: u64,
    },

    /// Specify the workspace address
    Workspace {
        address: u64,
    },

    Equals {
        left: String,
        right: Expression,
    },

    Regs {
        register: String,
        expression: Expression,
    },

    Group {
        name: String,
        attributes: Vec<Attribute>,
    },

    Section {
        name: String,
        group: Option<String>,
        attributes: Vec<Attribute>,
    },

    Alias {
        name: String,
        target: String,
    },

    Unit {
        unitnum: u64,
    },

    Global {
        symbols: Vec<String>,
    },

    XDef {
        symbols: Vec<String>,
    },

    XRef {
        symbols: Vec<String>,
    },

    Public {
        public: bool,
    },

    DC {
        size: Size,
        expression: Vec<Expression>,
    },
}

fn parse_file_name(input: &mut &str) -> ModalResult<String> {
    let s = take_while(1.., |c| c != '"').parse_next(input)?;
    Ok(s.to_string())
}

fn parse_symbol(input: &mut &str) -> ModalResult<String> {
    let s = (seq!(
        take_while(1, (('a'..='z'), ('A'..='Z'), '_')),
        take_while(0.., (('a'..='z'), ('A'..='Z'), ('0'..='9'), '?', '_', '.'))
    ))
    .parse_next(input)?;
    Ok(format!("{}{}", s.0, s.1))
}

fn parse_bin_digits(input: &mut &str) -> ModalResult<u64> {
    let digits = take_while(1.., '0'..='1').parse_next(input)?;
    match u64::from_str_radix(digits, 2) {
        Ok(i) => Ok(i),
        Err(_e) => Err(ErrMode::Cut(ContextError::new())),
    }
}

fn parse_decimal_digits(input: &mut &str) -> ModalResult<u64> {
    let digits = digit1.parse_next(input)?;
    match digits.parse::<u64>() {
        Ok(i) => Ok(i),
        Err(_e) => Err(ErrMode::Cut(ContextError::new())),
    }
}

fn parse_hex_digits(input: &mut &str) -> ModalResult<u64> {
    let digits = hex_digit1.parse_next(input)?;
    match u64::from_str_radix(digits, 16) {
        Ok(i) => Ok(i),
        Err(_e) => Err(ErrMode::Cut(ContextError::new())),
    }
}

fn parse_prefixed_digits(input: &mut &str) -> ModalResult<u64> {
    let i = alt((
        ('$', cut_err(parse_hex_digits)),
        ('%', cut_err(parse_bin_digits)),
        fail.context(StrContext::Label("integer constant")),
    ))
    .parse_next(input)?;

    Ok(i.1)
}

fn parse_integer_constant(input: &mut &str) -> ModalResult<u64> {
    alt((
        parse_decimal_digits,
        parse_prefixed_digits,
        fail.context(StrContext::Label("integer constant")),
    ))
    .parse_next(input)
}

fn parse_symbol_list(input: &mut &str) -> ModalResult<Vec<String>> {
    separated(1.., parse_symbol, (space0, ',', space0)).parse_next(input)
}

// Parse known function names
fn parse_function_name(input: &mut &str) -> ModalResult<String> {
    alt((
        "sectstart",
        "sectend",
        "sectbase",
        "sectof",
        "offs",
        "bank",
        "groupstart",
        "groupof",
        "grouporg",
        "seg",
    ))
    .map(|s: &str| s.to_lowercase())
    .parse_next(input)
}

/// Parse a primary expression (atomic unit)
fn parse_primary(input: &mut &str) -> ModalResult<Expression> {
    preceded(
        space0,
        alt((
            // Function call: func(expr)
            (parse_function_name, delimited('(', parse_expression, ')')).map(|(name, arg)| {
                Expression::Function {
                    name,
                    arg: Box::new(arg),
                }
            }),
            // Parenthesized expression: (expr)
            delimited('(', parse_expression, ')').map(|expr| Expression::Parens(Box::new(expr))),
            // Integer constant
            parse_integer_constant.map(Expression::Constant),
            // Symbol
            parse_symbol.map(Expression::Symbol),
            // Error fallback
            fail.context(StrContext::Label("expression")),
        )),
    )
    .parse_next(input)
}

/// Parse a unary expression
fn parse_unary(input: &mut &str) -> ModalResult<Expression> {
    preceded(
        space0,
        alt((
            // Unary operators
            preceded('-', cut_err(parse_unary)).map(|operand| Expression::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(operand),
            }),
            preceded('~', cut_err(parse_unary)).map(|operand| Expression::Unary {
                op: UnaryOp::Not,
                operand: Box::new(operand),
            }),
            preceded('!', cut_err(parse_unary)).map(|operand| Expression::Unary {
                op: UnaryOp::LogNot,
                operand: Box::new(operand),
            }),
            // Primary expression
            parse_primary,
        )),
    )
    .parse_next(input)
}

/// Parse binary operator
fn parse_binary_op(input: &mut &str) -> ModalResult<BinaryOp> {
    preceded(
        space0,
        alt((
            // Two-character operators (must come before single-char)
            "<<".value(BinaryOp::Shl),
            ">>".value(BinaryOp::Shr),
            "==".value(BinaryOp::Eq),
            "!=".value(BinaryOp::Ne),
            "<=".value(BinaryOp::Le),
            ">=".value(BinaryOp::Ge),
            "&&".value(BinaryOp::LogAnd),
            "||".value(BinaryOp::LogOr),
            // Single-character operators
            '+'.value(BinaryOp::Add),
            '-'.value(BinaryOp::Sub),
            '*'.value(BinaryOp::Mul),
            '/'.value(BinaryOp::Div),
            '%'.value(BinaryOp::Mod),
            '&'.value(BinaryOp::And),
            '|'.value(BinaryOp::Or),
            '^'.value(BinaryOp::Xor),
            '<'.value(BinaryOp::Lt),
            '>'.value(BinaryOp::Gt),
        )),
    )
    .parse_next(input)
}

/// Parse a binary expression using precedence climbing
///
/// This implements a Pratt parser, which handles operator precedence
/// and associativity elegantly.
fn parse_binary_rhs(
    input: &mut &str,
    min_precedence: Precedence,
    mut lhs: Expression,
) -> ModalResult<Expression> {
    loop {
        // Try to parse an operator
        let checkpoint = input.checkpoint();
        let op = match opt(parse_binary_op).parse_next(input) {
            Ok(Some(op)) => op,
            Ok(None) => {
                // No more operators
                break;
            }
            Err(e) => return Err(e),
        };

        let precedence = op.precedence();

        // If this operator has lower precedence than what we're looking for,
        // backtrack and return what we have
        if precedence < min_precedence {
            input.reset(&checkpoint);
            break;
        }

        // Parse the right-hand side
        let mut rhs = parse_unary(input)?;

        // Look ahead to see if the next operator has higher precedence
        loop {
            let checkpoint2 = input.checkpoint();
            let next_op = match opt(parse_binary_op).parse_next(input) {
                Ok(Some(op)) => op,
                Ok(None) => {
                    break;
                }
                Err(e) => return Err(e),
            };

            let next_precedence = next_op.precedence();

            // If next operator has higher precedence (or same precedence but right-associative),
            // recursively parse the RHS with higher precedence requirement
            if next_precedence > precedence
                || (next_precedence == precedence && !next_op.is_left_associative())
            {
                input.reset(&checkpoint2);
                rhs = parse_binary_rhs(input, next_precedence, rhs)?;
            } else {
                // Next operator has lower/equal precedence, stop lookahead
                input.reset(&checkpoint2);
                break;
            }
        }

        // Build the binary expression
        lhs = Expression::Binary {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        };
    }

    Ok(lhs)
}

/// Parse a complete expression
pub fn parse_expression(input: &mut &str) -> ModalResult<Expression> {
    let lhs = parse_unary(input)?;
    parse_binary_rhs(input, Precedence::LOWEST, lhs)
}

fn parse_command_generic_filename(command: &str, input: &mut &str) -> ModalResult<String> {
    let c = (
        space0,
        Caseless(command),
        space1,
        "\"",
        parse_file_name,
        "\"",
    )
        .parse_next(input)?;
    Ok(c.4.to_string())
}

fn parse_command_include(input: &mut &str) -> ModalResult<Command> {
    let filename = parse_command_generic_filename("include", input)?;
    Ok(Command::Include { filename })
}

fn parse_command_inclib(input: &mut &str) -> ModalResult<Command> {
    let filename = parse_command_generic_filename("inclib", input)?;
    Ok(Command::IncLib { filename })
}

fn parse_command_origin(input: &mut &str) -> ModalResult<Command> {
    let c = (space0, Caseless("org"), space1, parse_integer_constant).parse_next(input)?;
    Ok(Command::Origin { address: c.3 })
}

fn parse_command_workspace(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        Caseless("workspace"),
        space1,
        parse_integer_constant,
    )
        .parse_next(input)?;
    Ok(Command::Workspace { address: c.3 })
}

fn parse_command_equals(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        parse_symbol,
        alt(((space0, "=", space0), (space1, "EQU", space1))),
        parse_expression,
        space0,
    )
        .parse_next(input)?;

    Ok(Command::Equals {
        left: c.1,
        right: c.3,
    })
}

fn parse_command_regs(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        Caseless("regs"),
        space1,
        parse_symbol,
        "=",
        parse_expression,
    )
        .parse_next(input)?;

    Ok(Command::Regs {
        register: c.3,
        expression: c.5,
    })
}

fn parse_attribute_bss(input: &mut &str) -> ModalResult<Attribute> {
    Caseless("bss").parse_next(input)?;
    Ok(Attribute::BSS)
}

fn parse_attribute_org(input: &mut &str) -> ModalResult<Attribute> {
    let c = (Caseless("org"), "(", parse_integer_constant, ")").parse_next(input)?;
    Ok(Attribute::Origin { address: c.2 })
}

fn parse_attribute_obj(input: &mut &str) -> ModalResult<Attribute> {
    let c = (Caseless("obj"), "(", opt(parse_integer_constant), ")").parse_next(input)?;
    Ok(Attribute::Obj { address: c.2 })
}

fn parse_attribute_over(input: &mut &str) -> ModalResult<Attribute> {
    let c = (Caseless("over"), "(", parse_symbol, ")").parse_next(input)?;
    Ok(Attribute::Over { group: c.2 })
}

fn parse_attribute_word(input: &mut &str) -> ModalResult<Attribute> {
    Caseless("word").parse_next(input)?;
    Ok(Attribute::Word)
}

fn parse_attribute_file(input: &mut &str) -> ModalResult<Attribute> {
    let c = (Caseless("file"), "(\"", parse_file_name, "\")").parse_next(input)?;
    Ok(Attribute::File {
        filename: c.2.to_string(),
    })
}

fn parse_attribute_size(input: &mut &str) -> ModalResult<Attribute> {
    let c = (Caseless("size"), "(", parse_integer_constant, ")").parse_next(input)?;
    Ok(Attribute::Size { maxsize: c.2 })
}

fn parse_attribute(input: &mut &str) -> ModalResult<Attribute> {
    alt((
        parse_attribute_bss,
        parse_attribute_org,
        parse_attribute_obj,
        parse_attribute_over,
        parse_attribute_word,
        parse_attribute_file,
        parse_attribute_size,
    ))
    .parse_next(input)
}

fn parse_attribute_list(input: &mut &str) -> ModalResult<Vec<Attribute>> {
    separated(0.., parse_attribute, (space0, ',', space0)).parse_next(input)
}

fn parse_optional_attribute_list(input: &mut &str) -> ModalResult<Vec<Attribute>> {
    let c = opt((space1, parse_attribute_list)).parse_next(input)?;
    Ok(c.map_or_else(Vec::new, |(_, attr_list)| attr_list))
}

fn parse_command_group(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        parse_symbol,
        space1,
        Caseless("group"),
        parse_optional_attribute_list,
    )
        .parse_next(input)?;

    Ok(Command::Group {
        name: c.1,
        attributes: c.4,
    })
}

fn parse_command_section_with_attributes(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        parse_symbol,
        space1,
        Caseless("section"),
        parse_optional_attribute_list,
    )
        .parse_next(input)?;

    Ok(Command::Section {
        name: c.1,
        group: None,
        attributes: c.4,
    })
}

fn parse_command_section_with_name(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        Caseless("section"),
        space1,
        parse_symbol,
        opt((",", parse_symbol)),
    )
        .parse_next(input)?;

    let group = c.4.map(|(_, group)| group);

    Ok(Command::Section {
        name: c.3,
        group,
        attributes: vec![],
    })
}

fn parse_command_section(input: &mut &str) -> ModalResult<Command> {
    alt((
        parse_command_section_with_attributes,
        parse_command_section_with_name,
    ))
    .parse_next(input)
}

fn parse_command_alias(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        parse_symbol,
        space1,
        Caseless("alias"),
        space1,
        parse_symbol,
    )
        .parse_next(input)?;

    Ok(Command::Alias {
        name: c.1,
        target: c.5,
    })
}

fn parse_command_unit(input: &mut &str) -> ModalResult<Command> {
    let c = (space0, Caseless("unit"), space1, parse_integer_constant).parse_next(input)?;

    Ok(Command::Unit { unitnum: c.3 })
}

fn parse_command_public(input: &mut &str) -> ModalResult<Command> {
    let c = (
        space0,
        Caseless("public"),
        space1,
        alt((Caseless("on"), Caseless("off")))
            .map(|s: &str| s.to_lowercase())
            .context(StrContext::Label("public"))
            .context(StrContext::Expected(StrContextValue::Description(
                "on or off",
            ))),
    )
        .parse_next(input)?;

    Ok(Command::Public {
        public: c.3 == "on",
    })
}

fn parse_command_generic_symbol_list(command: &str, input: &mut &str) -> ModalResult<Vec<String>> {
    let c = (space0, Caseless(command), space1, parse_symbol_list).parse_next(input)?;
    Ok(c.3)
}

fn parse_command_global(input: &mut &str) -> ModalResult<Command> {
    let symbols = parse_command_generic_symbol_list("global", input)?;
    Ok(Command::Global { symbols })
}

fn parse_command_xdef(input: &mut &str) -> ModalResult<Command> {
    let symbols = parse_command_generic_symbol_list("xdef", input)?;
    Ok(Command::XDef { symbols })
}

fn parse_command_xref(input: &mut &str) -> ModalResult<Command> {
    let symbols = parse_command_generic_symbol_list("xref", input)?;
    Ok(Command::XRef { symbols })
}

#[derive(Debug)]
pub struct Comment {
    pub comment: String,
}

fn parse_comment(input: &mut &str) -> ModalResult<Comment> {
    let c = (space0, ";", space0, take_while(0.., |c| c != '\n')).parse_next(input)?;
    Ok(Comment {
        comment: c.3.into(),
    })
}

pub fn parse_line(input: &mut &str) -> ModalResult<(Option<Command>, Option<Comment>)> {
    let command = opt(alt((
        parse_command_include,
        parse_command_inclib,
        parse_command_origin,
        parse_command_workspace,
        parse_command_equals,
        parse_command_regs,
        parse_command_group,
        parse_command_section,
        parse_command_alias,
        parse_command_unit,
        parse_command_global,
        parse_command_xdef,
        parse_command_xref,
        parse_command_public,
    )))
    .parse_next(input)?;

    let comment = opt(parse_comment).parse_next(input)?;

    Ok((command, comment))
}

#[cfg(test)]
mod test {
    use super::*;

    fn parse_command(input: &str) -> Command {
        let mut input = input;
        parse_line.parse_next(&mut input).unwrap().0.unwrap()
    }

    #[test]
    fn test_parse_integer_constant() {
        let mut input = "1234";
        let output = parse_integer_constant.parse_next(&mut input).unwrap();
        assert_eq!(1234, output);

        let mut input = "$1234";
        let output = parse_integer_constant.parse_next(&mut input).unwrap();
        assert_eq!(0x1234, output);

        let mut input = "%1010";
        let output = parse_integer_constant.parse_next(&mut input).unwrap();
        assert_eq!(10, output);
    }

    #[test]
    fn test_parse_command_include() {
        let output = parse_command("include \"foo.obj\"");

        match output {
            Command::Include { filename } => assert_eq!("foo.obj", filename),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_inclib() {
        let output = parse_command("inclib \"bar.lib\"");

        match output {
            Command::IncLib { filename } => assert_eq!("bar.lib", filename),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_org() {
        let output = parse_command("org 1234");
        match output {
            Command::Origin { address } => assert_eq!(1234, address),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("org $1234");
        match output {
            Command::Origin { address } => assert_eq!(0x1234, address),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("org %1010");
        match output {
            Command::Origin { address } => assert_eq!(10, address),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_workspace() {
        let output = parse_command("workspace 1234");
        match output {
            Command::Workspace { address } => assert_eq!(1234, address),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("workspace $1234");
        match output {
            Command::Workspace { address } => assert_eq!(0x1234, address),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("workspace %1010");
        match output {
            Command::Workspace { address } => assert_eq!(10, address),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_equals() {
        let output = parse_command("foo = bar");
        match output {
            Command::Equals { left, right } => {
                assert_eq!("foo", left);
                let Expression::Symbol(symbol) = right else {
                    panic!("unexpected value: {:?}", right);
                };
                assert_eq!("bar", symbol);
            }
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_regs() {
        let output = parse_command("regs pc=ENTRY_POINT");

        match output {
            Command::Regs {
                register,
                expression,
            } => {
                assert_eq!("pc", register);
                let Expression::Symbol(symbol) = expression else {
                    panic!("unexpected value: {:?}", expression);
                };
                assert_eq!("ENTRY_POINT", symbol);
            }
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn parse_command_group() {
        let output = parse_command("anim group");

        match output {
            Command::Group { name, attributes } => {
                assert_eq!("anim", name);
                assert!(attributes.is_empty());
            }
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("anim group bss");

        match output {
            Command::Group { name, attributes } => {
                assert_eq!("anim", name);
                assert_eq!(vec![Attribute::BSS], attributes);
            }
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_section() {
        let output = parse_command("anim section");

        match output {
            Command::Section {
                name,
                group: _,
                attributes,
            } => {
                assert_eq!("anim", name);
                assert!(attributes.is_empty());
            }
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("anim section bss");

        let Command::Section {
            name,
            group: _,
            attributes,
        } = output
        else {
            panic!("unexpected output: {:?}", output);
        };
        assert_eq!("anim", name);
        assert_eq!(vec![Attribute::BSS], attributes);

        let output = parse_command("section anim");
        let Command::Section {
            name,
            group,
            attributes,
        } = output
        else {
            panic!("unexpected output: {:?}", output);
        };
        assert_eq!("anim", name);
        assert!(group.is_none());
        assert!(attributes.is_empty());

        let output = parse_command("section anim,squares");
        let Command::Section {
            name,
            group,
            attributes,
        } = output
        else {
            panic!("unexpected output: {:?}", output);
        };
        assert_eq!("anim", name);
        let Some(group) = group else {
            panic!("unexpected output: {:?}", group);
        };
        assert_eq!("squares".to_string(), group);
        assert!(attributes.is_empty());
    }

    #[test]
    fn test_parse_command_alias() {
        let output = parse_command("foo alias bar");
        let Command::Alias { name, target } = output else {
            panic!("unexpected output: {:?}", output);
        };
        assert_eq!("foo".to_string(), name);
        assert_eq!("bar".to_string(), target);
    }

    #[test]
    fn test_parse_command_unit() {
        let output = parse_command("unit %1010");
        let Command::Unit { unitnum } = output else {
            panic!("unexpected output: {:?}", output);
        };
        assert_eq!(10, unitnum);
    }

    #[test]
    fn test_parse_command_global() {
        let output = parse_command("global foo");

        match output {
            Command::Global { symbols } => assert_eq!(vec!["foo".to_string()], symbols),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("global foo, bar , baz");

        match output {
            Command::Global { symbols } => assert_eq!(
                vec!["foo".to_string(), "bar".to_string(), "baz".to_string(),],
                symbols
            ),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_xdef() {
        let output = parse_command("xdef foo, bar, baz");

        match output {
            Command::XDef { symbols } => assert_eq!(
                vec!["foo".to_string(), "bar".to_string(), "baz".to_string(),],
                symbols
            ),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_xref() {
        let output = parse_command("xref foo, bar, baz");

        match output {
            Command::XRef { symbols } => assert_eq!(
                vec!["foo".to_string(), "bar".to_string(), "baz".to_string(),],
                symbols
            ),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_command_public() {
        let output = parse_command("public on");
        match output {
            Command::Public { public } => assert!(public),
            _ => panic!("unexpected output: {:?}", output),
        }

        let output = parse_command("PUBLIC OFF");
        match output {
            Command::Public { public } => assert!(!public),
            _ => panic!("unexpected output: {:?}", output),
        }
    }

    #[test]
    fn test_parse_comment() {
        // line with only a comment
        let mut input = "; hello, world!";
        let line = parse_line.parse_next(&mut input).unwrap();

        assert!(line.0.is_none());
        assert_eq!("hello, world!", line.1.unwrap().comment);

        // line with command & comment
        let mut input = "global foo; my global\nnot comment content";
        let line = parse_line.parse_next(&mut input).unwrap();

        match line.0 {
            Some(Command::Global { symbols }) => assert_eq!(vec!["foo".to_string()], symbols),
            _ => panic!("unexpected output: {:?}", line),
        }
        assert_eq!("my global", line.1.unwrap().comment);

        // line with command no comment
        let mut input = "global foo";
        let line = parse_line.parse_next(&mut input).unwrap();

        match line.0 {
            Some(Command::Global { symbols }) => assert_eq!(vec!["foo".to_string()], symbols),
            _ => panic!("unexpected output: {:?}", line),
        }
        assert!(line.1.is_none());

        // empty line
        let mut input = "   \t ";
        let line = parse_line.parse_next(&mut input).unwrap();
        assert!(line.0.is_none());
        assert!(line.1.is_none());
    }

    #[test]
    fn test_parse_attribute_list() {
        let mut input = "bss,word,file(\"foo\")";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(3, attributes.len());

        assert!(matches!(attributes.first(), Some(Attribute::BSS)));
        assert!(matches!(attributes.get(1), Some(Attribute::Word)));
        let Some(Attribute::File { filename }) = attributes.get(2) else {
            panic!("unexpected value: {:?}", attributes.get(2));
        };
        assert_eq!("foo", filename);

        let mut input = "";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert!(attributes.is_empty());

        let mut input = "bss";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        assert!(matches!(attributes.first(), Some(Attribute::BSS)));

        let mut input = "size(42)";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        assert!(matches!(
            attributes.first(),
            Some(Attribute::Size { maxsize: 42 })
        ));

        let mut input = "over(squares)";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        let Some(Attribute::Over { group }) = attributes.first() else {
            panic!("unexpected value: {:?}", attributes.first());
        };
        assert_eq!("squares", group);

        let mut input = "org($1234)";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        let Some(Attribute::Origin { address }) = attributes.first() else {
            panic!("unexpected value: {:?}", attributes.first());
        };
        assert_eq!(0x1234, *address);

        let mut input = "obj($4567)";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        let Some(Attribute::Obj { address }) = attributes.first() else {
            panic!("unexpected value: {:?}", attributes.first());
        };
        assert!(matches!(address, Some(0x4567)));

        let mut input = "obj()";
        let attributes = parse_attribute_list.parse_next(&mut input).unwrap();
        assert_eq!(1, attributes.len());
        let Some(Attribute::Obj { address }) = attributes.first() else {
            panic!("unexpected value: {:?}", attributes.first());
        };
        assert!(address.is_none());
    }

    fn parse_expr(input: &str) -> Expression {
        let mut input = input;
        parse_expression(&mut input).expect("parse failed")
    }

    #[test]
    fn test_constant() {
        assert_eq!(parse_expr("42"), Expression::Constant(42));
        assert_eq!(parse_expr("$ABCD"), Expression::Constant(0xABCD));
        assert_eq!(parse_expr("%1010"), Expression::Constant(0b1010));
    }

    #[test]
    fn test_symbol() {
        assert_eq!(parse_expr("foo"), Expression::Symbol("foo".into()));
        assert_eq!(parse_expr("_start"), Expression::Symbol("_start".into()));
        assert_eq!(parse_expr("var123"), Expression::Symbol("var123".into()));
    }

    #[test]
    fn test_simple_binary() {
        let expr = parse_expr("1 + 2");
        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Constant(1)),
                op: BinaryOp::Add,
                right: Box::new(Expression::Constant(2)),
            }
        );
    }

    #[test]
    fn test_precedence() {
        // 1 + 2 * 3 should parse as 1 + (2 * 3)
        let expr = parse_expr("1 + 2 * 3");
        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Constant(1)),
                op: BinaryOp::Add,
                right: Box::new(Expression::Binary {
                    left: Box::new(Expression::Constant(2)),
                    op: BinaryOp::Mul,
                    right: Box::new(Expression::Constant(3)),
                }),
            }
        );
    }

    #[test]
    fn test_left_associativity() {
        // 1 - 2 - 3 should parse as (1 - 2) - 3
        let expr = parse_expr("1 - 2 - 3");
        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Binary {
                    left: Box::new(Expression::Constant(1)),
                    op: BinaryOp::Sub,
                    right: Box::new(Expression::Constant(2)),
                }),
                op: BinaryOp::Sub,
                right: Box::new(Expression::Constant(3)),
            }
        );
    }

    #[test]
    fn test_parentheses() {
        // (1 + 2) * 3
        let expr = parse_expr("(1 + 2) * 3");
        assert_eq!(
            expr,
            Expression::Binary {
                left: Box::new(Expression::Parens(Box::new(Expression::Binary {
                    left: Box::new(Expression::Constant(1)),
                    op: BinaryOp::Add,
                    right: Box::new(Expression::Constant(2)),
                }))),
                op: BinaryOp::Mul,
                right: Box::new(Expression::Constant(3)),
            }
        );
    }

    #[test]
    fn test_unary() {
        assert_eq!(
            parse_expr("-42"),
            Expression::Unary {
                op: UnaryOp::Neg,
                operand: Box::new(Expression::Constant(42)),
            }
        );

        assert_eq!(
            parse_expr("~$FF"),
            Expression::Unary {
                op: UnaryOp::Not,
                operand: Box::new(Expression::Constant(0xFF)),
            }
        );
    }

    #[test]
    fn test_function_call() {
        let expr = parse_expr("sectstart(text)");
        assert_eq!(
            expr,
            Expression::Function {
                name: "sectstart".into(),
                arg: Box::new(Expression::Symbol("text".into())),
            }
        );
    }

    #[test]
    fn test_complex_expression() {
        // base + (offset & $FFFF) | $8000
        let expr = parse_expr("base + (offset & $FFFF) | $8000");

        // Should parse as: (base + (offset & 0xFFFF)) | 0x8000
        // Because: | has lower precedence than + and &
        match expr {
            Expression::Binary {
                left,
                op: BinaryOp::Or,
                right,
            } => {
                // Right should be $8000
                assert_eq!(*right, Expression::Constant(0x8000));

                // Left should be base + (offset & $FFFF)
                match *left {
                    Expression::Binary {
                        left: base,
                        op: BinaryOp::Add,
                        right: mask_expr,
                    } => {
                        assert_eq!(*base, Expression::Symbol("base".into()));

                        // mask_expr should be (offset & $FFFF)
                        match *mask_expr {
                            Expression::Parens(inner) => match *inner {
                                Expression::Binary {
                                    left,
                                    op: BinaryOp::And,
                                    right,
                                } => {
                                    assert_eq!(*left, Expression::Symbol("offset".into()));
                                    assert_eq!(*right, Expression::Constant(0xFFFF));
                                }
                                _ => panic!("unexpected inner expression"),
                            },
                            _ => panic!("expected parenthesized expression"),
                        }
                    }
                    _ => panic!("unexpected left side"),
                }
            }
            _ => panic!("expected binary OR expression"),
        }
    }

    #[test]
    fn test_bitwise_operators() {
        parse_expr("a & b");
        parse_expr("a | b");
        parse_expr("a ^ b");
        parse_expr("a << 4");
        parse_expr("a >> 2");
    }

    #[test]
    fn test_comparison_operators() {
        parse_expr("a == b");
        parse_expr("a != b");
        parse_expr("a < b");
        parse_expr("a <= b");
        parse_expr("a > b");
        parse_expr("a >= b");
    }

    #[test]
    fn test_logical_operators() {
        parse_expr("a && b");
        parse_expr("a || b");
        parse_expr("!a");
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(parse_expr("1+2"), parse_expr("1 + 2"));
        assert_eq!(parse_expr("  1  +  2  "), parse_expr("1+2"));
    }

    #[test]
    fn test_real_world_examples() {
        // From actual PSY-Q linker scripts
        parse_expr("BUFFER_END = BUFFER_START + $1000");
        parse_expr("(base & $FFFF0000) | $8000");
        parse_expr("sectstart(text) + $100");
        parse_expr("-(offset + 4)");
        parse_expr("~(flags | $FF)");
    }

    #[test]
    fn test_display() {
        let expr = Expression::Binary {
            left: Box::new(Expression::Symbol("a".into())),
            op: BinaryOp::Add,
            right: Box::new(Expression::Constant(0x100)),
        };
        assert_eq!(format!("{}", expr), "(a + $100)");
    }
}
