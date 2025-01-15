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

//! Generated file from `SyncRogueMapRoomScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SyncRogueMapRoomScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SyncRogueMapRoomScNotify {
    // message fields
    // @@protoc_insertion_point(field:SyncRogueMapRoomScNotify.CHNEBLIKCGP)
    pub CHNEBLIKCGP: ::protobuf::MessageField<super::EJCNEMNLGEE::EJCNEMNLGEE>,
    // @@protoc_insertion_point(field:SyncRogueMapRoomScNotify.PBIGGPACCPB)
    pub PBIGGPACCPB: u32,
    // special fields
    // @@protoc_insertion_point(special_field:SyncRogueMapRoomScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SyncRogueMapRoomScNotify {
    fn default() -> &'a SyncRogueMapRoomScNotify {
        <SyncRogueMapRoomScNotify as ::protobuf::Message>::default_instance()
    }
}

impl SyncRogueMapRoomScNotify {
    pub fn new() -> SyncRogueMapRoomScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::EJCNEMNLGEE::EJCNEMNLGEE>(
            "CHNEBLIKCGP",
            |m: &SyncRogueMapRoomScNotify| { &m.CHNEBLIKCGP },
            |m: &mut SyncRogueMapRoomScNotify| { &mut m.CHNEBLIKCGP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PBIGGPACCPB",
            |m: &SyncRogueMapRoomScNotify| { &m.PBIGGPACCPB },
            |m: &mut SyncRogueMapRoomScNotify| { &mut m.PBIGGPACCPB },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SyncRogueMapRoomScNotify>(
            "SyncRogueMapRoomScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SyncRogueMapRoomScNotify {
    const NAME: &'static str = "SyncRogueMapRoomScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.CHNEBLIKCGP)?;
                },
                72 => {
                    self.PBIGGPACCPB = is.read_uint32()?;
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
        if let Some(v) = self.CHNEBLIKCGP.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.PBIGGPACCPB != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.PBIGGPACCPB);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.CHNEBLIKCGP.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        if self.PBIGGPACCPB != 0 {
            os.write_uint32(9, self.PBIGGPACCPB)?;
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

    fn new() -> SyncRogueMapRoomScNotify {
        SyncRogueMapRoomScNotify::new()
    }

    fn clear(&mut self) {
        self.CHNEBLIKCGP.clear();
        self.PBIGGPACCPB = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SyncRogueMapRoomScNotify {
        static instance: SyncRogueMapRoomScNotify = SyncRogueMapRoomScNotify {
            CHNEBLIKCGP: ::protobuf::MessageField::none(),
            PBIGGPACCPB: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SyncRogueMapRoomScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SyncRogueMapRoomScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SyncRogueMapRoomScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SyncRogueMapRoomScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eSyncRogueMapRoomScNotify.proto\x1a\x11EJCNEMNLGEE.proto\"l\n\x18Sy\
    ncRogueMapRoomScNotify\x12.\n\x0bCHNEBLIKCGP\x18\x0c\x20\x01(\x0b2\x0c.E\
    JCNEMNLGEER\x0bCHNEBLIKCGP\x12\x20\n\x0bPBIGGPACCPB\x18\t\x20\x01(\rR\
    \x0bPBIGGPACCPBb\x06proto3\
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
            deps.push(super::EJCNEMNLGEE::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SyncRogueMapRoomScNotify::generated_message_descriptor_data());
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
