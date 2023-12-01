// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketLifecycleConfigurationInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
}
impl  GetBucketLifecycleConfigurationInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
}
impl GetBucketLifecycleConfigurationInput {
    /// Creates a new builder-style object to manufacture [`GetBucketLifecycleConfigurationInput`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationInput).
    pub fn builder() -> crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationInputBuilder {
        crate::operation::get_bucket_lifecycle_configuration::builders::GetBucketLifecycleConfigurationInputBuilder::default()
    }
}

/// A builder for [`GetBucketLifecycleConfigurationInput`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketLifecycleConfigurationInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
}
impl GetBucketLifecycleConfigurationInputBuilder {
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
    /// Consumes the builder and constructs a [`GetBucketLifecycleConfigurationInput`](crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::get_bucket_lifecycle_configuration::GetBucketLifecycleConfigurationInput {
                bucket: self.bucket
                ,
            }
        )
    }
}

