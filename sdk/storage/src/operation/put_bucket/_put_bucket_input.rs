// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
}
impl  PutBucketInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
}
impl PutBucketInput {
    /// Creates a new builder-style object to manufacture [`PutBucketInput`](crate::operation::put_bucket::PutBucketInput).
    pub fn builder() -> crate::operation::put_bucket::builders::PutBucketInputBuilder {
        crate::operation::put_bucket::builders::PutBucketInputBuilder::default()
    }
}

/// A builder for [`PutBucketInput`](crate::operation::put_bucket::PutBucketInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
}
impl PutBucketInputBuilder {
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
    /// Consumes the builder and constructs a [`PutBucketInput`](crate::operation::put_bucket::PutBucketInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_bucket::PutBucketInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::put_bucket::PutBucketInput {
                bucket: self.bucket
                ,
            }
        )
    }
}

