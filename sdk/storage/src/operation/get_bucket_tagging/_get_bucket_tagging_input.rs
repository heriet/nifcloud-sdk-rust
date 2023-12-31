// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketTaggingInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
}
impl  GetBucketTaggingInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
}
impl GetBucketTaggingInput {
    /// Creates a new builder-style object to manufacture [`GetBucketTaggingInput`](crate::operation::get_bucket_tagging::GetBucketTaggingInput).
    pub fn builder() -> crate::operation::get_bucket_tagging::builders::GetBucketTaggingInputBuilder {
        crate::operation::get_bucket_tagging::builders::GetBucketTaggingInputBuilder::default()
    }
}

/// A builder for [`GetBucketTaggingInput`](crate::operation::get_bucket_tagging::GetBucketTaggingInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketTaggingInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
}
impl GetBucketTaggingInputBuilder {
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
    /// Consumes the builder and constructs a [`GetBucketTaggingInput`](crate::operation::get_bucket_tagging::GetBucketTaggingInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::get_bucket_tagging::GetBucketTaggingInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::get_bucket_tagging::GetBucketTaggingInput {
                bucket: self.bucket
                ,
            }
        )
    }
}

