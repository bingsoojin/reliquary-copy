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

//! Generated file from `RogueTournLevel.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:RogueTournLevel)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct RogueTournLevel {
    // message fields
    // @@protoc_insertion_point(field:RogueTournLevel.cur_room_index)
    pub cur_room_index: u32,
    // @@protoc_insertion_point(field:RogueTournLevel.status)
    pub status: ::protobuf::EnumOrUnknown<super::RogueTournLayerStatus::RogueTournLayerStatus>,
    // @@protoc_insertion_point(field:RogueTournLevel.ext_room_list)
    pub ext_room_list: ::std::vec::Vec<super::RogueTournRoomList::RogueTournRoomList>,
    // @@protoc_insertion_point(field:RogueTournLevel.level_index)
    pub level_index: u32,
    // @@protoc_insertion_point(field:RogueTournLevel.layer_id)
    pub layer_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:RogueTournLevel.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a RogueTournLevel {
    fn default() -> &'a RogueTournLevel {
        <RogueTournLevel as ::protobuf::Message>::default_instance()
    }
}

impl RogueTournLevel {
    pub fn new() -> RogueTournLevel {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_room_index",
            |m: &RogueTournLevel| { &m.cur_room_index },
            |m: &mut RogueTournLevel| { &mut m.cur_room_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &RogueTournLevel| { &m.status },
            |m: &mut RogueTournLevel| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ext_room_list",
            |m: &RogueTournLevel| { &m.ext_room_list },
            |m: &mut RogueTournLevel| { &mut m.ext_room_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level_index",
            |m: &RogueTournLevel| { &m.level_index },
            |m: &mut RogueTournLevel| { &mut m.level_index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "layer_id",
            |m: &RogueTournLevel| { &m.layer_id },
            |m: &mut RogueTournLevel| { &mut m.layer_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<RogueTournLevel>(
            "RogueTournLevel",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for RogueTournLevel {
    const NAME: &'static str = "RogueTournLevel";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.cur_room_index = is.read_uint32()?;
                },
                56 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                122 => {
                    self.ext_room_list.push(is.read_message()?);
                },
                104 => {
                    self.level_index = is.read_uint32()?;
                },
                48 => {
                    self.layer_id = is.read_uint32()?;
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
        if self.cur_room_index != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.cur_room_index);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTournLayerStatus::RogueTournLayerStatus::ROGUE_TOURN_LAYER_STATUS_NONE) {
            my_size += ::protobuf::rt::int32_size(7, self.status.value());
        }
        for value in &self.ext_room_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.level_index != 0 {
            my_size += ::protobuf::rt::uint32_size(13, self.level_index);
        }
        if self.layer_id != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.layer_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.cur_room_index != 0 {
            os.write_uint32(11, self.cur_room_index)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(super::RogueTournLayerStatus::RogueTournLayerStatus::ROGUE_TOURN_LAYER_STATUS_NONE) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        for v in &self.ext_room_list {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.level_index != 0 {
            os.write_uint32(13, self.level_index)?;
        }
        if self.layer_id != 0 {
            os.write_uint32(6, self.layer_id)?;
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

    fn new() -> RogueTournLevel {
        RogueTournLevel::new()
    }

    fn clear(&mut self) {
        self.cur_room_index = 0;
        self.status = ::protobuf::EnumOrUnknown::new(super::RogueTournLayerStatus::RogueTournLayerStatus::ROGUE_TOURN_LAYER_STATUS_NONE);
        self.ext_room_list.clear();
        self.level_index = 0;
        self.layer_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static RogueTournLevel {
        static instance: RogueTournLevel = RogueTournLevel {
            cur_room_index: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            ext_room_list: ::std::vec::Vec::new(),
            level_index: 0,
            layer_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for RogueTournLevel {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("RogueTournLevel").unwrap()).clone()
    }
}

impl ::std::fmt::Display for RogueTournLevel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RogueTournLevel {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15RogueTournLevel.proto\x1a\x1bRogueTournLayerStatus.proto\x1a\x18Ro\
    gueTournRoomList.proto\"\xdc\x01\n\x0fRogueTournLevel\x12$\n\x0ecur_room\
    _index\x18\x0b\x20\x01(\rR\x0ccurRoomIndex\x12.\n\x06status\x18\x07\x20\
    \x01(\x0e2\x16.RogueTournLayerStatusR\x06status\x127\n\rext_room_list\
    \x18\x0f\x20\x03(\x0b2\x13.RogueTournRoomListR\x0bextRoomList\x12\x1f\n\
    \x0blevel_index\x18\r\x20\x01(\rR\nlevelIndex\x12\x19\n\x08layer_id\x18\
    \x06\x20\x01(\rR\x07layerIdB\x15\n\x13emu.lunarcore.protob\x06proto3\
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
            deps.push(super::RogueTournLayerStatus::file_descriptor().clone());
            deps.push(super::RogueTournRoomList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(RogueTournLevel::generated_message_descriptor_data());
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
