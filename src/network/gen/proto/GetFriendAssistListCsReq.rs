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

//! Generated file from `GetFriendAssistListCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetFriendAssistListCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetFriendAssistListCsReq {
    // message fields
    // @@protoc_insertion_point(field:GetFriendAssistListCsReq.AAGCBEJEPCF)
    pub AAGCBEJEPCF: u32,
    // @@protoc_insertion_point(field:GetFriendAssistListCsReq.NLNBNHPBJCN)
    pub NLNBNHPBJCN: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetFriendAssistListCsReq.NMIOPGHIFJE)
    pub NMIOPGHIFJE: ::protobuf::EnumOrUnknown<super::CNAPNFFHICM::CNAPNFFHICM>,
    // @@protoc_insertion_point(field:GetFriendAssistListCsReq.BELKBOOPHIM)
    pub BELKBOOPHIM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:GetFriendAssistListCsReq.NCCHMLNHNDI)
    pub NCCHMLNHNDI: bool,
    // special fields
    // @@protoc_insertion_point(special_field:GetFriendAssistListCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetFriendAssistListCsReq {
    fn default() -> &'a GetFriendAssistListCsReq {
        <GetFriendAssistListCsReq as ::protobuf::Message>::default_instance()
    }
}

impl GetFriendAssistListCsReq {
    pub fn new() -> GetFriendAssistListCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AAGCBEJEPCF",
            |m: &GetFriendAssistListCsReq| { &m.AAGCBEJEPCF },
            |m: &mut GetFriendAssistListCsReq| { &mut m.AAGCBEJEPCF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NLNBNHPBJCN",
            |m: &GetFriendAssistListCsReq| { &m.NLNBNHPBJCN },
            |m: &mut GetFriendAssistListCsReq| { &mut m.NLNBNHPBJCN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NMIOPGHIFJE",
            |m: &GetFriendAssistListCsReq| { &m.NMIOPGHIFJE },
            |m: &mut GetFriendAssistListCsReq| { &mut m.NMIOPGHIFJE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "BELKBOOPHIM",
            |m: &GetFriendAssistListCsReq| { &m.BELKBOOPHIM },
            |m: &mut GetFriendAssistListCsReq| { &mut m.BELKBOOPHIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NCCHMLNHNDI",
            |m: &GetFriendAssistListCsReq| { &m.NCCHMLNHNDI },
            |m: &mut GetFriendAssistListCsReq| { &mut m.NCCHMLNHNDI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetFriendAssistListCsReq>(
            "GetFriendAssistListCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetFriendAssistListCsReq {
    const NAME: &'static str = "GetFriendAssistListCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                24 => {
                    self.AAGCBEJEPCF = is.read_uint32()?;
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.NLNBNHPBJCN)?;
                },
                8 => {
                    self.NLNBNHPBJCN.push(is.read_uint32()?);
                },
                48 => {
                    self.NMIOPGHIFJE = is.read_enum_or_unknown()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.BELKBOOPHIM)?;
                },
                16 => {
                    self.BELKBOOPHIM.push(is.read_uint32()?);
                },
                72 => {
                    self.NCCHMLNHNDI = is.read_bool()?;
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
        if self.AAGCBEJEPCF != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.AAGCBEJEPCF);
        }
        for value in &self.NLNBNHPBJCN {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.NMIOPGHIFJE != ::protobuf::EnumOrUnknown::new(super::CNAPNFFHICM::CNAPNFFHICM::ASSIST_AVATAR_UNKNOW) {
            my_size += ::protobuf::rt::int32_size(6, self.NMIOPGHIFJE.value());
        }
        for value in &self.BELKBOOPHIM {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.NCCHMLNHNDI != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AAGCBEJEPCF != 0 {
            os.write_uint32(3, self.AAGCBEJEPCF)?;
        }
        for v in &self.NLNBNHPBJCN {
            os.write_uint32(1, *v)?;
        };
        if self.NMIOPGHIFJE != ::protobuf::EnumOrUnknown::new(super::CNAPNFFHICM::CNAPNFFHICM::ASSIST_AVATAR_UNKNOW) {
            os.write_enum(6, ::protobuf::EnumOrUnknown::value(&self.NMIOPGHIFJE))?;
        }
        for v in &self.BELKBOOPHIM {
            os.write_uint32(2, *v)?;
        };
        if self.NCCHMLNHNDI != false {
            os.write_bool(9, self.NCCHMLNHNDI)?;
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

    fn new() -> GetFriendAssistListCsReq {
        GetFriendAssistListCsReq::new()
    }

    fn clear(&mut self) {
        self.AAGCBEJEPCF = 0;
        self.NLNBNHPBJCN.clear();
        self.NMIOPGHIFJE = ::protobuf::EnumOrUnknown::new(super::CNAPNFFHICM::CNAPNFFHICM::ASSIST_AVATAR_UNKNOW);
        self.BELKBOOPHIM.clear();
        self.NCCHMLNHNDI = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetFriendAssistListCsReq {
        static instance: GetFriendAssistListCsReq = GetFriendAssistListCsReq {
            AAGCBEJEPCF: 0,
            NLNBNHPBJCN: ::std::vec::Vec::new(),
            NMIOPGHIFJE: ::protobuf::EnumOrUnknown::from_i32(0),
            BELKBOOPHIM: ::std::vec::Vec::new(),
            NCCHMLNHNDI: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetFriendAssistListCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetFriendAssistListCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetFriendAssistListCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetFriendAssistListCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1eGetFriendAssistListCsReq.proto\x1a\x11CNAPNFFHICM.proto\"\xd2\x01\
    \n\x18GetFriendAssistListCsReq\x12\x20\n\x0bAAGCBEJEPCF\x18\x03\x20\x01(\
    \rR\x0bAAGCBEJEPCF\x12\x20\n\x0bNLNBNHPBJCN\x18\x01\x20\x03(\rR\x0bNLNBN\
    HPBJCN\x12.\n\x0bNMIOPGHIFJE\x18\x06\x20\x01(\x0e2\x0c.CNAPNFFHICMR\x0bN\
    MIOPGHIFJE\x12\x20\n\x0bBELKBOOPHIM\x18\x02\x20\x03(\rR\x0bBELKBOOPHIM\
    \x12\x20\n\x0bNCCHMLNHNDI\x18\t\x20\x01(\x08R\x0bNCCHMLNHNDIb\x06proto3\
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
            deps.push(super::CNAPNFFHICM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetFriendAssistListCsReq::generated_message_descriptor_data());
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
