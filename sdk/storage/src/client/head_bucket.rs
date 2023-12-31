// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`HeadBucket`](crate::operation::head_bucket::builders::HeadBucketFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::head_bucket::builders::HeadBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::head_bucket::builders::HeadBucketFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`HeadBucketOutput`](crate::operation::head_bucket::HeadBucketOutput)
                            /// - On failure, responds with [`SdkError<HeadBucketError>`](crate::operation::head_bucket::HeadBucketError)
    pub fn head_bucket(&self) -> crate::operation::head_bucket::builders::HeadBucketFluentBuilder {
                                crate::operation::head_bucket::builders::HeadBucketFluentBuilder::new(self.handle.clone())
                            }
}

