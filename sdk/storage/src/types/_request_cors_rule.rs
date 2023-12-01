// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestCorsRule  {
    #[allow(missing_docs)] // documentation missing in model
    pub allowed_header: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub allowed_origin: ::std::string::String,
    #[allow(missing_docs)] // documentation missing in model
    pub expose_header: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub list_of_request_allowed_method: ::std::vec::Vec::<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub max_age_seconds: ::std::option::Option<i32>,
}
impl  RequestCorsRule  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn allowed_header(&self) -> ::std::option::Option<& str> {
        self.allowed_header.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn allowed_origin(&self) -> & str {
        use std::ops::Deref; self.allowed_origin.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn expose_header(&self) -> ::std::option::Option<& str> {
        self.expose_header.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(&self) -> ::std::option::Option<& str> {
        self.id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn list_of_request_allowed_method(&self) -> & [::std::string::String] {
        use std::ops::Deref; self.list_of_request_allowed_method.deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_age_seconds(&self) -> ::std::option::Option<i32> {
        self.max_age_seconds
    }
}
impl RequestCorsRule {
    /// Creates a new builder-style object to manufacture [`RequestCorsRule`](crate::types::RequestCorsRule).
    pub fn builder() -> crate::types::builders::RequestCorsRuleBuilder {
        crate::types::builders::RequestCorsRuleBuilder::default()
    }
}

/// A builder for [`RequestCorsRule`](crate::types::RequestCorsRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestCorsRuleBuilder {
    pub(crate) allowed_header: ::std::option::Option<::std::string::String>,
    pub(crate) allowed_origin: ::std::option::Option<::std::string::String>,
    pub(crate) expose_header: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) list_of_request_allowed_method: ::std::option::Option<::std::vec::Vec::<::std::string::String>>,
    pub(crate) max_age_seconds: ::std::option::Option<i32>,
}
impl RequestCorsRuleBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn allowed_header(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.allowed_header = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_allowed_header(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.allowed_header = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_allowed_header(&self) -> &::std::option::Option<::std::string::String> {
        &self.allowed_header
    }
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn allowed_origin(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.allowed_origin = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_allowed_origin(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.allowed_origin = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_allowed_origin(&self) -> &::std::option::Option<::std::string::String> {
        &self.allowed_origin
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn expose_header(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.expose_header = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_expose_header(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.expose_header = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_expose_header(&self) -> &::std::option::Option<::std::string::String> {
        &self.expose_header
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Appends an item to `list_of_request_allowed_method`.
    ///
    /// To override the contents of this collection use [`set_list_of_request_allowed_method`](Self::set_list_of_request_allowed_method).
    ///
    pub fn list_of_request_allowed_method(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.list_of_request_allowed_method.unwrap_or_default();
                        v.push(input.into());
                        self.list_of_request_allowed_method = ::std::option::Option::Some(v);
                        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_list_of_request_allowed_method(mut self, input: ::std::option::Option<::std::vec::Vec::<::std::string::String>>) -> Self {
        self.list_of_request_allowed_method = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_list_of_request_allowed_method(&self) -> &::std::option::Option<::std::vec::Vec::<::std::string::String>> {
        &self.list_of_request_allowed_method
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_age_seconds(mut self, input: i32) -> Self {
        self.max_age_seconds = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_age_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_age_seconds = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_age_seconds(&self) -> &::std::option::Option<i32> {
        &self.max_age_seconds
    }
    /// Consumes the builder and constructs a [`RequestCorsRule`](crate::types::RequestCorsRule).
    /// This method will fail if any of the following fields are not set:
    /// - [`allowed_origin`](crate::types::builders::RequestCorsRuleBuilder::allowed_origin)
    /// - [`list_of_request_allowed_method`](crate::types::builders::RequestCorsRuleBuilder::list_of_request_allowed_method)
    pub fn build(self) -> ::std::result::Result<crate::types::RequestCorsRule, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::types::RequestCorsRule {
                allowed_header: self.allowed_header
                ,
                allowed_origin: self.allowed_origin
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("allowed_origin", "allowed_origin was not specified but it is required when building RequestCorsRule")
                    )?
                ,
                expose_header: self.expose_header
                ,
                id: self.id
                ,
                list_of_request_allowed_method: self.list_of_request_allowed_method
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("list_of_request_allowed_method", "list_of_request_allowed_method was not specified but it is required when building RequestCorsRule")
                    )?
                ,
                max_age_seconds: self.max_age_seconds
                ,
            }
        )
    }
}

