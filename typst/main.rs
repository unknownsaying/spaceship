// src/main.rs
use typst_math::*;
use std::fs;

fn main() -> Result<()> {
    // Example 1: Simple equation
    let input = "x = \\frac{-b \\pm \\sqrt{b^2 - 4ac}}{2a}";
    
    let mut parser = parser::Parser::new(input)?;
    let expr = parser.parse_math_expression()?;
    
    let font_metrics = layout::FontMetrics {
        x_height: 10.0,
        cap_height: 12.0,
        ascender: 14.0,
        descender: 4.0,
        italic_correction: 0.1,
        script_scale: 0.7,
        script_script_scale: 0.5,
    };
    
    let mut layout_engine = layout::LayoutEngine::new(font_metrics);
    let layout_box = layout_engine.layout_expression(&expr)?;
    
    let mut svg_renderer = render::svg::SvgRenderer::new();
    let svg_output = svg_renderer.render(&layout_box)?;
    
    fs::write("equation.svg", svg_output)?;
    println!("Equation rendered to equation.svg");
    
    // Example 2: Complex mathematical expression
    let complex_input = "\\sum_{n=1}^\\infty \\frac{1}{n^2} = \\frac{\\pi^2}{6}";
    let mut parser = parser::Parser::new(complex_input)?;
    let expr = parser.parse_math_expression()?;
    
    let layout_box = layout_engine.layout_expression(&expr)?;
    let svg_output = svg_renderer.render(&layout_box)?;
    
    fs::write("complex_equation.svg", svg_output)?;
    println!("Complex equation rendered to complex_equation.svg");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_equation() -> Result<()> {
        let input = "a + b = c";
        let mut parser = parser::Parser::new(input)?;
        let expr = parser.parse_math_expression()?;
        
        let font_metrics = layout::FontMetrics {
            x_height: 10.0,
            cap_height: 12.0,
            ascender: 14.0,
            descender: 4.0,
            italic_correction: 0.1,
            script_scale: 0.7,
            script_script_scale: 0.5,
        };
        
        let mut layout_engine = layout::LayoutEngine::new(font_metrics);
        let layout_box = layout_engine.layout_expression(&expr)?;
        
        assert!(layout_box.bounds.size.width > 0.0);
        assert!(layout_box.bounds.size.height > 0.0);
        
        Ok(())
    }
    
    #[test]
    fn test_fraction() -> Result<()> {
        let input = "\\frac{1}{2}";
        let mut parser = parser::Parser::new(input)?;
        let expr = parser.parse_math_expression()?;
        
        let font_metrics = layout::FontMetrics {
            x_height: 10.0,
            cap_height: 12.0,
            ascender: 14.0,
            descender: 4.0,
            italic_correction: 0.1,
            script_scale: 0.7,
            script_script_scale: 0.5,
        };
        
        let mut layout_engine = layout::LayoutEngine::new(font_metrics);
        let layout_box = layout_engine.layout_expression(&expr)?;
        
        // Fraction should be taller than a single character
        assert!(layout_box.bounds.size.height > 15.0);
        
        Ok(())
    }
}