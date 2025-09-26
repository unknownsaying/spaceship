// src/layout.rs
use euclid::{Point2D, Size2D, Rect};
use crate::ast::*;
use crate::error::{MathError, Result};

pub struct LayoutEngine {
    font_metrics: FontMetrics,
    current_style: MathStyle,
    position: Point2D<f32>,
}

#[derive(Debug, Clone)]
pub struct LayoutBox {
    pub bounds: Rect<f32>,
    pub content: LayoutContent,
    pub style: MathStyle,
    pub children: Vec<LayoutBox>,
}

#[derive(Debug, Clone)]
pub enum LayoutContent {
    Glyph(char),
    Horizontal(Vec<LayoutBox>),
    Vertical(Vec<LayoutBox>),
    Fraction {
        numerator: Box<LayoutBox>,
        denominator: Box<LayoutBox>,
        rule_thickness: f32,
    },
    Radical {
        radicand: Box<LayoutBox>,
        degree: Option<Box<LayoutBox>>,
    },
    Script {
        base: Box<LayoutBox>,
        superscript: Option<Box<LayoutBox>>,
        subscript: Option<Box<LayoutBox>>,
    },
}

#[derive(Debug, Clone)]
pub struct FontMetrics {
    pub x_height: f32,
    pub cap_height: f32,
    pub ascender: f32,
    pub descender: f32,
    pub italic_correction: f32,
    pub script_scale: f32,
    pub script_script_scale: f32,
}

impl LayoutEngine {
    pub fn new(font_metrics: FontMetrics) -> Self {
        Self {
            font_metrics,
            current_style: MathStyle::default(),
            position: Point2D::new(0.0, 0.0),
        }
    }
    
    pub fn layout_expression(&mut self, expr: &MathExpr) -> Result<LayoutBox> {
        self.current_style = expr.style.clone();
        self.layout_node(&expr.root)
    }
    
    fn layout_node(&mut self, node: &MathNode) -> Result<LayoutBox> {
        match node {
            MathNode::Identifier(name) => self.layout_identifier(name),
            MathNode::Number(value) => self.layout_number(*value),
            MathNode::BinaryOp { op, left, right } => self.layout_binary_op(op, left, right),
            MathNode::Fraction { numerator, denominator } => {
                self.layout_fraction(numerator, denominator)
            }
            MathNode::Superscript { base, superscript } => {
                self.layout_script(base, Some(superscript), None)
            }
            MathNode::Subscript { base, subscript } => {
                self.layout_script(base, None, Some(subscript))
            }
            MathNode::Subsuperscript { base, subscript, superscript } => {
                self.layout_script(base, Some(superscript), Some(subscript))
            }
            MathNode::Root { degree, radicand } => self.layout_root(degree.as_ref(), radicand),
            MathNode::Group(content) => self.layout_group(content),
            _ => Err(MathError::Layout {
                message: "Unsupported node type".to_string(),
            }),
        }
    }
    
