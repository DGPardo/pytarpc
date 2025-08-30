///! Procedural macro to generate Python bindings for tarpc services
///! 
///! Usage:
///! ```rust
///! #[tarpc_python_client]
///! #[tarpc::service]
///! trait RpcAPI {
///!     async fn hello(name: String) -> String;
///!     async fn add(a: i32, b: i32) -> i32;
///!     async fn get_user(id: u64) -> Option<User>;
///! }
///! ```

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemTrait, Pat};


#[proc_macro_attribute]
pub fn tarpc_python_client(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input_trait = parse_macro_input!(input as ItemTrait);
    
    let trait_name = &input_trait.ident;
    let client_name = format!("{}Client", trait_name);
    let py_client_name = format!("Py{}", client_name);
    
    let client_ident = Ident::new(&client_name, Span::call_site());
    let py_client_ident = Ident::new(&py_client_name, Span::call_site());
    
    // Generate method implementations
    let method_impls = input_trait.items.iter().filter_map(|item| {
        if let syn::TraitItem::Fn(method) = item {
            generate_python_method(method)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    
    let expanded = quote! {
        // Keep the original trait
        #input_trait
        
        // Generate the Python client wrapper
        #[pyo3::pyclass]
        pub struct #py_client_ident {
            client: #client_ident,
        }
        
        #[pyo3::pymethods]
        impl #py_client_ident {
            /// Connect to the RPC server
            #[staticmethod]
            fn connect<'p>(py: pyo3::Python<'p>, address: String) -> pyo3::PyResult<pyo3::Bound<'p, pyo3::PyAny>> {
                pyo3_async_runtimes::tokio::future_into_py(py, async move {
                    use std::net::SocketAddr;
                    use tarpc::{client, serde_transport::tcp};
                    use tokio_serde::formats::Bincode;
                    
                    let addr: SocketAddr = address.parse().map_err(|e| {
                        pyo3::PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid address: {}", e))
                    })?;

                    let transport = tcp::connect(addr, Bincode::default).await.map_err(|e| {
                        pyo3::PyErr::new::<pyo3::exceptions::PyConnectionError, _>(format!(
                            "Failed to connect: {}",
                            e
                        ))
                    })?;

                    let client = #client_ident::new(client::Config::default(), transport).spawn();

                    Ok(#py_client_ident { client })
                })
            }
            
            #(#method_impls)*
        }
    };
    
    TokenStream::from(expanded)
}

fn generate_python_method(method: &syn::TraitItemFn) -> Option<proc_macro2::TokenStream> {
    let method_name = &method.sig.ident;
    
    // Extract parameters (skip &self and context)
    let params = method.sig.inputs.iter()
        .filter_map(|arg| {
            if let FnArg::Typed(pat_type) = arg {
                if let Pat::Ident(pat_ident) = &*pat_type.pat {
                    let param_name = &pat_ident.ident;
                    let param_type = &*pat_type.ty;
                    Some((param_name.clone(), param_type.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    
    // Generate parameter list for Python method
    let py_params = params.iter().map(|(name, ty)| {
        quote! { #name: #ty }
    });
    
    // Generate parameter list for RPC call
    let rpc_params = params.iter().map(|(name, _)| name);

    Some(quote! {
        fn #method_name<'p>(&self, py: pyo3::Python<'p>, #(#py_params),*) -> pyo3::PyResult<pyo3::Bound<'p, pyo3::PyAny>> {
            let client = self.client.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                let response = client.#method_name(tarpc::context::current(), #(#rpc_params),*).await;
                match response {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        let error_msg = format!("{:?}", e);
                        Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(error_msg))
                    }
                }
            })
        }
    })
}

// Helper macro for generating the complete setup
#[proc_macro]
pub fn setup_tarpc_python_module(input: TokenStream) -> TokenStream {
    let module_name = parse_macro_input!(input as syn::Ident);
    
    let expanded = quote! {
        use pyo3::prelude::*;
        
        #[pyo3::pymodule]
        fn #module_name(m: &Bound<'_, PyModule>) -> PyResult<()> {
            // Auto-register all PyRpcClient classes
            // This would need to be manually updated or use inventory crate
            // for automatic registration
            Ok(())
        }
    };
    
    TokenStream::from(expanded)
}