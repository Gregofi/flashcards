export {};

declare global {
  interface Window {
    MathJax: {
      typesetPromise: () => Promise<void>;
      tex: any,
      svg: any,
    };
  }
}

(window as any).MathJax = {
  tex: {
    inlineMath: [['$', '$'], ['\\(', '\\)']]
  },
  svg: {
    fontCache: 'global'
  }
};

(() => {
  const script = document.createElement('script');
  script.src = '/external/mathjax/tex-chtml.js';
  script.async = true;
  document.head.appendChild(script);
})();
