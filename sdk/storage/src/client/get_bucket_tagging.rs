// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketTagging`](crate::operation::get_bucket_tagging::builders::GetBucketTaggingFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_tagging::builders::GetBucketTaggingFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_tagging::builders::GetBucketTaggingFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`GetBucketTaggingOutput`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput::content_type): (undocumented)
    ///   - [`tag_set(Option<Vec::<TagSetOfGetBucketTagging>>)`](crate::operation::get_bucket_tagging::GetBucketTaggingOutput::tag_set): (undocumented)
                            /// - On failure, responds with [`SdkError<GetBucketTaggingError>`](crate::operation::get_bucket_tagging::GetBucketTaggingError)
    pub fn get_bucket_tagging(&self) -> crate::operation::get_bucket_tagging::builders::GetBucketTaggingFluentBuilder {
                                crate::operation::get_bucket_tagging::builders::GetBucketTaggingFluentBuilder::new(self.handle.clone())
                            }
}

