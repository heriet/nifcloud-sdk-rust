// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestAnd  {
    #[allow(missing_docs)] // documentation missing in model
    pub request_tag: ::std::option::Option<crate::types::RequestTag>,
}
impl  RequestAnd  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_tag(&self) -> ::std::option::Option<& crate::types::RequestTag> {
        self.request_tag.as_ref()
    }
}
impl RequestAnd {
    /// Creates a new builder-style object to manufacture [`RequestAnd`](crate::types::RequestAnd).
    pub fn builder() -> crate::types::builders::RequestAndBuilder {
        crate::types::builders::RequestAndBuilder::default()
    }
}

/// A builder for [`RequestAnd`](crate::types::RequestAnd).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestAndBuilder {
    pub(crate) request_tag: ::std::option::Option<crate::types::RequestTag>,
}
impl RequestAndBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_tag(mut self, input: crate::types::RequestTag) -> Self {
        self.request_tag = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_tag(mut self, input: ::std::option::Option<crate::types::RequestTag>) -> Self {
        self.request_tag = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_tag(&self) -> &::std::option::Option<crate::types::RequestTag> {
        &self.request_tag
    }
    /// Consumes the builder and constructs a [`RequestAnd`](crate::types::RequestAnd).
    pub fn build(self) -> crate::types::RequestAnd {
        crate::types::RequestAnd {
            request_tag: self.request_tag
            ,
        }
    }
}
