// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HeadObjectOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub accept_ranges: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub e_tag: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub last_modified: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_expiration: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_mp_parts_count: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl  HeadObjectOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn accept_ranges(&self) -> ::std::option::Option<& str> {
        self.accept_ranges.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn e_tag(&self) -> ::std::option::Option<& str> {
        self.e_tag.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn last_modified(&self) -> ::std::option::Option<& str> {
        self.last_modified.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_expiration(&self) -> ::std::option::Option<& str> {
        self.x_amz_expiration.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_mp_parts_count(&self) -> ::std::option::Option<& str> {
        self.x_amz_mp_parts_count.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_version_id(&self) -> ::std::option::Option<& str> {
        self.x_amz_version_id.as_deref()
    }
}
impl HeadObjectOutput {
    /// Creates a new builder-style object to manufacture [`HeadObjectOutput`](crate::operation::head_object::HeadObjectOutput).
    pub fn builder() -> crate::operation::head_object::builders::HeadObjectOutputBuilder {
        crate::operation::head_object::builders::HeadObjectOutputBuilder::default()
    }
}

/// A builder for [`HeadObjectOutput`](crate::operation::head_object::HeadObjectOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HeadObjectOutputBuilder {
    pub(crate) accept_ranges: ::std::option::Option<::std::string::String>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) last_modified: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_expiration: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_mp_parts_count: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl HeadObjectOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn accept_ranges(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.accept_ranges = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_accept_ranges(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.accept_ranges = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_accept_ranges(&self) -> &::std::option::Option<::std::string::String> {
        &self.accept_ranges
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
    pub fn e_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.e_tag = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_e_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.e_tag = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_e_tag(&self) -> &::std::option::Option<::std::string::String> {
        &self.e_tag
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn last_modified(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.last_modified = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_last_modified(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.last_modified = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_last_modified(&self) -> &::std::option::Option<::std::string::String> {
        &self.last_modified
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_expiration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_expiration = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_expiration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_expiration = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_expiration(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_expiration
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_mp_parts_count(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_mp_parts_count = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_mp_parts_count(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_mp_parts_count = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_mp_parts_count(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_mp_parts_count
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_server_side_encryption
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_version_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_version_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_version_id
    }
    /// Consumes the builder and constructs a [`HeadObjectOutput`](crate::operation::head_object::HeadObjectOutput).
    pub fn build(self) -> crate::operation::head_object::HeadObjectOutput {
        crate::operation::head_object::HeadObjectOutput {
            accept_ranges: self.accept_ranges
            ,
            content_type: self.content_type
            ,
            e_tag: self.e_tag
            ,
            last_modified: self.last_modified
            ,
            x_amz_expiration: self.x_amz_expiration
            ,
            x_amz_mp_parts_count: self.x_amz_mp_parts_count
            ,
            x_amz_server_side_encryption: self.x_amz_server_side_encryption
            ,
            x_amz_version_id: self.x_amz_version_id
            ,
        }
    }
}

