// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucket`](crate::operation::get_bucket::builders::GetBucketFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`delimiter(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::delimiter) / [`set_delimiter(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_delimiter):<br>required: **false**<br>(undocumented)<br>
    ///   - [`encoding_type(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::encoding_type) / [`set_encoding_type(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_encoding_type):<br>required: **false**<br>(undocumented)<br>
    ///   - [`marker(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_marker):<br>required: **false**<br>(undocumented)<br>
    ///   - [`max_keys(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::max_keys) / [`set_max_keys(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_max_keys):<br>required: **false**<br>(undocumented)<br>
    ///   - [`prefix(impl Into<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::prefix) / [`set_prefix(Option<String>)`](crate::operation::get_bucket::builders::GetBucketFluentBuilder::set_prefix):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`GetBucketOutput`](crate::operation::get_bucket::GetBucketOutput) with field(s):
    ///   - [`common_prefixes(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::common_prefixes): (undocumented)
    ///   - [`contents(Option<Vec::<Contents>>)`](crate::operation::get_bucket::GetBucketOutput::contents): (undocumented)
    ///   - [`content_type(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::content_type): (undocumented)
    ///   - [`delimiter(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::delimiter): (undocumented)
    ///   - [`encoding_type(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::encoding_type): (undocumented)
    ///   - [`is_truncated(Option<bool>)`](crate::operation::get_bucket::GetBucketOutput::is_truncated): (undocumented)
    ///   - [`marker(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::marker): (undocumented)
    ///   - [`max_keys(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::max_keys): (undocumented)
    ///   - [`name(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::name): (undocumented)
    ///   - [`next_marker(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::next_marker): (undocumented)
    ///   - [`prefix(Option<String>)`](crate::operation::get_bucket::GetBucketOutput::prefix): (undocumented)
                            /// - On failure, responds with [`SdkError<GetBucketError>`](crate::operation::get_bucket::GetBucketError)
    pub fn get_bucket(&self) -> crate::operation::get_bucket::builders::GetBucketFluentBuilder {
                                crate::operation::get_bucket::builders::GetBucketFluentBuilder::new(self.handle.clone())
                            }
}
