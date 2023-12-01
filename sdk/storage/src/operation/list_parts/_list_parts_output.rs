// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListPartsOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub bucket: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub content_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub encoding_type: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub initiator: ::std::option::Option<crate::types::Initiator>,
    #[allow(missing_docs)] // documentation missing in model
    pub is_truncated: ::std::option::Option<bool>,
    #[allow(missing_docs)] // documentation missing in model
    pub key: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub max_parts: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub next_part_number_marker: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub owner: ::std::option::Option<crate::types::Owner>,
    #[allow(missing_docs)] // documentation missing in model
    pub part: ::std::option::Option<::std::vec::Vec::<crate::types::Part>>,
    #[allow(missing_docs)] // documentation missing in model
    pub part_number_marker: ::std::option::Option<i32>,
    #[allow(missing_docs)] // documentation missing in model
    pub storage_class: ::std::option::Option<::std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    pub upload_id: ::std::option::Option<::std::string::String>,
}
impl  ListPartsOutput  {
    #[allow(missing_docs)] // documentation missing in model
    pub fn bucket(&self) -> ::std::option::Option<& str> {
        self.bucket.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn content_type(&self) -> ::std::option::Option<& str> {
        self.content_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn encoding_type(&self) -> ::std::option::Option<& str> {
        self.encoding_type.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn initiator(&self) -> ::std::option::Option<& crate::types::Initiator> {
        self.initiator.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn is_truncated(&self) -> ::std::option::Option<bool> {
        self.is_truncated
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn key(&self) -> ::std::option::Option<& str> {
        self.key.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_parts(&self) -> ::std::option::Option<i32> {
        self.max_parts
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_part_number_marker(&self) -> ::std::option::Option<i32> {
        self.next_part_number_marker
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn owner(&self) -> ::std::option::Option<& crate::types::Owner> {
        self.owner.as_ref()
    }
    #[allow(missing_docs)] // documentation missing in model
    /// 
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.part.is_none()`.
    pub fn part(&self) -> & [crate::types::Part] {
        self.part.as_deref()
        .unwrap_or_default()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn part_number_marker(&self) -> ::std::option::Option<i32> {
        self.part_number_marker
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn storage_class(&self) -> ::std::option::Option<& str> {
        self.storage_class.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn upload_id(&self) -> ::std::option::Option<& str> {
        self.upload_id.as_deref()
    }
}
impl ListPartsOutput {
    /// Creates a new builder-style object to manufacture [`ListPartsOutput`](crate::operation::list_parts::ListPartsOutput).
    pub fn builder() -> crate::operation::list_parts::builders::ListPartsOutputBuilder {
        crate::operation::list_parts::builders::ListPartsOutputBuilder::default()
    }
}

/// A builder for [`ListPartsOutput`](crate::operation::list_parts::ListPartsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ListPartsOutputBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) content_type: ::std::option::Option<::std::string::String>,
    pub(crate) encoding_type: ::std::option::Option<::std::string::String>,
    pub(crate) initiator: ::std::option::Option<crate::types::Initiator>,
    pub(crate) is_truncated: ::std::option::Option<bool>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) max_parts: ::std::option::Option<i32>,
    pub(crate) next_part_number_marker: ::std::option::Option<i32>,
    pub(crate) owner: ::std::option::Option<crate::types::Owner>,
    pub(crate) part: ::std::option::Option<::std::vec::Vec::<crate::types::Part>>,
    pub(crate) part_number_marker: ::std::option::Option<i32>,
    pub(crate) storage_class: ::std::option::Option<::std::string::String>,
    pub(crate) upload_id: ::std::option::Option<::std::string::String>,
}
impl ListPartsOutputBuilder {
    #[allow(missing_docs)] // documentation missing in model
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
    pub fn initiator(mut self, input: crate::types::Initiator) -> Self {
        self.initiator = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_initiator(mut self, input: ::std::option::Option<crate::types::Initiator>) -> Self {
        self.initiator = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_initiator(&self) -> &::std::option::Option<crate::types::Initiator> {
        &self.initiator
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_is_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_truncated = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_is_truncated(&self) -> &::std::option::Option<bool> {
        &self.is_truncated
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn max_parts(mut self, input: i32) -> Self {
        self.max_parts = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_max_parts(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_parts = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_max_parts(&self) -> &::std::option::Option<i32> {
        &self.max_parts
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn next_part_number_marker(mut self, input: i32) -> Self {
        self.next_part_number_marker = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_next_part_number_marker(mut self, input: ::std::option::Option<i32>) -> Self {
        self.next_part_number_marker = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_next_part_number_marker(&self) -> &::std::option::Option<i32> {
        &self.next_part_number_marker
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
    /// Appends an item to `part`.
    ///
    /// To override the contents of this collection use [`set_part`](Self::set_part).
    ///
    pub fn part(mut self, input: crate::types::Part) -> Self {
        let mut v = self.part.unwrap_or_default();
                        v.push(input);
                        self.part = ::std::option::Option::Some(v);
                        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_part(mut self, input: ::std::option::Option<::std::vec::Vec::<crate::types::Part>>) -> Self {
        self.part = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_part(&self) -> &::std::option::Option<::std::vec::Vec::<crate::types::Part>> {
        &self.part
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn part_number_marker(mut self, input: i32) -> Self {
        self.part_number_marker = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_part_number_marker(mut self, input: ::std::option::Option<i32>) -> Self {
        self.part_number_marker = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_part_number_marker(&self) -> &::std::option::Option<i32> {
        &self.part_number_marker
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn storage_class(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.storage_class = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_storage_class(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.storage_class = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_storage_class(&self) -> &::std::option::Option<::std::string::String> {
        &self.storage_class
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn upload_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.upload_id = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_upload_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.upload_id = input; self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_upload_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.upload_id
    }
    /// Consumes the builder and constructs a [`ListPartsOutput`](crate::operation::list_parts::ListPartsOutput).
    pub fn build(self) -> crate::operation::list_parts::ListPartsOutput {
        crate::operation::list_parts::ListPartsOutput {
            bucket: self.bucket
            ,
            content_type: self.content_type
            ,
            encoding_type: self.encoding_type
            ,
            initiator: self.initiator
            ,
            is_truncated: self.is_truncated
            ,
            key: self.key
            ,
            max_parts: self.max_parts
            ,
            next_part_number_marker: self.next_part_number_marker
            ,
            owner: self.owner
            ,
            part: self.part
            ,
            part_number_marker: self.part_number_marker
            ,
            storage_class: self.storage_class
            ,
            upload_id: self.upload_id
            ,
        }
    }
}

