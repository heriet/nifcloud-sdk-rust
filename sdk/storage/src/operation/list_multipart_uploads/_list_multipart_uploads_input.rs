// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListMultipartUploadsInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub encoding_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub key_marker: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub max_uploads: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub prefix: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub upload_id_marker: ::std::option::Option<::std::string::String>,
}
impl  ListMultipartUploadsInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn encoding_type(&self) -> ::std::option::Option<& str> {
        self.encoding_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn key_marker(&self) -> ::std::option::Option<& str> {
        self.key_marker.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_uploads(&self) -> ::std::option::Option<i32> {
        self.max_uploads
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn prefix(&self) -> ::std::option::Option<& str> {
        self.prefix.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn upload_id_marker(&self) -> ::std::option::Option<& str> {
        self.upload_id_marker.as_deref()
    }
}
impl ListMultipartUploadsInput {
    /// Creates a new builder-style object to manufacture [`ListMultipartUploadsInput`](crate::operation::list_multipart_uploads::ListMultipartUploadsInput).
    pub fn builder() -> crate::operation::list_multipart_uploads::builders::ListMultipartUploadsInputBuilder {
        crate::operation::list_multipart_uploads::builders::ListMultipartUploadsInputBuilder::default()
    }
}

/// A builder for [`ListMultipartUploadsInput`](crate::operation::list_multipart_uploads::ListMultipartUploadsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListMultipartUploadsInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) encoding_type: ::std::option::Option<::std::string::String>,
    pub(crate) key_marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_uploads: ::std::option::Option<i32>,
    pub(crate) prefix: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id_marker: ::std::option::Option<::std::string::String>,
}
impl ListMultipartUploadsInputBuilder {
    #[allow(missing_docs)] // documentation missing in model
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn encoding_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.encoding_type = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_encoding_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.encoding_type = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_encoding_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.encoding_type
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn key_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_marker = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_key_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_marker = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_key_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_marker
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_uploads(mut self, input: i32) -> Self {
        self.max_uploads = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_uploads(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_uploads = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_uploads(&self) -> &::std::option::Option<i32> {
        &self.max_uploads
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
    pub fn upload_id_marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id_marker = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_upload_id_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id_marker = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_upload_id_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id_marker
    }
    /// Consumes the builder and constructs a [`ListMultipartUploadsInput`](crate::operation::list_multipart_uploads::ListMultipartUploadsInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::list_multipart_uploads::ListMultipartUploadsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::list_multipart_uploads::ListMultipartUploadsInput {
                bucket: self.bucket
                ,
                encoding_type: self.encoding_type
                ,
                key_marker: self.key_marker
                ,
                max_uploads: self.max_uploads
                ,
                prefix: self.prefix
                ,
                upload_id_marker: self.upload_id_marker
                ,
            }
        )
    }
}

