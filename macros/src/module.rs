use std::collections::HashSet;

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
	parse_quote, punctuated::Punctuated, Attribute, ImplItem, ImplItemFn, ItemImpl, 
	Token, Type, TypeParam, TypeParamBound, Visibility
};

use crate::utils::{paths::*, spanned_error};

const REQUIRES_ATTR: &str = "requires";
const MODULE_IMPL_FUNCTIONS: [&str; 2] = ["setup", "update"];

pub(crate) fn expand_module_impl(
	generic_ty: Ident,
	mut impl_block: ItemImpl
) -> syn::Result<TokenStream2> {

	let crate_name = glued_crate_name();
	let param = create_param(&mut impl_block, &generic_ty, &crate_name)?;

	let mut required_fns: HashSet<&str> = MODULE_IMPL_FUNCTIONS.into();

	for func in impl_block.items
		.iter_mut()
		.filter_map(fn_item_mut) {
			validate_function_declaration(func, &mut required_fns)?;
			// Add param to each functon cause it can't be added to impl block level
			func.sig.generics.params.push(param.clone().into());
	}

	impl_block.items.extend(
		create_missing_functions(required_fns, &param)
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
	let module_trait = module_trait(crate_name);
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

fn create_param(
	impl_block: &mut ItemImpl,
	generic_ty: &Ident,
	crate_name: &TokenStream2
) -> syn::Result<TypeParam> {
	let modular_app_trait = modular_app_trait(crate_name);
	let with_trait = with_trait(crate_name);

	let mut param: TypeParam = parse_quote!(#generic_ty: #modular_app_trait);
	let Some(requires_attr) = take_attr(&mut impl_block.attrs, REQUIRES_ATTR) else {
		return Ok(param);
	};

	let module_types = requires_attr
		.parse_args_with(Punctuated::<Type, Token![,]>::parse_terminated)?;

	param.bounds.extend(
		module_types
			.into_iter()
			.map(|m| -> TypeParamBound { parse_quote!(#with_trait<#m>) })
	);
	Ok(param)
}

fn create_missing_functions(
	missing_fns: HashSet<&str>,
	param: &TypeParam
) -> impl Iterator<Item = ImplItem> {
	let param_ident = &param.ident;
	missing_fns.into_iter().map(move |fn_name| {
		let ident = Ident::new(fn_name, Span::call_site());
		parse_quote! {
			#[allow(dead_code)]
			pub fn #ident<#param>(_app: &mut #param_ident) {}
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
