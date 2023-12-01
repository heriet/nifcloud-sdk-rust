// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HeadBucketInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
}
impl  HeadBucketInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
}
impl HeadBucketInput {
    /// Creates a new builder-style object to manufacture [`HeadBucketInput`](crate::operation::head_bucket::HeadBucketInput).
    pub fn builder() -> crate::operation::head_bucket::builders::HeadBucketInputBuilder {
        crate::operation::head_bucket::builders::HeadBucketInputBuilder::default()
    }
}

/// A builder for [`HeadBucketInput`](crate::operation::head_bucket::HeadBucketInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HeadBucketInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
}
impl HeadBucketInputBuilder {
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
    /// Consumes the builder and constructs a [`HeadBucketInput`](crate::operation::head_bucket::HeadBucketInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::head_bucket::HeadBucketInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::head_bucket::HeadBucketInput {
                bucket: self.bucket
                ,
            }
        )
    }
}
