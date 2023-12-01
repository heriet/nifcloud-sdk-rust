// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_consistency_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_bucket_consistency::PutBucketConsistencyOutput, crate::operation::put_bucket_consistency::PutBucketConsistencyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::put_bucket_consistency::PutBucketConsistencyError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::put_bucket_consistency::PutBucketConsistencyError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_bucket_consistency_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::put_bucket_consistency::PutBucketConsistencyOutput, crate::operation::put_bucket_consistency::PutBucketConsistencyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_bucket_consistency::builders::PutBucketConsistencyOutputBuilder::default();
        output = output.set_comtent_type(
            crate::protocol_serde::shape_put_bucket_consistency_output::de_comtent_type_header(_response_headers)
                                    .map_err(|_|crate::operation::put_bucket_consistency::PutBucketConsistencyError::unhandled("Failed to parse ComtentType from header `Comtent-Type"))?
        );
        output.build()
    })
}

