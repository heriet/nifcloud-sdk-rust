// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestNoncurrentVersionExpiration  {
    #[allow(missing_docs)] // documentation missing in model
    pub noncurrent_days: ::std::option::Option<i32>,
}
impl  RequestNoncurrentVersionExpiration  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn noncurrent_days(&self) -> ::std::option::Option<i32> {
        self.noncurrent_days
    }
}
impl RequestNoncurrentVersionExpiration {
    /// Creates a new builder-style object to manufacture [`RequestNoncurrentVersionExpiration`](crate::types::RequestNoncurrentVersionExpiration).
    pub fn builder() -> crate::types::builders::RequestNoncurrentVersionExpirationBuilder {
        crate::types::builders::RequestNoncurrentVersionExpirationBuilder::default()
    }
}

/// A builder for [`RequestNoncurrentVersionExpiration`](crate::types::RequestNoncurrentVersionExpiration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestNoncurrentVersionExpirationBuilder {
    pub(crate) noncurrent_days: ::std::option::Option<i32>,
}
impl RequestNoncurrentVersionExpirationBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn noncurrent_days(mut self, input: i32) -> Self {
        self.noncurrent_days = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_noncurrent_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.noncurrent_days = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_noncurrent_days(&self) -> &::std::option::Option<i32> {
        &self.noncurrent_days
    }
    /// Consumes the builder and constructs a [`RequestNoncurrentVersionExpiration`](crate::types::RequestNoncurrentVersionExpiration).
    pub fn build(self) -> crate::types::RequestNoncurrentVersionExpiration {
        crate::types::RequestNoncurrentVersionExpiration {
            noncurrent_days: self.noncurrent_days
            ,
        }
    }
}

