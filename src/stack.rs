pub struct Stack<T>(Vec<T>);

fn s() -> anyhow::Error {
	anyhow::anyhow!("Stack is empty")
}
fn too_small(needed: usize) -> anyhow::Error {
	anyhow::anyhow!("Stack does not have {needed} elements")
}

impl<T> Stack<T> {
	pub fn new() -> Self {
		Self(Vec::with_capacity(10))
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
	pub fn nth(&self, n: usize) -> anyhow::Result<&T> {
		let len = self.0.len();
		if n >= len {
			return Err(too_small(n));
		}
		self.0.get(len - n - 1).ok_or_else(|| too_small(n))
	}
	pub fn nth_mut(&mut self, n: usize) -> anyhow::Result<&mut T> {
		let len = self.0.len();
		if n >= len {
			return Err(too_small(n));
		}
		self.0.get_mut(len - n - 1).ok_or_else(|| too_small(n))
	}
	pub fn top(&self) -> anyhow::Result<&T> {
		self.0.last().ok_or_else(s)
	}
	pub fn top_mut(&mut self) -> anyhow::Result<&mut T> {
		self.0.last_mut().ok_or_else(s)
	}
	pub fn pop(&mut self) -> anyhow::Result<T> {
		self.0.pop().ok_or_else(s)
	}
	pub fn push(&mut self, val: T) {
		self.0.push(val)
	}
	pub fn clear(&mut self) {
		self.0.clear()
	}
	pub fn rotate(&mut self, amount: isize) -> anyhow::Result<()> {
		let rotate_right = amount < 0;
		let amount = amount.abs() as usize;
		if amount == 0 {
			return Ok(());
		}
		let len = self.0.len();
		if amount > len {
			return Err(too_small(amount));
		}
		if rotate_right {
			(&mut self.0[len - amount..]).rotate_right(1);
		} else {
			(&mut self.0[len - amount..]).rotate_left(1);
		}
		Ok(())
	}
}

use std::fmt::{self, Display, Formatter};
impl<T: Display> Display for Stack<T> {
	fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
		let len = self.0.len();
		for (idx, item) in self.0.iter().enumerate() {
			let idx = len - idx - 1;
			writeln!(formatter, "{idx}: {item}")?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod test {
	use super::Stack;
	use core::assert_matches::assert_matches;

	#[test]
	fn basic() {
		let mut stack = Stack::new();
		stack.push(3.0);
		stack.push(4.0);
		assert_eq!(stack.pop().unwrap(), 4.0);
		assert_eq!(stack.pop().unwrap(), 3.0);
		assert_matches!(stack.pop(), Err(_));
	}
	#[test]
	fn top() {
		let mut stack = Stack::new();
		assert_matches!(stack.top(), Err(_));
		stack.push(1.0);
		stack.push(2.0);
		assert_eq!(*stack.top().unwrap(), 2.0);
		*stack.top_mut().unwrap() = 3.0;
		assert_eq!(*stack.top().unwrap(), 3.0);
	}
	#[test]
	fn len_clear() {
		let mut stack = Stack::new();
		assert_eq!(stack.len(), 0);
		stack.push(1.0);
		assert_eq!(stack.len(), 1);
		stack.push(2.0);
		assert_eq!(stack.len(), 2);
		stack.push(3.0);
		assert_eq!(stack.len(), 3);
		stack.clear();
		assert_eq!(stack.len(), 0);
	}
	#[test]
	fn nth() {
		let mut stack = Stack::new();
		assert_matches!(stack.nth(2), Err(_));
		assert_matches!(stack.nth_mut(2), Err(_));
		stack.push(1.0);
		assert_matches!(stack.nth(2), Err(_));
		stack.push(2.0);
		assert_matches!(stack.nth(2), Err(_));
		stack.push(3.0);
		assert_eq!(*stack.nth(2).unwrap(), 1.0);
		*stack.nth_mut(2).unwrap() = 3.0;
		assert_eq!(*stack.nth(2).unwrap(), 3.0);
		*stack.nth_mut(0).unwrap() = 1.0;
		assert_eq!(stack.pop().unwrap(), 1.0);
	}
	#[test]
	fn rotate() {
		let mut stack = Stack::new();
		assert_matches!(stack.rotate(0), Ok(()));
		assert_matches!(stack.rotate(3), Err(_));
		stack.push(1.0);
		stack.push(2.0);
		stack.push(3.0);
		stack.push(4.0);
		stack.rotate(3).unwrap();
		assert_eq!(stack.0.as_slice(), &[1.0, 3.0, 4.0, 2.0]);
		stack.rotate(-3).unwrap();
		assert_eq!(stack.0.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
		stack.rotate(-3).unwrap();
		assert_eq!(stack.0.as_slice(), &[1.0, 4.0, 2.0, 3.0]);
		stack.rotate(3).unwrap();
		assert_eq!(stack.0.as_slice(), &[1.0, 2.0, 3.0, 4.0]);
	}
	#[test]
	fn display() {
		let mut stack = Stack::new();
		stack.push(1);
		stack.push(2);
		assert_eq!(stack.to_string(), "1: 1\n0: 2\n");
	}
}
