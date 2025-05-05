pub use glued_macros::Module;

/// # Safety
/// Should be only implemented via `#[module_impl(T)]`
pub unsafe trait Module {}

pub trait With<M: Module> {
	#[must_use]
	fn get(&self) -> &M;
	#[must_use]
	fn get_mut(&mut self) -> &mut M;
}

impl<M: Module> With<M> for M {
	#[inline(always)]
	fn get(&self) -> &Self { self }
	#[inline(always)]
	fn get_mut(&mut self) -> &mut Self { self }
}
