// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_lifecycle_configuration_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput, crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_lifecycle_configuration_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationOutput, crate::operation::put_bucket_lifecycle_configuration::PutBucketLifecycleConfigurationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_bucket_lifecycle_configuration::builders::PutBucketLifecycleConfigurationOutputBuilder::default();
        output.build()
    })
}

