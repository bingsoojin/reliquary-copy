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

//! Generated file from `PlayerBasicInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:PlayerBasicInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerBasicInfo {
    // message fields
    // @@protoc_insertion_point(field:PlayerBasicInfo.nickname)
    pub nickname: ::std::string::String,
    // @@protoc_insertion_point(field:PlayerBasicInfo.level)
    pub level: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.stamina)
    pub stamina: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.mcoin)
    pub mcoin: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.hcoin)
    pub hcoin: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.scoin)
    pub scoin: u32,
    // @@protoc_insertion_point(field:PlayerBasicInfo.world_level)
    pub world_level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:PlayerBasicInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerBasicInfo {
    fn default() -> &'a PlayerBasicInfo {
        <PlayerBasicInfo as ::protobuf::Message>::default_instance()
    }
}

impl PlayerBasicInfo {
    pub fn new() -> PlayerBasicInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "nickname",
            |m: &PlayerBasicInfo| { &m.nickname },
            |m: &mut PlayerBasicInfo| { &mut m.nickname },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &PlayerBasicInfo| { &m.level },
            |m: &mut PlayerBasicInfo| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &PlayerBasicInfo| { &m.exp },
            |m: &mut PlayerBasicInfo| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "stamina",
            |m: &PlayerBasicInfo| { &m.stamina },
            |m: &mut PlayerBasicInfo| { &mut m.stamina },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "mcoin",
            |m: &PlayerBasicInfo| { &m.mcoin },
            |m: &mut PlayerBasicInfo| { &mut m.mcoin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "hcoin",
            |m: &PlayerBasicInfo| { &m.hcoin },
            |m: &mut PlayerBasicInfo| { &mut m.hcoin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "scoin",
            |m: &PlayerBasicInfo| { &m.scoin },
            |m: &mut PlayerBasicInfo| { &mut m.scoin },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "world_level",
            |m: &PlayerBasicInfo| { &m.world_level },
            |m: &mut PlayerBasicInfo| { &mut m.world_level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerBasicInfo>(
            "PlayerBasicInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerBasicInfo {
    const NAME: &'static str = "PlayerBasicInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.nickname = is.read_string()?;
                },
                16 => {
                    self.level = is.read_uint32()?;
                },
                24 => {
                    self.exp = is.read_uint32()?;
                },
                32 => {
                    self.stamina = is.read_uint32()?;
                },
                40 => {
                    self.mcoin = is.read_uint32()?;
                },
                48 => {
                    self.hcoin = is.read_uint32()?;
                },
                56 => {
                    self.scoin = is.read_uint32()?;
                },
                64 => {
                    self.world_level = is.read_uint32()?;
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
        if !self.nickname.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.nickname);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.level);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.exp);
        }
        if self.stamina != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.stamina);
        }
        if self.mcoin != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.mcoin);
        }
        if self.hcoin != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.hcoin);
        }
        if self.scoin != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.scoin);
        }
        if self.world_level != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.world_level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.nickname.is_empty() {
            os.write_string(1, &self.nickname)?;
        }
        if self.level != 0 {
            os.write_uint32(2, self.level)?;
        }
        if self.exp != 0 {
            os.write_uint32(3, self.exp)?;
        }
        if self.stamina != 0 {
            os.write_uint32(4, self.stamina)?;
        }
        if self.mcoin != 0 {
            os.write_uint32(5, self.mcoin)?;
        }
        if self.hcoin != 0 {
            os.write_uint32(6, self.hcoin)?;
        }
        if self.scoin != 0 {
            os.write_uint32(7, self.scoin)?;
        }
        if self.world_level != 0 {
            os.write_uint32(8, self.world_level)?;
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

    fn new() -> PlayerBasicInfo {
        PlayerBasicInfo::new()
    }

    fn clear(&mut self) {
        self.nickname.clear();
        self.level = 0;
        self.exp = 0;
        self.stamina = 0;
        self.mcoin = 0;
        self.hcoin = 0;
        self.scoin = 0;
        self.world_level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerBasicInfo {
        static instance: PlayerBasicInfo = PlayerBasicInfo {
            nickname: ::std::string::String::new(),
            level: 0,
            exp: 0,
            stamina: 0,
            mcoin: 0,
            hcoin: 0,
            scoin: 0,
            world_level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerBasicInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerBasicInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerBasicInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerBasicInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x15PlayerBasicInfo.proto\"\xd2\x01\n\x0fPlayerBasicInfo\x12\x1a\n\x08\
    nickname\x18\x01\x20\x01(\tR\x08nickname\x12\x14\n\x05level\x18\x02\x20\
    \x01(\rR\x05level\x12\x10\n\x03exp\x18\x03\x20\x01(\rR\x03exp\x12\x18\n\
    \x07stamina\x18\x04\x20\x01(\rR\x07stamina\x12\x14\n\x05mcoin\x18\x05\
    \x20\x01(\rR\x05mcoin\x12\x14\n\x05hcoin\x18\x06\x20\x01(\rR\x05hcoin\
    \x12\x14\n\x05scoin\x18\x07\x20\x01(\rR\x05scoin\x12\x1f\n\x0bworld_leve\
    l\x18\x08\x20\x01(\rR\nworldLevelB\x15\n\x13emu.lunarcore.protob\x06prot\
    o3\
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
            messages.push(PlayerBasicInfo::generated_message_descriptor_data());
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
