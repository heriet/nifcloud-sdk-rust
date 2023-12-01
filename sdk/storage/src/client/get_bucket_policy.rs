// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetBucketPolicy`](crate::operation::get_bucket_policy::builders::GetBucketPolicyFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::get_bucket_policy::builders::GetBucketPolicyFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::get_bucket_policy::builders::GetBucketPolicyFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`GetBucketPolicyOutput`](crate::operation::get_bucket_policy::GetBucketPolicyOutput) with field(s):
    ///   - [`content_type(Option<String>)`](crate::operation::get_bucket_policy::GetBucketPolicyOutput::content_type): (undocumented)
    ///   - [`policy(Option<String>)`](crate::operation::get_bucket_policy::GetBucketPolicyOutput::policy): (undocumented)
                            /// - On failure, responds with [`SdkError<GetBucketPolicyError>`](crate::operation::get_bucket_policy::GetBucketPolicyError)
    pub fn get_bucket_policy(&self) -> crate::operation::get_bucket_policy::builders::GetBucketPolicyFluentBuilder {
                                crate::operation::get_bucket_policy::builders::GetBucketPolicyFluentBuilder::new(self.handle.clone())
                            }
}

