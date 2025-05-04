use crate::ModularApp;

pub trait AppRunner {
	type Context;

	fn run<A>()
	where A: ModularApp + From<Self::Context>;
}
