# dot-mine-parser
A project to aid in the learning on the Rust programming language.
A parser for the custom c style language in dot mine.

The goal is to use this as a programming language for a game. So I may not want as many options as a real programming language.
I don't want to make the language too powerfull making the problems trival.

## Syntax
The proposed syntax is currently. This isn't exhaustive and I may change these.


### Variables
```
let a : int = 1;
let b := 2;      // Type: int
let c;
```
Both vars ``a`` and ``b`` are of type int the second using type infrance. C has been initalised.
The plan is to have the compiler find the first instance c is ever set and use type infrance there.

### Loops
```
Loop {
  // Code runs here untill we
  break;
}

For (i:=0;i >= 5; i++;) {
  // Do something
}
```

### Functions
A function must have a returning type. These can be overloaded.
```
int add(int:a, int:b) {
  return a + b;
}
```
