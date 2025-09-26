// types.ts

export type Token = 
  | { type: 'whitespace'; value: string }
  | { type: 'brace'; value: '{' | '}' }
  | { type: 'bracket'; value: '[' | ']' }
  | { type: 'superscript'; value: '^' }
  | { type: 'subscript'; value: '_' }
  | { type: 'command'; value: string }
  | { type: 'identifier'; value: string }
  | { type: 'number'; value: string }
  | { type: 'operator'; value: string }
  | { type: 'punctuation'; value: string };

export type MathNode = 
  | Identifier
  | Number
  | Operator
  | Superscript
  | Subscript
  | Fraction
  | SquareRoot
  | Group
  | Sequence;

export interface Identifier {
  type: 'identifier';
  name: string;
}

export interface Number {
  type: 'number';
  value: string;
}

export interface Operator {
  type: 'operator';
  operator: string;
  left: MathNode;
  right: MathNode;
}

export interface Superscript {
  type: 'superscript';
  base: MathNode;
  exponent: MathNode;
}

export interface Subscript {
  type: 'subscript';
  base: MathNode;
  subscript: MathNode;
}

export interface Fraction {
  type: 'fraction';
  numerator: MathNode;
  denominator: MathNode;
}

export interface SquareRoot {
  type: 'sqrt';
  body: MathNode;
}

export interface Group {
  type: 'group';
  body: MathNode;
  delimiter?: string; // We'll add for future
}

export interface Sequence {
  type: 'sequence';
  children: MathNode[];
}