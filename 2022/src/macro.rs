use proc_macro::TokenStream;

#[proc_macro]
pub fn make_problem_import(_item: TokenStream) -> TokenStream {
    format!(
        "use problem_{0}::run as run{0};

mod problem_{0} {{
    pub mod run;
}}",
        _item.to_string()
    )
    .parse()
    .unwrap()
}
