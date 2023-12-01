// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_part(decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::types::Part, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Part::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ETag") /* ETag com.nifcloud.api.storage#Part$ETag */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_e_tag(var_1);
            }
            ,
            s if s.matches("LastModified") /* LastModified com.nifcloud.api.storage#Part$LastModified */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `smithy.api#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified(var_2);
            }
            ,
            s if s.matches("PartNumber") /* PartNumber com.nifcloud.api.storage#Part$PartNumber */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `smithy.api#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_part_number(var_3);
            }
            ,
            s if s.matches("Size") /* Size com.nifcloud.api.storage#Part$Size */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `smithy.api#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_size(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

