// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucket`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`DeleteBucketOutput`](crate::operation::delete_bucket::DeleteBucketOutput)
                            /// - On failure, responds with [`SdkError<DeleteBucketError>`](crate::operation::delete_bucket::DeleteBucketError)
    pub fn delete_bucket(&self) -> crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder {
                                crate::operation::delete_bucket::builders::DeleteBucketFluentBuilder::new(self.handle.clone())
                            }
}

