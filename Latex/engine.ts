// math-typesetting-engine.ts

// === CORE TYPES AND INTERFACES ===
interface MathSymbol {
    unicode: string;
    latex: string;
    mathml: string;
    category: 'operator' | 'relation' | 'punctuation' | 'arrow' | 'misc';
}

interface MathEnvironment {
    name: string;
    begin: string;
    end: string;
    render: (content: string) => string;
}

interface DocumentClass {
    name: string;
    fontSize: number;
    pageLayout: PageLayout;
    sections: SectionStyle[];
}

interface PageLayout {
    margins: { top: number; right: number; bottom: number; left: number };
    paperSize: 'a4' | 'letter' | 'legal' | 'a3';
}

interface SectionStyle {
    level: number;
    fontsize: number;
    spacing: { before: number; after: number };
}

// === MATHEMATICAL SYMBOLS DATABASE ===
class MathSymbols {
    private symbols: Map<string, MathSymbol> = new Map();
    
    constructor() {
        this.initializeSymbols();
    }
    
    private initializeSymbols(): void {
        // Greek letters
        this.addSymbol('alpha', 'α', '\\alpha', 'α');
        this.addSymbol('beta', 'β', '\\beta', 'β');
        this.addSymbol('gamma', 'γ', '\\gamma', 'γ');
        this.addSymbol('Gamma', 'Γ', '\\Gamma', 'Γ');
        
        // Operators
        this.addSymbol('sum', '∑', '\\sum', '∑', 'operator');
        this.addSymbol('prod', '∏', '\\prod', '∏', 'operator');
        this.addSymbol('int', '∫', '\\int', '∫', 'operator');
        
        // Relations
        this.addSymbol('leq', '≤', '\\leq', '≤', 'relation');
        this.addSymbol('geq', '≥', '\\geq', '≥', 'relation');
        this.addSymbol('neq', '≠', '\\neq', '≠', 'relation');
        
        // Arrows
        this.addSymbol('rightarrow', '→', '\\rightarrow', '→', 'arrow');
        this.addSymbol('leftarrow', '←', '\\leftarrow', '←', 'arrow');
    }
    
    private addSymbol(name: string, unicode: string, latex: string, mathml: string, 
                     category: MathSymbol['category'] = 'misc'): void {
        this.symbols.set(name, { unicode, latex, mathml, category });
    }
    
    getSymbol(name: string): MathSymbol | undefined {
        return this.symbols.get(name);
    }
    
    resolveLatexCommand(cmd: string): string | undefined {
        return this.symbols.get(cmd)?.unicode;
    }
}

// === LEXER AND PARSER ===
type TokenType = 'COMMAND' | 'BRACE_OPEN' | 'BRACE_CLOSE' | 'BRACKET_OPEN' | 
                 'BRACKET_CLOSE' | 'SUPERSCRIPT' | 'SUBSCRIPT' | 'TEXT' | 'ENVIRONMENT';

interface Token {
    type: TokenType;
    value: string;
    position: number;
}

class LatexLexer {
    private input: string;
    private position: number = 0;
    
    constructor(input: string) {
        this.input = input;
    }
    
    tokenize(): Token[] {
        const tokens: Token[] = [];
        
        while (this.position < this.input.length) {
            const char = this.input[this.position];
            
            if (char === '\\') {
                tokens.push(this.parseCommand());
            } else if (char === '{') {
                tokens.push({ type: 'BRACE_OPEN', value: char, position: this.position++ });
            } else if (char === '}') {
                tokens.push({ type: 'BRACE_CLOSE', value: char, position: this.position++ });
            } else if (char === '[') {
                tokens.push({ type: 'BRACKET_OPEN', value: char, position: this.position++ });
            } else if (char === ']') {
                tokens.push({ type: 'BRACKET_CLOSE', value: char, position: this.position++ });
            } else if (char === '^') {
                tokens.push({ type: 'SUPERSCRIPT', value: char, position: this.position++ });
            } else if (char === '_') {
                tokens.push({ type: 'SUBSCRIPT', value: char, position: this.position++ });
            } else if (/\s/.test(char)) {
                this.position++; // Skip whitespace
            } else {
                tokens.push(this.parseText());
            }
        }
        
        return tokens;
    }
    
    private parseCommand(): Token {
        this.position++; // Skip backslash
        let command = '';
        
        while (this.position < this.input.length && /[a-zA-Z]/.test(this.input[this.position])) {
            command += this.input[this.position++];
        }
        
        return { type: 'COMMAND', value: command, position: this.position - command.length - 1 };
    }
    
