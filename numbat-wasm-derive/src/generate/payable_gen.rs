use super::util::*;
use crate::model::{Method, MethodArgument, MethodPayableMetadata};

pub fn generate_payable_snippet(m: &Method) -> proc_macro2::TokenStream {
	payable_snippet_for_metadata(m.payable_metadata(), &m.payment_arg(), &m.token_arg())
}

fn payable_snippet_for_metadata(
	mpm: MethodPayableMetadata,
	payment_arg: &Option<MethodArgument>,
	token_arg: &Option<MethodArgument>,
) -> proc_macro2::TokenStream {
	match mpm {
		MethodPayableMetadata::NoMetadata => quote! {},
		MethodPayableMetadata::NotPayable => {
			let payment_init = if let Some(arg) = payment_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = BigUint::zero();
				}
			} else {
				quote! {}
			};
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::rewa();
				}
			} else {
				quote! {}
			};
			quote! {
				self.call_value().check_not_payable();
				#payment_init
				#token_init
			}
		},
		MethodPayableMetadata::Rewa => {
			let payment_var_name = var_name_or_underscore(payment_arg);
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::rewa();
				}
			} else {
				quote! {}
			};
			quote! {
				let #payment_var_name = self.call_value().require_rewa();
				#token_init
			}
		},
		MethodPayableMetadata::SingleDcdtToken(token_name) => {
			let token_literal = byte_str_slice_literal(token_name.as_bytes());
			let payment_var_name = var_name_or_underscore(payment_arg);
			let token_init = if let Some(arg) = token_arg {
				let pat = &arg.pat;
				quote! {
					let #pat = TokenIdentifier::from(#token_literal);
				}
			} else {
				quote! {}
			};
			quote! {
				let #payment_var_name = self.call_value().require_dcdt(#token_literal);
				#token_init
			}
		},
		MethodPayableMetadata::AnyToken => {
			if payment_arg.is_none() && token_arg.is_none() {
				quote! {}
			} else {
				let payment_var_name = var_name_or_underscore(payment_arg);
				let token_var_name = var_name_or_underscore(token_arg);
				quote! {
					let (#payment_var_name, #token_var_name) = self.call_value().payment_token_pair();
				}
			}
		},
	}
}

fn var_name_or_underscore(opt_arg: &Option<MethodArgument>) -> proc_macro2::TokenStream {
	if let Some(arg) = opt_arg {
		let pat = &arg.pat;
		quote! { #pat }
	} else {
		quote! { _ }
	}
}