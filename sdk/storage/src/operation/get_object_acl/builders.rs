// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_object_acl::_get_object_acl_output::GetObjectAclOutputBuilder;

pub use crate::operation::get_object_acl::_get_object_acl_input::GetObjectAclInputBuilder;

impl GetObjectAclInputBuilder {
    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::get_object_acl::GetObjectAclOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::get_object_acl::GetObjectACLError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
                    > {
                        let mut fluent_builder = client.get_object_acl();
                        fluent_builder.inner = self;
                        fluent_builder.send().await
                    }
}
/// Fluent builder constructing a request to `GetObjectACL`.
/// 
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetObjectACLFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::get_object_acl::builders::GetObjectAclInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::get_object_acl::GetObjectAclOutput,
                    crate::operation::get_object_acl::GetObjectACLError,
                > for GetObjectACLFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::get_object_acl::GetObjectAclOutput,
                        crate::operation::get_object_acl::GetObjectACLError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl GetObjectACLFluentBuilder {
    /// Creates a new `GetObjectACL`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle, inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetObjectACL as a reference.
    pub fn as_input(&self) -> &crate::operation::get_object_acl::builders::GetObjectAclInputBuilder {
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
                    pub async fn send(self) -> ::std::result::Result<crate::operation::get_object_acl::GetObjectAclOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_object_acl::GetObjectACLError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
                        let runtime_plugins = crate::operation::get_object_acl::GetObjectACL::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
                        crate::operation::get_object_acl::GetObjectACL::orchestrate(&runtime_plugins, input).await
                    }
    
                    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
                    ) -> crate::client::customize::CustomizableOperation<crate::operation::get_object_acl::GetObjectAclOutput, crate::operation::get_object_acl::GetObjectACLError, Self> {
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
    pub fn object(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.object(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_object(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_object(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_object(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_object()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id()
    }
}

