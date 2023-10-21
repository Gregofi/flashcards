import { escape, escapeLatex, unescapeLatex } from './markdown';

it('escape', () => {
    const encode = (text: string) => '>' + text + '<';
    expect(escape('abc', /d/, encode)).toBe('abc');
    expect(escape('abc', /a/, encode)).toBe('abc');
    expect(escape('aa', /a/, encode)).toBe(`a><a`);
    expect(escape('aba', /a/, encode)).toBe(`a>b<a`);
    expect(escape('abbba', /a/, encode)).toBe(`a>bbb<a`);
    expect(escape('xoabbbar', /a/, encode)).toBe(`xoa>bbb<ar`);
    expect(escape('xoabababar', /a/, encode)).toBe(`xoa>b<aba>b<ar`);
    expect(escape('xoxooxxooxxlo', /oo/, encode)).toBe(`xoxoo>xx<ooxxlo`);
    expect(escape('xoxooxxooxxlo', /oo?/, encode)).toBe(`xo>x<oo>xx<oo>xxl<o`);
    expect(escape('xooxooxxooxxlo', /oo?/, encode)).toBe(`xoo>x<ooxxooxxlo`);
});

it('escapeLatex', () => {
    expect(escapeLatex('abc')).toBe('abc');
    expect(escapeLatex('a$$b')).toBe('a$$b');
    expect(escapeLatex('a$$b$$c')).toBe(`a$$${btoa('b')}$$c`);
    expect(escapeLatex('a$$x^2$$c')).toBe(`a$$${btoa('x^2')}$$c`);
    expect(unescapeLatex(escapeLatex('a$$x^2$$c'))).toBe(`a$$x^2$$c`);
});

describe('escapeLatex real cases', () => {
    const testEscape = (text: string) => escape(text, /\$\$?/, (text) => `>${text}<`);

    it('escapeLatex real case 1"', () => {
        expect(testEscape('average $\\overline{X}_n$ from')).toBe(
            'average $>\\overline{X}_n<$ from'
        );

        const text = `Lets say we want to estimate if a person is sick or healthy. We do some measurement of blood and measure some value $X_i$, we do this multiple times and take the average $\\overline{X}_n$ from this.
  If the measured value is larger than $10 = \\mu$ then the person is sick. However, there can be measurement errors and we do not want to give medicine to healthy person, because the medicine has bad side effects.
  How do we choose $H_0$ and $H_A$?`;
        const escaped = testEscape(text);
        expect(escaped)
            .toBe(`Lets say we want to estimate if a person is sick or healthy. We do some measurement of blood and measure some value $>X_i<$, we do this multiple times and take the average $>\\overline{X}_n<$ from this.
  If the measured value is larger than $>10 = \\mu<$ then the person is sick. However, there can be measurement errors and we do not want to give medicine to healthy person, because the medicine has bad side effects.
  How do we choose $>H_0<$ and $>H_A<$?`);
    });
});
