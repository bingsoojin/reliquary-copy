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

//! Generated file from `MLMEMDJGGOF.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MLMEMDJGGOF)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MLMEMDJGGOF {
    // message fields
    // @@protoc_insertion_point(field:MLMEMDJGGOF.LGGIHKPMAEN)
    pub LGGIHKPMAEN: ::std::vec::Vec<super::IJPLNNLEDFB::IJPLNNLEDFB>,
    // @@protoc_insertion_point(field:MLMEMDJGGOF.GDIHBJJJLKH)
    pub GDIHBJJJLKH: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:MLMEMDJGGOF.JKGGJELFOBL)
    pub JKGGJELFOBL: ::std::collections::HashMap<u32, u32>,
    // @@protoc_insertion_point(field:MLMEMDJGGOF.NEADHFLPCJN)
    pub NEADHFLPCJN: ::std::vec::Vec<super::NDKJDOACMAG::NDKJDOACMAG>,
    // special fields
    // @@protoc_insertion_point(special_field:MLMEMDJGGOF.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MLMEMDJGGOF {
    fn default() -> &'a MLMEMDJGGOF {
        <MLMEMDJGGOF as ::protobuf::Message>::default_instance()
    }
}

impl MLMEMDJGGOF {
    pub fn new() -> MLMEMDJGGOF {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LGGIHKPMAEN",
            |m: &MLMEMDJGGOF| { &m.LGGIHKPMAEN },
            |m: &mut MLMEMDJGGOF| { &mut m.LGGIHKPMAEN },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "GDIHBJJJLKH",
            |m: &MLMEMDJGGOF| { &m.GDIHBJJJLKH },
            |m: &mut MLMEMDJGGOF| { &mut m.GDIHBJJJLKH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "JKGGJELFOBL",
            |m: &MLMEMDJGGOF| { &m.JKGGJELFOBL },
            |m: &mut MLMEMDJGGOF| { &mut m.JKGGJELFOBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NEADHFLPCJN",
            |m: &MLMEMDJGGOF| { &m.NEADHFLPCJN },
            |m: &mut MLMEMDJGGOF| { &mut m.NEADHFLPCJN },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MLMEMDJGGOF>(
            "MLMEMDJGGOF",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MLMEMDJGGOF {
    const NAME: &'static str = "MLMEMDJGGOF";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                106 => {
                    self.LGGIHKPMAEN.push(is.read_message()?);
                },
                18 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.GDIHBJJJLKH.insert(key, value);
                },
                58 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            16 => value = is.read_uint32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.JKGGJELFOBL.insert(key, value);
                },
                50 => {
                    self.NEADHFLPCJN.push(is.read_message()?);
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
        for value in &self.LGGIHKPMAEN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for (k, v) in &self.GDIHBJJJLKH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for (k, v) in &self.JKGGJELFOBL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.NEADHFLPCJN {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.LGGIHKPMAEN {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for (k, v) in &self.GDIHBJJJLKH {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(18)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for (k, v) in &self.JKGGJELFOBL {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            entry_size += ::protobuf::rt::uint32_size(2, *v);
            os.write_raw_varint32(58)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            os.write_uint32(2, *v)?;
        };
        for v in &self.NEADHFLPCJN {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
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

    fn new() -> MLMEMDJGGOF {
        MLMEMDJGGOF::new()
    }

    fn clear(&mut self) {
        self.LGGIHKPMAEN.clear();
        self.GDIHBJJJLKH.clear();
        self.JKGGJELFOBL.clear();
        self.NEADHFLPCJN.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MLMEMDJGGOF {
        static instance: ::protobuf::rt::Lazy<MLMEMDJGGOF> = ::protobuf::rt::Lazy::new();
        instance.get(MLMEMDJGGOF::new)
    }
}

impl ::protobuf::MessageFull for MLMEMDJGGOF {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MLMEMDJGGOF").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MLMEMDJGGOF {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MLMEMDJGGOF {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11MLMEMDJGGOF.proto\x1a\x11IJPLNNLEDFB.proto\x1a\x11NDKJDOACMAG.prot\
    o\"\xef\x02\n\x0bMLMEMDJGGOF\x12.\n\x0bLGGIHKPMAEN\x18\r\x20\x03(\x0b2\
    \x0c.IJPLNNLEDFBR\x0bLGGIHKPMAEN\x12?\n\x0bGDIHBJJJLKH\x18\x02\x20\x03(\
    \x0b2\x1d.MLMEMDJGGOF.GDIHBJJJLKHEntryR\x0bGDIHBJJJLKH\x12?\n\x0bJKGGJEL\
    FOBL\x18\x07\x20\x03(\x0b2\x1d.MLMEMDJGGOF.JKGGJELFOBLEntryR\x0bJKGGJELF\
    OBL\x12.\n\x0bNEADHFLPCJN\x18\x06\x20\x03(\x0b2\x0c.NDKJDOACMAGR\x0bNEAD\
    HFLPCJN\x1a>\n\x10GDIHBJJJLKHEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\
    \x03key\x12\x14\n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01\x1a>\n\
    \x10JKGGJELFOBLEntry\x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\x14\
    \n\x05value\x18\x02\x20\x01(\rR\x05value:\x028\x01b\x06proto3\
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
            deps.push(super::IJPLNNLEDFB::file_descriptor().clone());
            deps.push(super::NDKJDOACMAG::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MLMEMDJGGOF::generated_message_descriptor_data());
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
