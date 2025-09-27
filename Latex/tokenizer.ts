// tokenizer.ts

import { Token } from './types';

export function tokenize(input: string): Token[] {
  const tokens: Token[] = [];
  let current = 0;

  while (current < input.length) {
    let char = input[current];

    // Whitespace
    if (/\s/.test(char)) {
      let value = '';
      while (current < input.length && /\s/.test(input[current])) {
        value += input[current];
        current++;
      }
      tokens.push({ type: 'whitespace', value });
      continue;
    }

    // Braces
    if (char === '{' || char === '}') {
      tokens.push({ type: 'brace', value: char });
      current++;
      continue;
    }

    // Superscript and subscript
    if (char === '^') {
      tokens.push({ type: 'superscript', value: '^' });
      current++;
      continue;
    }

    if (char === '_') {
      tokens.push({ type: 'subscript', value: '_' });
      current++;
      continue;
    }

    // Commands: start with backslash
    if (char === '\\') {
      let value = '';
      current++; // skip the backslash
      // Read the command name: letters only
      while (current < input.length && /[a-zA-Z]/.test(input[current])) {
        value += input[current];
        current++;
      }
      tokens.push({ type: 'command', value });
      continue;
    }

    // Numbers: digits and possibly a decimal point
    if (/[0-9]/.test(char)) {
      let value = '';
      while (current < input.length && /[0-9.]/.test(input[current])) {
        value += input[current];
        current++;
      }
      tokens.push({ type: 'number', value });
      continue;
    }

    // Operators: +, -, *, /
    if (['+', '-', '*', '/'].includes(char)) {
      tokens.push({ type: 'operator', value: char });
      current++;
      continue;
    }

    // Identifiers: letters
    if (/[a-zA-Z]/.test(char)) {
      let value = '';
      while (current < input.length && /[a-zA-Z]/.test(input[current])) {
        value += input[current];
        current++;
      }
      tokens.push({ type: 'identifier', value });
      continue;
    }

    // If we get here, we don't know what it is, so we skip one character
    current++;
  }

  return tokens;
}
