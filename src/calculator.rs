use crate::registers::Registers;
use crate::stack::Stack;
use std::iter::Iterator;

pub type Value = f64;

pub struct Calculator {
	input_radix: u32,
	stack: Stack<Value>,
	registers: Registers<Value>,
}

struct CalculatorInterface<'a> {
	input_radix: &'a mut u32,
	stack: &'a mut Stack<Value>,
}

impl CalculatorInterface<'_> {
	const MAX_RADIX: u32 = 16;

	fn is_numeric(input_radix: u32, c: char) -> bool {
		c.is_digit(input_radix)
	}
	fn number_char(input_radix: u32, c: char) -> bool {
		Self::is_numeric(input_radix, c) || c == '.'
	}
	fn collect_f64(input_radix: u32, iter: impl Iterator<Item = anyhow::Result<char>>) -> anyhow::Result<(Option<char>, f64)> {
		let mut ret = 0.0;
		let mut digits_past_decimal: Option<usize> = None;
		for digit_or_period in iter {
			let digit_or_period = digit_or_period?;
			if !Self::number_char(input_radix, digit_or_period) {
				return Ok((Some(digit_or_period), ret));
			}
			if let Some(ref mut digits_past_decimal) = digits_past_decimal {
				let digit = digit_or_period
					.to_digit(input_radix)
					.ok_or_else(|| anyhow::anyhow!("Number literal has multiple decimal points (second decimal was found {digits_past_decimal} chars after first)"))?;
				*digits_past_decimal += 1;
				let mut digit = f64::from(digit);
				let radix_reciprocal = 1.0 / f64::from(input_radix);
				for _ in 0..*digits_past_decimal {
					digit *= radix_reciprocal;
				}
				ret += digit;
			} else if let Some(digit) = digit_or_period.to_digit(input_radix) {
				ret *= f64::from(input_radix);
				ret += f64::from(digit);
			} else {
				assert_eq!(digit_or_period, '.');
				digits_past_decimal = Some(0);
			}
		}
		Ok((None, ret))
	}

	fn handle(&mut self, mut first_char: char, mut iter: &mut impl Iterator<Item = anyhow::Result<char>>) -> anyhow::Result<()> {
		if first_char.is_numeric() {
			let (non_numeric, value) = Self::collect_f64(*self.input_radix, std::iter::once(Ok(first_char)).chain(&mut iter))?;
			self.stack.push(value);
			match non_numeric {
				Some(next_char) => first_char = next_char,
				None => return Ok(()),
			}
		}
		match first_char {
			'v' => println!("{}", self.stack.top()?),
			'V' => print!("{}", self.stack),
			'p' => println!("{}", self.stack.pop()?),
			'+' => {
				let addend = self.stack.pop()?;
				*self.stack.top_mut()? += addend;
			}
			'-' => {
				let subtrahend = self.stack.pop()?;
				*self.stack.top_mut()? -= subtrahend;
			}
			'*' => {
				let factor = self.stack.pop()?;
				*self.stack.top_mut()? *= factor;
			}
			'/' => {
				let divisor = self.stack.pop()?;
				*self.stack.top_mut()? /= divisor;
			}
			'%' => {
				let modulus = self.stack.pop()?;
				*self.stack.top_mut()? %= modulus;
			}
			'"' => {
				let &dividend = self.stack.nth(0)?;
				let &divisor = self.stack.nth(1)?;
				*self.stack.nth_mut(1)? = dividend / divisor;
				*self.stack.nth_mut(0)? = dividend % divisor;
			}
			'^' => {
				let amount = self.stack.pop()?;
				*self.stack.top_mut()? = self.stack.top()?.powf(amount);
			}
			'~' => {
				*self.stack.top_mut()? = -*self.stack.top()?;
			}
			// skipping '|'
			'c' => self.stack.clear(),
			'd' => self.stack.push(*self.stack.top()?),
			'r' => {
				let (&a, &b) = (self.stack.nth(0)?, self.stack.nth(1)?);
				*self.stack.nth_mut(0)? = b;
				*self.stack.nth_mut(1)? = a;
			}
			'R' => {
				let amount = self.stack.pop()? as isize;
				self.stack.rotate(amount)?;
			}
			'i' => {
				let radix = *self.stack.top()?;
				if radix as u32 > Self::MAX_RADIX {
					eprintln!("Maximum radix is {radix}", radix = Self::MAX_RADIX);
				} else {
					*self.input_radix = self.stack.pop()? as u32;
				}
			}
			'I' => self.stack.push(*self.input_radix as f64),
			' ' | '\t' | '\0' => (),
			'\n' => {
				eprint!("> ");
			}
			'z' => self.stack.push(self.stack.len() as f64),
			_ => {
				anyhow::bail!("Invalid command {first_char:?}");
			}
		}
		Ok(())
	}
}

