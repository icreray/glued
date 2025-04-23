pub mod module;

pub use glued_macros::{ModularApp, module_impl};
use crate::module::{Module, With};

pub unsafe trait ModularApp {
	fn module<M: Module>(&self) -> &M
	where Self: With<M> {
		With::<M>::get(self)
	}
	fn module_mut<M: Module>(&mut self) -> &mut M
	where Self: With<M> {
		With::<M>::get_mut(self)
	}
}

#[cfg(test)]
mod test {

	#[test]
	fn module_communication() {
		use glued_macros::module_impl;
		use crate::{Module, ModularApp};

		#[derive(Module, Default)]
		struct A(u32);

		#[derive(Module, Default)]
		struct B(u32);

		
		#[module_impl(T)]
		impl A {
			#[requires(B)]
			pub fn update(app: &mut T) {
				app.module_mut::<B>().0 = 2;
			}
		}

		#[module_impl(T)]
		impl B {
			#[requires(Self, A)]
			pub fn update(app: &mut T) {
				app.module_mut::<Self>().0 += 10;
				app.module_mut::<A>().0 = 1;
			}
		}

		#[derive(ModularApp)]
		struct App(A, B);

		let mut app = App(A::default(), B::default());
		app.update();
		
		assert_eq!(app.module::<A>().0, 1u32);
		assert_eq!(app.module::<B>().0, 12u32);
	}
}
