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

//! Generated file from `ReplaceLineupCsReq.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:ReplaceLineupCsReq)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct ReplaceLineupCsReq {
    // message fields
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.leader_slot)
    pub leader_slot: u32,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.index)
    pub index: u32,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.slots)
    pub slots: ::std::vec::Vec<super::LineupSlotData::LineupSlotData>,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.extra_lineup_type)
    pub extra_lineup_type: ::protobuf::EnumOrUnknown<super::ExtraLineupType::ExtraLineupType>,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.GFKIHHOPJDG)
    pub GFKIHHOPJDG: u32,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.is_virtual)
    pub is_virtual: bool,
    // @@protoc_insertion_point(field:ReplaceLineupCsReq.plane_id)
    pub plane_id: u32,
    // special fields
    // @@protoc_insertion_point(special_field:ReplaceLineupCsReq.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a ReplaceLineupCsReq {
    fn default() -> &'a ReplaceLineupCsReq {
        <ReplaceLineupCsReq as ::protobuf::Message>::default_instance()
    }
}

impl ReplaceLineupCsReq {
    pub fn new() -> ReplaceLineupCsReq {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "leader_slot",
            |m: &ReplaceLineupCsReq| { &m.leader_slot },
            |m: &mut ReplaceLineupCsReq| { &mut m.leader_slot },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "index",
            |m: &ReplaceLineupCsReq| { &m.index },
            |m: &mut ReplaceLineupCsReq| { &mut m.index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "slots",
            |m: &ReplaceLineupCsReq| { &m.slots },
            |m: &mut ReplaceLineupCsReq| { &mut m.slots },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "extra_lineup_type",
            |m: &ReplaceLineupCsReq| { &m.extra_lineup_type },
            |m: &mut ReplaceLineupCsReq| { &mut m.extra_lineup_type },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GFKIHHOPJDG",
            |m: &ReplaceLineupCsReq| { &m.GFKIHHOPJDG },
            |m: &mut ReplaceLineupCsReq| { &mut m.GFKIHHOPJDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "is_virtual",
            |m: &ReplaceLineupCsReq| { &m.is_virtual },
            |m: &mut ReplaceLineupCsReq| { &mut m.is_virtual },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "plane_id",
            |m: &ReplaceLineupCsReq| { &m.plane_id },
            |m: &mut ReplaceLineupCsReq| { &mut m.plane_id },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<ReplaceLineupCsReq>(
            "ReplaceLineupCsReq",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for ReplaceLineupCsReq {
    const NAME: &'static str = "ReplaceLineupCsReq";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                88 => {
                    self.leader_slot = is.read_uint32()?;
                },
                72 => {
                    self.index = is.read_uint32()?;
                },
                50 => {
                    self.slots.push(is.read_message()?);
                },
                112 => {
                    self.extra_lineup_type = is.read_enum_or_unknown()?;
                },
                24 => {
                    self.GFKIHHOPJDG = is.read_uint32()?;
                },
                80 => {
                    self.is_virtual = is.read_bool()?;
                },
                32 => {
                    self.plane_id = is.read_uint32()?;
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
        if self.leader_slot != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.leader_slot);
        }
        if self.index != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.index);
        }
        for value in &self.slots {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            my_size += ::protobuf::rt::int32_size(14, self.extra_lineup_type.value());
        }
        if self.GFKIHHOPJDG != 0 {
            my_size += ::protobuf::rt::uint32_size(3, self.GFKIHHOPJDG);
        }
        if self.is_virtual != false {
            my_size += 1 + 1;
        }
        if self.plane_id != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.plane_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.leader_slot != 0 {
            os.write_uint32(11, self.leader_slot)?;
        }
        if self.index != 0 {
            os.write_uint32(9, self.index)?;
        }
        for v in &self.slots {
            ::protobuf::rt::write_message_field_with_cached_size(6, v, os)?;
        };
        if self.extra_lineup_type != ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE) {
            os.write_enum(14, ::protobuf::EnumOrUnknown::value(&self.extra_lineup_type))?;
        }
        if self.GFKIHHOPJDG != 0 {
            os.write_uint32(3, self.GFKIHHOPJDG)?;
        }
        if self.is_virtual != false {
            os.write_bool(10, self.is_virtual)?;
        }
        if self.plane_id != 0 {
            os.write_uint32(4, self.plane_id)?;
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

    fn new() -> ReplaceLineupCsReq {
        ReplaceLineupCsReq::new()
    }

    fn clear(&mut self) {
        self.leader_slot = 0;
        self.index = 0;
        self.slots.clear();
        self.extra_lineup_type = ::protobuf::EnumOrUnknown::new(super::ExtraLineupType::ExtraLineupType::LINEUP_NONE);
        self.GFKIHHOPJDG = 0;
        self.is_virtual = false;
        self.plane_id = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static ReplaceLineupCsReq {
        static instance: ReplaceLineupCsReq = ReplaceLineupCsReq {
            leader_slot: 0,
            index: 0,
            slots: ::std::vec::Vec::new(),
            extra_lineup_type: ::protobuf::EnumOrUnknown::from_i32(0),
            GFKIHHOPJDG: 0,
            is_virtual: false,
            plane_id: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for ReplaceLineupCsReq {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("ReplaceLineupCsReq").unwrap()).clone()
    }
}

impl ::std::fmt::Display for ReplaceLineupCsReq {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplaceLineupCsReq {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18ReplaceLineupCsReq.proto\x1a\x15ExtraLineupType.proto\x1a\x14Lineu\
    pSlotData.proto\"\x8c\x02\n\x12ReplaceLineupCsReq\x12\x1f\n\x0bleader_sl\
    ot\x18\x0b\x20\x01(\rR\nleaderSlot\x12\x14\n\x05index\x18\t\x20\x01(\rR\
    \x05index\x12%\n\x05slots\x18\x06\x20\x03(\x0b2\x0f.LineupSlotDataR\x05s\
    lots\x12<\n\x11extra_lineup_type\x18\x0e\x20\x01(\x0e2\x10.ExtraLineupTy\
    peR\x0fextraLineupType\x12\x20\n\x0bGFKIHHOPJDG\x18\x03\x20\x01(\rR\x0bG\
    FKIHHOPJDG\x12\x1d\n\nis_virtual\x18\n\x20\x01(\x08R\tisVirtual\x12\x19\
    \n\x08plane_id\x18\x04\x20\x01(\rR\x07planeIdB\x15\n\x13emu.lunarcore.pr\
    otob\x06proto3\
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
            deps.push(super::ExtraLineupType::file_descriptor().clone());
            deps.push(super::LineupSlotData::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(ReplaceLineupCsReq::generated_message_descriptor_data());
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
