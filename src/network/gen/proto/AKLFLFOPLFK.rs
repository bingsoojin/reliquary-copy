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

//! Generated file from `AKLFLFOPLFK.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:AKLFLFOPLFK)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AKLFLFOPLFK {
    // message fields
    // @@protoc_insertion_point(field:AKLFLFOPLFK.KECGEOCBDEJ)
    pub KECGEOCBDEJ: u32,
    // @@protoc_insertion_point(field:AKLFLFOPLFK.EFCCJKGHJDK)
    pub EFCCJKGHJDK: u32,
    // @@protoc_insertion_point(field:AKLFLFOPLFK.unique_id)
    pub unique_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:AKLFLFOPLFK.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AKLFLFOPLFK {
    fn default() -> &'a AKLFLFOPLFK {
        <AKLFLFOPLFK as ::protobuf::Message>::default_instance()
    }
}

impl AKLFLFOPLFK {
    pub fn new() -> AKLFLFOPLFK {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KECGEOCBDEJ",
            |m: &AKLFLFOPLFK| { &m.KECGEOCBDEJ },
            |m: &mut AKLFLFOPLFK| { &mut m.KECGEOCBDEJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EFCCJKGHJDK",
            |m: &AKLFLFOPLFK| { &m.EFCCJKGHJDK },
            |m: &mut AKLFLFOPLFK| { &mut m.EFCCJKGHJDK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "unique_id",
            |m: &AKLFLFOPLFK| { &m.unique_id },
            |m: &mut AKLFLFOPLFK| { &mut m.unique_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AKLFLFOPLFK>(
            "AKLFLFOPLFK",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AKLFLFOPLFK {
    const NAME: &'static str = "AKLFLFOPLFK";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                112 => {
                    self.KECGEOCBDEJ = is.read_uint32()?;
                },
                40 => {
                    self.EFCCJKGHJDK = is.read_uint32()?;
                },
                80 => {
                    self.unique_id = is.read_uint32()?;
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
        if self.KECGEOCBDEJ != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.KECGEOCBDEJ);
        }
        if self.EFCCJKGHJDK != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EFCCJKGHJDK);
        }
        if self.unique_id != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.unique_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.KECGEOCBDEJ != 0 {
            os.write_uint32(14, self.KECGEOCBDEJ)?;
        }
        if self.EFCCJKGHJDK != 0 {
            os.write_uint32(5, self.EFCCJKGHJDK)?;
        }
        if self.unique_id != 0 {
            os.write_uint32(10, self.unique_id)?;
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

    fn new() -> AKLFLFOPLFK {
        AKLFLFOPLFK::new()
    }

    fn clear(&mut self) {
        self.KECGEOCBDEJ = 0;
        self.EFCCJKGHJDK = 0;
        self.unique_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AKLFLFOPLFK {
        static instance: AKLFLFOPLFK = AKLFLFOPLFK {
            KECGEOCBDEJ: 0,
            EFCCJKGHJDK: 0,
            unique_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AKLFLFOPLFK {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AKLFLFOPLFK").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AKLFLFOPLFK {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AKLFLFOPLFK {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11AKLFLFOPLFK.proto\"n\n\x0bAKLFLFOPLFK\x12\x20\n\x0bKECGEOCBDEJ\x18\
    \x0e\x20\x01(\rR\x0bKECGEOCBDEJ\x12\x20\n\x0bEFCCJKGHJDK\x18\x05\x20\x01\
    (\rR\x0bEFCCJKGHJDK\x12\x1b\n\tunique_id\x18\n\x20\x01(\rR\x08uniqueIdb\
    \x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(AKLFLFOPLFK::generated_message_descriptor_data());
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
