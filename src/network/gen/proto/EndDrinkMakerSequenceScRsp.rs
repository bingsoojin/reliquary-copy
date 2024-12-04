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

//! Generated file from `EndDrinkMakerSequenceScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:EndDrinkMakerSequenceScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct EndDrinkMakerSequenceScRsp {
    // message fields
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.next_sequence_id)
    pub next_sequence_id: u32,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.guest)
    pub guest: ::protobuf::MessageField<super::DrinkMakerGuest::DrinkMakerGuest>,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.tips)
    pub tips: u32,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.level)
    pub level: u32,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.retcode)
    pub retcode: u32,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.request_list)
    pub request_list: ::std::vec::Vec<super::FGMHFCNKNBD::FGMHFCNKNBD>,
    // @@protoc_insertion_point(field:EndDrinkMakerSequenceScRsp.reward)
    pub reward: ::protobuf::MessageField<super::ItemList::ItemList>,
    // special fields
    // @@protoc_insertion_point(special_field:EndDrinkMakerSequenceScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a EndDrinkMakerSequenceScRsp {
    fn default() -> &'a EndDrinkMakerSequenceScRsp {
        <EndDrinkMakerSequenceScRsp as ::protobuf::Message>::default_instance()
    }
}

impl EndDrinkMakerSequenceScRsp {
    pub fn new() -> EndDrinkMakerSequenceScRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "next_sequence_id",
            |m: &EndDrinkMakerSequenceScRsp| { &m.next_sequence_id },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.next_sequence_id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::DrinkMakerGuest::DrinkMakerGuest>(
            "guest",
            |m: &EndDrinkMakerSequenceScRsp| { &m.guest },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.guest },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "tips",
            |m: &EndDrinkMakerSequenceScRsp| { &m.tips },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.tips },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &EndDrinkMakerSequenceScRsp| { &m.exp },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &EndDrinkMakerSequenceScRsp| { &m.level },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.level },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &EndDrinkMakerSequenceScRsp| { &m.retcode },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "request_list",
            |m: &EndDrinkMakerSequenceScRsp| { &m.request_list },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.request_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "reward",
            |m: &EndDrinkMakerSequenceScRsp| { &m.reward },
            |m: &mut EndDrinkMakerSequenceScRsp| { &mut m.reward },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<EndDrinkMakerSequenceScRsp>(
            "EndDrinkMakerSequenceScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for EndDrinkMakerSequenceScRsp {
    const NAME: &'static str = "EndDrinkMakerSequenceScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.next_sequence_id = is.read_uint32()?;
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.guest)?;
                },
                120 => {
                    self.tips = is.read_uint32()?;
                },
                112 => {
                    self.exp = is.read_uint32()?;
                },
                48 => {
                    self.level = is.read_uint32()?;
                },
                80 => {
                    self.retcode = is.read_uint32()?;
                },
                90 => {
                    self.request_list.push(is.read_message()?);
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.reward)?;
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
        if self.next_sequence_id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.next_sequence_id);
        }
        if let Some(v) = self.guest.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.tips != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.tips);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.exp);
        }
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.level);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.retcode);
        }
        for value in &self.request_list {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.reward.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.next_sequence_id != 0 {
            os.write_uint32(2, self.next_sequence_id)?;
        }
        if let Some(v) = self.guest.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
        }
        if self.tips != 0 {
            os.write_uint32(15, self.tips)?;
        }
        if self.exp != 0 {
            os.write_uint32(14, self.exp)?;
        }
        if self.level != 0 {
            os.write_uint32(6, self.level)?;
        }
        if self.retcode != 0 {
            os.write_uint32(10, self.retcode)?;
        }
        for v in &self.request_list {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
        };
        if let Some(v) = self.reward.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
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

    fn new() -> EndDrinkMakerSequenceScRsp {
        EndDrinkMakerSequenceScRsp::new()
    }

    fn clear(&mut self) {
        self.next_sequence_id = 0;
        self.guest.clear();
        self.tips = 0;
        self.exp = 0;
        self.level = 0;
        self.retcode = 0;
        self.request_list.clear();
        self.reward.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static EndDrinkMakerSequenceScRsp {
        static instance: EndDrinkMakerSequenceScRsp = EndDrinkMakerSequenceScRsp {
            next_sequence_id: 0,
            guest: ::protobuf::MessageField::none(),
            tips: 0,
            exp: 0,
            level: 0,
            retcode: 0,
            request_list: ::std::vec::Vec::new(),
            reward: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for EndDrinkMakerSequenceScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("EndDrinkMakerSequenceScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for EndDrinkMakerSequenceScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EndDrinkMakerSequenceScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20EndDrinkMakerSequenceScRsp.proto\x1a\x15DrinkMakerGuest.proto\x1a\
    \x11FGMHFCNKNBD.proto\x1a\x0eItemList.proto\"\x98\x02\n\x1aEndDrinkMaker\
    SequenceScRsp\x12(\n\x10next_sequence_id\x18\x02\x20\x01(\rR\x0enextSequ\
    enceId\x12&\n\x05guest\x18\x01\x20\x01(\x0b2\x10.DrinkMakerGuestR\x05gue\
    st\x12\x12\n\x04tips\x18\x0f\x20\x01(\rR\x04tips\x12\x10\n\x03exp\x18\
    \x0e\x20\x01(\rR\x03exp\x12\x14\n\x05level\x18\x06\x20\x01(\rR\x05level\
    \x12\x18\n\x07retcode\x18\n\x20\x01(\rR\x07retcode\x12/\n\x0crequest_lis\
    t\x18\x0b\x20\x03(\x0b2\x0c.FGMHFCNKNBDR\x0brequestList\x12!\n\x06reward\
    \x18\x0c\x20\x01(\x0b2\t.ItemListR\x06rewardb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(3);
            deps.push(super::DrinkMakerGuest::file_descriptor().clone());
            deps.push(super::FGMHFCNKNBD::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(EndDrinkMakerSequenceScRsp::generated_message_descriptor_data());
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