    fn layout_identifier(&mut self, name: &str) -> Result<LayoutBox> {
        let mut children = Vec::new();
        let mut width = 0.0;
        let mut height = 0.0;
        let mut depth = 0.0;
        
        for ch in name.chars() {
            let glyph_box = self.layout_glyph(ch)?;
            width += glyph_box.bounds.size.width;
            height = height.max(glyph_box.bounds.size.height);
            depth = depth.max(glyph_box.bounds.size.height - self.font_metrics.x_height);
            children.push(glyph_box);
        }
        
        // Position glyphs horizontally
        let mut x = 0.0;
        for child in &mut children {
            child.bounds.origin.x = x;
            x += child.bounds.size.width;
        }
        
        Ok(LayoutBox {
            bounds: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(width, height)),
            content: LayoutContent::Horizontal(children),
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_glyph(&mut self, ch: char) -> Result<LayoutBox> {
        let font_size = self.current_style.font_size;
        let scale = font_size / self.font_metrics.x_height;
        
        // Simplified glyph metrics - in real implementation, use font tables
        let (width, height) = match ch {
            'a'..='z' => (0.5, 0.7),
            'A'..='Z' => (0.6, 0.8),
            _ => (0.5, 0.7),
        };
        
        let scaled_width = width * scale;
        let scaled_height = height * scale;
        
        Ok(LayoutBox {
            bounds: Rect::new(
                Point2D::new(0.0, 0.0),
                Size2D::new(scaled_width, scaled_height)
            ),
            content: LayoutContent::Glyph(ch),
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_number(&mut self, value: f64) -> Result<LayoutBox> {
        let text = value.to_string();
        self.layout_identifier(&text)
    }
    
    fn layout_binary_op(&mut self, op: &BinaryOperator, left: &MathNode, right: &MathNode) -> Result<LayoutBox> {
        let left_box = self.layout_node(left)?;
        let op_box = self.layout_operator(op)?;
        let right_box = self.layout_node(right)?;
        
        let total_width = left_box.bounds.size.width + op_box.bounds.size.width + right_box.bounds.size.width;
        let max_height = left_box.bounds.size.height
            .max(op_box.bounds.size.height)
            .max(right_box.bounds.size.height);
        
        let mut children = vec![left_box, op_box, right_box];
        
        // Position elements
        let mut x = 0.0;
        for child in &mut children {
            let y = (max_height - child.bounds.size.height) / 2.0;
            child.bounds.origin = Point2D::new(x, y);
            x += child.bounds.size.width;
        }
        
        Ok(LayoutBox {
            bounds: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(total_width, max_height)),
            content: LayoutContent::Horizontal(children),
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_operator(&mut self, op: &BinaryOperator) -> Result<LayoutBox> {
        let symbol = match op {
            BinaryOperator::Plus => '+',
            BinaryOperator::Minus => '−',
            BinaryOperator::Multiply => '×',
            BinaryOperator::Divide => '÷',
            BinaryOperator::Equal => '=',
            BinaryOperator::NotEqual => '≠',
            BinaryOperator::Less => '<',
            BinaryOperator::Greater => '>',
            BinaryOperator::LessEqual => '≤',
            BinaryOperator::GreaterEqual => '≥',
            _ => '?',
        };
        
        self.layout_glyph(symbol)
    }
    
    fn layout_fraction(&mut self, numerator: &MathNode, denominator: &MathNode) -> Result<LayoutBox> {
        let mut num_style = self.current_style.clone();
        num_style.script_level = match self.current_style.script_level {
            ScriptLevel::Display => ScriptLevel::Text,
            ScriptLevel::Text => ScriptLevel::Script,
            ScriptLevel::Script => ScriptLevel::ScriptScript,
            ScriptLevel::ScriptScript => ScriptLevel::ScriptScript,
        };
        num_style.font_size *= self.font_metrics.script_scale;
        
        let den_style = num_style.clone();
        
        // Save current style
        let saved_style = self.current_style.clone();
        
        // Layout numerator
        self.current_style = num_style;
        let numerator_box = self.layout_node(numerator)?;
        
        // Layout denominator
        self.current_style = den_style;
        let denominator_box = self.layout_node(denominator)?;
        
        // Restore style
        self.current_style = saved_style;
        
        let rule_thickness = 0.05 * self.current_style.font_size;
        let num_den_sep = 0.1 * self.current_style.font_size;
        
        let width = numerator_box.bounds.size.width.max(denominator_box.bounds.size.width);
        let height = numerator_box.bounds.size.height + rule_thickness + num_den_sep + denominator_box.bounds.size.height;
        
        Ok(LayoutBox {
            bounds: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(width, height)),
            content: LayoutContent::Fraction {
                numerator: Box::new(numerator_box),
                denominator: Box::new(denominator_box),
                rule_thickness,
            },
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_script(
        &mut self,
        base: &MathNode,
        superscript: Option<&MathNode>,
        subscript: Option<&MathNode>,
    ) -> Result<LayoutBox> {
        let base_box = self.layout_node(base)?;
        
        let mut script_style = self.current_style.clone();
        script_style.script_level = match self.current_style.script_level {
            ScriptLevel::Display => ScriptLevel::Script,
            ScriptLevel::Text => ScriptLevel::Script,
            ScriptLevel::Script => ScriptLevel::ScriptScript,
            ScriptLevel::ScriptScript => ScriptLevel::ScriptScript,
        };
        script_style.font_size *= self.font_metrics.script_scale;
        
        let saved_style = self.current_style.clone();
        self.current_style = script_style.clone();
        
        let sup_box = superscript.map(|node| self.layout_node(node)).transpose()?;
        let sub_box = subscript.map(|node| self.layout_node(node)).transpose()?;
        
        self.current_style = saved_style;
        
        // Calculate positioning
        let script_offset = 0.5 * self.font_metrics.x_height;
        let superscript_drop = 0.6 * base_box.bounds.size.height;
        let subscript_drop = 0.2 * base_box.bounds.size.height;
        
        let mut width = base_box.bounds.size.width;
        if let Some(sup_box) = &sup_box {
            width = width.max(base_box.bounds.size.width + sup_box.bounds.size.width - script_offset);
        }
        if let Some(sub_box) = &sub_box {
            width = width.max(base_box.bounds.size.width + sub_box.bounds.size.width - script_offset);
        }
        
        let height = base_box.bounds.size.height;
        let depth = base_box.bounds.size.height; // Simplified
        
        Ok(LayoutBox {
            bounds: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(width, height + depth)),
            content: LayoutContent::Script {
                base: Box::new(base_box),
                superscript: sup_box.map(Box::new),
                subscript: sub_box.map(Box::new),
            },
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_root(&mut self, degree: Option<&MathNode>, radicand: &MathNode) -> Result<LayoutBox> {
        let radicand_box = self.layout_node(radicand)?;
        
        let degree_box = if let Some(deg) = degree {
            let mut deg_style = self.current_style.clone();
            deg_style.font_size *= self.font_metrics.script_script_scale;
            
            let saved_style = self.current_style.clone();
            self.current_style = deg_style;
            let box_result = self.layout_node(deg);
            self.current_style = saved_style;
            
            Some(Box::new(box_result?))
        } else {
            None
        };
        
        let radical_symbol_width = 0.3 * self.current_style.font_size;
        let horizontal_padding = 0.1 * self.current_style.font_size;
        let vertical_padding = 0.1 * self.current_style.font_size;
        
        let width = radical_symbol_width + horizontal_padding + radicand_box.bounds.size.width;
        let height = radicand_box.bounds.size.height + vertical_padding;
        
        Ok(LayoutBox {
            bounds: Rect::new(Point2D::new(0.0, 0.0), Size2D::new(width, height)),
            content: LayoutContent::Radical {
                radicand: Box::new(radicand_box),
                degree: degree_box,
            },
            style: self.current_style.clone(),
            children: Vec::new(),
        })
    }
    
    fn layout_group(&mut self, content: &MathNode) -> Result<LayoutBox> {
        self.layout_node(content)
    }
}