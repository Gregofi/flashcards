# Flashcards

[![Build and Lint](https://github.com/Gregofi/flashcards/actions/workflows/app-action.yaml/badge.svg)](https://github.com/Gregofi/flashcards/actions/workflows/app-action.yaml)

A spaced repetition flashcard app similar to Anki. It is designed to work with
flashcard driven note taking. For example, suppose that you have your notes
written in markdown files like this:

```md
Haskell is a functional programming language.
It is statically typed.

How to type a function that takes two numbers and return back their sum? #flashcard

In Haskell, a function does not accept multiple parameters in the tradditional sense,
so `add(int, int) -> int` would not work here. Rather, it takes one number and returns
a function that takes one number and returns an int:
`Add :: (Integral a) => a -> a -> a`

---

Haskell is a functional language, that means that you cannot "mutate" a state
... Rest of your notes.
```

The app can then watch folders with your notes and automatically sync them.

## How to build and run

To build the application, first install dependencies via `npm` or `pnpm`

```
pnpm i
```

And then run the application

```
pnpm run tauri dev
```
