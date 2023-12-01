// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_consistency_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_consistency::GetBucketConsistencyOutput, crate::operation::get_bucket_consistency::GetBucketConsistencyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_bucket_consistency::GetBucketConsistencyError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_consistency::GetBucketConsistencyError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_consistency_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_consistency::GetBucketConsistencyOutput, crate::operation::get_bucket_consistency::GetBucketConsistencyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_consistency::builders::GetBucketConsistencyOutputBuilder::default();
        output = output.set_consistency(
            crate::protocol_serde::shape_get_bucket_consistency_output::de_consistency_payload(_response_body)?
        );
        output = output.set_content_type(
            crate::protocol_serde::shape_get_bucket_consistency_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_bucket_consistency::GetBucketConsistencyError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}
