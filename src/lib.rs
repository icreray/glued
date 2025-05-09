pub use module::*;
pub use runner::*;

mod module;
mod runner;

pub use glued_macros::*;

/// # Safety
/// Should be only implemented via `#[derive(ModularApp)]`
pub unsafe trait ModularApp {
	#[must_use]
	fn module<M: Module>(&self) -> &M
	where Self: With<M> {
		With::<M>::get(self)
	}

	#[must_use]
	fn module_mut<M: Module>(&mut self) -> &mut M
	where Self: With<M> {
		With::<M>::get_mut(self)
	}

// Schedule
	fn setup(&mut self);
	fn update(&mut self);
}

#[cfg(test)]
mod test {
	#[test]
	fn module_communication() {
		use crate::{ModularApp, module_impl};

		#[derive(Default)]
		struct A(u32);

		#[derive(Default)]
		struct B(u32);
		
		#[module_impl(T)]
		#[dependencies(B)]
		impl A {
			pub fn update(app: &mut T) {
				app.module_mut::<B>().0 = 2;
			}
		}

		#[module_impl(T)]
		#[dependencies(Self, A)]
		impl B {
			pub fn update(app: &mut T) {
				app.module_mut::<Self>().0 += 10;
				app.module_mut::<A>().0 = 1;
			}
		}

		#[derive(ModularApp, Default)]
		struct App(A, B);

		let mut app = App::default();
		app.update();
		
		assert_eq!(app.module::<A>().0, 1u32);
		assert_eq!(app.module::<B>().0, 12u32);
	}

	#[test]
	fn generic_modules() {
		use crate::{ModularApp, module_impl};

		struct ModuleA<'a, T> {
			handle: &'a T
		}

		#[module_impl(A)]
		impl<'a, T> ModuleA<'a, T> {}

		#[derive(ModularApp)]
		struct App<'a>(ModuleA<'a, u32>);

		let foo = 1;
		let app = App(ModuleA { handle: &foo });
		assert_eq!(*app.module::<ModuleA<'_, u32>>().handle, 1);
	}
}
