// renderer.ts

import { MathNode } from './types';

export function renderToMathML(node: MathNode): string {
  switch (node.type) {
    case 'identifier':
      return `<mi>${node.name}</mi>`;
    case 'number':
      return `<mn>${node.value}</mn>`;
    case 'operator':
      return `<mo>${node.operator}</mo>`;
    case 'superscript':
      return `<msup>${renderToMathML(node.base)}${renderToMathML(node.exponent)}</msup>`;
    case 'subscript':
      return `<msub>${renderToMathML(node.base)}${renderToMathML(node.subscript)}</msub>`;
    case 'fraction':
      return `<mfrac>${renderToMathML(node.numerator)}${renderToMathML(node.denominator)}</mfrac>`;
    case 'sqrt':
      return `<msqrt>${renderToMathML(node.body)}</msqrt>`;
    case 'group':
      return renderToMathML(node.body);
    case 'sequence':
      return node.children.map(renderToMathML).join('');
    default:
      return '';
  }
}