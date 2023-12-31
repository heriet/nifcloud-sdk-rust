// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestFilter  {
    #[allow(missing_docs)] // documentation missing in model
    pub request_and: ::std::option::Option<crate::types::RequestAnd>,
}
impl  RequestFilter  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_and(&self) -> ::std::option::Option<& crate::types::RequestAnd> {
        self.request_and.as_ref()
    }
}
impl RequestFilter {
    /// Creates a new builder-style object to manufacture [`RequestFilter`](crate::types::RequestFilter).
    pub fn builder() -> crate::types::builders::RequestFilterBuilder {
        crate::types::builders::RequestFilterBuilder::default()
    }
}

/// A builder for [`RequestFilter`](crate::types::RequestFilter).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestFilterBuilder {
    pub(crate) request_and: ::std::option::Option<crate::types::RequestAnd>,
}
impl RequestFilterBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn request_and(mut self, input: crate::types::RequestAnd) -> Self {
        self.request_and = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_request_and(mut self, input: ::std::option::Option<crate::types::RequestAnd>) -> Self {
        self.request_and = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_request_and(&self) -> &::std::option::Option<crate::types::RequestAnd> {
        &self.request_and
    }
    /// Consumes the builder and constructs a [`RequestFilter`](crate::types::RequestFilter).
    pub fn build(self) -> crate::types::RequestFilter {
        crate::types::RequestFilter {
            request_and: self.request_and
            ,
        }
    }
}

