use proc_macro::TokenStream;

mod html_template;
mod preview;

#[proc_macro]
pub fn html_template(input: TokenStream) -> TokenStream {
    html_template::expand(input)
}

#[proc_macro_attribute]
pub fn preview(attr: TokenStream, item: TokenStream) -> TokenStream {
    preview::expand(attr, item)
}
