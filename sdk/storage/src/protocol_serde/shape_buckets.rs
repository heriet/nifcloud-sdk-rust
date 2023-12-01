// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_buckets(decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::types::Buckets, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Buckets::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CreationDate") /* CreationDate com.nifcloud.api.storage#Buckets$CreationDate */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `smithy.api#Timestamp`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_date(var_1);
            }
            ,
            s if s.matches("Name") /* Name com.nifcloud.api.storage#Buckets$Name */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
