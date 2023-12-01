// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UploadPartCopyOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub e_tag: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_version_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
}
impl  UploadPartCopyOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn e_tag(&self) -> ::std::option::Option<& str> {
        self.e_tag.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn last_modified(&self) -> ::std::option::Option<& ::aws_smithy_types::DateTime> {
        self.last_modified.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_version_id(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_version_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption.as_deref()
    }
}
impl UploadPartCopyOutput {
    /// Creates a new builder-style object to manufacture [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
    pub fn builder() -> crate::operation::upload_part_copy::builders::UploadPartCopyOutputBuilder {
        crate::operation::upload_part_copy::builders::UploadPartCopyOutputBuilder::default()
    }
}

/// A builder for [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UploadPartCopyOutputBuilder {
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) last_modified: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) x_amz_copy_source_version_id: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
}
impl UploadPartCopyOutputBuilder {
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
    pub fn last_modified(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_last_modified(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_last_modified(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_version_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_version_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_version_id
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
    /// Consumes the builder and constructs a [`UploadPartCopyOutput`](crate::operation::upload_part_copy::UploadPartCopyOutput).
    pub fn build(self) -> crate::operation::upload_part_copy::UploadPartCopyOutput {
        crate::operation::upload_part_copy::UploadPartCopyOutput {
            content_type: self.content_type
            ,
            e_tag: self.e_tag
            ,
            last_modified: self.last_modified
            ,
            x_amz_copy_source_version_id: self.x_amz_copy_source_version_id
            ,
            x_amz_server_side_encryption: self.x_amz_server_side_encryption
            ,
        }
    }
}
