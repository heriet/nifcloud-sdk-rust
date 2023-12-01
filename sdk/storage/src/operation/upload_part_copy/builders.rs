// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::upload_part_copy::_upload_part_copy_output::UploadPartCopyOutputBuilder;

pub use crate::operation::upload_part_copy::_upload_part_copy_input::UploadPartCopyInputBuilder;

impl UploadPartCopyInputBuilder {
    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::upload_part_copy::UploadPartCopyOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::upload_part_copy::UploadPartCopyError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
                    > {
                        let mut fluent_builder = client.upload_part_copy();
                        fluent_builder.inner = self;
                        fluent_builder.send().await
                    }
}
/// Fluent builder constructing a request to `UploadPartCopy`.
/// 
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UploadPartCopyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::upload_part_copy::builders::UploadPartCopyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::upload_part_copy::UploadPartCopyOutput,
                    crate::operation::upload_part_copy::UploadPartCopyError,
                > for UploadPartCopyFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::upload_part_copy::UploadPartCopyOutput,
                        crate::operation::upload_part_copy::UploadPartCopyError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl UploadPartCopyFluentBuilder {
    /// Creates a new `UploadPartCopy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle, inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UploadPartCopy as a reference.
    pub fn as_input(&self) -> &crate::operation::upload_part_copy::builders::UploadPartCopyInputBuilder {
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
                    pub async fn send(self) -> ::std::result::Result<crate::operation::upload_part_copy::UploadPartCopyOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::upload_part_copy::UploadPartCopyError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>> {
                        let input = self.inner.build().map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
                        let runtime_plugins = crate::operation::upload_part_copy::UploadPartCopy::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
                        crate::operation::upload_part_copy::UploadPartCopy::orchestrate(&runtime_plugins, input).await
                    }
    
                    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
                    ) -> crate::client::customize::CustomizableOperation<crate::operation::upload_part_copy::UploadPartCopyOutput, crate::operation::upload_part_copy::UploadPartCopyError, Self> {
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
    pub fn part_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.part_number(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_part_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_part_number(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_part_number(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_part_number()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.upload_id(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_upload_id(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_upload_id()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_if_match(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_if_match(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_if_match()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_modified_since(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_if_modified_since(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_modified_since(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_if_modified_since(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_modified_since(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_if_modified_since()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_none_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_if_none_match(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_none_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_if_none_match(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_none_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_if_none_match()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_unmodified_since(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_if_unmodified_since(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_unmodified_since(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_if_unmodified_since(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_unmodified_since(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_if_unmodified_since()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_range(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_range(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_range(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_range(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_range(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_range()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest) -> Self {
        self.inner = self.inner.x_amz_copy_source_server_side_encryption_customer_algorithm(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_server_side_encryption_customer_algorithm(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest> {
        self.inner.get_x_amz_copy_source_server_side_encryption_customer_algorithm()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_server_side_encryption_customer_key(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_server_side_encryption_customer_key(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_server_side_encryption_customer_key()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_copy_source_server_side_encryption_customer_key_md5(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_copy_source_server_side_encryption_customer_key_md5(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_copy_source_server_side_encryption_customer_key_md5()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest) -> Self {
        self.inner = self.inner.x_amz_server_side_encryption_customer_algorithm(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest>) -> Self {
        self.inner = self.inner.set_x_amz_server_side_encryption_customer_algorithm(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfUploadPartCopyRequest> {
        self.inner.get_x_amz_server_side_encryption_customer_algorithm()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_server_side_encryption_customer_key(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_server_side_encryption_customer_key(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_server_side_encryption_customer_key()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.x_amz_server_side_encryption_customer_key_md5(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_x_amz_server_side_encryption_customer_key_md5(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_x_amz_server_side_encryption_customer_key_md5()
    }
}

