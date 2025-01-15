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

//! Generated file from `EnterSummonActivityStageCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterSummonActivityStageCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterSummonActivityStageCsReq {
    // message fields
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.IBAFDOBBEGD)
    pub IBAFDOBBEGD: u32,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.avatar_list)
    pub avatar_list: ::std::vec::Vec<super::AJCMJBMFDDM::AJCMJBMFDDM>,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // @@protoc_insertion_point(field:EnterSummonActivityStageCsReq.GJBOBBPFKMD)
    pub GJBOBBPFKMD: ::protobuf::MessageField<super::AJCMJBMFDDM::AJCMJBMFDDM>,
    // special fields
    // @@protoc_insertion_point(special_field:EnterSummonActivityStageCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterSummonActivityStageCsReq {
    fn default() -> &'a EnterSummonActivityStageCsReq {
        <EnterSummonActivityStageCsReq as ::protobuf::Message>::default_instance()
    }
}

impl EnterSummonActivityStageCsReq {
    pub fn new() -> EnterSummonActivityStageCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IBAFDOBBEGD",
            |m: &EnterSummonActivityStageCsReq| { &m.IBAFDOBBEGD },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.IBAFDOBBEGD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_list",
            |m: &EnterSummonActivityStageCsReq| { &m.avatar_list },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.avatar_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &EnterSummonActivityStageCsReq| { &m.IOPPGEGDHGL },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.IOPPGEGDHGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::AJCMJBMFDDM::AJCMJBMFDDM>(
            "GJBOBBPFKMD",
            |m: &EnterSummonActivityStageCsReq| { &m.GJBOBBPFKMD },
            |m: &mut EnterSummonActivityStageCsReq| { &mut m.GJBOBBPFKMD },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterSummonActivityStageCsReq>(
            "EnterSummonActivityStageCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterSummonActivityStageCsReq {
    const NAME: &'static str = "EnterSummonActivityStageCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.IBAFDOBBEGD = is.read_uint32()?;
                },
                26 => {
                    self.avatar_list.push(is.read_message()?);
                },
                56 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
                },
                34 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.GJBOBBPFKMD)?;
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
        if self.IBAFDOBBEGD != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.IBAFDOBBEGD);
        }
        for value in &self.avatar_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.IOPPGEGDHGL);
        }
        if let Some(v) = self.GJBOBBPFKMD.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IBAFDOBBEGD != 0 {
            os.write_uint32(12, self.IBAFDOBBEGD)?;
        }
        for v in &self.avatar_list {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(7, self.IOPPGEGDHGL)?;
        }
        if let Some(v) = self.GJBOBBPFKMD.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
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

    fn new() -> EnterSummonActivityStageCsReq {
        EnterSummonActivityStageCsReq::new()
    }

    fn clear(&mut self) {
        self.IBAFDOBBEGD = 0;
        self.avatar_list.clear();
        self.IOPPGEGDHGL = 0;
        self.GJBOBBPFKMD.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterSummonActivityStageCsReq {
        static instance: EnterSummonActivityStageCsReq = EnterSummonActivityStageCsReq {
            IBAFDOBBEGD: 0,
            avatar_list: ::std::vec::Vec::new(),
            IOPPGEGDHGL: 0,
            GJBOBBPFKMD: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterSummonActivityStageCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterSummonActivityStageCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterSummonActivityStageCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterSummonActivityStageCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#EnterSummonActivityStageCsReq.proto\x1a\x11AJCMJBMFDDM.proto\"\xc2\
    \x01\n\x1dEnterSummonActivityStageCsReq\x12\x20\n\x0bIBAFDOBBEGD\x18\x0c\
    \x20\x01(\rR\x0bIBAFDOBBEGD\x12-\n\x0bavatar_list\x18\x03\x20\x03(\x0b2\
    \x0c.AJCMJBMFDDMR\navatarList\x12\x20\n\x0bIOPPGEGDHGL\x18\x07\x20\x01(\
    \rR\x0bIOPPGEGDHGL\x12.\n\x0bGJBOBBPFKMD\x18\x04\x20\x01(\x0b2\x0c.AJCMJ\
    BMFDDMR\x0bGJBOBBPFKMDb\x06proto3\
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
            deps.push(super::AJCMJBMFDDM::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterSummonActivityStageCsReq::generated_message_descriptor_data());
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
