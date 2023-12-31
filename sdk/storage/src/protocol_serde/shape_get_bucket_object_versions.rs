// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_object_versions_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput, crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bucket_object_versions_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_bucket_object_versions::GetBucketObjectVersionsOutput, crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bucket_object_versions::builders::GetBucketObjectVersionsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_bucket_object_versions::de_get_bucket_object_versions(_response_body, output).map_err(crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_get_bucket_object_versions_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_bucket_object_versions::GetBucketObjectVersionsError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_bucket_object_versions(inp: &[u8], mut builder: crate::operation::get_bucket_object_versions::builders::GetBucketObjectVersionsOutputBuilder) -> Result<crate::operation::get_bucket_object_versions::builders::GetBucketObjectVersionsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetBucketObjectVersionsResult") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetBucketObjectVersionsResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("VersionIdMarker") /* VersionIdMarker com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$VersionIdMarker */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_version_id_marker(var_1);
            }
            ,
            s if s.matches("Version") /* Version com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$Version */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::vec::Vec::<crate::types::Version>, ::aws_smithy_xml::decode::XmlDecodeError>::Ok({
                            let mut list_3 = builder.version.take().unwrap_or_default();
                            list_3.push(
                                crate::protocol_serde::shape_version::de_version(&mut tag)
                                ?
                            );
                            list_3
                        })
                        ?
                    )
                ;
                builder = builder.set_version(var_2);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$IsTruncated */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `smithy.api#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_4);
            }
            ,
            s if s.matches("Prefix") /* Prefix com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$Prefix */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix(var_5);
            }
            ,
            s if s.matches("MaxKeys") /* MaxKeys com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$MaxKeys */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_max_keys(var_6);
            }
            ,
            s if s.matches("KeyMarker") /* KeyMarker com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$KeyMarker */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key_marker(var_7);
            }
            ,
            s if s.matches("Name") /* Name com.nifcloud.api.storage.synthetic#GetBucketObjectVersionsOutput$Name */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}

