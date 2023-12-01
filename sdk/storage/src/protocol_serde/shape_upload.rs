// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_upload(decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::types::Upload, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Upload::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DisplayName") /* DisplayName com.nifcloud.api.storage#Upload$DisplayName */ =>  {
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
            s if s.matches("ID") /* ID com.nifcloud.api.storage#Upload$ID */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_id(var_2);
            }
            ,
            s if s.matches("Initiated") /* Initiated com.nifcloud.api.storage#Upload$Initiated */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `smithy.api#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_initiated(var_3);
            }
            ,
            s if s.matches("Initiator") /* Initiator com.nifcloud.api.storage#Upload$Initiator */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_initiator::de_initiator(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_initiator(var_4);
            }
            ,
            s if s.matches("Key") /* Key com.nifcloud.api.storage#Upload$Key */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_5);
            }
            ,
            s if s.matches("Owner") /* Owner com.nifcloud.api.storage#Upload$Owner */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_owner::de_owner(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_owner(var_6);
            }
            ,
            s if s.matches("StorageClass") /* StorageClass com.nifcloud.api.storage#Upload$StorageClass */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_storage_class(var_7);
            }
            ,
            s if s.matches("UploadId") /* UploadId com.nifcloud.api.storage#Upload$UploadId */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_upload_id(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
