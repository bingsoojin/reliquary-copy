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

//! Generated file from `PPAGBCJIBNL.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PPAGBCJIBNL)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PPAGBCJIBNL {
    // message fields
    // @@protoc_insertion_point(field:PPAGBCJIBNL.LEEACMNGDFL)
    pub LEEACMNGDFL: ::protobuf::EnumOrUnknown<super::RogueModifierSourceType::RogueModifierSourceType>,
    // @@protoc_insertion_point(field:PPAGBCJIBNL.AOMMMPOIPPP)
    pub AOMMMPOIPPP: u64,
    // @@protoc_insertion_point(field:PPAGBCJIBNL.BGFHGKMPFNH)
    pub BGFHGKMPFNH: ::protobuf::MessageField<super::INLDKPMBPGJ::INLDKPMBPGJ>,
    // message oneof groups
    pub DKFKPIPNNLF: ::std::option::Option<ppagbcjibnl::DKFKPIPNNLF>,
    // special fields
    // @@protoc_insertion_point(special_field:PPAGBCJIBNL.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PPAGBCJIBNL {
    fn default() -> &'a PPAGBCJIBNL {
        <PPAGBCJIBNL as ::protobuf::Message>::default_instance()
    }
}

impl PPAGBCJIBNL {
    pub fn new() -> PPAGBCJIBNL {
        ::std::default::Default::default()
    }

    // .BAHCLOBILLH JPPHKOOCFJO = 1088;

    pub fn JPPHKOOCFJO(&self) -> &super::BAHCLOBILLH::BAHCLOBILLH {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(ref v)) => v,
            _ => <super::BAHCLOBILLH::BAHCLOBILLH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JPPHKOOCFJO(&mut self) {
        self.DKFKPIPNNLF = ::std::option::Option::None;
    }

