// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetService`](crate::operation::get_service::builders::GetServiceFluentBuilder) operation.
                            ///
                            /// - The fluent builder takes no input, just [`send`](crate::operation::get_service::builders::GetServiceFluentBuilder::send) it.
                            /// - On success, responds with [`GetServiceOutput`](crate::operation::get_service::GetServiceOutput) with field(s):
    ///   - [`buckets(Option<Vec::<Buckets>>)`](crate::operation::get_service::GetServiceOutput::buckets): (undocumented)
    ///   - [`owner(Option<Owner>)`](crate::operation::get_service::GetServiceOutput::owner): (undocumented)
                            /// - On failure, responds with [`SdkError<GetServiceError>`](crate::operation::get_service::GetServiceError)
    pub fn get_service(&self) -> crate::operation::get_service::builders::GetServiceFluentBuilder {
                                crate::operation::get_service::builders::GetServiceFluentBuilder::new(self.handle.clone())
                            }
}
