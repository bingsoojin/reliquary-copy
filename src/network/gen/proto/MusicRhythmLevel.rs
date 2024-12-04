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

//! Generated file from `MusicRhythmLevel.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MusicRhythmLevel)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MusicRhythmLevel {
    // message fields
    // @@protoc_insertion_point(field:MusicRhythmLevel.now_star_num)
    pub now_star_num: u32,
    // @@protoc_insertion_point(field:MusicRhythmLevel.is_full_combo)
    pub is_full_combo: bool,
    // @@protoc_insertion_point(field:MusicRhythmLevel.level_id)
    pub level_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MusicRhythmLevel.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MusicRhythmLevel {
    fn default() -> &'a MusicRhythmLevel {
        <MusicRhythmLevel as ::protobuf::Message>::default_instance()
    }
}

impl MusicRhythmLevel {
    pub fn new() -> MusicRhythmLevel {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "now_star_num",
            |m: &MusicRhythmLevel| { &m.now_star_num },
            |m: &mut MusicRhythmLevel| { &mut m.now_star_num },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_full_combo",
            |m: &MusicRhythmLevel| { &m.is_full_combo },
            |m: &mut MusicRhythmLevel| { &mut m.is_full_combo },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_id",
            |m: &MusicRhythmLevel| { &m.level_id },
            |m: &mut MusicRhythmLevel| { &mut m.level_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MusicRhythmLevel>(
            "MusicRhythmLevel",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MusicRhythmLevel {
    const NAME: &'static str = "MusicRhythmLevel";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                104 => {
                    self.now_star_num = is.read_uint32()?;
                },
                64 => {
                    self.is_full_combo = is.read_bool()?;
                },
                40 => {
                    self.level_id = is.read_uint32()?;
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
        if self.now_star_num != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.now_star_num);
        }
        if self.is_full_combo != false {
            my_size += 1 + 1;
        }
        if self.level_id != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.level_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.now_star_num != 0 {
            os.write_uint32(13, self.now_star_num)?;
        }
        if self.is_full_combo != false {
            os.write_bool(8, self.is_full_combo)?;
        }
        if self.level_id != 0 {
            os.write_uint32(5, self.level_id)?;
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

    fn new() -> MusicRhythmLevel {
        MusicRhythmLevel::new()
    }

    fn clear(&mut self) {
        self.now_star_num = 0;
        self.is_full_combo = false;
        self.level_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MusicRhythmLevel {
        static instance: MusicRhythmLevel = MusicRhythmLevel {
            now_star_num: 0,
            is_full_combo: false,
            level_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MusicRhythmLevel {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MusicRhythmLevel").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MusicRhythmLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MusicRhythmLevel {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16MusicRhythmLevel.proto\"s\n\x10MusicRhythmLevel\x12\x20\n\x0cnow_s\
    tar_num\x18\r\x20\x01(\rR\nnowStarNum\x12\"\n\ris_full_combo\x18\x08\x20\
    \x01(\x08R\x0bisFullCombo\x12\x19\n\x08level_id\x18\x05\x20\x01(\rR\x07l\
    evelIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            messages.push(MusicRhythmLevel::generated_message_descriptor_data());
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
