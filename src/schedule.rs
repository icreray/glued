pub use glued_macros::ScheduleLabel;

use crate::{module::Module, ModularApp};


/// # Safety
/// Should be implemented via `#[derive(ScheduleLabel)]`
pub unsafe trait ScheduleLabel {}


pub trait System<L, A>: Module
where L: ScheduleLabel, A: ModularApp {

	#[allow(unused_variables)]
	fn run(app: &mut A);
}


impl<L, A, M> System<L, A> for M
where 
	L: ScheduleLabel,
	A: ModularApp,
	M: Module {
		default fn run(_app: &mut A) {}
}
