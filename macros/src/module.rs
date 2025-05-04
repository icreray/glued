use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
	parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned,
	Attribute, ImplItem, ImplItemFn, ItemImpl, Token, Type, Visibility,
	WherePredicate
};

use crate::utils::{spanned_error, paths::*};

const REQUIRES_ATTR: &str = "requires";
const MODULE_IMPL_FUNCTIONS: [&str; 2] = ["setup", "update"];

pub(crate) fn expand_module_impl(
	generic_ty: Ident,
	mut impl_block: ItemImpl
) -> syn::Result<TokenStream2> {

	let crate_name = glued_crate_name();
	let modular_app_trait = modular_app_trait(&crate_name);
	let with_trait = with_trait(&crate_name);

	let mut required_fns: HashSet<&str> = MODULE_IMPL_FUNCTIONS.into();

	for func in impl_block.items
		.iter_mut()
		.filter_map(fn_item_mut) {
			validate_function_declaration(func, &mut required_fns)?;
			// fn foo(...) => fn foo<T: ModularApp>(...)
			func.sig.generics.params.push(parse_quote!(#generic_ty: #modular_app_trait));
			add_module_bounds(func, &generic_ty, &with_trait)?;
	}

	impl_block.items.extend(
		create_missing_functions(required_fns, &generic_ty)
	);

	let module_impl = derive_module(&impl_block, &crate_name);

	Ok(quote! {
		#module_impl
		#impl_block
	})
}

fn derive_module(
	impl_block: &ItemImpl, 
	crate_name: &TokenStream2
) -> TokenStream2 {
	let module_trait = module_trait(&crate_name);
	let ident = &impl_block.self_ty;
	let (impl_generics, _, where_clause) = impl_block.generics.split_for_impl();
	quote! {
		unsafe impl #impl_generics #module_trait for #ident #where_clause {}
	}
}

fn validate_function_declaration(
	func: &ImplItemFn, 
	required_fns: &mut HashSet<&str>
) -> syn::Result<()> {
	let fn_name = func.sig.ident.to_string();
	if !required_fns.remove(fn_name.as_str()) {
		return spanned_error!(
			func,
			format!("#[module_impl(T)] annotated impl block may contain only specific functions: {:?}", MODULE_IMPL_FUNCTIONS)
		);
	}
	if !matches!(func.vis, Visibility::Public(_)) {
		spanned_error!(
			func,
			"#[module_impl] functions must be public"
		)
	} else {
		Ok(())
	}
}

fn add_module_bounds(
	func: &mut ImplItemFn, 
	generic_ty: &Ident,
	with_trait: &TokenStream2
) -> syn::Result<()> {
	let Some(required_attr) = take_attr(&mut func.attrs, REQUIRES_ATTR) else {
		return Ok(());
	};
	let module_types = required_attr.parse_args_with(Punctuated::<Type, Token![,]>::parse_terminated)?;
	func.sig
		.generics
		.make_where_clause()
		.predicates
		.extend(
			module_types.into_iter().map(|m| -> WherePredicate {
				parse_quote_spanned!(m.span()=>
						#generic_ty: #with_trait<#m>
				)
			})
		);
	Ok(())
}

fn create_missing_functions(
	missing_fns: HashSet<&str>,
	generic_ty: &Ident
) -> impl Iterator<Item = ImplItem> {
	missing_fns.into_iter().map(move |fn_name| {
		let ident = Ident::new(fn_name, Span::call_site());
		parse_quote! {
			#[allow(dead_code)]
			pub fn #ident<#generic_ty>(_app: &mut #generic_ty) {}
		}
	})
}

fn fn_item_mut(item: &mut ImplItem) -> Option<&mut ImplItemFn> {
	match item {
		ImplItem::Fn(func) => Some(func),
		_ => None
	}
}

fn take_attr(attrs: &mut Vec<Attribute>, name: &str) -> Option<Attribute> {
	let pos = attrs.iter().position(|attr| attr.path().is_ident(name))?;
	Some(attrs.remove(pos))
}
