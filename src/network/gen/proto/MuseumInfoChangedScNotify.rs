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

//! Generated file from `MuseumInfoChangedScNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:MuseumInfoChangedScNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct MuseumInfoChangedScNotify {
    // message fields
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.BDMNAPIFCIM)
    pub BDMNAPIFCIM: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.ECHIDIDECCG)
    pub ECHIDIDECCG: ::std::vec::Vec<super::JOOLADOGHGE::JOOLADOGHGE>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.FIHPGEEHMMB)
    pub FIHPGEEHMMB: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.EHMIPHNNFMK)
    pub EHMIPHNNFMK: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.FNMJJMBPCJF)
    pub FNMJJMBPCJF: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.exp)
    pub exp: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.HNHCLCBJAIM)
    pub HNHCLCBJAIM: ::std::vec::Vec<super::BJDAAPOAPAI::BJDAAPOAPAI>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.MOCHEECBDIJ)
    pub MOCHEECBDIJ: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.GCNMIIFKHGI)
    pub GCNMIIFKHGI: u32,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.OAPJALCOECF)
    pub OAPJALCOECF: ::protobuf::MessageField<super::NKMJCMMDHHJ::NKMJCMMDHHJ>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.DACHMECJPEI)
    pub DACHMECJPEI: ::protobuf::MessageField<super::IGPFNHKHMPK::IGPFNHKHMPK>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.FBCHKLNGJFM)
    pub FBCHKLNGJFM: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:MuseumInfoChangedScNotify.level)
    pub level: u32,
    // special fields
    // @@protoc_insertion_point(special_field:MuseumInfoChangedScNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MuseumInfoChangedScNotify {
    fn default() -> &'a MuseumInfoChangedScNotify {
        <MuseumInfoChangedScNotify as ::protobuf::Message>::default_instance()
    }
}

