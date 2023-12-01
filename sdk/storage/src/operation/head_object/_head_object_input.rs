// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HeadObjectInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub consistency_control: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub object: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub part_number: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub version_id: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
}
impl  HeadObjectInput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn consistency_control(&self) -> ::std::option::Option<& str> {
        self.consistency_control.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn object(&self) -> ::std::option::Option<& str> {
        self.object.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn part_number(&self) -> ::std::option::Option<& str> {
        self.part_number.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version_id(&self) -> ::std::option::Option<& str> {
        self.version_id.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_algorithm(&self) -> ::std::option::Option<& crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest> {
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
}
impl HeadObjectInput {
    /// Creates a new builder-style object to manufacture [`HeadObjectInput`](crate::operation::head_object::HeadObjectInput).
    pub fn builder() -> crate::operation::head_object::builders::HeadObjectInputBuilder {
        crate::operation::head_object::builders::HeadObjectInputBuilder::default()
    }
}

/// A builder for [`HeadObjectInput`](crate::operation::head_object::HeadObjectInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HeadObjectInputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) consistency_control: ::std::option::Option<::std::string::String>,
    pub(crate) object: ::std::option::Option<::std::string::String>,
    pub(crate) part_number: ::std::option::Option<::std::string::String>,
    pub(crate) version_id: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_algorithm: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest>,
    pub(crate) x_amz_server_side_encryption_customer_key: ::std::option::Option<::std::string::String>,
    pub(crate) x_amz_server_side_encryption_customer_key_md5: ::std::option::Option<::std::string::String>,
}
impl HeadObjectInputBuilder {
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
    pub fn consistency_control(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.consistency_control = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_consistency_control(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.consistency_control = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_consistency_control(&self) -> &::std::option::Option<::std::string::String> {
        &self.consistency_control
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
    pub fn part_number(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.part_number = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_part_number(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.part_number = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_part_number(&self) -> &::std::option::Option<::std::string::String> {
        &self.part_number
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.version_id
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn x_amz_server_side_encryption_customer_algorithm(mut self, input: crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_x_amz_server_side_encryption_customer_algorithm(mut self, input: ::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest>) -> Self {
        self.x_amz_server_side_encryption_customer_algorithm = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_x_amz_server_side_encryption_customer_algorithm(&self) -> &::std::option::Option<crate::types::XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest> {
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
    /// Consumes the builder and constructs a [`HeadObjectInput`](crate::operation::head_object::HeadObjectInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::head_object::HeadObjectInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(
            crate::operation::head_object::HeadObjectInput {
                bucket: self.bucket
                ,
                consistency_control: self.consistency_control
                ,
                object: self.object
                ,
                part_number: self.part_number
                ,
                version_id: self.version_id
                ,
                x_amz_server_side_encryption_customer_algorithm: self.x_amz_server_side_encryption_customer_algorithm
                ,
                x_amz_server_side_encryption_customer_key: self.x_amz_server_side_encryption_customer_key
                ,
                x_amz_server_side_encryption_customer_key_md5: self.x_amz_server_side_encryption_customer_key_md5
                ,
            }
        )
    }
}
