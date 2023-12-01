// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetBucketCorsOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub cors_rule: ::std::option::Option<::std::vec::Vec::<crate::types::CorsRule>>,
}
impl  GetBucketCorsOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    /// 
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.cors_rule.is_none()`.
    pub fn cors_rule(&self) -> & [crate::types::CorsRule] {
        self.cors_rule.as_deref()
        .unwrap_or_default()
    }
}
impl GetBucketCorsOutput {
    /// Creates a new builder-style object to manufacture [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
    pub fn builder() -> crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder {
        crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder::default()
    }
}

/// A builder for [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetBucketCorsOutputBuilder {
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) cors_rule: ::std::option::Option<::std::vec::Vec::<crate::types::CorsRule>>,
}
impl GetBucketCorsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_type = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_type = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_type
    }
    /// Appends an item to `cors_rule`.
    ///
    /// To override the contents of this collection use [`set_cors_rule`](Self::set_cors_rule).
    ///
    pub fn cors_rule(mut self, input: crate::types::CorsRule) -> Self {
        let mut v = self.cors_rule.unwrap_or_default();
                        v.push(input);
                        self.cors_rule = ::std::option::Option::Some(v);
                        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_cors_rule(mut self, input: ::std::option::Option<::std::vec::Vec::<crate::types::CorsRule>>) -> Self {
        self.cors_rule = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_cors_rule(&self) -> &::std::option::Option<::std::vec::Vec::<crate::types::CorsRule>> {
        &self.cors_rule
    }
    /// Consumes the builder and constructs a [`GetBucketCorsOutput`](crate::operation::get_bucket_cors::GetBucketCorsOutput).
    pub fn build(self) -> crate::operation::get_bucket_cors::GetBucketCorsOutput {
        crate::operation::get_bucket_cors::GetBucketCorsOutput {
            content_type: self.content_type
            ,
            cors_rule: self.cors_rule
            ,
        }
    }
}
