// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketCorsInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub cors_configuration: ::std::option::Option<crate::types::RequestCorsConfiguration>,
}
impl  PutBucketCorsInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn cors_configuration(&self) -> ::std::option::Option<& crate::types::RequestCorsConfiguration> {
        self.cors_configuration.as_ref()
    }
}
impl PutBucketCorsInput {
    /// Creates a new builder-style object to manufacture [`PutBucketCorsInput`](crate::operation::put_bucket_cors::PutBucketCorsInput).
    pub fn builder() -> crate::operation::put_bucket_cors::builders::PutBucketCorsInputBuilder {
        crate::operation::put_bucket_cors::builders::PutBucketCorsInputBuilder::default()
    }
}

/// A builder for [`PutBucketCorsInput`](crate::operation::put_bucket_cors::PutBucketCorsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketCorsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) cors_configuration: ::std::option::Option<crate::types::RequestCorsConfiguration>,
}
impl PutBucketCorsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn cors_configuration(mut self, input: crate::types::RequestCorsConfiguration) -> Self {
        self.cors_configuration = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_cors_configuration(mut self, input: ::std::option::Option<crate::types::RequestCorsConfiguration>) -> Self {
        self.cors_configuration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_cors_configuration(&self) -> &::std::option::Option<crate::types::RequestCorsConfiguration> {
        &self.cors_configuration
    }
    /// Consumes the builder and constructs a [`PutBucketCorsInput`](crate::operation::put_bucket_cors::PutBucketCorsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_bucket_cors::PutBucketCorsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::put_bucket_cors::PutBucketCorsInput {
                bucket: self.bucket
                ,
                cors_configuration: self.cors_configuration
                ,
            }
        )
    }
}