    private parseText(): Token {
        let text = '';
        const startPos = this.position;
        
        while (this.position < this.input.length) {
            const char = this.input[this.position];
            if ('\\{}[]^_'.includes(char)) break;
            text += char;
            this.position++;
        }
        
        return { type: 'TEXT', value: text, position: startPos };
    }
}

// === AST NODES ===
abstract class MathNode {
    abstract render(): string;
    abstract toMathML(): string;
}

class CommandNode extends MathNode {
    constructor(
        public command: string,
        public args: MathNode[] = []
    ) {
        super();
    }
    
    render(): string {
        const symbols = new MathSymbols();
        const symbol = symbols.getSymbol(this.command);
        
        if (symbol) {
            return symbol.unicode;
        }
        
        // Handle commands with arguments
        switch (this.command) {
            case 'frac':
                if (this.args.length === 2) {
                    return `${this.args[0].render()}/${this.args[1].render()}`;
                }
                break;
            case 'sqrt':
                if (this.args.length === 1) {
                    return `√${this.args[0].render()}`;
                }
                break;
        }
        
        return `\\${this.command}`;
    }
    
    toMathML(): string {
        switch (this.command) {
            case 'frac':
                return `<mfrac>${this.args.map(arg => arg.toMathML()).join('')}</mfrac>`;
            case 'sqrt':
                return `<msqrt>${this.args.map(arg => arg.toMathML()).join('')}</msqrt>`;
            default:
                const symbols = new MathSymbols();
                const symbol = symbols.getSymbol(this.command);
                return symbol ? `<mi>${symbol.mathml}</mi>` : `<mi>\\${this.command}</mi>`;
        }
    }
}

class TextNode extends MathNode {
    constructor(public content: string) {
        super();
    }
    
    render(): string {
        return this.content;
    }
    
    toMathML(): string {
        return `<mn>${this.content}</mn>`;
    }
}

class SuperscriptNode extends MathNode {
    constructor(public base: MathNode, public exponent: MathNode) {
        super();
    }
    
    render(): string {
        return `${this.base.render()}^${this.exponent.render()}`;
    }
    
    toMathML(): string {
        return `<msup>${this.base.toMathML()}${this.exponent.toMathML()}</msup>`;
    }
}

class SubscriptNode extends MathNode {
    constructor(public base: MathNode, public subscript: MathNode) {
        super();
    }
    
    render(): string {
        return `${this.base.render()}_${this.subscript.render()}`;
    }
    
    toMathML(): string {
        return `<msub>${this.base.toMathML()}${this.subscript.toMathML()}</msub>`;
    }
}

// === PARSER ===
class LatexParser {
    private tokens: Token[];
    private position: number = 0;
    
    constructor(tokens: Token[]) {
        this.tokens = tokens;
    }
    
    parse(): MathNode {
        return this.parseExpression();
    }
    
    private parseExpression(): MathNode {
        let left = this.parseAtom();
        
        while (this.position < this.tokens.length) {
            const token = this.tokens[this.position];
            
            if (token.type === 'SUPERSCRIPT') {
                this.position++;
                const exponent = this.parseAtom();
                left = new SuperscriptNode(left, exponent);
            } else if (token.type === 'SUBSCRIPT') {
                this.position++;
                const subscript = this.parseAtom();
                left = new SubscriptNode(left, subscript);
            } else {
                break;
            }
        }
        
        return left;
    }
    
    private parseAtom(): MathNode {
        const token = this.tokens[this.position++];
        
        switch (token.type) {
            case 'COMMAND':
                return this.parseCommand(token);
            case 'TEXT':
                return new TextNode(token.value);
            case 'BRACE_OPEN':
                const expr = this.parseExpression();
                this.expect('BRACE_CLOSE');
                return expr;
            default:
                throw new Error(`Unexpected token: ${token.type} at position ${token.position}`);
        }
    }
    
    private parseCommand(token: Token): CommandNode {
        const args: MathNode[] = [];
        
        // Look for arguments
        while (this.position < this.tokens.length) {
            const nextToken = this.tokens[this.position];
            if (nextToken.type === 'BRACE_OPEN') {
                this.position++;
                args.push(this.parseExpression());
                this.expect('BRACE_CLOSE');
            } else if (nextToken.type === 'BRACKET_OPEN') {
                this.position++;
                args.push(this.parseExpression());
                this.expect('BRACKET_CLOSE');
            } else {
                break;
            }
        }
        
        return new CommandNode(token.value, args);
    }
    
    private expect(expectedType: TokenType): void {
        if (this.position >= this.tokens.length) {
            throw new Error(`Expected ${expectedType} but reached end of input`);
        }
        
        const token = this.tokens[this.position++];
        if (token.type !== expectedType) {
            throw new Error(`Expected ${expectedType} but got ${token.type} at position ${token.position}`);
        }
    }
}

