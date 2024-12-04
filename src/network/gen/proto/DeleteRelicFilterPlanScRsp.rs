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

//! Generated file from `DeleteRelicFilterPlanScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:DeleteRelicFilterPlanScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct DeleteRelicFilterPlanScRsp {
    // message fields
    // @@protoc_insertion_point(field:DeleteRelicFilterPlanScRsp.ADADHIHDHJC)
    pub ADADHIHDHJC: u32,
    // @@protoc_insertion_point(field:DeleteRelicFilterPlanScRsp.FJKGBMGLLGL)
    pub FJKGBMGLLGL: bool,
    // @@protoc_insertion_point(field:DeleteRelicFilterPlanScRsp.JMBLJCIFCON)
    pub JMBLJCIFCON: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:DeleteRelicFilterPlanScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a DeleteRelicFilterPlanScRsp {
    fn default() -> &'a DeleteRelicFilterPlanScRsp {
        <DeleteRelicFilterPlanScRsp as ::protobuf::Message>::default_instance()
    }
}

impl DeleteRelicFilterPlanScRsp {
    pub fn new() -> DeleteRelicFilterPlanScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "ADADHIHDHJC",
            |m: &DeleteRelicFilterPlanScRsp| { &m.ADADHIHDHJC },
            |m: &mut DeleteRelicFilterPlanScRsp| { &mut m.ADADHIHDHJC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FJKGBMGLLGL",
            |m: &DeleteRelicFilterPlanScRsp| { &m.FJKGBMGLLGL },
            |m: &mut DeleteRelicFilterPlanScRsp| { &mut m.FJKGBMGLLGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JMBLJCIFCON",
            |m: &DeleteRelicFilterPlanScRsp| { &m.JMBLJCIFCON },
            |m: &mut DeleteRelicFilterPlanScRsp| { &mut m.JMBLJCIFCON },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<DeleteRelicFilterPlanScRsp>(
            "DeleteRelicFilterPlanScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for DeleteRelicFilterPlanScRsp {
    const NAME: &'static str = "DeleteRelicFilterPlanScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                72 => {
                    self.ADADHIHDHJC = is.read_uint32()?;
                },
                48 => {
                    self.FJKGBMGLLGL = is.read_bool()?;
                },
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.JMBLJCIFCON)?;
                },
                40 => {
                    self.JMBLJCIFCON.push(is.read_uint32()?);
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
        if self.ADADHIHDHJC != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.ADADHIHDHJC);
        }
        if self.FJKGBMGLLGL != false {
            my_size += 1 + 1;
        }
        for value in &self.JMBLJCIFCON {
            my_size += ::protobuf::rt::uint32_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.ADADHIHDHJC != 0 {
            os.write_uint32(9, self.ADADHIHDHJC)?;
        }
        if self.FJKGBMGLLGL != false {
            os.write_bool(6, self.FJKGBMGLLGL)?;
        }
        for v in &self.JMBLJCIFCON {
            os.write_uint32(5, *v)?;
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

    fn new() -> DeleteRelicFilterPlanScRsp {
        DeleteRelicFilterPlanScRsp::new()
    }

    fn clear(&mut self) {
        self.ADADHIHDHJC = 0;
        self.FJKGBMGLLGL = false;
        self.JMBLJCIFCON.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static DeleteRelicFilterPlanScRsp {
        static instance: DeleteRelicFilterPlanScRsp = DeleteRelicFilterPlanScRsp {
            ADADHIHDHJC: 0,
            FJKGBMGLLGL: false,
            JMBLJCIFCON: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for DeleteRelicFilterPlanScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("DeleteRelicFilterPlanScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for DeleteRelicFilterPlanScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRelicFilterPlanScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20DeleteRelicFilterPlanScRsp.proto\"\x82\x01\n\x1aDeleteRelicFilterP\
    lanScRsp\x12\x20\n\x0bADADHIHDHJC\x18\t\x20\x01(\rR\x0bADADHIHDHJC\x12\
    \x20\n\x0bFJKGBMGLLGL\x18\x06\x20\x01(\x08R\x0bFJKGBMGLLGL\x12\x20\n\x0b\
    JMBLJCIFCON\x18\x05\x20\x03(\rR\x0bJMBLJCIFCONb\x06proto3\
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
            messages.push(DeleteRelicFilterPlanScRsp::generated_message_descriptor_data());
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
