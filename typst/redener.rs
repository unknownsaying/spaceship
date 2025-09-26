// src/render/svg.rs
use crate::layout::{LayoutBox, LayoutContent};
use crate::error::Result;
use std::fmt::Write;

pub struct SvgRenderer {
    output: String,
    indent_level: usize,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent_level: 0,
        }
    }
    
    pub fn render(&mut self, layout: &LayoutBox) -> Result<String> {
        self.output.clear();
        
        writeln!(self.output, r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {} {}">"#,
            layout.bounds.size.width, layout.bounds.size.height).unwrap();
        
        self.indent_level += 1;
        self.render_box(layout)?;
        self.indent_level -= 1;
        
        writeln!(self.output, "</svg>").unwrap();
        
        Ok(self.output.clone())
    }
    
    fn render_box(&mut self, box_: &LayoutBox) -> Result<()> {
        let indent = "  ".repeat(self.indent_level);
        
        match &box_.content {
            LayoutContent::Glyph(ch) => {
                let x = box_.bounds.origin.x;
                let y = box_.bounds.origin.y + box_.bounds.size.height * 0.8; // Baseline adjustment
                
                writeln!(self.output, r#"{}<text x="{}" y="{}" font-size="{}" fill="rgb({},{},{})">{}</text>"#,
                    indent, x, y, box_.style.font_size,
                    box_.style.color.r, box_.style.color.g, box_.style.color.b,
                    ch).unwrap();
            }
            LayoutContent::Horizontal(children) => {
                writeln!(self.output, "{}<g>", indent).unwrap();
                self.indent_level += 1;
                
                for child in children {
                    self.render_box(child)?;
                }
                
                self.indent_level -= 1;
                writeln!(self.output, "{}</g>", indent).unwrap();
            }
            LayoutContent::Fraction { numerator, denominator, rule_thickness } => {
                let rule_y = numerator.bounds.size.height + *rule_thickness / 2.0;
                let rule_width = numerator.bounds.size.width.max(denominator.bounds.size.width);
                
                // Render fraction line
                writeln!(self.output, r#"{}<line x1="0" y1="{}" x2="{}" y2="{}" stroke="black" stroke-width="{}"/>"#,
                    indent, rule_y, rule_width, rule_y, rule_thickness).unwrap();
                
                // Render numerator
                writeln!(self.output, "{}<g transform=\"translate(0, 0)\">", indent).unwrap();
                self.indent_level += 1;
                self.render_box(numerator)?;
                self.indent_level -= 1;
                writeln!(self.output, "{}</g>", indent).unwrap();
                
                // Render denominator
                let den_y = numerator.bounds.size.height + *rule_thickness;
                writeln!(self.output, r#"{}<g transform="translate(0, {})">"#, indent, den_y).unwrap();
                self.indent_level += 1;
                self.render_box(denominator)?;
                self.indent_level -= 1;
                writeln!(self.output, "{}</g>", indent).unwrap();
            }
            LayoutContent::Radical { radicand, degree } => {
                // Render radical symbol (simplified as a line)
                let radical_height = radicand.bounds.size.height;
                writeln!(self.output, r#"{}<path d="M 0,{} L {},{} L {},0" stroke="black" fill="none"/>"#,
                    indent, radical_height, radical_height * 0.3, radical_height * 0.7,
                    radical_height * 0.3).unwrap();
                
                // Render degree if present
                if let Some(deg) = degree {
                    writeln!(self.output, r#"{}<g transform="translate({}, {})">"#,
                        indent, radical_height * 0.1, radical_height * 0.1).unwrap();
                    self.indent_level += 1;
                    self.render_box(deg)?;
                    self.indent_level -= 1;
                    writeln!(self.output, "{}</g>", indent).unwrap();
                }
                
                // Render radicand
                writeln!(self.output, r#"{}<g transform="translate({}, 0)">"#,
                    indent, radical_height * 0.3).unwrap();
                self.indent_level += 1;
                self.render_box(radicand)?;
                self.indent_level -= 1;
                writeln!(self.output, "{}</g>", indent).unwrap();
            }
            _ => {
                // Handle other content types
                for child in &box_.children {
                    self.render_box(child)?;
                }
            }
        }
        
        Ok(())
    }
}
