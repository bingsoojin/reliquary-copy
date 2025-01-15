// This file is generated by rust-protobuf 3.4.0. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `UpdateTrackMainMissionIdCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UpdateTrackMainMissionIdCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UpdateTrackMainMissionIdCsReq {
    // message fields
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdCsReq.ECCLGIGAAOO)
    pub ECCLGIGAAOO: u32,
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdCsReq.POKBILKJIAE)
    pub POKBILKJIAE: u32,
    // @@protoc_insertion_point(field:UpdateTrackMainMissionIdCsReq.PJOPGADPDHD)
    pub PJOPGADPDHD: ::protobuf::EnumOrUnknown<super::TrackMainMissionUpdateReasonId::TrackMainMissionUpdateReasonId>,
    // special fields
    // @@protoc_insertion_point(special_field:UpdateTrackMainMissionIdCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UpdateTrackMainMissionIdCsReq {
    fn default() -> &'a UpdateTrackMainMissionIdCsReq {
        <UpdateTrackMainMissionIdCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UpdateTrackMainMissionIdCsReq {
    pub fn new() -> UpdateTrackMainMissionIdCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ECCLGIGAAOO",
            |m: &UpdateTrackMainMissionIdCsReq| { &m.ECCLGIGAAOO },
            |m: &mut UpdateTrackMainMissionIdCsReq| { &mut m.ECCLGIGAAOO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "POKBILKJIAE",
            |m: &UpdateTrackMainMissionIdCsReq| { &m.POKBILKJIAE },
            |m: &mut UpdateTrackMainMissionIdCsReq| { &mut m.POKBILKJIAE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PJOPGADPDHD",
            |m: &UpdateTrackMainMissionIdCsReq| { &m.PJOPGADPDHD },
            |m: &mut UpdateTrackMainMissionIdCsReq| { &mut m.PJOPGADPDHD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UpdateTrackMainMissionIdCsReq>(
            "UpdateTrackMainMissionIdCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UpdateTrackMainMissionIdCsReq {
    const NAME: &'static str = "UpdateTrackMainMissionIdCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.ECCLGIGAAOO = is.read_uint32()?;
                },
                40 => {
                    self.POKBILKJIAE = is.read_uint32()?;
                },
                32 => {
                    self.PJOPGADPDHD = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.ECCLGIGAAOO != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.ECCLGIGAAOO);
        }
        if self.POKBILKJIAE != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.POKBILKJIAE);
        }
        if self.PJOPGADPDHD != ::protobuf::EnumOrUnknown::new(super::TrackMainMissionUpdateReasonId::TrackMainMissionUpdateReasonId::TRACK_MAIN_MISSION_UPDATE_NONE) {
            my_size += ::protobuf::rt::int32_size(4, self.PJOPGADPDHD.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ECCLGIGAAOO != 0 {
            os.write_uint32(6, self.ECCLGIGAAOO)?;
        }
        if self.POKBILKJIAE != 0 {
            os.write_uint32(5, self.POKBILKJIAE)?;
        }
        if self.PJOPGADPDHD != ::protobuf::EnumOrUnknown::new(super::TrackMainMissionUpdateReasonId::TrackMainMissionUpdateReasonId::TRACK_MAIN_MISSION_UPDATE_NONE) {
            os.write_enum(4, ::protobuf::EnumOrUnknown::value(&self.PJOPGADPDHD))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> UpdateTrackMainMissionIdCsReq {
        UpdateTrackMainMissionIdCsReq::new()
    }

    fn clear(&mut self) {
        self.ECCLGIGAAOO = 0;
        self.POKBILKJIAE = 0;
        self.PJOPGADPDHD = ::protobuf::EnumOrUnknown::new(super::TrackMainMissionUpdateReasonId::TrackMainMissionUpdateReasonId::TRACK_MAIN_MISSION_UPDATE_NONE);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UpdateTrackMainMissionIdCsReq {
        static instance: UpdateTrackMainMissionIdCsReq = UpdateTrackMainMissionIdCsReq {
            ECCLGIGAAOO: 0,
            POKBILKJIAE: 0,
            PJOPGADPDHD: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UpdateTrackMainMissionIdCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UpdateTrackMainMissionIdCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UpdateTrackMainMissionIdCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UpdateTrackMainMissionIdCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#UpdateTrackMainMissionIdCsReq.proto\x1a$TrackMainMissionUpdateReasonI\
    d.proto\"\xa6\x01\n\x1dUpdateTrackMainMissionIdCsReq\x12\x20\n\x0bECCLGI\
    GAAOO\x18\x06\x20\x01(\rR\x0bECCLGIGAAOO\x12\x20\n\x0bPOKBILKJIAE\x18\
    \x05\x20\x01(\rR\x0bPOKBILKJIAE\x12A\n\x0bPJOPGADPDHD\x18\x04\x20\x01(\
    \x0e2\x1f.TrackMainMissionUpdateReasonIdR\x0bPJOPGADPDHDb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(1);
            deps.push(super::TrackMainMissionUpdateReasonId::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UpdateTrackMainMissionIdCsReq::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