    pub fn has_JPPHKOOCFJO(&self) -> bool {
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JPPHKOOCFJO(&mut self, v: super::BAHCLOBILLH::BAHCLOBILLH) {
        self.DKFKPIPNNLF = ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JPPHKOOCFJO(&mut self) -> &mut super::BAHCLOBILLH::BAHCLOBILLH {
        if let ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(_)) = self.DKFKPIPNNLF {
        } else {
            self.DKFKPIPNNLF = ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(super::BAHCLOBILLH::BAHCLOBILLH::new()));
        }
        match self.DKFKPIPNNLF {
            ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JPPHKOOCFJO(&mut self) -> super::BAHCLOBILLH::BAHCLOBILLH {
        if self.has_JPPHKOOCFJO() {
            match self.DKFKPIPNNLF.take() {
                ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::BAHCLOBILLH::BAHCLOBILLH::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LEEACMNGDFL",
            |m: &PPAGBCJIBNL| { &m.LEEACMNGDFL },
            |m: &mut PPAGBCJIBNL| { &mut m.LEEACMNGDFL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AOMMMPOIPPP",
            |m: &PPAGBCJIBNL| { &m.AOMMMPOIPPP },
            |m: &mut PPAGBCJIBNL| { &mut m.AOMMMPOIPPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::INLDKPMBPGJ::INLDKPMBPGJ>(
            "BGFHGKMPFNH",
            |m: &PPAGBCJIBNL| { &m.BGFHGKMPFNH },
            |m: &mut PPAGBCJIBNL| { &mut m.BGFHGKMPFNH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::BAHCLOBILLH::BAHCLOBILLH>(
            "JPPHKOOCFJO",
            PPAGBCJIBNL::has_JPPHKOOCFJO,
            PPAGBCJIBNL::JPPHKOOCFJO,
            PPAGBCJIBNL::mut_JPPHKOOCFJO,
            PPAGBCJIBNL::set_JPPHKOOCFJO,
        ));
        oneofs.push(ppagbcjibnl::DKFKPIPNNLF::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PPAGBCJIBNL>(
            "PPAGBCJIBNL",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PPAGBCJIBNL {
    const NAME: &'static str = "PPAGBCJIBNL";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.LEEACMNGDFL = is.read_enum_or_unknown()?;
                },
                96 => {
                    self.AOMMMPOIPPP = is.read_uint64()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BGFHGKMPFNH)?;
                },
                8706 => {
                    self.DKFKPIPNNLF = ::std::option::Option::Some(ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(is.read_message()?));
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
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            my_size += ::protobuf::rt::int32_size(5, self.LEEACMNGDFL.value());
        }
        if self.AOMMMPOIPPP != 0 {
            my_size += ::protobuf::rt::uint64_size(12, self.AOMMMPOIPPP);
        }
        if let Some(v) = self.BGFHGKMPFNH.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(ref v) => {
                    let len = v.compute_size();
                    my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.LEEACMNGDFL != ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.LEEACMNGDFL))?;
        }
        if self.AOMMMPOIPPP != 0 {
            os.write_uint64(12, self.AOMMMPOIPPP)?;
        }
        if let Some(v) = self.BGFHGKMPFNH.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.DKFKPIPNNLF {
            match v {
                &ppagbcjibnl::DKFKPIPNNLF::JPPHKOOCFJO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1088, v, os)?;
                },
            };
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

    fn new() -> PPAGBCJIBNL {
        PPAGBCJIBNL::new()
    }

    fn clear(&mut self) {
        self.LEEACMNGDFL = ::protobuf::EnumOrUnknown::new(super::RogueModifierSourceType::RogueModifierSourceType::ROGUE_MODIFIER_SOURCE_NONE);
        self.AOMMMPOIPPP = 0;
        self.BGFHGKMPFNH.clear();
        self.DKFKPIPNNLF = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PPAGBCJIBNL {
        static instance: PPAGBCJIBNL = PPAGBCJIBNL {
            LEEACMNGDFL: ::protobuf::EnumOrUnknown::from_i32(0),
            AOMMMPOIPPP: 0,
            BGFHGKMPFNH: ::protobuf::MessageField::none(),
            DKFKPIPNNLF: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PPAGBCJIBNL {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PPAGBCJIBNL").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PPAGBCJIBNL {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PPAGBCJIBNL {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `PPAGBCJIBNL`
pub mod ppagbcjibnl {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:PPAGBCJIBNL.DKFKPIPNNLF)
    pub enum DKFKPIPNNLF {
        // @@protoc_insertion_point(oneof_field:PPAGBCJIBNL.JPPHKOOCFJO)
        JPPHKOOCFJO(super::super::BAHCLOBILLH::BAHCLOBILLH),
    }

    impl ::protobuf::Oneof for DKFKPIPNNLF {
    }

    impl ::protobuf::OneofFull for DKFKPIPNNLF {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::PPAGBCJIBNL as ::protobuf::MessageFull>::descriptor().oneof_by_name("DKFKPIPNNLF").unwrap()).clone()
        }
    }

    impl DKFKPIPNNLF {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<DKFKPIPNNLF>("DKFKPIPNNLF")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11PPAGBCJIBNL.proto\x1a\x11BAHCLOBILLH.proto\x1a\x11INLDKPMBPGJ.prot\
    o\x1a\x1dRogueModifierSourceType.proto\"\xdd\x01\n\x0bPPAGBCJIBNL\x12:\n\
    \x0bLEEACMNGDFL\x18\x05\x20\x01(\x0e2\x18.RogueModifierSourceTypeR\x0bLE\
    EACMNGDFL\x12\x20\n\x0bAOMMMPOIPPP\x18\x0c\x20\x01(\x04R\x0bAOMMMPOIPPP\
    \x12.\n\x0bBGFHGKMPFNH\x18\x02\x20\x01(\x0b2\x0c.INLDKPMBPGJR\x0bBGFHGKM\
    PFNH\x121\n\x0bJPPHKOOCFJO\x18\xc0\x08\x20\x01(\x0b2\x0c.BAHCLOBILLHH\0R\
    \x0bJPPHKOOCFJOB\r\n\x0bDKFKPIPNNLFb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::BAHCLOBILLH::file_descriptor().clone());
            deps.push(super::INLDKPMBPGJ::file_descriptor().clone());
            deps.push(super::RogueModifierSourceType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PPAGBCJIBNL::generated_message_descriptor_data());
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
