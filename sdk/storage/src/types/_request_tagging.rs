// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestTagging  {
    #[allow(missing_docs)] // documentation missing in model
    pub request_tag_set: ::std::option::Option<crate::types::RequestTagSet>,
}
impl  RequestTagging  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_tag_set(&self) -> ::std::option::Option<& crate::types::RequestTagSet> {
        self.request_tag_set.as_ref()
    }
}
impl RequestTagging {
    /// Creates a new builder-style object to manufacture [`RequestTagging`](crate::types::RequestTagging).
    pub fn builder() -> crate::types::builders::RequestTaggingBuilder {
        crate::types::builders::RequestTaggingBuilder::default()
    }
}

/// A builder for [`RequestTagging`](crate::types::RequestTagging).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestTaggingBuilder {
    pub(crate) request_tag_set: ::std::option::Option<crate::types::RequestTagSet>,
}
impl RequestTaggingBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_tag_set(mut self, input: crate::types::RequestTagSet) -> Self {
        self.request_tag_set = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_tag_set(mut self, input: ::std::option::Option<crate::types::RequestTagSet>) -> Self {
        self.request_tag_set = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_tag_set(&self) -> &::std::option::Option<crate::types::RequestTagSet> {
        &self.request_tag_set
    }
    /// Consumes the builder and constructs a [`RequestTagging`](crate::types::RequestTagging).
    pub fn build(self) -> crate::types::RequestTagging {
        crate::types::RequestTagging {
            request_tag_set: self.request_tag_set
            ,
        }
    }
}

