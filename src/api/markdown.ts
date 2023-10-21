import Showdown from 'showdown';
const converter = new Showdown.Converter();

export type TransformFunction<T> = (cur: T, index: number, arr: T[]) => T;

export const escape = (
    text: string,
    delimRegexp: RegExp,
    encoding: (text: string) => string
): string => {
    const _escape = (text: string): string => {
        const match = text.match(delimRegexp);
        if (match === null) {
            return text;
        }
        const begin = match.index!;
        const delim = match[0];
        console.log(begin);
        console.log(delim);

        const end = text.indexOf(delim, begin + delim.length);
        console.log(end);
        if (end === -1) {
            return text;
        }

        const toEscape = text.slice(begin + delim.length, end);
        return (
            text.slice(0, begin) +
            delim +
            encoding(toEscape) +
            delim +
            _escape(text.slice(end + delim.length))
        );
    };

    return _escape(text);
};

export const escapeLatex = (text: string) => escape(text, /\$\$?/, (text: string) => btoa(text));
export const unescapeLatex = (text: string) => escape(text, /\$\$?/, (text: string) => atob(text));

export const markdownToHtml = (text: string) =>
    unescapeLatex(converter.makeHtml(escapeLatex(text)));
