// src/symbols.rs
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::ast::{MathSymbol, SymbolCategory};

pub static SYMBOL_TABLE: Lazy<HashMap<&'static str, MathSymbol>> = Lazy::new(|| {
    let mut table = HashMap::new();
    
    // Greek letters
    table.insert("alpha", MathSymbol {
        unicode: 'α',
        name: "alpha".to_string(),
        category: SymbolCategory::Letter,
    });
    
    table.insert("beta", MathSymbol {
        unicode: 'β',
        name: "beta".to_string(),
        category: SymbolCategory::Letter,
    });
    
    // Operators
    table.insert("sum", MathSymbol {
        unicode: '∑',
        name: "sum".to_string(),
        category: SymbolCategory::Operator,
    });
    
    table.insert("int", MathSymbol {
        unicode: '∫',
        name: "integral".to_string(),
        category: SymbolCategory::Operator,
    });
    
    // Relations
    table.insert("leq", MathSymbol {
        unicode: '≤',
        name: "less-than-or-equal".to_string(),
        category: SymbolCategory::Relation,
    });
    
    // Add more symbols as needed...
    
    table
});

pub fn get_symbol(name: &str) -> Option<&MathSymbol> {
    SYMBOL_TABLE.get(name)
}

pub fn resolve_latex_command(cmd: &str) -> Option<char> {
    SYMBOL_TABLE.get(cmd).map(|s| s.unicode)
}