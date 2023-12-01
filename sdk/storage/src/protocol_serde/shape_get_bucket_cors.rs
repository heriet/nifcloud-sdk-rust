// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_cors_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_cors::GetBucketCorsOutput, crate::operation::get_bucket_cors::GetBucketCorsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_bucket_cors::GetBucketCorsError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_cors::GetBucketCorsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_cors_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_cors::GetBucketCorsOutput, crate::operation::get_bucket_cors::GetBucketCorsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_bucket_cors::de_get_bucket_cors(_response_body, output).map_err(crate::operation::get_bucket_cors::GetBucketCorsError::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_get_bucket_cors_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_bucket_cors::GetBucketCorsError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_bucket_cors(inp: &[u8], mut builder: crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder) -> Result<crate::operation::get_bucket_cors::builders::GetBucketCorsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetBucketCorsResult") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetBucketCorsResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CORSRule") /* CORSRule com.nifcloud.api.storage.synthetic#GetBucketCorsOutput$CORSRule */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::CorsRule>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_2 = builder.cors_rule.take().unwrap_or_default();
                            list_2.push(
                                crate::protocol_serde::shape_cors_rule::de_cors_rule(&mut tag)
                                ?
                            );
                            list_2
                        })
                        ?
                    )
                ;
                builder = builder.set_cors_rule(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

