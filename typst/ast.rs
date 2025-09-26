// src/ast.rs
use serde::{Deserialize, Serialize};
use smallvec::{SmallVec, smallvec};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathDocument {
    pub elements: Vec<MathElement>,
    pub metadata: DocumentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub font_size: f32,
    pub line_height: f32,
    pub page_width: f32,
    pub page_height: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MathElement {
    Equation(Equation),
    DisplayMath(DisplayMath),
    InlineMath(InlineMath),
    Text(TextElement),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equation {
    pub content: MathExpr,
    pub label: Option<String>,
    pub number: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayMath {
    pub content: MathExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineMath {
    pub content: MathExpr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextElement {
    pub content: String,
    pub style: TextStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathExpr {
    pub root: MathNode,
    pub style: MathStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MathNode {
    // Basic elements
    Identifier(String),
    Number(f64),
    Symbol(MathSymbol),
    
    // Operations
    BinaryOp {
        op: BinaryOperator,
        left: Box<MathNode>,
        right: Box<MathNode>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<MathNode>,
    },
    
    // Grouping
    Group(Box<MathNode>),
    Bracket {
        left: BracketType,
        content: Box<MathNode>,
        right: BracketType,
    },
    
    // Fractions and roots
    Fraction {
        numerator: Box<MathNode>,
        denominator: Box<MathNode>,
    },
    Root {
        degree: Option<Box<MathNode>>,
        radicand: Box<MathNode>,
    },
    
    // Scripts
    Superscript {
        base: Box<MathNode>,
        superscript: Box<MathNode>,
    },
    Subscript {
        base: Box<MathNode>,
        subscript: Box<MathNode>,
    },
    Subsuperscript {
        base: Box<MathNode>,
        subscript: Box<MathNode>,
        superscript: Box<MathNode>,
    },
    
    // Large operators
    Sum {
        lower_limit: Option<Box<MathNode>>,
        upper_limit: Option<Box<MathNode>>,
        body: Box<MathNode>,
    },
    Integral {
        lower_limit: Option<Box<MathNode>>,
        upper_limit: Option<Box<MathNode>>,
        body: Box<MathNode>,
        variant: IntegralVariant,
    },
    
    // Matrices
    Matrix {
        rows: Vec<Vec<MathNode>>,
        bracket_type: MatrixBracket,
    },
    
    // Functions
    Function {
        name: String,
        argument: Box<MathNode>,
    },
    
    // Accents
    Accent {
        base: Box<MathNode>,
        accent: AccentType,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Dot,
    Cross,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Factorial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathSymbol {
    pub unicode: char,
    pub name: String,
    pub category: SymbolCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymbolCategory {
    Letter,
    Operator,
    Relation,
    Punctuation,
    Arrow,
    Miscellaneous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BracketType {
    Parenthesis,    // ( )
    Square,         // [ ]
    Curly,          // { }
    Angle,          // ⟨ ⟩
    Vertical,       | |
    DoubleVertical, // ‖ ‖
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntegralVariant {
    Single,
    Double,
    Triple,
    Contour,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MatrixBracket {
    None,
    Parenthesis,
    Square,
    Curly,
    Vertical,
    DoubleVertical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccentType {
    Hat,        // ^
    Tilde,      // ~
    Dot,        // .
    DoubleDot,  // ..
    Bar,        // -
    Vector,     // vec
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathStyle {
    pub font_size: f32,
    pub font_family: MathFont,
    pub color: Color,
    pub bold: bool,
    pub italic: bool,
    pub script_level: ScriptLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MathFont {
    Roman,
    Bold,
    Italic,
    BoldItalic,
    Script,
    Fraktur,
    DoubleStruck,
    SansSerif,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptLevel {
    Display,
    Text,
    Script,
    ScriptScript,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextStyle {
    pub font_family: String,
    pub font_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub color: Color,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Default for MathStyle {
    fn default() -> Self {
        Self {
            font_size: 12.0,
            font_family: MathFont::Roman,
            color: Color::default(),
            bold: false,
            italic: true, // Math is italic by default
            script_level: ScriptLevel::Text,
        }
    }
}