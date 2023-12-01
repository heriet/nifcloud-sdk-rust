// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutObjectOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub e_tag: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl  PutObjectOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn e_tag(&self) -> ::std::option::Option<& str> {
        self.e_tag.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_version_id(&self) -> ::std::option::Option<& str> {
        self.x_amz_version_id.as_deref()
    }
}
impl PutObjectOutput {
    /// Creates a new builder-style object to manufacture [`PutObjectOutput`](crate::operation::put_object::PutObjectOutput).
    pub fn builder() -> crate::operation::put_object::builders::PutObjectOutputBuilder {
        crate::operation::put_object::builders::PutObjectOutputBuilder::default()
    }
}

/// A builder for [`PutObjectOutput`](crate::operation::put_object::PutObjectOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutObjectOutputBuilder {
    pub(crate) e_tag: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_version_id: ::std::option::Option<::std::string::String>,
}
impl PutObjectOutputBuilder {
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
    /// Consumes the builder and constructs a [`PutObjectOutput`](crate::operation::put_object::PutObjectOutput).
    pub fn build(self) -> crate::operation::put_object::PutObjectOutput {
        crate::operation::put_object::PutObjectOutput {
            e_tag: self.e_tag
            ,
            x_amz_version_id: self.x_amz_version_id
            ,
        }
    }
}

