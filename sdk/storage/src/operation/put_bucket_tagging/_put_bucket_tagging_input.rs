// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutBucketTaggingInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_md5: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub tagging: ::std::option::Option<crate::types::RequestTaggingOfPutBucketTagging>,
}
impl  PutBucketTaggingInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_md5(&self) -> ::std::option::Option<& str> {
        self.content_md5.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn tagging(&self) -> ::std::option::Option<& crate::types::RequestTaggingOfPutBucketTagging> {
        self.tagging.as_ref()
    }
}
impl PutBucketTaggingInput {
    /// Creates a new builder-style object to manufacture [`PutBucketTaggingInput`](crate::operation::put_bucket_tagging::PutBucketTaggingInput).
    pub fn builder() -> crate::operation::put_bucket_tagging::builders::PutBucketTaggingInputBuilder {
        crate::operation::put_bucket_tagging::builders::PutBucketTaggingInputBuilder::default()
    }
}

/// A builder for [`PutBucketTaggingInput`](crate::operation::put_bucket_tagging::PutBucketTaggingInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutBucketTaggingInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) content_md5: ::std::option::Option<::std::string::String>,
    pub(crate) tagging: ::std::option::Option<crate::types::RequestTaggingOfPutBucketTagging>,
}
impl PutBucketTaggingInputBuilder {
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
    pub fn content_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_md5 = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_md5 = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_md5
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn tagging(mut self, input: crate::types::RequestTaggingOfPutBucketTagging) -> Self {
        self.tagging = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_tagging(mut self, input: ::std::option::Option<crate::types::RequestTaggingOfPutBucketTagging>) -> Self {
        self.tagging = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_tagging(&self) -> &::std::option::Option<crate::types::RequestTaggingOfPutBucketTagging> {
        &self.tagging
    }
    /// Consumes the builder and constructs a [`PutBucketTaggingInput`](crate::operation::put_bucket_tagging::PutBucketTaggingInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_bucket_tagging::PutBucketTaggingInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::put_bucket_tagging::PutBucketTaggingInput {
                bucket: self.bucket
                ,
                content_md5: self.content_md5
                ,
                tagging: self.tagging
                ,
            }
        )
    }
}
