// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_bucket_lifecycle_configuration::_put_bucket_lifecycle_configuration_output::PutBucketLifecycleConfigurationOutputBuilder;

pub use crate::operation::put_bucket_lifecycle_configuration::_put_bucket_lifecycle_configuration_input::PutBucketLifecycleConfigurationInputBuilder;

impl PutBucketLifecycleConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
                    > {
                        let mut fluent_builder = client.put_bucket_lifecycle_configuration();
                        fluent_builder.inner = self;
                        fluent_builder.send().await
                    }
}
/// Fluent builder constructing a request to `PutBucketLifecycleConfiguration`.
/// 
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutBucketLifecycleConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput,
                    crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
                > for PutBucketLifecycleConfigurationFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput,
                        crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl PutBucketLifecycleConfigurationFluentBuilder {
    /// Creates a new `PutBucketLifecycleConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle, inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutBucketLifecycleConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationInputBuilder {
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
                    pub async fn send(self) -> ::std::result::Result<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
                        let runtime_plugins = crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfiguration::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
                        crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfiguration::orchestrate(&runtime_plugins, input).await
                    }
    
                    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
                    ) -> crate::client::customize::CustomizableOperation<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput, crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError, Self> {
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
    pub fn lifecycle_configuration(mut self, input: crate::types::RequestLifecycleConfiguration) -> Self {
        self.inner = self.inner.lifecycle_configuration(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_lifecycle_configuration(mut self, input: ::std::option::Option<crate::types::RequestLifecycleConfiguration>) -> Self {
        self.inner = self.inner.set_lifecycle_configuration(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_lifecycle_configuration(&self) -> &::std::option::Option<crate::types::RequestLifecycleConfiguration> {
        self.inner.get_lifecycle_configuration()
    }
}

