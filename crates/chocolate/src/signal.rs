use std::{
	borrow::BorrowMut,
	cell::{Ref, RefCell, RefMut},
	collections::{HashMap, HashSet, VecDeque},
	rc::{Rc, Weak},
};

use wasm_bindgen::prelude::*;

type Callback<T> = dyn FnMut(T, T);

pub struct Signal<T>
where
	T: Copy,
{
	pub value: RefCell<Rc<T>>,
	subscribers: Vec<Box<Callback<T>>>,
}

impl<T> Signal<T>
where
	T: Copy,
{
	pub fn get(&self) -> Rc<T> {
		self.value.borrow().clone()
	}
	pub fn get_untracked(&self) -> Rc<T> {
		self.value.borrow().clone()
	}
	pub fn set(&mut self, value: T) {
		let old_value = self.value.borrow().clone();
		self.value.replace(Rc::new(value));
		self.dispatch(*old_value, value);
	}
	fn dispatch(&mut self, old_value: T, new_value: T) {
		let subs = self.subscribers.iter_mut();
		for sub in subs {
			sub(old_value, new_value);
		}
		// self.subscribers.replace_with(subs);
	}
	pub fn subscribe(&mut self, cb: impl FnMut(T, T) + 'static) -> usize {
		self.subscribers.push(Box::new(cb));
		self.subscribers.len() - 1
	}
	pub fn unsubscribe(&mut self, id: usize) {
		self.subscribers.remove(id);
	}
}

pub fn create_signal<T>(value: T) -> Signal<T>
where
	T: Copy,
{
	Signal {
		value: RefCell::new(Rc::new(value.clone())),
		subscribers: Vec::new(),
	}
}

// let sig: Signal<i32> = create_signal(0);

// sig.get();
// sig.set();

// sig.get_untracked();
// sig.untracked().get();

// create_effect(|_| {
// 	if sig.get() {
// 		console.log(sig2.get());
// 	}
// }, [sig2]);