impl Calculator {
	pub fn new() -> Self {
		Self {
			input_radix: 10,
			stack: Stack::new(),
			registers: Registers::new(),
		}
	}

	fn print_help(for_command: char) -> anyhow::Result<()> {
		match for_command {
			'\n' | ' ' | '\t' | '\0' => {
				eprint!("{:?} requires a command that you would like help for", '?');
				if for_command == '\n' {
					eprint!("\n> ");
				}
				return Ok(());
			}
			_ => (),
		}
		eprint!("{for_command:?}: ");
		match for_command {
			'v' => eprintln!("Prints the value at the top of the stack without popping it."),
			'V' => eprintln!("Prints the whole stack including indices. Index `0` represents the top of the stack."),
			'p' => eprintln!("Same as `v` but pops the value from the stack before printing it."),
			'+'| '-'| '*'| '/' => eprintln!("Exactly what you expect. The first operand is popped first."),
			'%' => eprintln!("Pops `a`, then pops `b`, then pushes `b % a`."),
			'"' => eprintln!("A combination of `/` and `%`. Same argument order as `%`, but pushes the quotient before pushing the remainder aka modulus."),
			'^' => eprintln!("Pops `a`, then pops `b`, then pushes `b ^ a`."),
			'~' => eprintln!("Pops `a`, then pushes `-a`. In other words, negates the value at the top of the stack."),
			'c' => eprintln!("Clears the stack."),
			'd' => eprintln!("Pops `a`, then pushes `a` twice."),
			'r' => eprintln!("Swaps the two values at the top of the stack."),
			'i' => eprintln!("Pops `a`, then sets the input radix to `a`. The input radix determines how numbers are parsed. In radices less than or equal to 10, all numbers can be input as normal. In radices greater than 10, the number must start with a digit (0 to 9), but you can prefix the number with `0` and it will result in the same value. For example, in base 16, trying to input `c2` as a number would run the `c` command, then input the number `2`, so instead you should input `0c2`."),
			'I' => eprintln!("Pushes the input radix."),
			'z' => eprintln!("Pushes the current length of the stack, not including this value that is about to be pushed."),
			'?' => eprintln!("(Args <command>) Gets help for a command."),
			register_command @ ('s' | 'l' | 'S' | 'L' | '&') => {
				match register_command {
					's' => eprintln!("(Args <reg>) Pops a value from the global stack and pushes it to the specified register."),
					'l' => eprintln!("(Args <reg>) Pops a value from the specified register and pushes it to the global stack."),
					'S' => eprintln!("(Args <reg>) Peeks a value from the stack without popping it and pushes it to the specified register."),
					'L' => eprintln!("(Args <reg>) Peeks a value from the specified register and pushes it to the global stack."),
					'&' => eprintln!("(Args <reg><operation>) Performs `operation` inside the specified register. Nested register operations such as `&asb` are not permitted."),
					_ => unreachable!(),
				}
				eprintln!("Note: Registers are indexed by their UTF-8 codepoint, which can be any single character. They are their own fully-fledged, freestanding stacks. All register operations take a register operand that is a single character directly after it.");
			},
			_unknown => eprintln!("Unknown command"),
		}
		Ok(())
	}