// === DOCUMENT ENGINE ===
class DocumentEngine {
    private documentClass: DocumentClass;
    private packages: Set<string> = new Set();
    private content: string[] = [];
    
    constructor(documentClass: string = 'article') {
        this.documentClass = this.loadDocumentClass(documentClass);
    }
    
    private loadDocumentClass(name: string): DocumentClass {
        const classes: { [key: string]: DocumentClass } = {
            article: {
                name: 'article',
                fontSize: 10,
                pageLayout: {
                    margins: { top: 72, right: 72, bottom: 72, left: 72 },
                    paperSize: 'letter'
                },
                sections: [
                    { level: 1, fontsize: 18, spacing: { before: 24, after: 12 } },
                    { level: 2, fontsize: 14, spacing: { before: 18, after: 9 } },
                    { level: 3, fontsize: 12, spacing: { before: 12, after: 6 } }
                ]
            }
        };
        
        return classes[name] || classes.article;
    }
    
    usePackage(pkg: string): void {
        this.packages.add(pkg);
    }
    
    beginDocument(): void {
        this.content = [];
        this.content.push('<!DOCTYPE html>');
        this.content.push('<html lang="en">');
        this.content.push('<head>');
        this.content.push('<meta charset="UTF-8">');
        this.content.push('<title>LaTeX-like Document</title>');
        this.content.push('<style>');
        this.content.push(this.generateCSS());
        this.content.push('</style>');
        this.content.push('</head>');
        this.content.push('<body>');
        this.content.push('<div class="document">');
    }
    
    endDocument(): void {
        this.content.push('</div>');
        this.content.push('</body>');
        this.content.push('</html>');
    }
    
    section(title: string, level: number = 1): void {
        const tag = `h${Math.min(level, 6)}`;
        this.content.push(`<${tag} class="section level-${level}">${title}</${tag}>`);
    }
    
    math(expression: string, displayMode: boolean = false): void {
        const lexer = new LatexLexer(expression);
        const tokens = lexer.tokenize();
        const parser = new LatexParser(tokens);
        const ast = parser.parse();
        
        const mathml = ast.toMathML();
        const className = displayMode ? 'display-math' : 'inline-math';
        
        this.content.push(`<span class="${className}">${mathml}</span>`);
    }
    
    paragraph(text: string): void {
        this.content.push(`<p>${text}</p>`);
    }
    
    beginEnvironment(name: string): void {
        this.content.push(`<div class="environment ${name}">`);
    }
    
    endEnvironment(name: string): void {
        this.content.push('</div>');
    }
    
    private generateCSS(): string {
        return `
            .document {
                font-family: "Times New Roman", serif;
                font-size: ${this.documentClass.fontSize}pt;
                line-height: 1.2;
                margin: ${this.documentClass.pageLayout.margins.top}px 
                        ${this.documentClass.pageLayout.margins.right}px 
                        ${this.documentClass.pageLayout.margins.bottom}px 
                        ${this.documentClass.pageLayout.margins.left}px;
            }
            
            .section.level-1 { font-size: 18pt; margin: 24px 0 12px 0; }
            .section.level-2 { font-size: 14pt; margin: 18px 0 9px 0; }
            .section.level-3 { font-size: 12pt; margin: 12px 0 6px 0; }
            
            .display-math { 
                display: block; 
                text-align: center; 
                margin: 12px 0;
            }
            
            .inline-math { 
                display: inline;
            }
            
            mfrac, msup, msub, msqrt {
                display: inline-block;
            }
            
            .environment {
                margin: 12px 0;
                padding: 8px;
            }
            
            .environment.theorem {
                border-left: 3px solid #4CAF50;
                padding-left: 12px;
                font-style: italic;
            }
        `;
    }
    
    compile(): string {
        return this.content.join('\n');
    }
    
    renderToDOM(container: HTMLElement): void {
        container.innerHTML = this.compile();
        
        // Add MathML polyfill for browsers that need it
        this.ensureMathMLElement();
    }
    
    private ensureMathMLElement(): void {
        // Ensure MathML is supported or polyfill
        if (!document.createElement('math').toString().includes('Math')) {
            this.loadMathJaxPolyfill();
        }
    }
    
    private loadMathJaxPolyfill(): void {
        // In a real implementation, you'd load MathJax or similar
        console.warn('MathML support recommended for full mathematical rendering');
    }
}

// === ADVANCED MATHEMATICAL ENVIRONMENTS ===
class TheoremEnvironment {
    private name: string;
    private counter: number = 0;
    
