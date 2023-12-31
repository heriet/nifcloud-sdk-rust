// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteBucketLifecycleInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
}
impl  DeleteBucketLifecycleInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
}
impl DeleteBucketLifecycleInput {
    /// Creates a new builder-style object to manufacture [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
    pub fn builder() -> crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleInputBuilder {
        crate::operation::delete_bucket_lifecycle::builders::DeleteBucketLifecycleInputBuilder::default()
    }
}

/// A builder for [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteBucketLifecycleInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
}
impl DeleteBucketLifecycleInputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteBucketLifecycleInput`](crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::delete_bucket_lifecycle::DeleteBucketLifecycleInput {
                bucket: self.bucket
                ,
            }
        )
    }
}

