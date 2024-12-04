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

//! Generated file from `FriendListInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:FriendListInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct FriendListInfo {
    // message fields
    // @@protoc_insertion_point(field:FriendListInfo.playing_state)
    pub playing_state: ::protobuf::EnumOrUnknown<super::PlayingState::PlayingState>,
    // @@protoc_insertion_point(field:FriendListInfo.sent_time)
    pub sent_time: i64,
    // @@protoc_insertion_point(field:FriendListInfo.is_marked)
    pub is_marked: bool,
    // @@protoc_insertion_point(field:FriendListInfo.friend_name)
    pub friend_name: ::std::string::String,
    // @@protoc_insertion_point(field:FriendListInfo.simple_info)
    pub simple_info: ::protobuf::MessageField<super::SimpleInfo::SimpleInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:FriendListInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a FriendListInfo {
    fn default() -> &'a FriendListInfo {
        <FriendListInfo as ::protobuf::Message>::default_instance()
    }
}

impl FriendListInfo {
    pub fn new() -> FriendListInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "playing_state",
            |m: &FriendListInfo| { &m.playing_state },
            |m: &mut FriendListInfo| { &mut m.playing_state },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "sent_time",
            |m: &FriendListInfo| { &m.sent_time },
            |m: &mut FriendListInfo| { &mut m.sent_time },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_marked",
            |m: &FriendListInfo| { &m.is_marked },
            |m: &mut FriendListInfo| { &mut m.is_marked },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "friend_name",
            |m: &FriendListInfo| { &m.friend_name },
            |m: &mut FriendListInfo| { &mut m.friend_name },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::SimpleInfo::SimpleInfo>(
            "simple_info",
            |m: &FriendListInfo| { &m.simple_info },
            |m: &mut FriendListInfo| { &mut m.simple_info },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<FriendListInfo>(
            "FriendListInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for FriendListInfo {
    const NAME: &'static str = "FriendListInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                64 => {
                    self.playing_state = is.read_enum_or_unknown()?;
                },
                40 => {
                    self.sent_time = is.read_int64()?;
                },
                56 => {
                    self.is_marked = is.read_bool()?;
                },
                74 => {
                    self.friend_name = is.read_string()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.simple_info)?;
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
        if self.playing_state != ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE) {
            my_size += ::protobuf::rt::int32_size(8, self.playing_state.value());
        }
        if self.sent_time != 0 {
            my_size += ::protobuf::rt::int64_size(5, self.sent_time);
        }
        if self.is_marked != false {
            my_size += 1 + 1;
        }
        if !self.friend_name.is_empty() {
            my_size += ::protobuf::rt::string_size(9, &self.friend_name);
        }
        if let Some(v) = self.simple_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.playing_state != ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE) {
            os.write_enum(8, ::protobuf::EnumOrUnknown::value(&self.playing_state))?;
        }
        if self.sent_time != 0 {
            os.write_int64(5, self.sent_time)?;
        }
        if self.is_marked != false {
            os.write_bool(7, self.is_marked)?;
        }
        if !self.friend_name.is_empty() {
            os.write_string(9, &self.friend_name)?;
        }
        if let Some(v) = self.simple_info.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
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

    fn new() -> FriendListInfo {
        FriendListInfo::new()
    }

    fn clear(&mut self) {
        self.playing_state = ::protobuf::EnumOrUnknown::new(super::PlayingState::PlayingState::PLAYING_STATE_NONE);
        self.sent_time = 0;
        self.is_marked = false;
        self.friend_name.clear();
        self.simple_info.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static FriendListInfo {
        static instance: FriendListInfo = FriendListInfo {
            playing_state: ::protobuf::EnumOrUnknown::from_i32(0),
            sent_time: 0,
            is_marked: false,
            friend_name: ::std::string::String::new(),
            simple_info: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for FriendListInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("FriendListInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for FriendListInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FriendListInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14FriendListInfo.proto\x1a\x10SimpleInfo.proto\x1a\x12PlayingState.p\
    roto\"\xcd\x01\n\x0eFriendListInfo\x122\n\rplaying_state\x18\x08\x20\x01\
    (\x0e2\r.PlayingStateR\x0cplayingState\x12\x1b\n\tsent_time\x18\x05\x20\
    \x01(\x03R\x08sentTime\x12\x1b\n\tis_marked\x18\x07\x20\x01(\x08R\x08isM\
    arked\x12\x1f\n\x0bfriend_name\x18\t\x20\x01(\tR\nfriendName\x12,\n\x0bs\
    imple_info\x18\x02\x20\x01(\x0b2\x0b.SimpleInfoR\nsimpleInfoB\x15\n\x13e\
    mu.lunarcore.protob\x06proto3\
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
            deps.push(super::SimpleInfo::file_descriptor().clone());
            deps.push(super::PlayingState::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(FriendListInfo::generated_message_descriptor_data());
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
