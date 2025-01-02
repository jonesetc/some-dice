# some-dice

A work-in-progress, independent implmementation of [AnyDice](https://anydice.com/).

## What?

AnyDice is a small language and runtime for looking at the distribution of various ways of rolling dice. This project is just an independent implementation of that which aims for as much compatibility as reasonable, while focusing on some different uses than the original implementation. Namely:

- Offline/Available through ordinary pakage management systems
- Gives exact probabilities (rationals)
- AST representation
- Interactive interpreter
- Easy to sample
- Easy to use only parts needed (parse, AST, interpret, sample)

## Why?

Because I want to learn a bit more about how to practically implement an interpreter, and I think that AnyDice is pretty cool. I would also like to be able to use it locally, embed it in other things, and streamline use for myself.

## Progress

### Parse/AST

- [ ] Expression
    - [x] Integer
    - [x] Dice Collection
        - [x] simple
        - [x] arbitrary
    - [x] Sequence
        - [x] single value
        - [x] range values
        - [x] repetitions
    - [x] Variable Reference
    - [x] Arithmetic Operation
        - [x] negate
        - [x] add
        - [x] subtract
        - [x] multiply
        - [x] divide
        - [x] exponent
    - [x] Boolean Operation
        - [x] not
        - [x] and
        - [x] or
        - [x] equals
        - [x] not equals
        - [x] less than
        - [x] less than or equal
        - [x] greater than
        - [x] greater than or equals
    - [x] Introspection Operation
        - [x] count
        - [x] index access
    - [ ] Function Call
- [ ] Statement
    - [ ] Variable Assignment
    - [ ] Conditional
        - [ ] if
        - [ ] else
    - [ ] Loop
    - [ ] Function definition
        - [ ] nested variable assignment
        - [ ] result
        - [ ] parameter types
    - [ ] Output
        - [x] unnamed
        - [ ] named
            - [x] static
            - [ ] interpolated
    - [x] Configuration
        - [x] expression
        - [x] string
- [x] Comment

After Parsing is complete, get syntax errors wrangled.

### Interpret

- [ ] Interpreter
    - [ ] run program
    - [ ] run statement
    - [ ] run expression
- [ ] Environment
    - [ ] storage
    - [ ] nested look-up
- [ ] Configuration
    - [ ] position order
    - [ ] max function depth
    - [ ] explode depth
- [ ] Output
    - Up weights to common divisor?
- [ ] Built-in functions
    - [ ] absolute
    - [ ] contains
    - [ ] count in
    - [ ] explode
    - [ ] highest of
    - [ ] lowest of
    - [ ] middle of
    - [ ] highest of and
    - [ ] lowest of and
    - [ ] maximum of
    - [ ] reverse
    - [ ] sort
- [ ] AST implementations
    - [ ] world's biggest TBD

### Sample

- [x] Sample an output once
- [x] Sample an input N times
    - [x] Vec
    - [x] Const generic
- [x] Sampling iterator
- [ ] TBD for any extra usability features

## Credits

AnyDice was created by [Jasper Flick](https://catlikecoding.com/jasper-flick/), not me, and this implementation is based fully off of playing with the online interpreter, reading the documentation, and asking him a few questions when I got lost.

Some useful internal functions (GCD and LCM) were copied from [rust-num](https://github.com/rust-num) because the generic implementation was not needed here, but they provide great implementations with a compatible license. Credit and license are commented directly in the source, but I thought it would still be nice to thank them here.
