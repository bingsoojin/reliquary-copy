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

//! Generated file from `BODJDMCEIHO.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BODJDMCEIHO)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BODJDMCEIHO {
    // message fields
    // @@protoc_insertion_point(field:BODJDMCEIHO.PNOAKGNANBO)
    pub PNOAKGNANBO: u32,
    // @@protoc_insertion_point(field:BODJDMCEIHO.FIKOICLOKFK)
    pub FIKOICLOKFK: bool,
    // @@protoc_insertion_point(field:BODJDMCEIHO.MDGLHFIFFLC)
    pub MDGLHFIFFLC: bool,
    // @@protoc_insertion_point(field:BODJDMCEIHO.JOIPFMCOINI)
    pub JOIPFMCOINI: u32,
    // @@protoc_insertion_point(field:BODJDMCEIHO.LCEEDIGELGM)
    pub LCEEDIGELGM: u32,
    // @@protoc_insertion_point(field:BODJDMCEIHO.FINLPBFNLHP)
    pub FINLPBFNLHP: u32,
    // @@protoc_insertion_point(field:BODJDMCEIHO.ELPMNKHEPKJ)
    pub ELPMNKHEPKJ: ::protobuf::MessageField<super::ItemList::ItemList>,
    // special fields
    // @@protoc_insertion_point(special_field:BODJDMCEIHO.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BODJDMCEIHO {
    fn default() -> &'a BODJDMCEIHO {
        <BODJDMCEIHO as ::protobuf::Message>::default_instance()
    }
}

impl BODJDMCEIHO {
    pub fn new() -> BODJDMCEIHO {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(7);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PNOAKGNANBO",
            |m: &BODJDMCEIHO| { &m.PNOAKGNANBO },
            |m: &mut BODJDMCEIHO| { &mut m.PNOAKGNANBO },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FIKOICLOKFK",
            |m: &BODJDMCEIHO| { &m.FIKOICLOKFK },
            |m: &mut BODJDMCEIHO| { &mut m.FIKOICLOKFK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MDGLHFIFFLC",
            |m: &BODJDMCEIHO| { &m.MDGLHFIFFLC },
            |m: &mut BODJDMCEIHO| { &mut m.MDGLHFIFFLC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "JOIPFMCOINI",
            |m: &BODJDMCEIHO| { &m.JOIPFMCOINI },
            |m: &mut BODJDMCEIHO| { &mut m.JOIPFMCOINI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LCEEDIGELGM",
            |m: &BODJDMCEIHO| { &m.LCEEDIGELGM },
            |m: &mut BODJDMCEIHO| { &mut m.LCEEDIGELGM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FINLPBFNLHP",
            |m: &BODJDMCEIHO| { &m.FINLPBFNLHP },
            |m: &mut BODJDMCEIHO| { &mut m.FINLPBFNLHP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::ItemList::ItemList>(
            "ELPMNKHEPKJ",
            |m: &BODJDMCEIHO| { &m.ELPMNKHEPKJ },
            |m: &mut BODJDMCEIHO| { &mut m.ELPMNKHEPKJ },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BODJDMCEIHO>(
            "BODJDMCEIHO",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BODJDMCEIHO {
    const NAME: &'static str = "BODJDMCEIHO";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.PNOAKGNANBO = is.read_uint32()?;
                },
                24 => {
                    self.FIKOICLOKFK = is.read_bool()?;
                },
                48 => {
                    self.MDGLHFIFFLC = is.read_bool()?;
                },
                32 => {
                    self.JOIPFMCOINI = is.read_uint32()?;
                },
                56 => {
                    self.LCEEDIGELGM = is.read_uint32()?;
                },
                96 => {
                    self.FINLPBFNLHP = is.read_uint32()?;
                },
                90 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.ELPMNKHEPKJ)?;
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
        if self.PNOAKGNANBO != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.PNOAKGNANBO);
        }
        if self.FIKOICLOKFK != false {
            my_size += 1 + 1;
        }
        if self.MDGLHFIFFLC != false {
            my_size += 1 + 1;
        }
        if self.JOIPFMCOINI != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.JOIPFMCOINI);
        }
        if self.LCEEDIGELGM != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.LCEEDIGELGM);
        }
        if self.FINLPBFNLHP != 0 {
            my_size += ::protobuf::rt::uint32_size(12, self.FINLPBFNLHP);
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.PNOAKGNANBO != 0 {
            os.write_uint32(10, self.PNOAKGNANBO)?;
        }
        if self.FIKOICLOKFK != false {
            os.write_bool(3, self.FIKOICLOKFK)?;
        }
        if self.MDGLHFIFFLC != false {
            os.write_bool(6, self.MDGLHFIFFLC)?;
        }
        if self.JOIPFMCOINI != 0 {
            os.write_uint32(4, self.JOIPFMCOINI)?;
        }
        if self.LCEEDIGELGM != 0 {
            os.write_uint32(7, self.LCEEDIGELGM)?;
        }
        if self.FINLPBFNLHP != 0 {
            os.write_uint32(12, self.FINLPBFNLHP)?;
        }
        if let Some(v) = self.ELPMNKHEPKJ.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
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

    fn new() -> BODJDMCEIHO {
        BODJDMCEIHO::new()
    }

    fn clear(&mut self) {
        self.PNOAKGNANBO = 0;
        self.FIKOICLOKFK = false;
        self.MDGLHFIFFLC = false;
        self.JOIPFMCOINI = 0;
        self.LCEEDIGELGM = 0;
        self.FINLPBFNLHP = 0;
        self.ELPMNKHEPKJ.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BODJDMCEIHO {
        static instance: BODJDMCEIHO = BODJDMCEIHO {
            PNOAKGNANBO: 0,
            FIKOICLOKFK: false,
            MDGLHFIFFLC: false,
            JOIPFMCOINI: 0,
            LCEEDIGELGM: 0,
            FINLPBFNLHP: 0,
            ELPMNKHEPKJ: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BODJDMCEIHO {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BODJDMCEIHO").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BODJDMCEIHO {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BODJDMCEIHO {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BODJDMCEIHO.proto\x1a\x0eItemList.proto\"\x86\x02\n\x0bBODJDMCEIHO\
    \x12\x20\n\x0bPNOAKGNANBO\x18\n\x20\x01(\rR\x0bPNOAKGNANBO\x12\x20\n\x0b\
    FIKOICLOKFK\x18\x03\x20\x01(\x08R\x0bFIKOICLOKFK\x12\x20\n\x0bMDGLHFIFFL\
    C\x18\x06\x20\x01(\x08R\x0bMDGLHFIFFLC\x12\x20\n\x0bJOIPFMCOINI\x18\x04\
    \x20\x01(\rR\x0bJOIPFMCOINI\x12\x20\n\x0bLCEEDIGELGM\x18\x07\x20\x01(\rR\
    \x0bLCEEDIGELGM\x12\x20\n\x0bFINLPBFNLHP\x18\x0c\x20\x01(\rR\x0bFINLPBFN\
    LHP\x12+\n\x0bELPMNKHEPKJ\x18\x0b\x20\x01(\x0b2\t.ItemListR\x0bELPMNKHEP\
    KJb\x06proto3\
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
            deps.push(super::ItemList::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BODJDMCEIHO::generated_message_descriptor_data());
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