    constructor(name: string) {
        this.name = name;
    }
    
    begin(title?: string): string {
        this.counter++;
        const theoremTitle = title || `${this.name} ${this.counter}`;
        return `
            <div class="environment theorem">
                <strong>${theoremTitle}</strong>
        `;
    }
    
    end(): string {
        return '</div>';
    }
}

class MatrixEnvironment {
    static render(matrix: string[][], brackets: 'pmatrix' | 'bmatrix' | 'vmatrix' = 'pmatrix'): string {
        const bracketMap = {
            pmatrix: ['(', ')'],
            bmatrix: ['[', ']'],
            vmatrix: ['|', '|']
        };
        
        const [left, right] = bracketMap[brackets];
        
        let html = `<span class="matrix">${left}`;
        html += '<mtable>';
        
        for (const row of matrix) {
            html += '<mtr>';
            for (const cell of row) {
                html += `<mtd><mn>${cell}</mn></mtd>`;
            }
            html += '</mtr>';
        }
        
        html += '</mtable>';
        html += `${right}</span>`;
        
        return html;
    }
}

// === API FACADE ===
class LatexTypeScript {
    private document: DocumentEngine;
    private theoremEnvs: Map<string, TheoremEnvironment> = new Map();
    
    constructor() {
        this.document = new DocumentEngine();
    }
    
    createDocument(classname: string = 'article'): LatexTypeScript {
        this.document = new DocumentEngine(classname);
        return this;
    }
    
    addPackage(pkg: string): LatexTypeScript {
        this.document.usePackage(pkg);
        return this;
    }
    
    documentStart(): LatexTypeScript {
        this.document.beginDocument();
        return this;
    }
    
    documentEnd(): LatexTypeScript {
        this.document.endDocument();
        return this;
    }
    
    section(title: string, level: number = 1): LatexTypeScript {
        this.document.section(title, level);
        return this;
    }
    
    mathInline(expression: string): LatexTypeScript {
        this.document.math(expression, false);
        return this;
    }
    
    mathDisplay(expression: string): LatexTypeScript {
        this.document.math(expression, true);
        return this;
    }
    
    paragraph(text: string): LatexTypeScript {
        this.document.paragraph(text);
        return this;
    }
    
    newTheorem(name: string, title?: string): LatexTypeScript {
        if (!this.theoremEnvs.has(name)) {
            this.theoremEnvs.set(name, new TheoremEnvironment(name));
        }
        
        const theorem = this.theoremEnvs.get(name)!;
        this.document.beginEnvironment('theorem');
        return this;
    }
    
    endTheorem(name: string): LatexTypeScript {
        this.document.endEnvironment('theorem');
        return this;
    }
    
    matrix(data: string[][], brackets: 'pmatrix' | 'bmatrix' | 'vmatrix' = 'pmatrix'): string {
        return MatrixEnvironment.render(data, brackets);
    }
    
    compile(): string {
        return this.document.compile();
    }
    
    render(container: HTMLElement): void {
        this.document.renderToDOM(container);
    }
    
    // Utility function to parse and render LaTeX math
    static renderMath(expression: string, displayMode: boolean = false): string {
        const lexer = new LatexLexer(expression);
        const tokens = lexer.tokenize();
        const parser = new LatexParser(tokens);
        const ast = parser.parse();
        
        return displayMode ? 
            `<div class="display-math">${ast.toMathML()}</div>` :
            `<span class="inline-math">${ast.toMathML()}</span>`;
    }
}

// === USAGE EXAMPLES ===

// Example 1: Basic document creation
function createSampleDocument(): string {
    const latex = new LatexTypeScript();
    
    return latex
        .createDocument('article')
        .documentStart()
        .section('Introduction', 1)
        .paragraph('This is a sample document with mathematical content.')
        .mathInline('E = mc^2')
        .paragraph('And a displayed equation:')
        .mathDisplay('\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}')
        .section('Matrix Example', 2)
        .mathDisplay(latex.matrix([['a', 'b'], ['c', 'd']], 'pmatrix'))
        .documentEnd()
        .compile();
}

// Example 2: Direct math rendering
function renderMathematicalFormula(): string {
    return LatexTypeScript.renderMath('\\sum_{n=1}^\\infty \\frac{1}{n^2} = \\frac{\\pi^2}{6}', true);
}

// Export for use in other modules
export {
    LatexTypeScript,
    DocumentEngine,
    LatexLexer,
    LatexParser,
    MathSymbols,
    TheoremEnvironment,
    MatrixEnvironment
};

// Global access for browser usage
if (typeof window !== 'undefined') {
    (window as any).LatexTypeScript = LatexTypeScript;
}