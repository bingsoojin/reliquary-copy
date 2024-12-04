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

//! Generated file from `VirtualLineupTrialAvatarChangeScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:VirtualLineupTrialAvatarChangeScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct VirtualLineupTrialAvatarChangeScNotify {
    // message fields
    // @@protoc_insertion_point(field:VirtualLineupTrialAvatarChangeScNotify.NLHFLLFPEHE)
    pub NLHFLLFPEHE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:VirtualLineupTrialAvatarChangeScNotify.HNELIDCNCCC)
    pub HNELIDCNCCC: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:VirtualLineupTrialAvatarChangeScNotify.MFFGPCCKLCG)
    pub MFFGPCCKLCG: bool,
    // @@protoc_insertion_point(field:VirtualLineupTrialAvatarChangeScNotify.KOCMDLFBKEI)
    pub KOCMDLFBKEI: u32,
    // special fields
    // @@protoc_insertion_point(special_field:VirtualLineupTrialAvatarChangeScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a VirtualLineupTrialAvatarChangeScNotify {
    fn default() -> &'a VirtualLineupTrialAvatarChangeScNotify {
        <VirtualLineupTrialAvatarChangeScNotify as ::protobuf::Message>::default_instance()
    }
}

impl VirtualLineupTrialAvatarChangeScNotify {
    pub fn new() -> VirtualLineupTrialAvatarChangeScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "NLHFLLFPEHE",
            |m: &VirtualLineupTrialAvatarChangeScNotify| { &m.NLHFLLFPEHE },
            |m: &mut VirtualLineupTrialAvatarChangeScNotify| { &mut m.NLHFLLFPEHE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HNELIDCNCCC",
            |m: &VirtualLineupTrialAvatarChangeScNotify| { &m.HNELIDCNCCC },
            |m: &mut VirtualLineupTrialAvatarChangeScNotify| { &mut m.HNELIDCNCCC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MFFGPCCKLCG",
            |m: &VirtualLineupTrialAvatarChangeScNotify| { &m.MFFGPCCKLCG },
            |m: &mut VirtualLineupTrialAvatarChangeScNotify| { &mut m.MFFGPCCKLCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "KOCMDLFBKEI",
            |m: &VirtualLineupTrialAvatarChangeScNotify| { &m.KOCMDLFBKEI },
            |m: &mut VirtualLineupTrialAvatarChangeScNotify| { &mut m.KOCMDLFBKEI },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<VirtualLineupTrialAvatarChangeScNotify>(
            "VirtualLineupTrialAvatarChangeScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for VirtualLineupTrialAvatarChangeScNotify {
    const NAME: &'static str = "VirtualLineupTrialAvatarChangeScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    is.read_repeated_packed_uint32_into(&mut self.NLHFLLFPEHE)?;
                },
                64 => {
                    self.NLHFLLFPEHE.push(is.read_uint32()?);
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.HNELIDCNCCC)?;
                },
                16 => {
                    self.HNELIDCNCCC.push(is.read_uint32()?);
                },
                8 => {
                    self.MFFGPCCKLCG = is.read_bool()?;
                },
                96 => {
                    self.KOCMDLFBKEI = is.read_uint32()?;
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
        for value in &self.NLHFLLFPEHE {
            my_size += ::protobuf::rt::uint32_size(8, *value);
        };
        for value in &self.HNELIDCNCCC {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.MFFGPCCKLCG != false {
            my_size += 1 + 1;
        }
        if self.KOCMDLFBKEI != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.KOCMDLFBKEI);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.NLHFLLFPEHE {
            os.write_uint32(8, *v)?;
        };
        for v in &self.HNELIDCNCCC {
            os.write_uint32(2, *v)?;
        };
        if self.MFFGPCCKLCG != false {
            os.write_bool(1, self.MFFGPCCKLCG)?;
        }
        if self.KOCMDLFBKEI != 0 {
            os.write_uint32(12, self.KOCMDLFBKEI)?;
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

    fn new() -> VirtualLineupTrialAvatarChangeScNotify {
        VirtualLineupTrialAvatarChangeScNotify::new()
    }

    fn clear(&mut self) {
        self.NLHFLLFPEHE.clear();
        self.HNELIDCNCCC.clear();
        self.MFFGPCCKLCG = false;
        self.KOCMDLFBKEI = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static VirtualLineupTrialAvatarChangeScNotify {
        static instance: VirtualLineupTrialAvatarChangeScNotify = VirtualLineupTrialAvatarChangeScNotify {
            NLHFLLFPEHE: ::std::vec::Vec::new(),
            HNELIDCNCCC: ::std::vec::Vec::new(),
            MFFGPCCKLCG: false,
            KOCMDLFBKEI: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for VirtualLineupTrialAvatarChangeScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("VirtualLineupTrialAvatarChangeScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for VirtualLineupTrialAvatarChangeScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VirtualLineupTrialAvatarChangeScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,VirtualLineupTrialAvatarChangeScNotify.proto\"\xb0\x01\n&VirtualLineu\
    pTrialAvatarChangeScNotify\x12\x20\n\x0bNLHFLLFPEHE\x18\x08\x20\x03(\rR\
    \x0bNLHFLLFPEHE\x12\x20\n\x0bHNELIDCNCCC\x18\x02\x20\x03(\rR\x0bHNELIDCN\
    CCC\x12\x20\n\x0bMFFGPCCKLCG\x18\x01\x20\x01(\x08R\x0bMFFGPCCKLCG\x12\
    \x20\n\x0bKOCMDLFBKEI\x18\x0c\x20\x01(\rR\x0bKOCMDLFBKEIb\x06proto3\
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
            messages.push(VirtualLineupTrialAvatarChangeScNotify::generated_message_descriptor_data());
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
