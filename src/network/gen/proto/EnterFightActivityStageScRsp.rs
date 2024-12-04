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

//! Generated file from `EnterFightActivityStageScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EnterFightActivityStageScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EnterFightActivityStageScRsp {
    // message fields
    // @@protoc_insertion_point(field:EnterFightActivityStageScRsp.KONGAADEJEL)
    pub KONGAADEJEL: u32,
    // @@protoc_insertion_point(field:EnterFightActivityStageScRsp.BBKGPAJCCBM)
    pub BBKGPAJCCBM: ::protobuf::MessageField<super::CHDPLFOHLCN::CHDPLFOHLCN>,
    // @@protoc_insertion_point(field:EnterFightActivityStageScRsp.ILIFHHJFMIH)
    pub ILIFHHJFMIH: u32,
    // @@protoc_insertion_point(field:EnterFightActivityStageScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // special fields
    // @@protoc_insertion_point(special_field:EnterFightActivityStageScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EnterFightActivityStageScRsp {
    fn default() -> &'a EnterFightActivityStageScRsp {
        <EnterFightActivityStageScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EnterFightActivityStageScRsp {
    pub fn new() -> EnterFightActivityStageScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KONGAADEJEL",
            |m: &EnterFightActivityStageScRsp| { &m.KONGAADEJEL },
            |m: &mut EnterFightActivityStageScRsp| { &mut m.KONGAADEJEL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::CHDPLFOHLCN::CHDPLFOHLCN>(
            "BBKGPAJCCBM",
            |m: &EnterFightActivityStageScRsp| { &m.BBKGPAJCCBM },
            |m: &mut EnterFightActivityStageScRsp| { &mut m.BBKGPAJCCBM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ILIFHHJFMIH",
            |m: &EnterFightActivityStageScRsp| { &m.ILIFHHJFMIH },
            |m: &mut EnterFightActivityStageScRsp| { &mut m.ILIFHHJFMIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &EnterFightActivityStageScRsp| { &m.ADADHIHDHJC },
            |m: &mut EnterFightActivityStageScRsp| { &mut m.ADADHIHDHJC },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EnterFightActivityStageScRsp>(
            "EnterFightActivityStageScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EnterFightActivityStageScRsp {
    const NAME: &'static str = "EnterFightActivityStageScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                48 => {
                    self.KONGAADEJEL = is.read_uint32()?;
                },
                66 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.BBKGPAJCCBM)?;
                },
                72 => {
                    self.ILIFHHJFMIH = is.read_uint32()?;
                },
                8 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
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
        if self.KONGAADEJEL != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.KONGAADEJEL);
        }
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.ILIFHHJFMIH != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ILIFHHJFMIH);
        }
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.ADADHIHDHJC);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KONGAADEJEL != 0 {
            os.write_uint32(6, self.KONGAADEJEL)?;
        }
        if let Some(v) = self.BBKGPAJCCBM.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        }
        if self.ILIFHHJFMIH != 0 {
            os.write_uint32(9, self.ILIFHHJFMIH)?;
        }
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(1, self.ADADHIHDHJC)?;
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

    fn new() -> EnterFightActivityStageScRsp {
        EnterFightActivityStageScRsp::new()
    }

    fn clear(&mut self) {
        self.KONGAADEJEL = 0;
        self.BBKGPAJCCBM.clear();
        self.ILIFHHJFMIH = 0;
        self.ADADHIHDHJC = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EnterFightActivityStageScRsp {
        static instance: EnterFightActivityStageScRsp = EnterFightActivityStageScRsp {
            KONGAADEJEL: 0,
            BBKGPAJCCBM: ::protobuf::MessageField::none(),
            ILIFHHJFMIH: 0,
            ADADHIHDHJC: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EnterFightActivityStageScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EnterFightActivityStageScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EnterFightActivityStageScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnterFightActivityStageScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"EnterFightActivityStageScRsp.proto\x1a\x11CHDPLFOHLCN.proto\"\xb4\
    \x01\n\x1cEnterFightActivityStageScRsp\x12\x20\n\x0bKONGAADEJEL\x18\x06\
    \x20\x01(\rR\x0bKONGAADEJEL\x12.\n\x0bBBKGPAJCCBM\x18\x08\x20\x01(\x0b2\
    \x0c.CHDPLFOHLCNR\x0bBBKGPAJCCBM\x12\x20\n\x0bILIFHHJFMIH\x18\t\x20\x01(\
    \rR\x0bILIFHHJFMIH\x12\x20\n\x0bADADHIHDHJC\x18\x01\x20\x01(\rR\x0bADADH\
    IHDHJCb\x06proto3\
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
            deps.push(super::CHDPLFOHLCN::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EnterFightActivityStageScRsp::generated_message_descriptor_data());
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
