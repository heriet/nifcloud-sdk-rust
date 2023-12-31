// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UploadPartOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub e_tag: ::std::option::Option<::std::string::String>,
}
impl  UploadPartOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn e_tag(&self) -> ::std::option::Option<& str> {
        self.e_tag.as_deref()
    }
}
impl UploadPartOutput {
    /// Creates a new builder-style object to manufacture [`UploadPartOutput`](crate::operation::upload_part::UploadPartOutput).
    pub fn builder() -> crate::operation::upload_part::builders::UploadPartOutputBuilder {
        crate::operation::upload_part::builders::UploadPartOutputBuilder::default()
    }
}

/// A builder for [`UploadPartOutput`](crate::operation::upload_part::UploadPartOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UploadPartOutputBuilder {
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
}
impl UploadPartOutputBuilder {
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
    /// Consumes the builder and constructs a [`UploadPartOutput`](crate::operation::upload_part::UploadPartOutput).
    pub fn build(self) -> crate::operation::upload_part::UploadPartOutput {
        crate::operation::upload_part::UploadPartOutput {
            e_tag: self.e_tag
            ,
        }
    }
}