	fn handle(&mut self, first_char: char, mut iter: impl Iterator<Item = anyhow::Result<char>>) -> anyhow::Result<()> {
		fn expected_reg(op: char) -> impl FnOnce() -> anyhow::Error {
			move || anyhow::anyhow!("Expected register for {op:?} command")
		}

		match first_char {
			'?' => {
				let err = || anyhow::anyhow!("Expected the command you want to know about for {:?} command, e.g., `?&` to learn about the {:?} command", '?', '&');
				let for_command = iter.next().ok_or_else(err)??;
				Self::print_help(for_command)?;
			}
			's' => {
				let reg = iter.next().ok_or_else(expected_reg('s'))??;
				let val = self.stack.pop()?;
				self.registers.push(reg, val);
			}
			'l' => {
				let reg = iter.next().ok_or_else(expected_reg('l'))??;
				let val = self.registers.pop(reg)?;
				self.stack.push(val);
			}
			'S' => {
				let reg = iter.next().ok_or_else(expected_reg('s'))??;
				self.registers.push(reg, *self.stack.top()?);
			}
			'L' => {
				let reg = iter.next().ok_or_else(expected_reg('L'))??;
				self.stack.push(*self.registers.top(reg)?);
			}
			'&' => {
				let reg = iter.next().ok_or_else(expected_reg('&'))??;
				let operation = iter.next().ok_or_else(|| anyhow::anyhow!("Expected operation for {:?}", '&'))??;
				CalculatorInterface {
					stack: self.registers.stack_mut(reg),
					input_radix: &mut self.input_radix,
				}
				.handle(operation, &mut iter)?;
			}
			non_register_operation => CalculatorInterface {
				stack: &mut self.stack,
				input_radix: &mut self.input_radix,
			}
			.handle(non_register_operation, &mut iter)?,
		};
		Ok(())
	}

	pub fn evaluate(&mut self, mut iter: impl Iterator<Item = anyhow::Result<char>>) -> anyhow::Result<()> {
		eprint!("> ");
		while let Some(first_char) = iter.next() {
			let first_char = first_char?;
			match self.handle(first_char, &mut iter) {
				Ok(()) => continue,
				Err(err) => eprintln!("Error: {err:?}"),
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::CalculatorInterface as I;

	#[test]
	fn is_numeric() {
		assert_eq!(I::is_numeric(10, '0'), true);
		assert_eq!(I::is_numeric(10, 'f'), false);
		assert_eq!(I::is_numeric(16, '0'), true);
		assert_eq!(I::is_numeric(16, 'f'), true);
	}
	#[test]
	fn number_char() {
		assert_eq!(I::number_char(10, '.'), true);
		assert_eq!(I::number_char(10, 'f'), false);
		assert_eq!(I::number_char(16, '.'), true);
		assert_eq!(I::number_char(16, 'f'), true);
	}
	#[test]
	fn collect_f64() {
		assert_eq!(I::collect_f64(10, "".chars().map(Ok)).unwrap(), (None, 0.0));
		assert_eq!(I::collect_f64(10, " ".chars().map(Ok)).unwrap(), (Some(' '), 0.0));
		assert_eq!(I::collect_f64(10, "0".chars().map(Ok)).unwrap(), (None, 0.0));
		assert_eq!(I::collect_f64(10, "1".chars().map(Ok)).unwrap(), (None, 1.0));
		assert_eq!(I::collect_f64(10, "10".chars().map(Ok)).unwrap(), (None, 10.0));
		assert_eq!(I::collect_f64(16, "10".chars().map(Ok)).unwrap(), (None, 16.0));
		assert_eq!(I::collect_f64(10, "1.5".chars().map(Ok)).unwrap(), (None, 1.5));
		assert_eq!(I::collect_f64(10, "1.5\n".chars().map(Ok)).unwrap(), (Some('\n'), 1.5));
		assert_eq!(I::collect_f64(16, "1.8\n".chars().map(Ok)).unwrap(), (Some('\n'), 1.5));
	}
}
