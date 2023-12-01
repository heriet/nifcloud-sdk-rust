// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutObjectCopyInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub object: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_if_match: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_if_modified_since: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_if_none_match: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_if_unmodified_since: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_copy_source_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_metadata_directive: ::std::option::Option<crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_storage_class: ::std::option::Option<crate::types::XAmzStorageClassOfPutObjectCopyRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_tagging: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_tagging_directive: ::std::option::Option<crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest>,
}
impl  PutObjectCopyInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn object(&self) -> ::std::option::Option<& str> {
        self.object.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_match(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_if_match.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_modified_since(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_if_modified_since.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_none_match(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_if_none_match.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_unmodified_since(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_if_unmodified_since.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_algorithm(&self) -> ::std::option::Option<& crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest> {
        self.x_amz_copy_source_server_side_encryption_customer_algorithm.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_server_side_encryption_customer_key.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key_md5(&self) -> ::std::option::Option<& str> {
        self.x_amz_copy_source_server_side_encryption_customer_key_md5.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_metadata_directive(&self) -> ::std::option::Option<& crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest> {
        self.x_amz_metadata_directive.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption(&self) -> ::std::option::Option<& str> {
        self.x_amz_server_side_encryption.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_algorithm(&self) -> ::std::option::Option<& crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest> {
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
    pub fn x_amz_storage_class(&self) -> ::std::option::Option<& crate::types::XAmzStorageClassOfPutObjectCopyRequest> {
        self.x_amz_storage_class.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging(&self) -> ::std::option::Option<& str> {
        self.x_amz_tagging.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging_directive(&self) -> ::std::option::Option<& crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest> {
        self.x_amz_tagging_directive.as_ref()
    }
}
impl PutObjectCopyInput {
    /// Creates a new builder-style object to manufacture [`PutObjectCopyInput`](crate::operation::put_object_copy::PutObjectCopyInput).
    pub fn builder() -> crate::operation::put_object_copy::builders::PutObjectCopyInputBuilder {
        crate::operation::put_object_copy::builders::PutObjectCopyInputBuilder::default()
    }
}

/// A builder for [`PutObjectCopyInput`](crate::operation::put_object_copy::PutObjectCopyInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PutObjectCopyInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) object: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_if_match: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_if_modified_since: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_if_none_match: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_if_unmodified_since: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>,
    pub(crate) x_amz_copy_source_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_copy_source_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_metadata_directive: ::std::option::Option<crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest>,
    pub(crate) x_amz_server_side_encryption: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>,
    pub(crate) x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_storage_class: ::std::option::Option<crate::types::XAmzStorageClassOfPutObjectCopyRequest>,
    pub(crate) x_amz_tagging: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_tagging_directive: ::std::option::Option<crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest>,
}
impl PutObjectCopyInputBuilder {
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
    /// This field is required.
    pub fn x_amz_copy_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_match = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_match = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_match(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_if_match
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_modified_since(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_modified_since = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_modified_since(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_modified_since = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_modified_since(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_if_modified_since
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_none_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_none_match = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_none_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_none_match = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_none_match(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_if_none_match
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_if_unmodified_since(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_unmodified_since = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_if_unmodified_since(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_if_unmodified_since = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_if_unmodified_since(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_if_unmodified_since
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_algorithm = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_algorithm = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest> {
        &self.x_amz_copy_source_server_side_encryption_customer_algorithm
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_key = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_server_side_encryption_customer_key
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_copy_source_server_side_encryption_customer_key_md5(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_key_md5 = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_copy_source_server_side_encryption_customer_key_md5(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_copy_source_server_side_encryption_customer_key_md5 = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_copy_source_server_side_encryption_customer_key_md5(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_copy_source_server_side_encryption_customer_key_md5
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_metadata_directive(mut self, input: crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest) -> Self {
        self.x_amz_metadata_directive = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_metadata_directive(mut self, input: ::std::option::Option<crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest>) -> Self {
        self.x_amz_metadata_directive = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_metadata_directive(&self) -> &::std::option::Option<crate::types::XAmzMetadataDirectiveOfPutObjectCopyRequest> {
        &self.x_amz_metadata_directive
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
    pub fn x_amz_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest> {
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
    pub fn x_amz_storage_class(mut self, input: crate::types::XAmzStorageClassOfPutObjectCopyRequest) -> Self {
        self.x_amz_storage_class = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_storage_class(mut self, input: ::std::option::Option<crate::types::XAmzStorageClassOfPutObjectCopyRequest>) -> Self {
        self.x_amz_storage_class = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_storage_class(&self) -> &::std::option::Option<crate::types::XAmzStorageClassOfPutObjectCopyRequest> {
        &self.x_amz_storage_class
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.x_amz_tagging = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_tagging(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.x_amz_tagging = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_tagging(&self) -> &::std::option::Option<::std::string::String> {
        &self.x_amz_tagging
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_tagging_directive(mut self, input: crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest) -> Self {
        self.x_amz_tagging_directive = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_tagging_directive(mut self, input: ::std::option::Option<crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest>) -> Self {
        self.x_amz_tagging_directive = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_tagging_directive(&self) -> &::std::option::Option<crate::types::XAmzTaggingDirectiveOfPutObjectCopyRequest> {
        &self.x_amz_tagging_directive
    }
    /// Consumes the builder and constructs a [`PutObjectCopyInput`](crate::operation::put_object_copy::PutObjectCopyInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::put_object_copy::PutObjectCopyInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::put_object_copy::PutObjectCopyInput {
                bucket: self.bucket
                ,
                object: self.object
                ,
                x_amz_copy_source: self.x_amz_copy_source
                ,
                x_amz_copy_source_if_match: self.x_amz_copy_source_if_match
                ,
                x_amz_copy_source_if_modified_since: self.x_amz_copy_source_if_modified_since
                ,
                x_amz_copy_source_if_none_match: self.x_amz_copy_source_if_none_match
                ,
                x_amz_copy_source_if_unmodified_since: self.x_amz_copy_source_if_unmodified_since
                ,
                x_amz_copy_source_server_side_encryption_customer_algorithm: self.x_amz_copy_source_server_side_encryption_customer_algorithm
                ,
                x_amz_copy_source_server_side_encryption_customer_key: self.x_amz_copy_source_server_side_encryption_customer_key
                ,
                x_amz_copy_source_server_side_encryption_customer_key_md5: self.x_amz_copy_source_server_side_encryption_customer_key_md5
                ,
                x_amz_metadata_directive: self.x_amz_metadata_directive
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
                x_amz_tagging_directive: self.x_amz_tagging_directive
                ,
            }
        )
    }
}
