// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Orchestration and serialization glue logic for `GetObjectACL`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetObjectACL;
impl GetObjectACL {
    /// Creates a new `GetObjectACL`
    pub fn new() -> Self {
        Self
    }
    pub(crate) async fn orchestrate(
                        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
                        input: crate::operation::get_object_acl::GetObjectAclInput,
                    ) -> ::std::result::Result<crate::operation::get_object_acl::GetObjectAclOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_object_acl::GetObjectACLError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let map_err = |err: ::aws_smithy_runtime_api::client::result::SdkError<::aws_smithy_runtime_api::client::interceptors::context::Error, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>| {
                            err.map_service_error(|err| {
                                err.downcast::<crate::operation::get_object_acl::GetObjectACLError>().expect("correct error type")
                            })
                        };
                        let context = Self::orchestrate_with_stop_point(runtime_plugins, input, ::aws_smithy_runtime::client::orchestrator::StopPoint::None)
                            .await
                            .map_err(map_err)?;
                        let output = context.finalize().map_err(map_err)?;
                        ::std::result::Result::Ok(output.downcast::<crate::operation::get_object_acl::GetObjectAclOutput>().expect("correct output type"))
                    }
    
                    pub(crate) async fn orchestrate_with_stop_point(
                        runtime_plugins: &::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
                        input: crate::operation::get_object_acl::GetObjectAclInput,
                        stop_point: ::aws_smithy_runtime::client::orchestrator::StopPoint,
                    ) -> ::std::result::Result<::aws_smithy_runtime_api::client::interceptors::context::InterceptorContext, ::aws_smithy_runtime_api::client::result::SdkError<::aws_smithy_runtime_api::client::interceptors::context::Error, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = ::aws_smithy_runtime_api::client::interceptors::context::Input::erase(input);
                        ::aws_smithy_runtime::client::orchestrator::invoke_with_stop_point(
                            "storage",
                            "GetObjectACL",
                            input,
                            runtime_plugins,
                            stop_point
                        ).await
                    }
    
                    pub(crate) fn operation_runtime_plugins(
                        client_runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
                        client_config: &crate::config::Config,
                        config_override: ::std::option::Option<crate::config::Builder>,
                    ) -> ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins {
                        let mut runtime_plugins = client_runtime_plugins.with_operation_plugin(Self::new());
                        runtime_plugins = runtime_plugins
                                    .with_client_plugin(crate::auth_plugin::DefaultAuthOptionsPlugin::new(vec![::aws_smithy_runtime::client::auth::no_auth::NO_AUTH_SCHEME_ID]));
                        if let ::std::option::Option::Some(config_override) = config_override {
                            for plugin in config_override.runtime_plugins.iter().cloned() {
                                runtime_plugins = runtime_plugins.with_operation_plugin(plugin);
                            }
                            runtime_plugins = runtime_plugins.with_operation_plugin(
                                crate::config::ConfigOverrideRuntimePlugin::new(config_override, client_config.config.clone(), &client_config.runtime_components)
                            );
                        }
                        runtime_plugins
                    }
}
impl ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugin for GetObjectACL {
                fn config(&self) -> ::std::option::Option<::aws_smithy_types::config_bag::FrozenLayer> {
                    let mut cfg = ::aws_smithy_types::config_bag::Layer::new("GetObjectACL");

                    cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedRequestSerializer::new(GetObjectACLRequestSerializer));
                    cfg.store_put(::aws_smithy_runtime_api::client::ser_de::SharedResponseDeserializer::new(GetObjectACLResponseDeserializer));

                    
                    cfg.store_put(::aws_smithy_runtime_api::client::auth::AuthSchemeOptionResolverParams::new(::aws_smithy_runtime_api::client::auth::static_resolver::StaticAuthSchemeOptionResolverParams::new()));

                    cfg.store_put(::aws_smithy_http::operation::Metadata::new(
                        "GetObjectACL",
                        "storage",
                    ));

                    ::std::option::Option::Some(cfg.freeze())
                }

                fn runtime_components(&self, _: &::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder) -> ::std::borrow::Cow<'_, ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder> {
                    #[allow(unused_mut)]
                    let mut rcb = ::aws_smithy_runtime_api::client::runtime_components::RuntimeComponentsBuilder::new("GetObjectACL")
                            .with_interceptor(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptor::new(::aws_smithy_runtime::client::stalled_stream_protection::StalledStreamProtectionInterceptorKind::ResponseBody))
