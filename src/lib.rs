//! A macro that generates new structs from the original with each field made optional.
//!
//! # Example
//!
//! ## Basic
//!
//! Add the `#[optional]` and `#[nested]` attributes as follows:
//!
//! ```
//! use umbra::optional;
//!
//! #[optional]
//! #[derive(Default)]
//! struct Foo {
//!   id: u32,
//!   name: String,
//!   #[nested]
//!   bar: Bar,
//! }
//!
//! #[optional]
//! #[derive(Default)]
//! struct Bar {
//!   name: String,
//!   value: Option<i32>,
//! }
//! ```
//!
//! The macro generates following structs:
//!
//! ```
//! # #[derive(Default)]
//! # struct Foo {
//! #   id: u32,
//! #   name: String,
//! #   bar: Bar,
//! # }
//! #
//! # #[derive(Default)]
//! # struct Bar {
//! #   name: String,
//! #   value: Option<i32>,
//! # }
//! #
//! struct OptionalFoo {
//!   id: Option<u32>,
//!   name: Option<String>,
//!   bar: Option<OptionalBar>,
//! }
//!
//! impl From<OptionalFoo> for Foo {
//!   fn from(optional: OptionalFoo) -> Self {
//!       let mut base = Self::default(); // create base values by default
//!       if let Some(value) = optional.id {
//!           base.id = value; // simple field
//!       }
//!       if let Some(value) = optional.bar {
//!           base.bar = value.into(); // nested field
//!       }
//!       // ...
//!       base
//!   }
//! }
//!
//! struct OptionalBar {
//!   name: Option<String>,
//!   value: Option<i32>,
//! }
//!
//! impl From<OptionalBar> for Bar {
//!   fn from(optional: OptionalBar) -> Self {
//!       let mut base = Self::default();
//!       // ...
//!       base
//!   }
//! }
//! ```
//!
//! ## Derives
//!
//! By using the `derives` attribute, the derive attribute can be added to the generated struct:
//!
//! ```
//! use umbra::optional;
//!
//! #[optional(derives = [Debug])]
//! #[derive(Default)]
//! struct Bar {
//!   name: String,
//!   value: Option<i32>,
//! }
//! ```
//!
//! The macro generates following structs:
//!
//! ```
//! # #[derive(Default)]
//! # struct Bar {
//! #   name: String,
//! #   value: Option<i32>,
//! # }
//! #
//! #[derive(Debug)] // The derive attribute is added
//! struct OptionalBar {
//!   name: Option<String>,
//!   value: Option<i32>,
//! }
//!
//! impl From<OptionalBar> for Bar {
//!   fn from(optional: OptionalBar) -> Self {
//!       let mut base = Self::default();
//!       // ...
//!       base
//!   }
//! }
//! ```
//!
//! ## Prefix / Suffix
//!
//! By using the `prefix` and `suffix` attributes, the prefix and suffix of the generated struct are changed:
//!
//! ```
//! use umbra::optional;
//!
//! #[optional(prefix = "Pre", suffix = "Suf")]
//! #[derive(Default)]
//! struct Foo {
//!   id: u32,
//!   name: String,
//! }
//! ```
//!
//! The macro generates following structs:
//!
//! ```
//! # #[derive(Default)]
//! # struct Foo {
//! #   id: u32,
//! #   name: String,
//! # }
//! #
//! struct PreFooSuf { // <Prefix><Base name><Suffix> format
//!   id: Option<u32>,
//!   name: Option<String>,
//! }
//!
//! impl From<PreFooSuf> for Foo {
//!   fn from(optional: PreFooSuf) -> Self {
//!       let mut base = Self::default();
//!       // ...
//!       base
//!   }
//! }
//! ```

mod internal;

#[proc_macro_attribute]
pub fn optional(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    internal::opt_impl(attr.into(), item.into()).into()
}
