// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `GetAvatarDataScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:GetAvatarDataScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetAvatarDataScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.is_get_all)
    pub is_get_all: bool,
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.ANKHENIIACH)
    pub ANKHENIIACH: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.JPNLPOPMKEJ)
    pub JPNLPOPMKEJ: u32,
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::Avatar::Avatar>,
    // @@protoc_insertion_point(field:GetAvatarDataScRsp.PNKCFEALAMI)
    pub PNKCFEALAMI: ::std::vec::Vec<::protobuf::EnumOrUnknown<super::GrowthTargetFunctionType::GrowthTargetFunctionType>>,
    // special fields
    // @@protoc_insertion_point(special_field:GetAvatarDataScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetAvatarDataScRsp {
    fn default() -> &'a GetAvatarDataScRsp {
        <GetAvatarDataScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetAvatarDataScRsp {
    pub fn new() -> GetAvatarDataScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_get_all",
            |m: &GetAvatarDataScRsp| { &m.is_get_all },
            |m: &mut GetAvatarDataScRsp| { &mut m.is_get_all },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetAvatarDataScRsp| { &m.retcode },
            |m: &mut GetAvatarDataScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ANKHENIIACH",
            |m: &GetAvatarDataScRsp| { &m.ANKHENIIACH },
            |m: &mut GetAvatarDataScRsp| { &mut m.ANKHENIIACH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JPNLPOPMKEJ",
            |m: &GetAvatarDataScRsp| { &m.JPNLPOPMKEJ },
            |m: &mut GetAvatarDataScRsp| { &mut m.JPNLPOPMKEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &GetAvatarDataScRsp| { &m.avatar_list },
            |m: &mut GetAvatarDataScRsp| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "PNKCFEALAMI",
            |m: &GetAvatarDataScRsp| { &m.PNKCFEALAMI },
            |m: &mut GetAvatarDataScRsp| { &mut m.PNKCFEALAMI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetAvatarDataScRsp>(
            "GetAvatarDataScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetAvatarDataScRsp {
    const NAME: &'static str = "GetAvatarDataScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.is_get_all = is.read_bool()?;
                },
                32 => {
                    self.retcode = is.read_uint32()?;
                },
                74 => {
                    is.read_repeated_packed_uint32_into(&mut self.ANKHENIIACH)?;
                },
                72 => {
                    self.ANKHENIIACH.push(is.read_uint32()?);
                },
                64 => {
                    self.JPNLPOPMKEJ = is.read_uint32()?;
                },
                18 => {
                    self.avatar_list.push(is.read_message()?);
                },
                88 => {
                    self.PNKCFEALAMI.push(is.read_enum_or_unknown()?);
                },
                90 => {
                    ::protobuf::rt::read_repeated_packed_enum_or_unknown_into(is, &mut self.PNKCFEALAMI)?
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
        if self.is_get_all != false {
            my_size += 1 + 1;
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.retcode);
        }
        my_size += ::protobuf::rt::vec_packed_uint32_size(9, &self.ANKHENIIACH);
        if self.JPNLPOPMKEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.JPNLPOPMKEJ);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::vec_packed_enum_or_unknown_size(11, &self.PNKCFEALAMI);
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.is_get_all != false {
            os.write_bool(13, self.is_get_all)?;
        }
        if self.retcode != 0 {
            os.write_uint32(4, self.retcode)?;
        }
        os.write_repeated_packed_uint32(9, &self.ANKHENIIACH)?;
        if self.JPNLPOPMKEJ != 0 {
            os.write_uint32(8, self.JPNLPOPMKEJ)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        os.write_repeated_packed_enum_or_unknown(11, &self.PNKCFEALAMI)?;
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> GetAvatarDataScRsp {
        GetAvatarDataScRsp::new()
    }

    fn clear(&mut self) {
        self.is_get_all = false;
        self.retcode = 0;
        self.ANKHENIIACH.clear();
        self.JPNLPOPMKEJ = 0;
        self.avatar_list.clear();
        self.PNKCFEALAMI.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetAvatarDataScRsp {
        static instance: GetAvatarDataScRsp = GetAvatarDataScRsp {
            is_get_all: false,
            retcode: 0,
            ANKHENIIACH: ::std::vec::Vec::new(),
            JPNLPOPMKEJ: 0,
            avatar_list: ::std::vec::Vec::new(),
            PNKCFEALAMI: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetAvatarDataScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetAvatarDataScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetAvatarDataScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetAvatarDataScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18GetAvatarDataScRsp.proto\x1a\x0cAvatar.proto\x1a\x1eGrowthTargetFu\
    nctionType.proto\"\xf7\x01\n\x12GetAvatarDataScRsp\x12\x1c\n\nis_get_all\
    \x18\r\x20\x01(\x08R\x08isGetAll\x12\x18\n\x07retcode\x18\x04\x20\x01(\r\
    R\x07retcode\x12\x20\n\x0bANKHENIIACH\x18\t\x20\x03(\rR\x0bANKHENIIACH\
    \x12\x20\n\x0bJPNLPOPMKEJ\x18\x08\x20\x01(\rR\x0bJPNLPOPMKEJ\x12(\n\x0ba\
    vatar_list\x18\x02\x20\x03(\x0b2\x07.AvatarR\navatarList\x12;\n\x0bPNKCF\
    EALAMI\x18\x0b\x20\x03(\x0e2\x19.GrowthTargetFunctionTypeR\x0bPNKCFEALAM\
    Ib\x06proto3\
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
            deps.push(super::Avatar::file_descriptor().clone());
            deps.push(super::GrowthTargetFunctionType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetAvatarDataScRsp::generated_message_descriptor_data());
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
