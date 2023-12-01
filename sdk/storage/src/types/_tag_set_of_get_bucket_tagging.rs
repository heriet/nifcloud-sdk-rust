// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TagSetOfGetBucketTagging  {
    #[allow(missing_docs)] // documentation missing in model
    pub key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub value: ::std::option::Option<::std::string::String>,
}
impl  TagSetOfGetBucketTagging  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn key(&self) -> ::std::option::Option<& str> {
        self.key.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(&self) -> ::std::option::Option<& str> {
        self.value.as_deref()
    }
}
impl TagSetOfGetBucketTagging {
    /// Creates a new builder-style object to manufacture [`TagSetOfGetBucketTagging`](crate::types::TagSetOfGetBucketTagging).
    pub fn builder() -> crate::types::builders::TagSetOfGetBucketTaggingBuilder {
        crate::types::builders::TagSetOfGetBucketTaggingBuilder::default()
    }
}

/// A builder for [`TagSetOfGetBucketTagging`](crate::types::TagSetOfGetBucketTagging).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TagSetOfGetBucketTaggingBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl TagSetOfGetBucketTaggingBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`TagSetOfGetBucketTagging`](crate::types::TagSetOfGetBucketTagging).
    pub fn build(self) -> crate::types::TagSetOfGetBucketTagging {
        crate::types::TagSetOfGetBucketTagging {
            key: self.key
            ,
            value: self.value
            ,
        }
    }
}

