// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_bucket_object_versions::_get_bucket_object_versions_output::GetBucketObjectVersionsOutputBuilder;

pub use crate::operation::get_bucket_object_versions::_get_bucket_object_versions_input::GetBucketObjectVersionsInputBuilder;

impl GetBucketObjectVersionsInputBuilder {
    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
                    > {
                        let mut fluent_builder = client.get_bucket_object_versions();
                        fluent_builder.inner = self;
                        fluent_builder.send().await
                    }
}
/// Fluent builder constructing a request to `GetBucketObjectVersions`.
/// 
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBucketObjectVersionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_bucket_object_versions::builders::GetBucketObjectVersionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput,
                    crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError,
                > for GetBucketObjectVersionsFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput,
                        crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl GetBucketObjectVersionsFluentBuilder {
    /// Creates a new `GetBucketObjectVersions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle, inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBucketObjectVersions as a reference.
    pub fn as_input(&self) -> &crate::operation::get_bucket_object_versions::builders::GetBucketObjectVersionsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
                    ///
                    /// If an error occurs, an `SdkError` will be returned with additional details that
                    /// can be matched against.
                    ///
                    /// By default, any retryable failures will be retried twice. Retry behavior
                    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
                    /// set when configuring the client.
                    pub async fn send(self) -> ::std::result::Result<crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
                        let runtime_plugins = crate::operation::get_bucket_object_versions::GetBucketObjectVersions::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
                        crate::operation::get_bucket_object_versions::GetBucketObjectVersions::orchestrate(&runtime_plugins, input).await
                    }
    
                    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
                    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput, crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError, Self> {
                        crate::client::customize::CustomizableOperation::new(self)
                    }
    pub(crate) fn config_override(
                        mut self,
                        config_override: impl Into<crate::config::Builder>,
                    ) -> Self {
                        self.set_config_override(Some(config_override.into()));
                        self
                    }
    
                    pub(crate) fn set_config_override(
                        &mut self,
                        config_override: Option<crate::config::Builder>,
                    ) -> &mut Self {
                        self.config_override = config_override;
                        self
                    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bucket(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bucket(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bucket()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn delimiter(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.delimiter(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_delimiter(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_delimiter(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_delimiter(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_delimiter()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn encoding_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.encoding_type(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_encoding_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_encoding_type(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_encoding_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_encoding_type()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_marker(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_marker(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_marker()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_keys(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.max_keys(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_keys(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_max_keys(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_keys(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_max_keys()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.prefix(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_prefix(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_prefix()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id_marker(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id_marker(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id_marker()
    }
}
