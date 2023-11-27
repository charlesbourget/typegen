use anyhow::{Context, Result};
use openapiv3::OpenAPI;
use progenitor::{GenerationSettings, Generator};
use quote::__private::TokenStream;
use quote::quote;
use std::fs;

pub fn generate_code(spec_file_path: &str, generation_settings: &GenerationSettings) -> Result<String> {
    let tokens = generate_tokens(spec_file_path, generation_settings)?;

    let ast = syn::parse2(tokens).unwrap();
    let content = prettyplease::unparse(&ast);

    Ok(content)
}

fn generate_tokens(spec_file_path: &str, generation_settings: &GenerationSettings) -> Result<TokenStream> {
    let spec_data = parse_openapi_spec(&spec_file_path)?;

    let mut generator = Generator::new(generation_settings);
    generator
        .generate_tokens(&spec_data)
        .context("Could not generate tokens")?;
    let type_space = generator.get_type_space();

    let types = type_space.to_stream();

    let tokens = quote! {
        pub mod types {
            use serde::{Deserialize, Serialize};

            #[allow(unused_imports)]
            use std::convert::TryFrom;

            #types
        }
    };

    Ok(tokens)
}

fn parse_openapi_spec(spec_file_path: &str) -> Result<OpenAPI> {
    let spec_data = fs::read_to_string(spec_file_path).context("Could not read spec file")?;
    let openapi: OpenAPI = serde_yaml::from_str(&spec_data).context("Could not parse spec file")?;

    Ok(openapi)
}
