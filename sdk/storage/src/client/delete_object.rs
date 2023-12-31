// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteObject`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`object(impl Into<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::object) / [`set_object(Option<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::set_object):<br>required: **true**<br>(undocumented)<br>
    ///   - [`version_id(impl Into<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::delete_object::builders::DeleteObjectFluentBuilder::set_version_id):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`DeleteObjectOutput`](crate::operation::delete_object::DeleteObjectOutput) with field(s):
    ///   - [`x_amz_version_id(Option<String>)`](crate::operation::delete_object::DeleteObjectOutput::x_amz_version_id): (undocumented)
                            /// - On failure, responds with [`SdkError<DeleteObjectError>`](crate::operation::delete_object::DeleteObjectError)
    pub fn delete_object(&self) -> crate::operation::delete_object::builders::DeleteObjectFluentBuilder {
                                crate::operation::delete_object::builders::DeleteObjectFluentBuilder::new(self.handle.clone())
                            }
}

