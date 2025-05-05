pub use runner::*;
pub use schedule::*;
pub use module::*;

mod runner;
mod schedule;
mod module;

pub use glued_macros::ModularApp;

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

	fn run<L: ScheduleLabel>(&mut self);
}

#[cfg(test)]
mod test {

	#[test]
	fn module_communication() {
		use crate::{Module, ModularApp, ScheduleLabel, System, With};

		#[derive(Module, Default)]
		struct ModuleA(u32);

		#[derive(Module, Default)]
		struct ModuleB(u32);

		#[derive(ScheduleLabel)]
		struct Update;
		
		impl<A> System<Update, A> for ModuleA
		where A: ModularApp + With<ModuleB> {
			fn run(app: &mut A) {
				app.module_mut::<ModuleB>()
					.0 = 2;
			}
		}

		impl<A> System<Update, A> for ModuleB
		where A: ModularApp + With<Self> + With<ModuleA> {
			fn run(app: &mut A) {
				app.module_mut::<Self>().0 += 10;
				app.module_mut::<ModuleA>().0 = 1;
			}
		}

		#[derive(ModularApp, Default)]
		struct App(ModuleA, ModuleB);

		let mut app = App::default();
		app.run::<Update>();
		
		assert_eq!(app.module::<ModuleA>().0, 1u32);
		assert_eq!(app.module::<ModuleB>().0, 12u32);
	}

	#[test]
	fn generic_modules() {
		use crate::{ModularApp, Module};

		#[derive(Module)]
		struct ModuleA<'a, T> {
			handle: &'a T
		}

		#[derive(ModularApp)]
		struct App<'a>(ModuleA<'a, u32>);

		let foo = 1;
		let app = App(ModuleA { handle: &foo });
		assert_eq!(*app.module::<ModuleA<'_, u32>>().handle, 1);
	}
}
