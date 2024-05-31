mod config;

// 宏的曝光只能如下写法，不能使用pub
#[proc_macro_attribute]
pub fn config(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    config::config(args, input)
}