.with_interceptor(GetObjectACLEndpointParamsInterceptor)
                            .with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::TransientErrorClassifier::<crate::operation::get_object_acl::GetObjectACLError>::new())
.with_retry_classifier(::aws_smithy_runtime::client::retries::classifiers::ModeledAsRetryableClassifier::<crate::operation::get_object_acl::GetObjectACLError>::new());

                    ::std::borrow::Cow::Owned(rcb)
                }
            }

            
#[derive(Debug)]
            struct GetObjectACLResponseDeserializer;
            impl ::aws_smithy_runtime_api::client::ser_de::DeserializeResponse for GetObjectACLResponseDeserializer {
                

                fn deserialize_nonstreaming(&self, response: &::aws_smithy_runtime_api::client::orchestrator::HttpResponse) -> ::aws_smithy_runtime_api::client::interceptors::context::OutputOrError {
                    let (success, status) = (response.status().is_success(), response.status().as_u16());
            let headers = response.headers();
            let body = response.body().bytes().expect("body loaded");
            #[allow(unused_mut)]
            let mut force_error = false;
            
            let parse_result = if !success && status != 200 || force_error {
                crate::protocol_serde::shape_get_object_acl::de_get_object_acl_http_error(status, headers, body)
            } else {
                crate::protocol_serde::shape_get_object_acl::de_get_object_acl_http_response(status, headers, body)
            };
            crate::protocol_serde::type_erase_result(parse_result)
                }
            }
#[derive(Debug)]
            struct GetObjectACLRequestSerializer;
            impl ::aws_smithy_runtime_api::client::ser_de::SerializeRequest for GetObjectACLRequestSerializer {
                #[allow(unused_mut, clippy::let_and_return, clippy::needless_borrow, clippy::useless_conversion)]
                fn serialize_input(&self, input: ::aws_smithy_runtime_api::client::interceptors::context::Input, _cfg: &mut ::aws_smithy_types::config_bag::ConfigBag) -> ::std::result::Result<::aws_smithy_runtime_api::client::orchestrator::HttpRequest, ::aws_smithy_runtime_api::box_error::BoxError> {
                    let input = input.downcast::<crate::operation::get_object_acl::GetObjectAclInput>().expect("correct type");
                    let _header_serialization_settings = _cfg.load::<crate::serialization_settings::HeaderSerializationSettings>().cloned().unwrap_or_default();
                    let mut request_builder = {
                        fn uri_base(_input: &crate::operation::get_object_acl::GetObjectAclInput, output: &mut ::std::string::String) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    use ::std::fmt::Write as _;
    let input_1 = &_input.bucket;
    let input_1 = input_1.as_ref().ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("bucket", "cannot be empty or unset"))?;
    let bucket = ::aws_smithy_http::label::fmt_string(input_1, ::aws_smithy_http::label::EncodingStrategy::Default);
    if bucket.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field("bucket", "cannot be empty or unset"))
                }
    let input_2 = &_input.object;
    let input_2 = input_2.as_ref().ok_or_else(|| ::aws_smithy_types::error::operation::BuildError::missing_field("object", "cannot be empty or unset"))?;
    let object = ::aws_smithy_http::label::fmt_string(input_2, ::aws_smithy_http::label::EncodingStrategy::Greedy);
    if object.is_empty() {
                    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field("object", "cannot be empty or unset"))
                }
    ::std::write!(output, "/{Bucket}/{Object}", Bucket = bucket, Object = object).expect("formatting should succeed");
    ::std::result::Result::Ok(())
}
fn uri_query(_input: &crate::operation::get_object_acl::GetObjectAclInput, mut output: &mut ::std::string::String) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::BuildError> {
    let mut query = ::aws_smithy_http::query::Writer::new(output);
    query.push_v("acl");
    query.push_kv("x-id", "GetObjectACL");
    if let ::std::option::Option::Some(inner_3) = &_input.version_id {
         {
            query.push_kv("versionId", &::aws_smithy_http::query::fmt_string(&inner_3));
        }
    }
    ::std::result::Result::Ok(())
}
#[allow(clippy::unnecessary_wraps)]
fn update_http_builder(
                input: &crate::operation::get_object_acl::GetObjectAclInput,
                builder: ::http::request::Builder
            ) -> ::std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    let mut uri = ::std::string::String::new();
    uri_base(input, &mut uri)?;
    uri_query(input, &mut uri)?;
    ::std::result::Result::Ok(builder.method("GET").uri(uri))
}
let mut builder = update_http_builder(&input, ::http::request::Builder::new())?;
builder
                    };
                    let body = ::aws_smithy_types::body::SdkBody::from("");
                    
                    ::std::result::Result::Ok(request_builder.body(body).expect("valid request").try_into().unwrap())
                }
            }
