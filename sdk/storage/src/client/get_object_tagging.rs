// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetObjectTagging`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`object(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::object) / [`set_object(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_object):<br>required: **true**<br>(undocumented)<br>
    ///   - [`version_id(impl Into<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::version_id) / [`set_version_id(Option<String>)`](crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::set_version_id):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`GetObjectTaggingOutput`](crate::operation::get_object_tagging::GetObjectTaggingOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::operation::get_object_tagging::GetObjectTaggingOutput::content_type): (undocumented)
    ///   - [`tag_set(Option<TagSet>)`](crate::operation::get_object_tagging::GetObjectTaggingOutput::tag_set): (undocumented)
    ///   - [`x_amz_version_id(Option<String>)`](crate::operation::get_object_tagging::GetObjectTaggingOutput::x_amz_version_id): (undocumented)
                            /// - On failure, responds with [`SdkError<GetObjectTaggingError>`](crate::operation::get_object_tagging::GetObjectTaggingError)
    pub fn get_object_tagging(&self) -> crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder {
                                crate::operation::get_object_tagging::builders::GetObjectTaggingFluentBuilder::new(self.handle.clone())
                            }
}

