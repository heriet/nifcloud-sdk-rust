// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteObjectTaggingOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl  DeleteObjectTaggingOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_version_id(&self) -> ::std::option::Option<& str> {
        self.x_amz_version_id.as_deref()
    }
}
impl DeleteObjectTaggingOutput {
    /// Creates a new builder-style object to manufacture [`DeleteObjectTaggingOutput`](crate::operation::delete_object_tagging::DeleteObjectTaggingOutput).
    pub fn builder() -> crate::operation::delete_object_tagging::builders::DeleteObjectTaggingOutputBuilder {
        crate::operation::delete_object_tagging::builders::DeleteObjectTaggingOutputBuilder::default()
    }
}

/// A builder for [`DeleteObjectTaggingOutput`](crate::operation::delete_object_tagging::DeleteObjectTaggingOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteObjectTaggingOutputBuilder {
    pub(crate) x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl DeleteObjectTaggingOutputBuilder {
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
    /// Consumes the builder and constructs a [`DeleteObjectTaggingOutput`](crate::operation::delete_object_tagging::DeleteObjectTaggingOutput).
    pub fn build(self) -> crate::operation::delete_object_tagging::DeleteObjectTaggingOutput {
        crate::operation::delete_object_tagging::DeleteObjectTaggingOutput {
            x_amz_version_id: self.x_amz_version_id
            ,
        }
    }
}

