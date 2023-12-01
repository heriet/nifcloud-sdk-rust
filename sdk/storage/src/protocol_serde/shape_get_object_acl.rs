// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_acl_http_error(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_object_acl::GetObjectAclOutput, crate::operation::get_object_acl::GetObjectACLError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body).map_err(crate::operation::get_object_acl::GetObjectACLError::unhandled)?;
    let generic = generic_builder.build();
    Err(crate::operation::get_object_acl::GetObjectACLError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_object_acl_http_response(_response_status: u16, _response_headers: &::aws_smithy_runtime_api::http::Headers, _response_body: &[u8]) -> std::result::Result<crate::operation::get_object_acl::GetObjectAclOutput, crate::operation::get_object_acl::GetObjectACLError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_object_acl::builders::GetObjectAclOutputBuilder::default();
        output = crate::protocol_serde::shape_get_object_acl::de_get_object_acl(_response_body, output).map_err(crate::operation::get_object_acl::GetObjectACLError::unhandled)?;
        output = output.set_content_type(
            crate::protocol_serde::shape_get_object_acl_output::de_content_type_header(_response_headers)
                                    .map_err(|_|crate::operation::get_object_acl::GetObjectACLError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_object_acl(inp: &[u8], mut builder: crate::operation::get_object_acl::builders::GetObjectAclOutputBuilder) -> Result<crate::operation::get_object_acl::builders::GetObjectAclOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !start_el.matches("GetObjectACLResult") {
                            return Err(
                                ::aws_smithy_xml::decode::XmlDecodeError::custom(
                                    format!("encountered invalid XML root: expected GetObjectACLResult but got {:?}. This is likely a bug in the SDK.", start_el)
                                )
                            )
                        }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("AccessControlList") /* AccessControlList com.nifcloud.api.storage.synthetic#GetObjectACLOutput$AccessControlList */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_access_control_list::de_access_control_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_access_control_list(var_1);
            }
            ,
            s if s.matches("Owner") /* Owner com.nifcloud.api.storage.synthetic#GetObjectACLOutput$Owner */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_owner::de_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
