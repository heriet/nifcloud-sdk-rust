// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InitiateMultipartUploadInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_disposition: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_encoding: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub object: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_meta: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_storage_class: ::std::option::Option<crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_tagging: ::std::option::Option<crate::types::XAmzTaggingOfInitiateMultipartUploadRequest>,
}
impl  InitiateMultipartUploadInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_disposition(&self) -> ::std::option::Option<& str> {
        self.content_disposition.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_encoding(&self) -> ::std::option::Option<& str> {
        self.content_encoding.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn object(&self) -> ::std::option::Option<& str> {
        self.object.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_meta(&self) -> ::std::option::Option<& str> {
        self.x_amz_meta.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_algorithm(&self) -> ::std::option::Option<& crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest> {
        self.x_amz_server_side_encryption_customer_algorithm.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption_customer_key.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key_md5(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption_customer_key_md5.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_storage_class(&self) -> ::std::option::Option<& crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest> {
        self.x_amz_storage_class.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging(&self) -> ::std::option::Option<& crate::types::XAmzTaggingOfInitiateMultipartUploadRequest> {
        self.x_amz_tagging.as_ref()
    }
}
impl InitiateMultipartUploadInput {
    /// Creates a new builder-style object to manufacture [`InitiateMultipartUploadInput`](crate::operation::initiate_multipart_upload::InitiateMultipartUploadInput).
    pub fn builder() -> crate::operation::initiate_multipart_upload::builders::InitiateMultipartUploadInputBuilder {
        crate::operation::initiate_multipart_upload::builders::InitiateMultipartUploadInputBuilder::default()
    }
}

/// A builder for [`InitiateMultipartUploadInput`](crate::operation::initiate_multipart_upload::InitiateMultipartUploadInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InitiateMultipartUploadInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) content_disposition: ::std::option::Option<::std::string::String>,
    pub(crate) content_encoding: ::std::option::Option<::std::string::String>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) object: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_meta: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest>,
    pub(crate) x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_storage_class: ::std::option::Option<crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest>,
    pub(crate) x_amz_tagging: ::std::option::Option<crate::types::XAmzTaggingOfInitiateMultipartUploadRequest>,
}
impl InitiateMultipartUploadInputBuilder {
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
    pub fn content_disposition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_disposition = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content_disposition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_disposition = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content_disposition(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_disposition
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_encoding(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.content_encoding = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_content_encoding(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.content_encoding = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_content_encoding(&self) -> &::std::option::Option<::std::string::String> {
        &self.content_encoding
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
    /// This field is required.
    pub fn object(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_object(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_object(&self) -> &::std::option::Option<::std::string::String> {
        &self.object
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_meta(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_meta = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_meta(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_meta = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_meta(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_meta
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
    pub fn x_amz_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest>) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfInitiateMultipartUploadRequest> {
        &self.x_amz_server_side_encryption_customer_algorithm
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption_customer_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption_customer_key = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_server_side_encryption_customer_key
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption_customer_key_md5 = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_server_side_encryption_customer_key_md5 = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_server_side_encryption_customer_key_md5
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_storage_class(mut self, input: crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest) -> Self {
        self.x_amz_storage_class = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_storage_class(mut self, input: ::std::option::Option<crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest>) -> Self {
        self.x_amz_storage_class = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_storage_class(&self) -> &::std::option::Option<crate::types::XAmzStorageClassOfInitiateMultipartUploadRequest> {
        &self.x_amz_storage_class
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging(mut self, input: crate::types::XAmzTaggingOfInitiateMultipartUploadRequest) -> Self {
        self.x_amz_tagging = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_tagging(mut self, input: ::std::option::Option<crate::types::XAmzTaggingOfInitiateMultipartUploadRequest>) -> Self {
        self.x_amz_tagging = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_tagging(&self) -> &::std::option::Option<crate::types::XAmzTaggingOfInitiateMultipartUploadRequest> {
        &self.x_amz_tagging
    }
    /// Consumes the builder and constructs a [`InitiateMultipartUploadInput`](crate::operation::initiate_multipart_upload::InitiateMultipartUploadInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::initiate_multipart_upload::InitiateMultipartUploadInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::initiate_multipart_upload::InitiateMultipartUploadInput {
                bucket: self.bucket
                ,
                content_disposition: self.content_disposition
                ,
                content_encoding: self.content_encoding
                ,
                content_type: self.content_type
                ,
                object: self.object
                ,
                x_amz_meta: self.x_amz_meta
                ,
                x_amz_server_side_encryption: self.x_amz_server_side_encryption
                ,
                x_amz_server_side_encryption_customer_algorithm: self.x_amz_server_side_encryption_customer_algorithm
                ,
                x_amz_server_side_encryption_customer_key: self.x_amz_server_side_encryption_customer_key
                ,
                x_amz_server_side_encryption_customer_key_md5: self.x_amz_server_side_encryption_customer_key_md5
                ,
                x_amz_storage_class: self.x_amz_storage_class
                ,
                x_amz_tagging: self.x_amz_tagging
                ,
            }
        )
    }
}
