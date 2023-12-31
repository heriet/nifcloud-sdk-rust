// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccessControlListOfGetBucketAcl  {
    #[allow(missing_docs)] // documentation missing in model
    pub grantee: ::std::option::Option<crate::types::Grantee>,
    #[allow(missing_docs)] // documentation missing in model
    pub permission: ::std::option::Option<::std::string::String>,
}
impl  AccessControlListOfGetBucketAcl  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn grantee(&self) -> ::std::option::Option<& crate::types::Grantee> {
        self.grantee.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn permission(&self) -> ::std::option::Option<& str> {
        self.permission.as_deref()
    }
}
impl AccessControlListOfGetBucketAcl {
    /// Creates a new builder-style object to manufacture [`AccessControlListOfGetBucketAcl`](crate::types::AccessControlListOfGetBucketAcl).
    pub fn builder() -> crate::types::builders::AccessControlListOfGetBucketAclBuilder {
        crate::types::builders::AccessControlListOfGetBucketAclBuilder::default()
    }
}

/// A builder for [`AccessControlListOfGetBucketAcl`](crate::types::AccessControlListOfGetBucketAcl).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AccessControlListOfGetBucketAclBuilder {
    pub(crate) grantee: ::std::option::Option<crate::types::Grantee>,
    pub(crate) permission: ::std::option::Option<::std::string::String>,
}
impl AccessControlListOfGetBucketAclBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn grantee(mut self, input: crate::types::Grantee) -> Self {
        self.grantee = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_grantee(mut self, input: ::std::option::Option<crate::types::Grantee>) -> Self {
        self.grantee = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_grantee(&self) -> &::std::option::Option<crate::types::Grantee> {
        &self.grantee
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn permission(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.permission = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_permission(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.permission = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_permission(&self) -> &::std::option::Option<::std::string::String> {
        &self.permission
    }
    /// Consumes the builder and constructs a [`AccessControlListOfGetBucketAcl`](crate::types::AccessControlListOfGetBucketAcl).
    pub fn build(self) -> crate::types::AccessControlListOfGetBucketAcl {
        crate::types::AccessControlListOfGetBucketAcl {
            grantee: self.grantee
            ,
            permission: self.permission
            ,
        }
    }
}

