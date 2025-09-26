// index.ts

import { tokenizer } from './tokenizer';
import { Parser } from './parser';
import { renderToMathML } from './redenrer';

export function latexToMathML(latex: string): string {
  const tokens = tokenizer(latex);
  const parser = new Parser(tokens);
  const ast = parser.parse();
  return renderToMathML(ast);
}