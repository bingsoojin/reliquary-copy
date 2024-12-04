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

//! Generated file from `GIHFAIDCENK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GIHFAIDCENK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GIHFAIDCENK {
    // message fields
    // @@protoc_insertion_point(field:GIHFAIDCENK.JKOCJIMAGBN)
    pub JKOCJIMAGBN: u32,
    // @@protoc_insertion_point(field:GIHFAIDCENK.HCEFGBHKLIH)
    pub HCEFGBHKLIH: u32,
    // @@protoc_insertion_point(field:GIHFAIDCENK.PILNCPHBMJB)
    pub PILNCPHBMJB: u32,
    // @@protoc_insertion_point(field:GIHFAIDCENK.CILFOCBCCNK)
    pub CILFOCBCCNK: ::std::vec::Vec<super::CDIIDOLJILK::CDIIDOLJILK>,
    // special fields
    // @@protoc_insertion_point(special_field:GIHFAIDCENK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GIHFAIDCENK {
    fn default() -> &'a GIHFAIDCENK {
        <GIHFAIDCENK as ::protobuf::Message>::default_instance()
    }
}

impl GIHFAIDCENK {
    pub fn new() -> GIHFAIDCENK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JKOCJIMAGBN",
            |m: &GIHFAIDCENK| { &m.JKOCJIMAGBN },
            |m: &mut GIHFAIDCENK| { &mut m.JKOCJIMAGBN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HCEFGBHKLIH",
            |m: &GIHFAIDCENK| { &m.HCEFGBHKLIH },
            |m: &mut GIHFAIDCENK| { &mut m.HCEFGBHKLIH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PILNCPHBMJB",
            |m: &GIHFAIDCENK| { &m.PILNCPHBMJB },
            |m: &mut GIHFAIDCENK| { &mut m.PILNCPHBMJB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "CILFOCBCCNK",
            |m: &GIHFAIDCENK| { &m.CILFOCBCCNK },
            |m: &mut GIHFAIDCENK| { &mut m.CILFOCBCCNK },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GIHFAIDCENK>(
            "GIHFAIDCENK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GIHFAIDCENK {
    const NAME: &'static str = "GIHFAIDCENK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                96 => {
                    self.JKOCJIMAGBN = is.read_uint32()?;
                },
                48 => {
                    self.HCEFGBHKLIH = is.read_uint32()?;
                },
                64 => {
                    self.PILNCPHBMJB = is.read_uint32()?;
                },
                90 => {
                    self.CILFOCBCCNK.push(is.read_message()?);
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
        if self.JKOCJIMAGBN != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.JKOCJIMAGBN);
        }
        if self.HCEFGBHKLIH != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.HCEFGBHKLIH);
        }
        if self.PILNCPHBMJB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.PILNCPHBMJB);
        }
        for value in &self.CILFOCBCCNK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.JKOCJIMAGBN != 0 {
            os.write_uint32(12, self.JKOCJIMAGBN)?;
        }
        if self.HCEFGBHKLIH != 0 {
            os.write_uint32(6, self.HCEFGBHKLIH)?;
        }
        if self.PILNCPHBMJB != 0 {
            os.write_uint32(8, self.PILNCPHBMJB)?;
        }
        for v in &self.CILFOCBCCNK {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> GIHFAIDCENK {
        GIHFAIDCENK::new()
    }

    fn clear(&mut self) {
        self.JKOCJIMAGBN = 0;
        self.HCEFGBHKLIH = 0;
        self.PILNCPHBMJB = 0;
        self.CILFOCBCCNK.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GIHFAIDCENK {
        static instance: GIHFAIDCENK = GIHFAIDCENK {
            JKOCJIMAGBN: 0,
            HCEFGBHKLIH: 0,
            PILNCPHBMJB: 0,
            CILFOCBCCNK: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GIHFAIDCENK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GIHFAIDCENK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GIHFAIDCENK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GIHFAIDCENK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11GIHFAIDCENK.proto\x1a\x11CDIIDOLJILK.proto\"\xa3\x01\n\x0bGIHFAIDC\
    ENK\x12\x20\n\x0bJKOCJIMAGBN\x18\x0c\x20\x01(\rR\x0bJKOCJIMAGBN\x12\x20\
    \n\x0bHCEFGBHKLIH\x18\x06\x20\x01(\rR\x0bHCEFGBHKLIH\x12\x20\n\x0bPILNCP\
    HBMJB\x18\x08\x20\x01(\rR\x0bPILNCPHBMJB\x12.\n\x0bCILFOCBCCNK\x18\x0b\
    \x20\x03(\x0b2\x0c.CDIIDOLJILKR\x0bCILFOCBCCNKb\x06proto3\
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
            deps.push(super::CDIIDOLJILK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GIHFAIDCENK::generated_message_descriptor_data());
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
