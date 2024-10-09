mod internal;

#[proc_macro_attribute]
pub fn optional(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    internal::opt_impl(attr.into(), item.into()).into()
}
