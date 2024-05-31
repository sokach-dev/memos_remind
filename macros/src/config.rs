// use proc_macro::TokenStream;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn config(args: TokenStream, input: TokenStream) -> TokenStream {
    let config_env = parse_macro_input!(args as syn::LitStr);
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_name = &derive_input.ident;

    let gen = quote! {
        use tokio::{fs, sync::OnceCell};
        use std::{env, str::FromStr, sync::Arc};

        #derive_input

        impl FromStr for #struct_name {
            type Err = toml::de::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                    toml::from_str(s)
            }
        }

        static GLOBAL_CONFIG: OnceCell<Arc<#struct_name>> = OnceCell::const_new();

        pub async fn get_global_config() -> &'static Arc<#struct_name> {
            let config_url = env::var(#config_env).expect(&format!("{} is not set, check env {}", #config_env, #config_env));
            GLOBAL_CONFIG
                .get_or_init(|| async {
                    Arc::new(
                        fs::read_to_string(config_url)
                            .await
                            .unwrap()
                            .parse()
                            .unwrap(),
                    )
                })
                .await
        }
    };

    gen.into()
}
