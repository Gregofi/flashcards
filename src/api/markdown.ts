import Showdown from 'showdown';
const converter = new Showdown.Converter();

export const escapeLatex = (text: string) =>
    text
        .split(/\$\$?/)
        .map((cur, idx, array) => {
            // if even then we're inside block.
            if (idx % 2 === 1) {
                const delim = array[idx + 1] === '$' ? '$$' : '$';
                return `${delim}${btoa(cur)}${delim}`;
            } else {
                return cur;
            }
        })
        .join('');

export const unescapeLatex = (text: string) =>
    text
        .split(/\$\$?/)
        .map((cur, idx, array) => {
            // if even then we're inside block.
            if (idx % 2 === 1) {
                const delim = array[idx + 1] === '$' ? '$$' : '$';
                return `${delim}${atob(cur)}${delim}`;
            } else {
                return cur;
            }
        })
        .join('');

export const HtmlToMarkdown = (text: string) =>
    unescapeLatex(converter.makeHtml(escapeLatex(text)));
