// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_contents(decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::types::Contents, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Contents::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DisplayName") /* DisplayName com.nifcloud.api.storage#Contents$DisplayName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_display_name(var_1);
            }
            ,
            s if s.matches("ETag") /* ETag com.nifcloud.api.storage#Contents$ETag */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_e_tag(var_2);
            }
            ,
            s if s.matches("ID") /* ID com.nifcloud.api.storage#Contents$ID */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_3);
            }
            ,
            s if s.matches("Key") /* Key com.nifcloud.api.storage#Contents$Key */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_4);
            }
            ,
            s if s.matches("LastModified") /* LastModified com.nifcloud.api.storage#Contents$LastModified */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `smithy.api#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_last_modified(var_5);
            }
            ,
            s if s.matches("Owner") /* Owner com.nifcloud.api.storage#Contents$Owner */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_owner::de_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_6);
            }
            ,
            s if s.matches("Size") /* Size com.nifcloud.api.storage#Contents$Size */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_size(var_7);
            }
            ,
            s if s.matches("StorageClass") /* StorageClass com.nifcloud.api.storage#Contents$StorageClass */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_class(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}

