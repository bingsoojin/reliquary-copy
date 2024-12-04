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

//! Generated file from `RogueExtLineupInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueExtLineupInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueExtLineupInfo {
    // message fields
    // @@protoc_insertion_point(field:RogueExtLineupInfo.avatar_id_list)
    pub avatar_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:RogueExtLineupInfo.rogue_revive_cost)
    pub rogue_revive_cost: ::protobuf::MessageField<super::ItemCostList::ItemCostList>,
    // special fields
    // @@protoc_insertion_point(special_field:RogueExtLineupInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueExtLineupInfo {
    fn default() -> &'a RogueExtLineupInfo {
        <RogueExtLineupInfo as ::protobuf::Message>::default_instance()
    }
}

impl RogueExtLineupInfo {
    pub fn new() -> RogueExtLineupInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "avatar_id_list",
            |m: &RogueExtLineupInfo| { &m.avatar_id_list },
            |m: &mut RogueExtLineupInfo| { &mut m.avatar_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemCostList::ItemCostList>(
            "rogue_revive_cost",
            |m: &RogueExtLineupInfo| { &m.rogue_revive_cost },
            |m: &mut RogueExtLineupInfo| { &mut m.rogue_revive_cost },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueExtLineupInfo>(
            "RogueExtLineupInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueExtLineupInfo {
    const NAME: &'static str = "RogueExtLineupInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                114 => {
                    is.read_repeated_packed_uint32_into(&mut self.avatar_id_list)?;
                },
                112 => {
                    self.avatar_id_list.push(is.read_uint32()?);
                },
                42 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.rogue_revive_cost)?;
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
        for value in &self.avatar_id_list {
            my_size += ::protobuf::rt::uint32_size(14, *value);
        };
        if let Some(v) = self.rogue_revive_cost.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.avatar_id_list {
            os.write_uint32(14, *v)?;
        };
        if let Some(v) = self.rogue_revive_cost.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> RogueExtLineupInfo {
        RogueExtLineupInfo::new()
    }

    fn clear(&mut self) {
        self.avatar_id_list.clear();
        self.rogue_revive_cost.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueExtLineupInfo {
        static instance: RogueExtLineupInfo = RogueExtLineupInfo {
            avatar_id_list: ::std::vec::Vec::new(),
            rogue_revive_cost: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueExtLineupInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueExtLineupInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueExtLineupInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueExtLineupInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18RogueExtLineupInfo.proto\x1a\x12ItemCostList.proto\"u\n\x12RogueEx\
    tLineupInfo\x12$\n\x0eavatar_id_list\x18\x0e\x20\x03(\rR\x0cavatarIdList\
    \x129\n\x11rogue_revive_cost\x18\x05\x20\x01(\x0b2\r.ItemCostListR\x0fro\
    gueReviveCostB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::ItemCostList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueExtLineupInfo::generated_message_descriptor_data());
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
