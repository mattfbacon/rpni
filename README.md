# RPN calculator

![License](https://img.shields.io/static/v1?label=License&message=AGPL-3.0-or-later&color=important&style=flat-square)
![Crates.io Version](https://img.shields.io/crates/v/rpni?style=flat-square)
![Dependencies](https://img.shields.io/librariesio/release/cargo/rpni?style=flat-square)

A basic command-line RPN calculator. It's similar to DC but with a few differences.

Numbers are stored as `f64` rather than arbitrary-precision integers. This is simpler but sometimes less precise.

## Command List

- `v`: Prints the value at the top of the stack without popping it.
- `V`: Prints the whole stack including indices. Index `0` represents the top of the stack.
- `p`: Same as `v` but pops the value from the stack before printing it.
- `+`, `-`, `*`, `/`: Exactly what you expect. The first operand is popped first.
- `%`: Pops `a`, then pops `b`, then pushes `a % b`.
- `"`: A combination of `/` and `%`. Same argument order as `%`, but pushes the quotient before pushing the remainder aka modulus.
- `^`: Pops `a`, then pops `b`, then pushes `a ^ b`.
- `~`: Pops `a`, then pushes `-a`. In other words, negates the value at the top of the stack.
- `c`: Clears the stack.
- `d`: Pops `a`, then pushes `a` twice.
- `r`: Swaps the two values at the top of the stack.
- `R`: Pops `N`, then rotates the bottom `abs(N)` values of the stack. The sign of `N` determines the rotation direction: negative is right-rotation, positive is left-rotation. The best way to understand this may be to try it yourself.
- `i`: Pops `a`, then sets the input radix to `a`. The input radix determines how numbers are parsed. In radices less than or equal to 10, all numbers can be input as normal. In radices greater than 10, the number must start with a digit (0 to 9), but you can prefix the number with `0` and it will result in the same value. For example, in base 16, trying to input `c2` as a number would run the `c` command, then input the number `2`, so instead you should input `0c2`.
- `I`: Pushes the input radix.
- ` ` (space), `\t` (tab), `\0` (ASCII NUL), `\n` (newline): Does nothing, but splits numbers and other operations, allowing for multiple on the same line in the case of space, tab, and NUL (e.g., `123 456` pushes `123` and `456` separately).
- `z`: Pushes the current length of the stack, not including this value that is about to be pushed.
- `?<command>`: Gets help for a command.

### Register Commands

Registers are indexed by their UTF-8 codepoint, which can be any single character. They are their own fully-fledged, freestanding stacks.

- `s<reg>`: Pops a value from the global stack and pushes it to the specified register.
- `l<reg>`: Pops a value from the specified register and pushes it to the global stack.
- `S<reg>`: Peeks a value from the stack without popping it and pushes it to the specified register.
- `L<reg>`: Peeks a value from the specified register and pushes it to the global stack.
- `&<reg><operation>`: Performs `operation` inside the specified register. Nested register operations such as `&asb` are not permitted.

## Usage Examples

```
> 2 2+v
4
> c 2~V
0: -2
> 16i c2 V
0: 2
> c 0c2 V
0: 194
> 0a i c 123 sl V
> &lV
0: 123
> 456 sl &l+ Ll v
579
> c 1 2 3 4 V
3: 1
2: 2
1: 3
0: 4
> 3RV
3: 1
2: 3
1: 4
0: 2
> 3~RV
3: 1
2: 2
1: 3
0: 4
> c 0 0 0 0 zp
4
> c 1 0 / p
inf
```
