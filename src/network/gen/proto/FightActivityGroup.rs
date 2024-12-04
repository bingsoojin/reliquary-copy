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

//! Generated file from `FightActivityGroup.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FightActivityGroup)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FightActivityGroup {
    // message fields
    // @@protoc_insertion_point(field:FightActivityGroup.passed_max_difficulty_level)
    pub passed_max_difficulty_level: u32,
    // @@protoc_insertion_point(field:FightActivityGroup.endless_max_wave)
    pub endless_max_wave: u32,
    // @@protoc_insertion_point(field:FightActivityGroup.taken_difficulty_level_reward_list)
    pub taken_difficulty_level_reward_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:FightActivityGroup.group_id)
    pub group_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:FightActivityGroup.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FightActivityGroup {
    fn default() -> &'a FightActivityGroup {
        <FightActivityGroup as ::protobuf::Message>::default_instance()
    }
}

impl FightActivityGroup {
    pub fn new() -> FightActivityGroup {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "passed_max_difficulty_level",
            |m: &FightActivityGroup| { &m.passed_max_difficulty_level },
            |m: &mut FightActivityGroup| { &mut m.passed_max_difficulty_level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "endless_max_wave",
            |m: &FightActivityGroup| { &m.endless_max_wave },
            |m: &mut FightActivityGroup| { &mut m.endless_max_wave },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "taken_difficulty_level_reward_list",
            |m: &FightActivityGroup| { &m.taken_difficulty_level_reward_list },
            |m: &mut FightActivityGroup| { &mut m.taken_difficulty_level_reward_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "group_id",
            |m: &FightActivityGroup| { &m.group_id },
            |m: &mut FightActivityGroup| { &mut m.group_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FightActivityGroup>(
            "FightActivityGroup",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FightActivityGroup {
    const NAME: &'static str = "FightActivityGroup";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                120 => {
                    self.passed_max_difficulty_level = is.read_uint32()?;
                },
                32 => {
                    self.endless_max_wave = is.read_uint32()?;
                },
                90 => {
                    is.read_repeated_packed_uint32_into(&mut self.taken_difficulty_level_reward_list)?;
                },
                88 => {
                    self.taken_difficulty_level_reward_list.push(is.read_uint32()?);
                },
                64 => {
                    self.group_id = is.read_uint32()?;
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
        if self.passed_max_difficulty_level != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.passed_max_difficulty_level);
        }
        if self.endless_max_wave != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.endless_max_wave);
        }
        for value in &self.taken_difficulty_level_reward_list {
            my_size += ::protobuf::rt::uint32_size(11, *value);
        };
        if self.group_id != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.group_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.passed_max_difficulty_level != 0 {
            os.write_uint32(15, self.passed_max_difficulty_level)?;
        }
        if self.endless_max_wave != 0 {
            os.write_uint32(4, self.endless_max_wave)?;
        }
        for v in &self.taken_difficulty_level_reward_list {
            os.write_uint32(11, *v)?;
        };
        if self.group_id != 0 {
            os.write_uint32(8, self.group_id)?;
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

    fn new() -> FightActivityGroup {
        FightActivityGroup::new()
    }

    fn clear(&mut self) {
        self.passed_max_difficulty_level = 0;
        self.endless_max_wave = 0;
        self.taken_difficulty_level_reward_list.clear();
        self.group_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FightActivityGroup {
        static instance: FightActivityGroup = FightActivityGroup {
            passed_max_difficulty_level: 0,
            endless_max_wave: 0,
            taken_difficulty_level_reward_list: ::std::vec::Vec::new(),
            group_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FightActivityGroup {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FightActivityGroup").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FightActivityGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FightActivityGroup {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18FightActivityGroup.proto\"\xe4\x01\n\x12FightActivityGroup\x12=\n\
    \x1bpassed_max_difficulty_level\x18\x0f\x20\x01(\rR\x18passedMaxDifficul\
    tyLevel\x12(\n\x10endless_max_wave\x18\x04\x20\x01(\rR\x0eendlessMaxWave\
    \x12J\n\"taken_difficulty_level_reward_list\x18\x0b\x20\x03(\rR\x1etaken\
    DifficultyLevelRewardList\x12\x19\n\x08group_id\x18\x08\x20\x01(\rR\x07g\
    roupIdb\x06proto3\
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
            messages.push(FightActivityGroup::generated_message_descriptor_data());
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
