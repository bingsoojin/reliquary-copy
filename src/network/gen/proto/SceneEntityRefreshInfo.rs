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

//! Generated file from `SceneEntityRefreshInfo.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:SceneEntityRefreshInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct SceneEntityRefreshInfo {
    // message oneof groups
    pub refresh_type: ::std::option::Option<scene_entity_refresh_info::Refresh_type>,
    // special fields
    // @@protoc_insertion_point(special_field:SceneEntityRefreshInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a SceneEntityRefreshInfo {
    fn default() -> &'a SceneEntityRefreshInfo {
        <SceneEntityRefreshInfo as ::protobuf::Message>::default_instance()
    }
}

impl SceneEntityRefreshInfo {
    pub fn new() -> SceneEntityRefreshInfo {
        ::std::default::Default::default()
    }

    // .SceneEntityInfo add_entity = 8;

    pub fn add_entity(&self) -> &super::SceneEntityInfo::SceneEntityInfo {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(ref v)) => v,
            _ => <super::SceneEntityInfo::SceneEntityInfo as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_add_entity(&mut self) {
        self.refresh_type = ::std::option::Option::None;
    }

    pub fn has_add_entity(&self) -> bool {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_add_entity(&mut self, v: super::SceneEntityInfo::SceneEntityInfo) {
        self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(v))
    }

    // Mutable pointer to the field.
    pub fn mut_add_entity(&mut self) -> &mut super::SceneEntityInfo::SceneEntityInfo {
        if let ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(_)) = self.refresh_type {
        } else {
            self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(super::SceneEntityInfo::SceneEntityInfo::new()));
        }
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_add_entity(&mut self) -> super::SceneEntityInfo::SceneEntityInfo {
        if self.has_add_entity() {
            match self.refresh_type.take() {
                ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(v)) => v,
                _ => panic!(),
            }
        } else {
            super::SceneEntityInfo::SceneEntityInfo::new()
        }
    }

    // uint32 del_entity = 11;

    pub fn del_entity(&self) -> u32 {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::DelEntity(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_del_entity(&mut self) {
        self.refresh_type = ::std::option::Option::None;
    }

    pub fn has_del_entity(&self) -> bool {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::DelEntity(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_del_entity(&mut self, v: u32) {
        self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::DelEntity(v))
    }

    // uint32 NNENDGJBGBE = 4;

    pub fn NNENDGJBGBE(&self) -> u32 {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(v)) => v,
            _ => 0,
        }
    }

    pub fn clear_NNENDGJBGBE(&mut self) {
        self.refresh_type = ::std::option::Option::None;
    }

    pub fn has_NNENDGJBGBE(&self) -> bool {
        match self.refresh_type {
            ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NNENDGJBGBE(&mut self, v: u32) {
        self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(v))
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(3);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::SceneEntityInfo::SceneEntityInfo>(
            "add_entity",
            SceneEntityRefreshInfo::has_add_entity,
            SceneEntityRefreshInfo::add_entity,
            SceneEntityRefreshInfo::mut_add_entity,
            SceneEntityRefreshInfo::set_add_entity,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "del_entity",
            SceneEntityRefreshInfo::has_del_entity,
            SceneEntityRefreshInfo::del_entity,
            SceneEntityRefreshInfo::set_del_entity,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_copy_has_get_set_simpler_accessors::<_, _>(
            "NNENDGJBGBE",
            SceneEntityRefreshInfo::has_NNENDGJBGBE,
            SceneEntityRefreshInfo::NNENDGJBGBE,
            SceneEntityRefreshInfo::set_NNENDGJBGBE,
        ));
        oneofs.push(scene_entity_refresh_info::Refresh_type::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<SceneEntityRefreshInfo>(
            "SceneEntityRefreshInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for SceneEntityRefreshInfo {
    const NAME: &'static str = "SceneEntityRefreshInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::AddEntity(is.read_message()?));
                },
                88 => {
                    self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::DelEntity(is.read_uint32()?));
                },
                32 => {
                    self.refresh_type = ::std::option::Option::Some(scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(is.read_uint32()?));
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
        if let ::std::option::Option::Some(ref v) = self.refresh_type {
            match v {
                &scene_entity_refresh_info::Refresh_type::AddEntity(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &scene_entity_refresh_info::Refresh_type::DelEntity(v) => {
                    my_size += ::protobuf::rt::uint32_size(11, v);
                },
                &scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(v) => {
                    my_size += ::protobuf::rt::uint32_size(4, v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let ::std::option::Option::Some(ref v) = self.refresh_type {
            match v {
                &scene_entity_refresh_info::Refresh_type::AddEntity(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &scene_entity_refresh_info::Refresh_type::DelEntity(v) => {
                    os.write_uint32(11, v)?;
                },
                &scene_entity_refresh_info::Refresh_type::NNENDGJBGBE(v) => {
                    os.write_uint32(4, v)?;
                },
            };
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

    fn new() -> SceneEntityRefreshInfo {
        SceneEntityRefreshInfo::new()
    }

    fn clear(&mut self) {
        self.refresh_type = ::std::option::Option::None;
        self.refresh_type = ::std::option::Option::None;
        self.refresh_type = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static SceneEntityRefreshInfo {
        static instance: SceneEntityRefreshInfo = SceneEntityRefreshInfo {
            refresh_type: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for SceneEntityRefreshInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("SceneEntityRefreshInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for SceneEntityRefreshInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SceneEntityRefreshInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `SceneEntityRefreshInfo`
pub mod scene_entity_refresh_info {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:SceneEntityRefreshInfo.refresh_type)
    pub enum Refresh_type {
        // @@protoc_insertion_point(oneof_field:SceneEntityRefreshInfo.add_entity)
        AddEntity(super::super::SceneEntityInfo::SceneEntityInfo),
        // @@protoc_insertion_point(oneof_field:SceneEntityRefreshInfo.del_entity)
        DelEntity(u32),
        // @@protoc_insertion_point(oneof_field:SceneEntityRefreshInfo.NNENDGJBGBE)
        NNENDGJBGBE(u32),
    }

    impl ::protobuf::Oneof for Refresh_type {
    }

    impl ::protobuf::OneofFull for Refresh_type {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::SceneEntityRefreshInfo as ::protobuf::MessageFull>::descriptor().oneof_by_name("refresh_type").unwrap()).clone()
        }
    }

    impl Refresh_type {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<Refresh_type>("refresh_type")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1cSceneEntityRefreshInfo.proto\x1a\x15SceneEntityInfo.proto\"\xa0\
    \x01\n\x16SceneEntityRefreshInfo\x121\n\nadd_entity\x18\x08\x20\x01(\x0b\
    2\x10.SceneEntityInfoH\0R\taddEntity\x12\x1f\n\ndel_entity\x18\x0b\x20\
    \x01(\rH\0R\tdelEntity\x12\"\n\x0bNNENDGJBGBE\x18\x04\x20\x01(\rH\0R\x0b\
    NNENDGJBGBEB\x0e\n\x0crefresh_typeB\x15\n\x13emu.lunarcore.protob\x06pro\
    to3\
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
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(SceneEntityRefreshInfo::generated_message_descriptor_data());
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