impl MuseumInfoChangedScNotify {
    pub fn new() -> MuseumInfoChangedScNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(13);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BDMNAPIFCIM",
            |m: &MuseumInfoChangedScNotify| { &m.BDMNAPIFCIM },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.BDMNAPIFCIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "ECHIDIDECCG",
            |m: &MuseumInfoChangedScNotify| { &m.ECHIDIDECCG },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.ECHIDIDECCG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FIHPGEEHMMB",
            |m: &MuseumInfoChangedScNotify| { &m.FIHPGEEHMMB },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.FIHPGEEHMMB },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "EHMIPHNNFMK",
            |m: &MuseumInfoChangedScNotify| { &m.EHMIPHNNFMK },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.EHMIPHNNFMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "FNMJJMBPCJF",
            |m: &MuseumInfoChangedScNotify| { &m.FNMJJMBPCJF },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.FNMJJMBPCJF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "exp",
            |m: &MuseumInfoChangedScNotify| { &m.exp },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.exp },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "HNHCLCBJAIM",
            |m: &MuseumInfoChangedScNotify| { &m.HNHCLCBJAIM },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.HNHCLCBJAIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "MOCHEECBDIJ",
            |m: &MuseumInfoChangedScNotify| { &m.MOCHEECBDIJ },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.MOCHEECBDIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "GCNMIIFKHGI",
            |m: &MuseumInfoChangedScNotify| { &m.GCNMIIFKHGI },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.GCNMIIFKHGI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::NKMJCMMDHHJ::NKMJCMMDHHJ>(
            "OAPJALCOECF",
            |m: &MuseumInfoChangedScNotify| { &m.OAPJALCOECF },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.OAPJALCOECF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::IGPFNHKHMPK::IGPFNHKHMPK>(
            "DACHMECJPEI",
            |m: &MuseumInfoChangedScNotify| { &m.DACHMECJPEI },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.DACHMECJPEI },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "FBCHKLNGJFM",
            |m: &MuseumInfoChangedScNotify| { &m.FBCHKLNGJFM },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.FBCHKLNGJFM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "level",
            |m: &MuseumInfoChangedScNotify| { &m.level },
            |m: &mut MuseumInfoChangedScNotify| { &mut m.level },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MuseumInfoChangedScNotify>(
            "MuseumInfoChangedScNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MuseumInfoChangedScNotify {
    const NAME: &'static str = "MuseumInfoChangedScNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                32 => {
                    self.BDMNAPIFCIM = is.read_uint32()?;
                },
                122 => {
                    self.ECHIDIDECCG.push(is.read_message()?);
                },
                64 => {
                    self.FIHPGEEHMMB = is.read_uint32()?;
                },
                18 => {
                    is.read_repeated_packed_uint32_into(&mut self.EHMIPHNNFMK)?;
                },
                16 => {
                    self.EHMIPHNNFMK.push(is.read_uint32()?);
                },
                40 => {
                    self.FNMJJMBPCJF = is.read_uint32()?;
                },
                88 => {
                    self.exp = is.read_uint32()?;
                },
                26 => {
                    self.HNHCLCBJAIM.push(is.read_message()?);
                },
                48 => {
                    self.MOCHEECBDIJ = is.read_uint32()?;
                },
                8 => {
                    self.GCNMIIFKHGI = is.read_uint32()?;
                },
                106 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OAPJALCOECF)?;
                },
                98 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.DACHMECJPEI)?;
                },
                82 => {
                    is.read_repeated_packed_uint32_into(&mut self.FBCHKLNGJFM)?;
                },
                80 => {
                    self.FBCHKLNGJFM.push(is.read_uint32()?);
                },
                56 => {
                    self.level = is.read_uint32()?;
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
        if self.BDMNAPIFCIM != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.BDMNAPIFCIM);
        }
        for value in &self.ECHIDIDECCG {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.FIHPGEEHMMB != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.FIHPGEEHMMB);
        }
        for value in &self.EHMIPHNNFMK {
            my_size += ::protobuf::rt::uint32_size(2, *value);
        };
        if self.FNMJJMBPCJF != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.FNMJJMBPCJF);
        }
        if self.exp != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.exp);
        }
        for value in &self.HNHCLCBJAIM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.MOCHEECBDIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.MOCHEECBDIJ);
        }
        if self.GCNMIIFKHGI != 0 {
            my_size += ::protobuf::rt::uint32_size(1, self.GCNMIIFKHGI);
        }
        if let Some(v) = self.OAPJALCOECF.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if let Some(v) = self.DACHMECJPEI.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        for value in &self.FBCHKLNGJFM {
            my_size += ::protobuf::rt::uint32_size(10, *value);
        };
        if self.level != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.level);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.BDMNAPIFCIM != 0 {
            os.write_uint32(4, self.BDMNAPIFCIM)?;
        }
        for v in &self.ECHIDIDECCG {
            ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
        };
        if self.FIHPGEEHMMB != 0 {
            os.write_uint32(8, self.FIHPGEEHMMB)?;
        }
        for v in &self.EHMIPHNNFMK {
            os.write_uint32(2, *v)?;
        };
        if self.FNMJJMBPCJF != 0 {
            os.write_uint32(5, self.FNMJJMBPCJF)?;
        }
        if self.exp != 0 {
            os.write_uint32(11, self.exp)?;
        }
        for v in &self.HNHCLCBJAIM {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.MOCHEECBDIJ != 0 {
            os.write_uint32(6, self.MOCHEECBDIJ)?;
        }
        if self.GCNMIIFKHGI != 0 {
            os.write_uint32(1, self.GCNMIIFKHGI)?;
        }
        if let Some(v) = self.OAPJALCOECF.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        }
        if let Some(v) = self.DACHMECJPEI.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
        }
        for v in &self.FBCHKLNGJFM {
            os.write_uint32(10, *v)?;
        };
        if self.level != 0 {
            os.write_uint32(7, self.level)?;
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

    fn new() -> MuseumInfoChangedScNotify {
        MuseumInfoChangedScNotify::new()
    }

    fn clear(&mut self) {
        self.BDMNAPIFCIM = 0;
        self.ECHIDIDECCG.clear();
        self.FIHPGEEHMMB = 0;
        self.EHMIPHNNFMK.clear();
        self.FNMJJMBPCJF = 0;
        self.exp = 0;
        self.HNHCLCBJAIM.clear();
        self.MOCHEECBDIJ = 0;
        self.GCNMIIFKHGI = 0;
        self.OAPJALCOECF.clear();
        self.DACHMECJPEI.clear();
        self.FBCHKLNGJFM.clear();
        self.level = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MuseumInfoChangedScNotify {
        static instance: MuseumInfoChangedScNotify = MuseumInfoChangedScNotify {
            BDMNAPIFCIM: 0,
            ECHIDIDECCG: ::std::vec::Vec::new(),
            FIHPGEEHMMB: 0,
            EHMIPHNNFMK: ::std::vec::Vec::new(),
            FNMJJMBPCJF: 0,
            exp: 0,
            HNHCLCBJAIM: ::std::vec::Vec::new(),
            MOCHEECBDIJ: 0,
            GCNMIIFKHGI: 0,
            OAPJALCOECF: ::protobuf::MessageField::none(),
            DACHMECJPEI: ::protobuf::MessageField::none(),
            FBCHKLNGJFM: ::std::vec::Vec::new(),
            level: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MuseumInfoChangedScNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MuseumInfoChangedScNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MuseumInfoChangedScNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MuseumInfoChangedScNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1fMuseumInfoChangedScNotify.proto\x1a\x11BJDAAPOAPAI.proto\x1a\x11IG\
    PFNHKHMPK.proto\x1a\x11JOOLADOGHGE.proto\x1a\x11NKMJCMMDHHJ.proto\"\xf1\
    \x03\n\x19MuseumInfoChangedScNotify\x12\x20\n\x0bBDMNAPIFCIM\x18\x04\x20\
    \x01(\rR\x0bBDMNAPIFCIM\x12.\n\x0bECHIDIDECCG\x18\x0f\x20\x03(\x0b2\x0c.\
    JOOLADOGHGER\x0bECHIDIDECCG\x12\x20\n\x0bFIHPGEEHMMB\x18\x08\x20\x01(\rR\
    \x0bFIHPGEEHMMB\x12\x20\n\x0bEHMIPHNNFMK\x18\x02\x20\x03(\rR\x0bEHMIPHNN\
    FMK\x12\x20\n\x0bFNMJJMBPCJF\x18\x05\x20\x01(\rR\x0bFNMJJMBPCJF\x12\x10\
    \n\x03exp\x18\x0b\x20\x01(\rR\x03exp\x12.\n\x0bHNHCLCBJAIM\x18\x03\x20\
    \x03(\x0b2\x0c.BJDAAPOAPAIR\x0bHNHCLCBJAIM\x12\x20\n\x0bMOCHEECBDIJ\x18\
    \x06\x20\x01(\rR\x0bMOCHEECBDIJ\x12\x20\n\x0bGCNMIIFKHGI\x18\x01\x20\x01\
    (\rR\x0bGCNMIIFKHGI\x12.\n\x0bOAPJALCOECF\x18\r\x20\x01(\x0b2\x0c.NKMJCM\
    MDHHJR\x0bOAPJALCOECF\x12.\n\x0bDACHMECJPEI\x18\x0c\x20\x01(\x0b2\x0c.IG\
    PFNHKHMPKR\x0bDACHMECJPEI\x12\x20\n\x0bFBCHKLNGJFM\x18\n\x20\x03(\rR\x0b\
    FBCHKLNGJFM\x12\x14\n\x05level\x18\x07\x20\x01(\rR\x05levelb\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(4);
            deps.push(super::BJDAAPOAPAI::file_descriptor().clone());
            deps.push(super::IGPFNHKHMPK::file_descriptor().clone());
            deps.push(super::JOOLADOGHGE::file_descriptor().clone());
            deps.push(super::NKMJCMMDHHJ::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(MuseumInfoChangedScNotify::generated_message_descriptor_data());
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
