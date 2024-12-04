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

//! Generated file from `MazeGroup.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MazeGroup)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MazeGroup {
    // message fields
    // @@protoc_insertion_point(field:MazeGroup.GFLMKIGHMDE)
    pub GFLMKIGHMDE: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MazeGroup.JJANEMOGDJF)
    pub JJANEMOGDJF: bool,
    // @@protoc_insertion_point(field:MazeGroup.group_id)
    pub group_id: u32,
    // @@protoc_insertion_point(field:MazeGroup.modify_time)
    pub modify_time: i64,
    // special fields
    // @@protoc_insertion_point(special_field:MazeGroup.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MazeGroup {
    fn default() -> &'a MazeGroup {
        <MazeGroup as ::protobuf::Message>::default_instance()
    }
}

impl MazeGroup {
    pub fn new() -> MazeGroup {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "GFLMKIGHMDE",
            |m: &MazeGroup| { &m.GFLMKIGHMDE },
            |m: &mut MazeGroup| { &mut m.GFLMKIGHMDE },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JJANEMOGDJF",
            |m: &MazeGroup| { &m.JJANEMOGDJF },
            |m: &mut MazeGroup| { &mut m.JJANEMOGDJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &MazeGroup| { &m.group_id },
            |m: &mut MazeGroup| { &mut m.group_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "modify_time",
            |m: &MazeGroup| { &m.modify_time },
            |m: &mut MazeGroup| { &mut m.modify_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MazeGroup>(
            "MazeGroup",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MazeGroup {
    const NAME: &'static str = "MazeGroup";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.GFLMKIGHMDE)?;
                },
                80 => {
                    self.GFLMKIGHMDE.push(is.read_uint32()?);
                },
                48 => {
                    self.JJANEMOGDJF = is.read_bool()?;
                },
                120 => {
                    self.group_id = is.read_uint32()?;
                },
                56 => {
                    self.modify_time = is.read_int64()?;
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
        for value in &self.GFLMKIGHMDE {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.JJANEMOGDJF != false {
            my_size += 1 + 1;
        }
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.group_id);
        }
        if self.modify_time != 0 {
            my_size += ::protobuf::rt::int64_size(7, self.modify_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.GFLMKIGHMDE {
            os.write_uint32(10, *v)?;
        };
        if self.JJANEMOGDJF != false {
            os.write_bool(6, self.JJANEMOGDJF)?;
        }
        if self.group_id != 0 {
            os.write_uint32(15, self.group_id)?;
        }
        if self.modify_time != 0 {
            os.write_int64(7, self.modify_time)?;
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

    fn new() -> MazeGroup {
        MazeGroup::new()
    }

    fn clear(&mut self) {
        self.GFLMKIGHMDE.clear();
        self.JJANEMOGDJF = false;
        self.group_id = 0;
        self.modify_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MazeGroup {
        static instance: MazeGroup = MazeGroup {
            GFLMKIGHMDE: ::std::vec::Vec::new(),
            JJANEMOGDJF: false,
            group_id: 0,
            modify_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MazeGroup {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MazeGroup").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MazeGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MazeGroup {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fMazeGroup.proto\"\x8b\x01\n\tMazeGroup\x12\x20\n\x0bGFLMKIGHMDE\
    \x18\n\x20\x03(\rR\x0bGFLMKIGHMDE\x12\x20\n\x0bJJANEMOGDJF\x18\x06\x20\
    \x01(\x08R\x0bJJANEMOGDJF\x12\x19\n\x08group_id\x18\x0f\x20\x01(\rR\x07g\
    roupId\x12\x1f\n\x0bmodify_time\x18\x07\x20\x01(\x03R\nmodifyTimeB\x15\n\
    \x13emu.lunarcore.protob\x06proto3\
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
            messages.push(MazeGroup::generated_message_descriptor_data());
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
