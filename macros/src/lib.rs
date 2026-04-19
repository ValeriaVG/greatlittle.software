use proc_macro::TokenStream;

mod html_template;

#[proc_macro]
pub fn html_template(input: TokenStream) -> TokenStream {
    html_template::expand(input)
}
