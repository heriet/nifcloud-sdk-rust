// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutObjectCopy`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`object(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::object) / [`set_object(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_object):<br>required: **true**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source) / [`set_x_amz_copy_source(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source):<br>required: **true**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_if_match(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_if_match) / [`set_x_amz_copy_source_if_match(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_if_match):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_if_modified_since(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_if_modified_since) / [`set_x_amz_copy_source_if_modified_since(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_if_modified_since):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_if_none_match(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_if_none_match) / [`set_x_amz_copy_source_if_none_match(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_if_none_match):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_if_unmodified_since(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_if_unmodified_since) / [`set_x_amz_copy_source_if_unmodified_since(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_if_unmodified_since):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_server_side_encryption_customer_algorithm(XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_server_side_encryption_customer_algorithm) / [`set_x_amz_copy_source_server_side_encryption_customer_algorithm(Option<XAmzCopySourceServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_server_side_encryption_customer_algorithm):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_server_side_encryption_customer_key(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_server_side_encryption_customer_key) / [`set_x_amz_copy_source_server_side_encryption_customer_key(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_server_side_encryption_customer_key):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_copy_source_server_side_encryption_customer_key_md5(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_copy_source_server_side_encryption_customer_key_md5) / [`set_x_amz_copy_source_server_side_encryption_customer_key_md5(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_copy_source_server_side_encryption_customer_key_md5):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_metadata_directive(XAmzMetadataDirectiveOfPutObjectCopyRequest)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_metadata_directive) / [`set_x_amz_metadata_directive(Option<XAmzMetadataDirectiveOfPutObjectCopyRequest>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_metadata_directive):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_server_side_encryption) / [`set_x_amz_server_side_encryption(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_server_side_encryption):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_algorithm(XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_server_side_encryption_customer_algorithm) / [`set_x_amz_server_side_encryption_customer_algorithm(Option<XAmzServerSideEncryptionCustomerAlgorithmOfPutObjectCopyRequest>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_server_side_encryption_customer_algorithm):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_key(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_server_side_encryption_customer_key) / [`set_x_amz_server_side_encryption_customer_key(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_server_side_encryption_customer_key):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_key_md5(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_server_side_encryption_customer_key_md5) / [`set_x_amz_server_side_encryption_customer_key_md5(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_server_side_encryption_customer_key_md5):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_storage_class(XAmzStorageClassOfPutObjectCopyRequest)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_storage_class) / [`set_x_amz_storage_class(Option<XAmzStorageClassOfPutObjectCopyRequest>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_storage_class):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_tagging(impl Into<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_tagging) / [`set_x_amz_tagging(Option<String>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_tagging):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_tagging_directive(XAmzTaggingDirectiveOfPutObjectCopyRequest)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::x_amz_tagging_directive) / [`set_x_amz_tagging_directive(Option<XAmzTaggingDirectiveOfPutObjectCopyRequest>)`](crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::set_x_amz_tagging_directive):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`PutObjectCopyOutput`](crate::operation::put_object_copy::PutObjectCopyOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::operation::put_object_copy::PutObjectCopyOutput::content_type): (undocumented)
    ///   - [`e_tag(Option<String>)`](crate::operation::put_object_copy::PutObjectCopyOutput::e_tag): (undocumented)
    ///   - [`last_modified(Option<DateTime>)`](crate::operation::put_object_copy::PutObjectCopyOutput::last_modified): (undocumented)
                            /// - On failure, responds with [`SdkError<PutObjectCopyError>`](crate::operation::put_object_copy::PutObjectCopyError)
    pub fn put_object_copy(&self) -> crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder {
                                crate::operation::put_object_copy::builders::PutObjectCopyFluentBuilder::new(self.handle.clone())
                            }
}

