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

//! Generated file from `UseItemCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:UseItemCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct UseItemCsReq {
    // message fields
    // @@protoc_insertion_point(field:UseItemCsReq.AACGCNJHDPF)
    pub AACGCNJHDPF: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.base_avatar_id)
    pub base_avatar_id: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.ENPCGJBKMGJ)
    pub ENPCGJBKMGJ: ::protobuf::EnumOrUnknown<super::AvatarType::AvatarType>,
    // @@protoc_insertion_point(field:UseItemCsReq.KIAOMBGCLHD)
    pub KIAOMBGCLHD: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.ADLEGBJLLEA)
    pub ADLEGBJLLEA: u32,
    // @@protoc_insertion_point(field:UseItemCsReq.DBIOHFLJCJF)
    pub DBIOHFLJCJF: bool,
    // special fields
    // @@protoc_insertion_point(special_field:UseItemCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a UseItemCsReq {
    fn default() -> &'a UseItemCsReq {
        <UseItemCsReq as ::protobuf::Message>::default_instance()
    }
}

impl UseItemCsReq {
    pub fn new() -> UseItemCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(6);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AACGCNJHDPF",
            |m: &UseItemCsReq| { &m.AACGCNJHDPF },
            |m: &mut UseItemCsReq| { &mut m.AACGCNJHDPF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "base_avatar_id",
            |m: &UseItemCsReq| { &m.base_avatar_id },
            |m: &mut UseItemCsReq| { &mut m.base_avatar_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ENPCGJBKMGJ",
            |m: &UseItemCsReq| { &m.ENPCGJBKMGJ },
            |m: &mut UseItemCsReq| { &mut m.ENPCGJBKMGJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KIAOMBGCLHD",
            |m: &UseItemCsReq| { &m.KIAOMBGCLHD },
            |m: &mut UseItemCsReq| { &mut m.KIAOMBGCLHD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADLEGBJLLEA",
            |m: &UseItemCsReq| { &m.ADLEGBJLLEA },
            |m: &mut UseItemCsReq| { &mut m.ADLEGBJLLEA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "DBIOHFLJCJF",
            |m: &UseItemCsReq| { &m.DBIOHFLJCJF },
            |m: &mut UseItemCsReq| { &mut m.DBIOHFLJCJF },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<UseItemCsReq>(
            "UseItemCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for UseItemCsReq {
    const NAME: &'static str = "UseItemCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.AACGCNJHDPF = is.read_uint32()?;
                },
                8 => {
                    self.base_avatar_id = is.read_uint32()?;
                },
                16 => {
                    self.ENPCGJBKMGJ = is.read_enum_or_unknown()?;
                },
                72 => {
                    self.KIAOMBGCLHD = is.read_uint32()?;
                },
                120 => {
                    self.ADLEGBJLLEA = is.read_uint32()?;
                },
                112 => {
                    self.DBIOHFLJCJF = is.read_bool()?;
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
        if self.AACGCNJHDPF != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.AACGCNJHDPF);
        }
        if self.base_avatar_id != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.base_avatar_id);
        }
        if self.ENPCGJBKMGJ != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            my_size += ::protobuf::rt::int32_size(2, self.ENPCGJBKMGJ.value());
        }
        if self.KIAOMBGCLHD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.KIAOMBGCLHD);
        }
        if self.ADLEGBJLLEA != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.ADLEGBJLLEA);
        }
        if self.DBIOHFLJCJF != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.AACGCNJHDPF != 0 {
            os.write_uint32(6, self.AACGCNJHDPF)?;
        }
        if self.base_avatar_id != 0 {
            os.write_uint32(1, self.base_avatar_id)?;
        }
        if self.ENPCGJBKMGJ != ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE) {
            os.write_enum(2, ::protobuf::EnumOrUnknown::value(&self.ENPCGJBKMGJ))?;
        }
        if self.KIAOMBGCLHD != 0 {
            os.write_uint32(9, self.KIAOMBGCLHD)?;
        }
        if self.ADLEGBJLLEA != 0 {
            os.write_uint32(15, self.ADLEGBJLLEA)?;
        }
        if self.DBIOHFLJCJF != false {
            os.write_bool(14, self.DBIOHFLJCJF)?;
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

    fn new() -> UseItemCsReq {
        UseItemCsReq::new()
    }

    fn clear(&mut self) {
        self.AACGCNJHDPF = 0;
        self.base_avatar_id = 0;
        self.ENPCGJBKMGJ = ::protobuf::EnumOrUnknown::new(super::AvatarType::AvatarType::AVATAR_TYPE_NONE);
        self.KIAOMBGCLHD = 0;
        self.ADLEGBJLLEA = 0;
        self.DBIOHFLJCJF = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static UseItemCsReq {
        static instance: UseItemCsReq = UseItemCsReq {
            AACGCNJHDPF: 0,
            base_avatar_id: 0,
            ENPCGJBKMGJ: ::protobuf::EnumOrUnknown::from_i32(0),
            KIAOMBGCLHD: 0,
            ADLEGBJLLEA: 0,
            DBIOHFLJCJF: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for UseItemCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("UseItemCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for UseItemCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UseItemCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12UseItemCsReq.proto\x1a\x10AvatarType.proto\"\xeb\x01\n\x0cUseItemC\
    sReq\x12\x20\n\x0bAACGCNJHDPF\x18\x06\x20\x01(\rR\x0bAACGCNJHDPF\x12$\n\
    \x0ebase_avatar_id\x18\x01\x20\x01(\rR\x0cbaseAvatarId\x12-\n\x0bENPCGJB\
    KMGJ\x18\x02\x20\x01(\x0e2\x0b.AvatarTypeR\x0bENPCGJBKMGJ\x12\x20\n\x0bK\
    IAOMBGCLHD\x18\t\x20\x01(\rR\x0bKIAOMBGCLHD\x12\x20\n\x0bADLEGBJLLEA\x18\
    \x0f\x20\x01(\rR\x0bADLEGBJLLEA\x12\x20\n\x0bDBIOHFLJCJF\x18\x0e\x20\x01\
    (\x08R\x0bDBIOHFLJCJFb\x06proto3\
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
            deps.push(super::AvatarType::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(UseItemCsReq::generated_message_descriptor_data());
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
