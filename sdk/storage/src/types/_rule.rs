// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Rule  {
    #[allow(missing_docs)] // documentation missing in model
    pub expiration: ::std::option::Option<crate::types::Expiration>,
    #[allow(missing_docs)] // documentation missing in model
    pub filter: ::std::option::Option<crate::types::Filter>,
    #[allow(missing_docs)] // documentation missing in model
    pub id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub noncurrent_version_expiration: ::std::option::Option<crate::types::NoncurrentVersionExpiration>,
    #[allow(missing_docs)] // documentation missing in model
    pub prefix: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub status: ::std::option::Option<::std::string::String>,
}
impl  Rule  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn expiration(&self) -> ::std::option::Option<& crate::types::Expiration> {
        self.expiration.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn filter(&self) -> ::std::option::Option<& crate::types::Filter> {
        self.filter.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn id(&self) -> ::std::option::Option<& str> {
        self.id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn noncurrent_version_expiration(&self) -> ::std::option::Option<& crate::types::NoncurrentVersionExpiration> {
        self.noncurrent_version_expiration.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(&self) -> ::std::option::Option<& str> {
        self.prefix.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(&self) -> ::std::option::Option<& str> {
        self.status.as_deref()
    }
}
impl Rule {
    /// Creates a new builder-style object to manufacture [`Rule`](crate::types::Rule).
    pub fn builder() -> crate::types::builders::RuleBuilder {
        crate::types::builders::RuleBuilder::default()
    }
}

/// A builder for [`Rule`](crate::types::Rule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RuleBuilder {
    pub(crate) expiration: ::std::option::Option<crate::types::Expiration>,
    pub(crate) filter: ::std::option::Option<crate::types::Filter>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
    pub(crate) noncurrent_version_expiration: ::std::option::Option<crate::types::NoncurrentVersionExpiration>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
}
impl RuleBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn expiration(mut self, input: crate::types::Expiration) -> Self {
        self.expiration = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_expiration(mut self, input: ::std::option::Option<crate::types::Expiration>) -> Self {
        self.expiration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_expiration(&self) -> &::std::option::Option<crate::types::Expiration> {
        &self.expiration
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn filter(mut self, input: crate::types::Filter) -> Self {
        self.filter = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_filter(mut self, input: ::std::option::Option<crate::types::Filter>) -> Self {
        self.filter = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_filter(&self) -> &::std::option::Option<crate::types::Filter> {
        &self.filter
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
    #[allow(missing_docs)] // documentation missing in model
    pub fn noncurrent_version_expiration(mut self, input: crate::types::NoncurrentVersionExpiration) -> Self {
        self.noncurrent_version_expiration = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_noncurrent_version_expiration(mut self, input: ::std::option::Option<crate::types::NoncurrentVersionExpiration>) -> Self {
        self.noncurrent_version_expiration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_noncurrent_version_expiration(&self) -> &::std::option::Option<crate::types::NoncurrentVersionExpiration> {
        &self.noncurrent_version_expiration
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// Consumes the builder and constructs a [`Rule`](crate::types::Rule).
    pub fn build(self) -> crate::types::Rule {
        crate::types::Rule {
            expiration: self.expiration
            ,
            filter: self.filter
            ,
            id: self.id
            ,
            noncurrent_version_expiration: self.noncurrent_version_expiration
            ,
            prefix: self.prefix
            ,
            status: self.status
            ,
        }
    }
}

