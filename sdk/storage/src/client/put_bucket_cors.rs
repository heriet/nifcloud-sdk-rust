// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutBucketCors`](crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder) operation.
                            ///
                            /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder::set_bucket):<br>required: **true**<br>(undocumented)<br>
    ///   - [`cors_configuration(RequestCorsConfiguration)`](crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder::cors_configuration) / [`set_cors_configuration(Option<RequestCorsConfiguration>)`](crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder::set_cors_configuration):<br>required: **true**<br>(undocumented)<br>
                            /// - On success, responds with [`PutBucketCorsOutput`](crate::operation::put_bucket_cors::PutBucketCorsOutput)
                            /// - On failure, responds with [`SdkError<PutBucketCorsError>`](crate::operation::put_bucket_cors::PutBucketCorsError)
    pub fn put_bucket_cors(&self) -> crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder {
                                crate::operation::put_bucket_cors::builders::PutBucketCorsFluentBuilder::new(self.handle.clone())
                            }
}

