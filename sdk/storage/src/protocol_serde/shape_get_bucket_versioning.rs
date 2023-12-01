// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_versioning_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_versioning::GetBucketVersioningOutput, crate::operation::get_bucket_versioning::GetBucketVersioningError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_bucket_versioning::GetBucketVersioningError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_versioning::GetBucketVersioningError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_versioning_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_versioning::GetBucketVersioningOutput, crate::operation::get_bucket_versioning::GetBucketVersioningError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_versioning::builders::GetBucketVersioningOutputBuilder::default();
        output = crate::protocol_serde::shape_get_bucket_versioning::de_get_bucket_versioning(_response_body, output).map_err(crate::operation::get_bucket_versioning::GetBucketVersioningError::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_get_bucket_versioning_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_bucket_versioning::GetBucketVersioningError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_bucket_versioning(inp: &[u8], mut builder: crate::operation::get_bucket_versioning::builders::GetBucketVersioningOutputBuilder) -> Result<crate::operation::get_bucket_versioning::builders::GetBucketVersioningOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetBucketVersioningResult") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetBucketVersioningResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Status") /* Status com.nifcloud.api.storage.synthetic#GetBucketVersioningOutput$Status */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
