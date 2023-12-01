// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`HeadObject`](crate::operation::head_object::builders::HeadObjectFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`consistency_control(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::consistency_control) / [`set_consistency_control(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_consistency_control):<br>required: **false**<br>(undocumented)<br>
    ///   - [`object(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::object) / [`set_object(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_object):<br>required: **true**<br>(undocumented)<br>
    ///   - [`part_number(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::part_number) / [`set_part_number(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_part_number):<br>required: **false**<br>(undocumented)<br>
    ///   - [`version_id(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_version_id):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_algorithm(XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::x_amz_server_side_encryption_customer_algorithm) / [`set_x_amz_server_side_encryption_customer_algorithm(Option<XAmzServerSideEncryptionCustomerAlgorithmOfHeadObjectRequest>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_x_amz_server_side_encryption_customer_algorithm):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_key(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::x_amz_server_side_encryption_customer_key) / [`set_x_amz_server_side_encryption_customer_key(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_x_amz_server_side_encryption_customer_key):<br>required: **false**<br>(undocumented)<br>
    ///   - [`x_amz_server_side_encryption_customer_key_md5(impl Into<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::x_amz_server_side_encryption_customer_key_md5) / [`set_x_amz_server_side_encryption_customer_key_md5(Option<String>)`](crate::operation::head_object::builders::HeadObjectFluentBuilder::set_x_amz_server_side_encryption_customer_key_md5):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`HeadObjectOutput`](crate::operation::head_object::HeadObjectOutput) with field(s):
    ///   - [`accept_ranges(Option<String>)`](crate::operation::head_object::HeadObjectOutput::accept_ranges): (undocumented)
    ///   - [`content_type(Option<String>)`](crate::operation::head_object::HeadObjectOutput::content_type): (undocumented)
    ///   - [`e_tag(Option<String>)`](crate::operation::head_object::HeadObjectOutput::e_tag): (undocumented)
    ///   - [`last_modified(Option<String>)`](crate::operation::head_object::HeadObjectOutput::last_modified): (undocumented)
    ///   - [`x_amz_expiration(Option<String>)`](crate::operation::head_object::HeadObjectOutput::x_amz_expiration): (undocumented)
    ///   - [`x_amz_mp_parts_count(Option<String>)`](crate::operation::head_object::HeadObjectOutput::x_amz_mp_parts_count): (undocumented)
    ///   - [`x_amz_server_side_encryption(Option<String>)`](crate::operation::head_object::HeadObjectOutput::x_amz_server_side_encryption): (undocumented)
    ///   - [`x_amz_version_id(Option<String>)`](crate::operation::head_object::HeadObjectOutput::x_amz_version_id): (undocumented)
                            /// - On failure, responds with [`SdkError<HeadObjectError>`](crate::operation::head_object::HeadObjectError)
    pub fn head_object(&self) -> crate::operation::head_object::builders::HeadObjectFluentBuilder {
                                crate::operation::head_object::builders::HeadObjectFluentBuilder::new(self.handle.clone())
                            }
}

