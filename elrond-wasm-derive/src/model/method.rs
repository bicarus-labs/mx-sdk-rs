use super::*;

/// Indicates how smart contract method implementations will be auto-generated based on their annotations.
#[derive(Clone, Debug)]
pub enum AutoImpl {
	LegacyEvent { identifier: Vec<u8> },
	Event { identifier: String },
	StorageGetter { identifier: String },
	StorageSetter { identifier: String },
	StorageMapper { identifier: String },
	StorageIsEmpty { identifier: String },
	StorageClear { identifier: String },
	Module { impl_path: proc_macro2::TokenTree },
}
#[derive(Clone, Debug)]
pub enum MethodImpl {
	/// Implementation auto-generated by the framework. There can (obviously) be only one per method.
	Generated(AutoImpl),

	/// Methods where the developer has provided an explicit implementation.
	Explicit(syn::Block),

	/// Methods that have no implementation and are not annotated as such.
	/// They are not allowed in contracts and modules, but they are used in call proxies.
	NoImplementation,
}

/// Models any method argument from a contract, module or callable proxy trait.
#[derive(Clone, Debug)]
pub struct Method {
	pub docs: Vec<String>,
	pub public_role: PublicRole,
	pub name: syn::Ident,
	pub generics: syn::Generics,
	pub remaining_attributes: Vec<syn::Attribute>,
	pub method_args: Vec<MethodArgument>,
	pub output_names: Vec<String>,
	pub return_type: syn::ReturnType,
	pub implementation: MethodImpl,
}

impl Method {
	pub fn payment_arg(&self) -> Option<MethodArgument> {
		self.method_args
			.iter()
			.find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::Payment))
			.cloned()
	}

	pub fn token_arg(&self) -> Option<MethodArgument> {
		self.method_args
			.iter()
			.find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentToken))
			.cloned()
	}

	pub fn is_payable(&self) -> bool {
		match &self.public_role {
			PublicRole::Init(init_metadata) => init_metadata.payable.is_payable(),
			PublicRole::Endpoint(endpoint_metadata) => endpoint_metadata.payable.is_payable(),
			PublicRole::Callback(_) | PublicRole::CallbackRaw => true,
			PublicRole::Private => false,
		}
	}

	pub fn payable_metadata(&self) -> MethodPayableMetadata {
		match &self.public_role {
			PublicRole::Init(init_metadata) => init_metadata.payable.clone(),
			PublicRole::Endpoint(endpoint_metadata) => endpoint_metadata.payable.clone(),
			PublicRole::Callback(_) | PublicRole::CallbackRaw => MethodPayableMetadata::AnyToken,
			PublicRole::Private => MethodPayableMetadata::NotPayable,
		}
	}

	/// Returns Some with the endpoint name as `String` if the method is public.
	/// None if the method is not [ublic.]
	pub fn endpoint_name(&self) -> Option<String> {
		match &self.public_role {
			PublicRole::Init(_) => Some("init".to_string()),
			PublicRole::Endpoint(endpoint_metadata) => {
				Some(endpoint_metadata.public_name.to_string())
			},
			_ => None,
		}
	}

	pub fn has_variable_nr_args(&self) -> bool {
		self.method_args.iter().any(|arg| arg.metadata.var_args)
	}

	pub fn is_module(&self) -> bool {
		matches!(
			self.implementation,
			MethodImpl::Generated(AutoImpl::Module { .. })
		)
	}
}
