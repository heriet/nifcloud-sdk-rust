// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetObjectAclOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub access_control_list: ::std::option::Option<crate::types::AccessControlList>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub owner: ::std::option::Option<crate::types::Owner>,
}
impl  GetObjectAclOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn access_control_list(&self) -> ::std::option::Option<& crate::types::AccessControlList> {
        self.access_control_list.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn owner(&self) -> ::std::option::Option<& crate::types::Owner> {
        self.owner.as_ref()
    }
}
impl GetObjectAclOutput {
    /// Creates a new builder-style object to manufacture [`GetObjectAclOutput`](crate::operation::get_object_acl::GetObjectAclOutput).
    pub fn builder() -> crate::operation::get_object_acl::builders::GetObjectAclOutputBuilder {
        crate::operation::get_object_acl::builders::GetObjectAclOutputBuilder::default()
    }
}

/// A builder for [`GetObjectAclOutput`](crate::operation::get_object_acl::GetObjectAclOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetObjectAclOutputBuilder {
    pub(crate) access_control_list: ::std::option::Option<crate::types::AccessControlList>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
}
impl GetObjectAclOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn access_control_list(mut self, input: crate::types::AccessControlList) -> Self {
        self.access_control_list = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_access_control_list(mut self, input: ::std::option::Option<crate::types::AccessControlList>) -> Self {
        self.access_control_list = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_access_control_list(&self) -> &::std::option::Option<crate::types::AccessControlList> {
        &self.access_control_list
    }
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
    #[allow(missing_docs)] // documentation missing in model
    pub fn owner(mut self, input: crate::types::Owner) -> Self {
        self.owner = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_owner(mut self, input: ::std::option::Option<crate::types::Owner>) -> Self {
        self.owner = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_owner(&self) -> &::std::option::Option<crate::types::Owner> {
        &self.owner
    }
    /// Consumes the builder and constructs a [`GetObjectAclOutput`](crate::operation::get_object_acl::GetObjectAclOutput).
    pub fn build(self) -> crate::operation::get_object_acl::GetObjectAclOutput {
        crate::operation::get_object_acl::GetObjectAclOutput {
            access_control_list: self.access_control_list
            ,
            content_type: self.content_type
            ,
            owner: self.owner
            ,
        }
    }
}