#[derive(Debug)]
            struct GetObjectACLEndpointParamsInterceptor;

            impl ::aws_smithy_runtime_api::client::interceptors::Intercept for GetObjectACLEndpointParamsInterceptor {
                fn name(&self) -> &'static str {
                    "GetObjectACLEndpointParamsInterceptor"
                }

                fn read_before_execution(
                    &self,
                    context: &::aws_smithy_runtime_api::client::interceptors::context::BeforeSerializationInterceptorContextRef<'_, ::aws_smithy_runtime_api::client::interceptors::context::Input, ::aws_smithy_runtime_api::client::interceptors::context::Output, ::aws_smithy_runtime_api::client::interceptors::context::Error>,
                    cfg: &mut ::aws_smithy_types::config_bag::ConfigBag,
                ) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
                    let _input = context.input()
                        .downcast_ref::<GetObjectAclInput>()
                        .ok_or("failed to downcast to GetObjectAclInput")?;

                    

                    let params = crate::config::endpoint::Params::builder()
                        
                        .build()
                        .map_err(|err| ::aws_smithy_runtime_api::client::interceptors::error::ContextAttachedError::new("endpoint params could not be built", err))?;
                    cfg.interceptor_state().store_put(::aws_smithy_runtime_api::client::endpoint::EndpointResolverParams::new(params));
                    ::std::result::Result::Ok(())
                }
            }

/// Error type for the `GetObjectACLError` operation.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum GetObjectACLError {
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
                    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-GetObjectACLError) for what information is available for the error.")]
                    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl GetObjectACLError {
    /// Creates the `GetObjectACLError::Unhandled` variant from any error type.
                    pub fn unhandled(err: impl ::std::convert::Into<::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>>) -> Self {
                        Self::Unhandled(crate::error::sealed_unhandled::Unhandled { source: err.into(), meta: ::std::default::Default::default() })
                    }
    
                    /// Creates the `GetObjectACLError::Unhandled` variant from an [`ErrorMetadata`](::aws_smithy_types::error::ErrorMetadata).
                    pub fn generic(err: ::aws_smithy_types::error::ErrorMetadata) -> Self {
                        Self::Unhandled(crate::error::sealed_unhandled::Unhandled { source: err.clone().into(), meta: err })
                    }
    /// 
    /// Returns error metadata, which includes the error code, message,
    /// request ID, and potentially additional information.
    /// 
    pub fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(e) => &e.meta,
        }
    }
}
impl ::std::error::Error for GetObjectACLError {
    fn source(&self) -> ::std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Self::Unhandled(_inner) => {
                ::std::option::Option::Some(&*_inner.source)
            }
        }
    }
}
impl ::std::fmt::Display for GetObjectACLError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Unhandled(_inner) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                                                    write!(f, "unhandled error ({code})")
                                                } else {
                                                    f.write_str("unhandled error")
                                                }
            }
        }
    }
}
impl ::aws_smithy_types::retry::ProvideErrorKind for GetObjectACLError {
    fn code(&self) -> ::std::option::Option<&str> {
        ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self)
    }
    fn retryable_error_kind(&self) -> ::std::option::Option<::aws_smithy_types::retry::ErrorKind> {
        ::std::option::Option::None
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for GetObjectACLError {
    fn meta(&self) -> &::aws_smithy_types::error::ErrorMetadata {
        match self {
            Self::Unhandled(_inner) => {
                &_inner.meta
            }
        }
    }
}
impl ::aws_smithy_runtime_api::client::result::CreateUnhandledError for GetObjectACLError {
    fn create_unhandled_error(
                        source: ::std::boxed::Box<dyn ::std::error::Error + ::std::marker::Send + ::std::marker::Sync + 'static>,
                        meta: ::std::option::Option<::aws_smithy_types::error::ErrorMetadata>
                    ) -> Self {
        Self::Unhandled(crate::error::sealed_unhandled::Unhandled { source, meta: meta.unwrap_or_default() })
    }
}

pub use crate::operation::get_object_acl::_get_object_acl_output::GetObjectAclOutput;

pub use crate::operation::get_object_acl::_get_object_acl_input::GetObjectAclInput;

mod _get_object_acl_input;

mod _get_object_acl_output;

/// Builders
pub mod builders;

