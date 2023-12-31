// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
                pub(crate) struct Handle {
                    pub(crate) conf: crate::Config,
                    #[allow(dead_code)] // unused when a service does not provide any operations
                    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
                }

                
/// 
/// An ergonomic client for the service.
/// 
/// This client allows ergonomic access to the service.
/// Each method corresponds to an API defined in the service's Smithy model,
/// and the request and response shapes are auto-generated from that same model.
/// 
/// Client for calling the service.
/// 
/// ## Constructing a `Client`
/// 
/// A `Client` requires a config in order to be constructed. With the default set of Cargo features,
/// this config will only require an endpoint to produce a functioning client. However, some Smithy
/// features will require additional configuration. For example, `@auth` requires some kind of identity
/// or identity resolver to be configured. The config is used to customize various aspects of the client,
/// such as:
/// 
///   - [HTTP Connector](crate::config::Builder::http_connector)
///   - [Retry](crate::config::Builder::retry_config)
///   - [Timeouts](crate::config::Builder::timeout_config)
///   - [... and more](crate::config::Builder)
/// 
/// Below is a minimal example of how to create a client:
/// 
/// ```rust,no_run
/// let config = storage::Config::builder()
///     .endpoint_url("http://localhost:1234")
///     .build();
/// let client = storage::Client::from_conf(config);
/// ```
/// 
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should be done
/// once at application start-up. Cloning a client is cheap (it's just an [`Arc`](std::sync::Arc) under the hood),
/// so creating it once at start-up and cloning it around the application as needed is recommended.
/// # Using the `Client`
/// 
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`AbortMultipartUpload`](crate::operation::abort_multipart_upload) operation has
/// a [`Client::abort_multipart_upload`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
/// 
/// ```rust,ignore
/// let result = client.abort_multipart_upload()
///     .bucket("example")
///     .send()
///     .await;
/// ```
/// 
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
                #[derive(::std::clone::Clone, ::std::fmt::Debug)]
                pub struct Client {
                    handle: ::std::sync::Arc<Handle>,
                }

                impl Client {
                    /// Creates a new client from the service [`Config`](crate::Config).
                    ///
                    /// # Panics
                    ///
                    /// This method will panic in the following cases:
                    ///
                    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
                    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
                    /// - No `behavior_version` is provided.
                    ///
                    /// The panic message for each of these will have instructions on how to resolve them.
                    #[track_caller]
                    pub fn from_conf(conf: crate::Config) -> Self {
                        let handle = Handle {
                            conf: conf.clone(),
                            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
                        };
                        if let Err(err) = Self::validate_config(&handle) {
                            panic!("Invalid client configuration: {err}");
                        }
                        Self {
                            handle: ::std::sync::Arc::new(handle)
                        }
                    }

                    /// Returns the client's configuration.
                    pub fn config(&self) -> &crate::Config {
                        &self.handle.conf
                    }

                    fn validate_config(handle: &Handle) -> Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
                        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
                        handle.runtime_plugins
                            .apply_client_configuration(&mut cfg)?
                            .validate_base_client_config(&cfg)?;
                        Ok(())
                    }
                }

mod abort_multipart_upload;

mod complete_multipart_upload;

/// Operation customization and supporting types.
/// 
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
/// 
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), storage::Error> {
/// # let client: storage::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
/// 
/// let result = client.abort_multipart_upload()
///     .customize()
///     .mutate_request(|req| {
///         // Add `x-example-header` with value
///         req.headers_mut()
///             .insert(
///                 HeaderName::from_static("x-example-header"),
///                 HeaderValue::from_static("1"),
///             );
///     })
///     .send()
///     .await;
/// # }
/// ```
pub mod customize;

mod delete_bucket;

mod delete_bucket_cors;

mod delete_bucket_lifecycle;

mod delete_bucket_policy;

mod delete_bucket_tagging;

mod delete_multiple_objects;

mod delete_object;

mod delete_object_tagging;

mod get_bucket;

mod get_bucket_acl;

mod get_bucket_consistency;

mod get_bucket_cors;

mod get_bucket_lifecycle_configuration;

mod get_bucket_object_versions;

mod get_bucket_policy;

mod get_bucket_tagging;

mod get_bucket_version2;

mod get_bucket_versioning;

mod get_object;

mod get_object_acl;

mod get_object_tagging;

mod get_service;

mod head_bucket;

mod head_object;

mod initiate_multipart_upload;

mod list_multipart_uploads;

mod list_parts;

mod put_bucket;

mod put_bucket_consistency;

mod put_bucket_cors;

mod put_bucket_lifecycle_configuration;

mod put_bucket_policy;

mod put_bucket_tagging;

mod put_bucket_versioning;

mod put_object;

mod put_object_copy;

mod put_object_tagging;

mod upload_part;

mod upload_part_copy;

