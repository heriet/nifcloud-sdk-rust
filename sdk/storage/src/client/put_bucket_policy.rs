// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketPolicy`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`content_md5(impl Into<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::content_md5) / [`set_content_md5(Option<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::set_content_md5):<br>required: **false**<br>(undocumented)<br>
    ///   - [`policy(impl Into<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::policy) / [`set_policy(Option<String>)`](crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::set_policy):<br>required: **false**<br>(undocumented)<br>
                            /// - On success, responds with [`PutBucketPolicyOutput`](crate::operation::put_bucket_policy::PutBucketPolicyOutput)
                            /// - On failure, responds with [`SdkError<PutBucketPolicyError>`](crate::operation::put_bucket_policy::PutBucketPolicyError)
    pub fn put_bucket_policy(&self) -> crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder {
                                crate::operation::put_bucket_policy::builders::PutBucketPolicyFluentBuilder::new(self.handle.clone())
                            }
}

