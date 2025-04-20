pub use glued_macros::Module;

pub unsafe trait Module: Default {}

pub trait With<M: Module> {
	fn get(&self) -> &M;
	fn get_mut(&mut self) -> &mut M;
}

impl<M: Module> With<M> for M {
	#[inline(always)]
	fn get(&self) -> &Self { self }
	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self { self }
}
