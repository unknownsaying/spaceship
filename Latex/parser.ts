// parser.ts

import { Token, MathNode, Sequence, Identifier, Number, Operator, Superscript, Subscript, Fraction, SquareRoot, Group } from './types';

export class Parser {
  private tokens: Token[];
  private current: number;

  constructor(tokens: Token[]) {
    this.tokens = tokens;
    this.current = 0;
  }

  parse(): MathNode {
    return this.parseExpression();
  }

  private parseExpression(): MathNode {
    let left = this.parseTerm();

    while (this.match('operator')) {
      const operator = this.previous().value;
      const right = this.parseTerm();
      left = {
        type: 'operator',
        operator,
        left,
        right
      } as Operator; // Note: We are extending Operator to have left and right? Let's adjust the type.
    }

    return left;
  }

  private parseTerm(): MathNode {
    let left = this.parseFactor();

    while (this.match('superscript') || this.match('subscript')) {
      const token = this.previous();
      if (token.type === 'superscript') {
        const exponent = this.parseFactor();
        left = {
          type: 'superscript',
          base: left,
          exponent
        } as Superscript;
      } else if (token.type === 'subscript') {
        const subscript = this.parseFactor();
        left = {
          type: 'subscript',
          base: left,
          subscript
        } as Subscript;
      }
    }

    return left;
  }

  private parseFactor(): MathNode {
    if (this.match('identifier')) {
      return { type: 'identifier', name: this.previous().value } as Identifier;
    }

    if (this.match('number')) {
      return { type: 'number', value: this.previous().value } as Number;
    }

    if (this.match('command')) {
      return this.parseCommand();
    }

    if (this.match('brace', '{')) {
      return this.parseGroup();
    }

    // If we get here, we have an error, but for now, return an empty identifier
    return { type: 'identifier', name: '' };
  }

  private parseCommand(): MathNode {
    const command = this.previous().value;

    if (command === 'frac') {
      // Expect two groups
      const num = this.parseGroup();
      const den = this.parseGroup();
      return {
        type: 'fraction',
        numerator: num,
        denominator: den
      } as Fraction;
    }

    if (command === 'sqrt') {
      const body = this.parseGroup();
      return {
        type: 'sqrt',
        body
      } as SquareRoot;
    }

    // Unknown command, treat as identifier?
    return { type: 'identifier', name: '\\' + command };
  }

  private parseGroup(): MathNode {
    // We already consumed the '{'
    const body = this.parseExpression();
    this.consume('brace', '}');
    return {
      type: 'group',
      body
    } as Group;
  }

  private match(type: string, value?: string): boolean {
    if (this.current >= this.tokens.length) return false;
    const token = this.tokens[this.current];
    if (token.type !== type) return false;
    if (value !== undefined && token.value !== value) return false;
    this.current++;
    return true;
  }

  private consume(type: string, value?: string): void {
    if (this.match(type, value)) {
      return;
    }
    throw new Error(`Expected token ${type} with value ${value}, but found ${this.tokens[this.current]}`);
  }

  private previous(): Token {
    return this.tokens[this.current - 1];
  }
}