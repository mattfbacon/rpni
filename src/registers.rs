use crate::stack::Stack;
use anyhow::Context;
use std::collections::HashMap;

pub struct Registers<T>(HashMap<char, Stack<T>>);

pub fn not_set(reg: char) -> anyhow::Error {
	anyhow::anyhow!("Register {reg:?} has not been set")
}

impl<T> Registers<T> {
	pub fn new() -> Self {
		Self(HashMap::new())
	}

	pub fn stack_mut(&mut self, reg: char) -> &mut Stack<T> {
		if !self.0.contains_key(&reg) {
			let stack = Stack::new();
			self.0.insert(reg, stack);
		}
		self.0.get_mut(&reg).unwrap()
	}
	pub fn push(&mut self, reg: char, value: T) {
		self.stack_mut(reg).push(value);
	}
	pub fn pop(&mut self, reg: char) -> anyhow::Result<T> {
		self.stack_mut(reg).pop().with_context(move || format!("Evaluating register {reg:?}"))
	}
	pub fn top(&self, reg: char) -> anyhow::Result<&T> {
		if let Some(stack) = self.0.get(&reg) {
			stack.top().with_context(move || format!("Evaluating register {reg:?}"))
		} else {
			return Err(not_set(reg));
		}
	}
}

#[cfg(test)]
mod test {
	use super::Registers;
	use core::assert_matches::assert_matches;

	#[test]
	fn new() {
		let _ = Registers::<f64>::new();
	}
	#[test]
	fn stack_mut() {
		let mut registers = Registers::<f64>::new();
		let stack = registers.stack_mut('a');
		assert_eq!(stack.len(), 0);
		stack.push(1.0);
		assert_eq!(stack.pop().unwrap(), 1.0);
	}
	#[test]
	fn push() {
		let mut registers = Registers::<f64>::new();
		registers.push('a', 1.0);
		let stack = registers.stack_mut('a');
		assert_eq!(stack.len(), 1);
		assert_eq!(stack.pop().unwrap(), 1.0);
	}
	#[test]
	fn pop_top() {
		let mut registers = Registers::<f64>::new();
		assert_matches!(registers.top('a'), Err(_));
		assert_matches!(registers.pop('a'), Err(_));
		registers.push('a', 1.0);
		assert_eq!(*registers.top('a').unwrap(), 1.0);
		assert_eq!(registers.pop('a').unwrap(), 1.0);
		assert_matches!(registers.pop('a'), Err(_));
		assert_matches!(registers.top('a'), Err(_));
	}
}
