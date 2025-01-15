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

//! Generated file from `CDHEBKKNMPI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:CDHEBKKNMPI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CDHEBKKNMPI {
    // message fields
    // @@protoc_insertion_point(field:CDHEBKKNMPI.KBDGEHKFGCA)
    pub KBDGEHKFGCA: ::protobuf::MessageField<super::Item::Item>,
    // @@protoc_insertion_point(field:CDHEBKKNMPI.EPEALHAOELC)
    pub EPEALHAOELC: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:CDHEBKKNMPI.NHAAGDBFBGO)
    pub NHAAGDBFBGO: ::protobuf::MessageField<super::ItemList::ItemList>,
    // @@protoc_insertion_point(field:CDHEBKKNMPI.OACOIHDALMG)
    pub OACOIHDALMG: bool,
    // special fields
    // @@protoc_insertion_point(special_field:CDHEBKKNMPI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CDHEBKKNMPI {
    fn default() -> &'a CDHEBKKNMPI {
        <CDHEBKKNMPI as ::protobuf::Message>::default_instance()
    }
}

impl CDHEBKKNMPI {
    pub fn new() -> CDHEBKKNMPI {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::Item::Item>(
            "KBDGEHKFGCA",
            |m: &CDHEBKKNMPI| { &m.KBDGEHKFGCA },
            |m: &mut CDHEBKKNMPI| { &mut m.KBDGEHKFGCA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "EPEALHAOELC",
            |m: &CDHEBKKNMPI| { &m.EPEALHAOELC },
            |m: &mut CDHEBKKNMPI| { &mut m.EPEALHAOELC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "NHAAGDBFBGO",
            |m: &CDHEBKKNMPI| { &m.NHAAGDBFBGO },
            |m: &mut CDHEBKKNMPI| { &mut m.NHAAGDBFBGO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OACOIHDALMG",
            |m: &CDHEBKKNMPI| { &m.OACOIHDALMG },
            |m: &mut CDHEBKKNMPI| { &mut m.OACOIHDALMG },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CDHEBKKNMPI>(
            "CDHEBKKNMPI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CDHEBKKNMPI {
    const NAME: &'static str = "CDHEBKKNMPI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                82 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.KBDGEHKFGCA)?;
                },
                26 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.EPEALHAOELC)?;
                },
                122 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.NHAAGDBFBGO)?;
                },
                64 => {
                    self.OACOIHDALMG = is.read_bool()?;
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
        if let Some(v) = self.KBDGEHKFGCA.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.EPEALHAOELC.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.NHAAGDBFBGO.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.OACOIHDALMG != false {
            my_size += 1 + 1;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.KBDGEHKFGCA.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
        }
        if let Some(v) = self.EPEALHAOELC.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        }
        if let Some(v) = self.NHAAGDBFBGO.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        }
        if self.OACOIHDALMG != false {
            os.write_bool(8, self.OACOIHDALMG)?;
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

    fn new() -> CDHEBKKNMPI {
        CDHEBKKNMPI::new()
    }

    fn clear(&mut self) {
        self.KBDGEHKFGCA.clear();
        self.EPEALHAOELC.clear();
        self.NHAAGDBFBGO.clear();
        self.OACOIHDALMG = false;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CDHEBKKNMPI {
        static instance: CDHEBKKNMPI = CDHEBKKNMPI {
            KBDGEHKFGCA: ::protobuf::MessageField::none(),
            EPEALHAOELC: ::protobuf::MessageField::none(),
            NHAAGDBFBGO: ::protobuf::MessageField::none(),
            OACOIHDALMG: false,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CDHEBKKNMPI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CDHEBKKNMPI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CDHEBKKNMPI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CDHEBKKNMPI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11CDHEBKKNMPI.proto\x1a\nItem.proto\x1a\x0eItemList.proto\"\xb2\x01\
    \n\x0bCDHEBKKNMPI\x12'\n\x0bKBDGEHKFGCA\x18\n\x20\x01(\x0b2\x05.ItemR\
    \x0bKBDGEHKFGCA\x12+\n\x0bEPEALHAOELC\x18\x03\x20\x01(\x0b2\t.ItemListR\
    \x0bEPEALHAOELC\x12+\n\x0bNHAAGDBFBGO\x18\x0f\x20\x01(\x0b2\t.ItemListR\
    \x0bNHAAGDBFBGO\x12\x20\n\x0bOACOIHDALMG\x18\x08\x20\x01(\x08R\x0bOACOIH\
    DALMGb\x06proto3\
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
            deps.push(super::Item::file_descriptor().clone());
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(CDHEBKKNMPI::generated_message_descriptor_data());
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
