use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use syn::{
	Attribute, DeriveInput, ImplItem, ImplItemFn, ItemImpl, Token, Type, WherePredicate,
	parse_quote, parse_quote_spanned, punctuated::Punctuated, spanned::Spanned
};

use crate::utils::{VisibilityExt, spanned_error};

const REQUIRES_ATTR: &str = "requires";

pub(crate) fn derive(ast: DeriveInput) -> TokenStream2 {
	let generics = ast.generics;
	let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
	let struct_ident = &ast.ident;
	quote! {
		unsafe impl #impl_generics Module for #struct_ident #ty_generics #where_clause {}
	}
}

pub(crate) fn module_impl(
	generic_ty: Ident,
	mut impl_block: ItemImpl
) -> syn::Result<TokenStream2> {
	for func in impl_block.items.iter_mut().filter_map(fn_item) {
		check_function_declaration(func)?;
		// fn foo(...) => fn foo<T: ModularApp>(...)
		func.sig.generics.params.push(parse_quote!(#generic_ty: ModularApp));
		add_required_module_bounds(func, &generic_ty)?;
	}
	Ok(impl_block.to_token_stream())
}

fn check_function_declaration(func: &ImplItemFn) -> syn::Result<()> {
	if !func.vis.is_public() {
		spanned_error!(
			func,
			"#[module_impl] annotated impl block may contain only public functions"
		)
	} else {
		Ok(())
	}
}

fn fn_item(item: &mut ImplItem) -> Option<&mut ImplItemFn> {
	match item {
		ImplItem::Fn(func) => Some(func),
		_ => None
	}
}

fn add_required_module_bounds(func: &mut ImplItemFn, generic_ty: &Ident) -> syn::Result<()> {
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
						#generic_ty: With<#m>
				)
			})
		);
	Ok(())
}

fn take_attr(attrs: &mut Vec<Attribute>, name: &str) -> Option<Attribute> {
	let pos = attrs.iter().position(|attr| attr.path().is_ident(name))?;
	Some(attrs.remove(pos))
}
