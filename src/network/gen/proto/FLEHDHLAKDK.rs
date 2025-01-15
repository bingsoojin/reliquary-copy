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

//! Generated file from `FLEHDHLAKDK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FLEHDHLAKDK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FLEHDHLAKDK {
    // message fields
    // @@protoc_insertion_point(field:FLEHDHLAKDK.NGDAJKNLELE)
    pub NGDAJKNLELE: ::std::vec::Vec<super::EOBCFPEGNJP::EOBCFPEGNJP>,
    // @@protoc_insertion_point(field:FLEHDHLAKDK.HPPMGEHCKHC)
    pub HPPMGEHCKHC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:FLEHDHLAKDK.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::FBDMBILJDMG::FBDMBILJDMG>,
    // special fields
    // @@protoc_insertion_point(special_field:FLEHDHLAKDK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FLEHDHLAKDK {
    fn default() -> &'a FLEHDHLAKDK {
        <FLEHDHLAKDK as ::protobuf::Message>::default_instance()
    }
}

impl FLEHDHLAKDK {
    pub fn new() -> FLEHDHLAKDK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NGDAJKNLELE",
            |m: &FLEHDHLAKDK| { &m.NGDAJKNLELE },
            |m: &mut FLEHDHLAKDK| { &mut m.NGDAJKNLELE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HPPMGEHCKHC",
            |m: &FLEHDHLAKDK| { &m.HPPMGEHCKHC },
            |m: &mut FLEHDHLAKDK| { &mut m.HPPMGEHCKHC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &FLEHDHLAKDK| { &m.avatar_list },
            |m: &mut FLEHDHLAKDK| { &mut m.avatar_list },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FLEHDHLAKDK>(
            "FLEHDHLAKDK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FLEHDHLAKDK {
    const NAME: &'static str = "FLEHDHLAKDK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.NGDAJKNLELE.push(is.read_message()?);
                },
                50 => {
                    is.read_repeated_packed_uint32_into(&mut self.HPPMGEHCKHC)?;
                },
                48 => {
                    self.HPPMGEHCKHC.push(is.read_uint32()?);
                },
                58 => {
                    self.avatar_list.push(is.read_message()?);
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
        for value in &self.NGDAJKNLELE {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.HPPMGEHCKHC {
            my_size += ::protobuf::rt::uint32_size(6, *value);
        };
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NGDAJKNLELE {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.HPPMGEHCKHC {
            os.write_uint32(6, *v)?;
        };
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> FLEHDHLAKDK {
        FLEHDHLAKDK::new()
    }

    fn clear(&mut self) {
        self.NGDAJKNLELE.clear();
        self.HPPMGEHCKHC.clear();
        self.avatar_list.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FLEHDHLAKDK {
        static instance: FLEHDHLAKDK = FLEHDHLAKDK {
            NGDAJKNLELE: ::std::vec::Vec::new(),
            HPPMGEHCKHC: ::std::vec::Vec::new(),
            avatar_list: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FLEHDHLAKDK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FLEHDHLAKDK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FLEHDHLAKDK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FLEHDHLAKDK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11FLEHDHLAKDK.proto\x1a\x11EOBCFPEGNJP.proto\x1a\x11FBDMBILJDMG.prot\
    o\"\x8e\x01\n\x0bFLEHDHLAKDK\x12.\n\x0bNGDAJKNLELE\x18\x08\x20\x03(\x0b2\
    \x0c.EOBCFPEGNJPR\x0bNGDAJKNLELE\x12\x20\n\x0bHPPMGEHCKHC\x18\x06\x20\
    \x03(\rR\x0bHPPMGEHCKHC\x12-\n\x0bavatar_list\x18\x07\x20\x03(\x0b2\x0c.\
    FBDMBILJDMGR\navatarListb\x06proto3\
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
            deps.push(super::EOBCFPEGNJP::file_descriptor().clone());
            deps.push(super::FBDMBILJDMG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FLEHDHLAKDK::generated_message_descriptor_data());
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
