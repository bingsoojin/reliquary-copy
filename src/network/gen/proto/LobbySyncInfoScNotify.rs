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

//! Generated file from `LobbySyncInfoScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:LobbySyncInfoScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct LobbySyncInfoScNotify {
    // message fields
    // @@protoc_insertion_point(field:LobbySyncInfoScNotify.LKLJKGKLHID)
    pub LKLJKGKLHID: ::std::vec::Vec<super::DBBLOFLAAMH::DBBLOFLAAMH>,
    // @@protoc_insertion_point(field:LobbySyncInfoScNotify.MPNJPFDCBDG)
    pub MPNJPFDCBDG: ::protobuf::EnumOrUnknown<super::LobbyModifyType::LobbyModifyType>,
    // @@protoc_insertion_point(field:LobbySyncInfoScNotify.LNCNOFOEHAA)
    pub LNCNOFOEHAA: u32,
    // special fields
    // @@protoc_insertion_point(special_field:LobbySyncInfoScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a LobbySyncInfoScNotify {
    fn default() -> &'a LobbySyncInfoScNotify {
        <LobbySyncInfoScNotify as ::protobuf::Message>::default_instance()
    }
}

impl LobbySyncInfoScNotify {
    pub fn new() -> LobbySyncInfoScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LKLJKGKLHID",
            |m: &LobbySyncInfoScNotify| { &m.LKLJKGKLHID },
            |m: &mut LobbySyncInfoScNotify| { &mut m.LKLJKGKLHID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MPNJPFDCBDG",
            |m: &LobbySyncInfoScNotify| { &m.MPNJPFDCBDG },
            |m: &mut LobbySyncInfoScNotify| { &mut m.MPNJPFDCBDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LNCNOFOEHAA",
            |m: &LobbySyncInfoScNotify| { &m.LNCNOFOEHAA },
            |m: &mut LobbySyncInfoScNotify| { &mut m.LNCNOFOEHAA },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<LobbySyncInfoScNotify>(
            "LobbySyncInfoScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for LobbySyncInfoScNotify {
    const NAME: &'static str = "LobbySyncInfoScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.LKLJKGKLHID.push(is.read_message()?);
                },
                80 => {
                    self.MPNJPFDCBDG = is.read_enum_or_unknown()?;
                },
                32 => {
                    self.LNCNOFOEHAA = is.read_uint32()?;
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
        for value in &self.LKLJKGKLHID {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None) {
            my_size += ::protobuf::rt::int32_size(10, self.MPNJPFDCBDG.value());
        }
        if self.LNCNOFOEHAA != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.LNCNOFOEHAA);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LKLJKGKLHID {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        if self.MPNJPFDCBDG != ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None) {
            os.write_enum(10, ::protobuf::EnumOrUnknown::value(&self.MPNJPFDCBDG))?;
        }
        if self.LNCNOFOEHAA != 0 {
            os.write_uint32(4, self.LNCNOFOEHAA)?;
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

    fn new() -> LobbySyncInfoScNotify {
        LobbySyncInfoScNotify::new()
    }

    fn clear(&mut self) {
        self.LKLJKGKLHID.clear();
        self.MPNJPFDCBDG = ::protobuf::EnumOrUnknown::new(super::LobbyModifyType::LobbyModifyType::LobbyModifyType_None);
        self.LNCNOFOEHAA = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static LobbySyncInfoScNotify {
        static instance: LobbySyncInfoScNotify = LobbySyncInfoScNotify {
            LKLJKGKLHID: ::std::vec::Vec::new(),
            MPNJPFDCBDG: ::protobuf::EnumOrUnknown::from_i32(0),
            LNCNOFOEHAA: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for LobbySyncInfoScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("LobbySyncInfoScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for LobbySyncInfoScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LobbySyncInfoScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bLobbySyncInfoScNotify.proto\x1a\x11DBBLOFLAAMH.proto\x1a\x15LobbyM\
    odifyType.proto\"\x9d\x01\n\x15LobbySyncInfoScNotify\x12.\n\x0bLKLJKGKLH\
    ID\x18\r\x20\x03(\x0b2\x0c.DBBLOFLAAMHR\x0bLKLJKGKLHID\x122\n\x0bMPNJPFD\
    CBDG\x18\n\x20\x01(\x0e2\x10.LobbyModifyTypeR\x0bMPNJPFDCBDG\x12\x20\n\
    \x0bLNCNOFOEHAA\x18\x04\x20\x01(\rR\x0bLNCNOFOEHAAb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(2);
            deps.push(super::DBBLOFLAAMH::file_descriptor().clone());
            deps.push(super::LobbyModifyType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(LobbySyncInfoScNotify::generated_message_descriptor_data());
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
