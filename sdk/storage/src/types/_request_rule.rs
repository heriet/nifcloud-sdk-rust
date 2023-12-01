// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestRule  {
    #[allow(missing_docs)] // documentation missing in model
    pub id: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub prefix: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub request_expiration: ::std::option::Option<crate::types::RequestExpiration>,
    #[allow(missing_docs)] // documentation missing in model
    pub request_filter: ::std::option::Option<crate::types::RequestFilter>,
    #[allow(missing_docs)] // documentation missing in model
    pub request_noncurrent_version_expiration: ::std::option::Option<crate::types::RequestNoncurrentVersionExpiration>,
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration>,
}
impl  RequestRule  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(&self) -> & str {
        use std::ops::Deref; self.id.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(&self) -> ::std::option::Option<& str> {
        self.prefix.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_expiration(&self) -> ::std::option::Option<& crate::types::RequestExpiration> {
        self.request_expiration.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_filter(&self) -> ::std::option::Option<& crate::types::RequestFilter> {
        self.request_filter.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_noncurrent_version_expiration(&self) -> ::std::option::Option<& crate::types::RequestNoncurrentVersionExpiration> {
        self.request_noncurrent_version_expiration.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<& crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration> {
        self.status.as_ref()
    }
}
impl RequestRule {
    /// Creates a new builder-style object to manufacture [`RequestRule`](crate::types::RequestRule).
    pub fn builder() -> crate::types::builders::RequestRuleBuilder {
        crate::types::builders::RequestRuleBuilder::default()
    }
}

/// A builder for [`RequestRule`](crate::types::RequestRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestRuleBuilder {
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) request_expiration: ::std::option::Option<crate::types::RequestExpiration>,
    pub(crate) request_filter: ::std::option::Option<crate::types::RequestFilter>,
    pub(crate) request_noncurrent_version_expiration: ::std::option::Option<crate::types::RequestNoncurrentVersionExpiration>,
    pub(crate) status: ::std::option::Option<crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration>,
}
impl RequestRuleBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_expiration(mut self, input: crate::types::RequestExpiration) -> Self {
        self.request_expiration = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_expiration(mut self, input: ::std::option::Option<crate::types::RequestExpiration>) -> Self {
        self.request_expiration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_expiration(&self) -> &::std::option::Option<crate::types::RequestExpiration> {
        &self.request_expiration
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_filter(mut self, input: crate::types::RequestFilter) -> Self {
        self.request_filter = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_filter(mut self, input: ::std::option::Option<crate::types::RequestFilter>) -> Self {
        self.request_filter = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_filter(&self) -> &::std::option::Option<crate::types::RequestFilter> {
        &self.request_filter
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_noncurrent_version_expiration(mut self, input: crate::types::RequestNoncurrentVersionExpiration) -> Self {
        self.request_noncurrent_version_expiration = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_noncurrent_version_expiration(mut self, input: ::std::option::Option<crate::types::RequestNoncurrentVersionExpiration>) -> Self {
        self.request_noncurrent_version_expiration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_noncurrent_version_expiration(&self) -> &::std::option::Option<crate::types::RequestNoncurrentVersionExpiration> {
        &self.request_noncurrent_version_expiration
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(mut self, input: crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration>) -> Self {
        self.status = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<crate::types::StatusOfLifecycleConfigurationForPutBucketLifecycleConfiguration> {
        &self.status
    }
    /// Consumes the builder and constructs a [`RequestRule`](crate::types::RequestRule).
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](crate::types::builders::RequestRuleBuilder::id)
    pub fn build(self) -> ::std::result::Result<crate::types::RequestRule, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::types::RequestRule {
                id: self.id
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("id", "id was not specified but it is required when building RequestRule")
                    )?
                ,
                prefix: self.prefix
                ,
                request_expiration: self.request_expiration
                ,
                request_filter: self.request_filter
                ,
                request_noncurrent_version_expiration: self.request_noncurrent_version_expiration
                ,
                status: self.status
                ,
            }
        )
    }
}
