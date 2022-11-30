# Srack Lang

Rouguelike programming language with RUST backend. 

## What is The Srack?

Do you know a Stack? So The Srack is almost the same, but all operations are completely random. For example `push` command can push to stack, or pop, or duplicate value, etc. Let's say: Srack = S(Random)ack.
So Srack Lang uses Srack for each operation.

## Example
```
seed 42
push World
push Hello
call println

push 1
push 2
call +
call println
```
Actually, the code above doesn't work, lol.

## How to run?
`run --package srack_lang --bin srack_lang -- PUT_YOUR_SRACK_FILE_HERE.srack`

## How to debug?
Specify `-v` after filename

## What the reason?
It's not a programming language, it's a game: come up with idea of task (e.g. Fibonacci) and try to defeat The Srack!

## Spec
### Commands
- `seed <Number>`: specify seed for random function. Must be specified at the beginning of the file
- `push`: push value to The Srack, can you predict the result?
- `poop`: it may look like pop command, but it can extract value multiple times. Can you predict how many?
- `call`: call function. Use values in The Srack
### Functions
- `println`: take the top value in The Srack and print
- Math functions: `+`, `-`, `*`, `/`: take 2 top values in The Srack, calculate and `push` to The Srack. Don't forget that `push` it's The Srack specific PUSH.

## Contribution
Lol, do you really want to do it? You're crazy! 