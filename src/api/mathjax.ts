export {};

declare global {
  interface Window {
    MathJax: {
      typesetPromise: () => Promise<void>;
      typeset: () => number;
    };
  }
}
