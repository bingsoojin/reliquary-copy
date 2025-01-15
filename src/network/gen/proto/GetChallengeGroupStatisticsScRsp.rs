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

//! Generated file from `GetChallengeGroupStatisticsScRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:GetChallengeGroupStatisticsScRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct GetChallengeGroupStatisticsScRsp {
    // message fields
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.IOPPGEGDHGL)
    pub IOPPGEGDHGL: u32,
    // @@protoc_insertion_point(field:GetChallengeGroupStatisticsScRsp.retcode)
    pub retcode: u32,
    // message oneof groups
    pub BLJFHNCKLFK: ::std::option::Option<get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK>,
    // special fields
    // @@protoc_insertion_point(special_field:GetChallengeGroupStatisticsScRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a GetChallengeGroupStatisticsScRsp {
    fn default() -> &'a GetChallengeGroupStatisticsScRsp {
        <GetChallengeGroupStatisticsScRsp as ::protobuf::Message>::default_instance()
    }
}

impl GetChallengeGroupStatisticsScRsp {
    pub fn new() -> GetChallengeGroupStatisticsScRsp {
        ::std::default::Default::default()
    }

    // .JOBPPMFDAKK DEFFOFDENKF = 12;

    pub fn DEFFOFDENKF(&self) -> &super::JOBPPMFDAKK::JOBPPMFDAKK {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(ref v)) => v,
            _ => <super::JOBPPMFDAKK::JOBPPMFDAKK as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_DEFFOFDENKF(&mut self) {
        self.BLJFHNCKLFK = ::std::option::Option::None;
    }

    pub fn has_DEFFOFDENKF(&self) -> bool {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_DEFFOFDENKF(&mut self, v: super::JOBPPMFDAKK::JOBPPMFDAKK) {
        self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(v))
    }

    // Mutable pointer to the field.
    pub fn mut_DEFFOFDENKF(&mut self) -> &mut super::JOBPPMFDAKK::JOBPPMFDAKK {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(_)) = self.BLJFHNCKLFK {
        } else {
            self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(super::JOBPPMFDAKK::JOBPPMFDAKK::new()));
        }
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_DEFFOFDENKF(&mut self) -> super::JOBPPMFDAKK::JOBPPMFDAKK {
        if self.has_DEFFOFDENKF() {
            match self.BLJFHNCKLFK.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JOBPPMFDAKK::JOBPPMFDAKK::new()
        }
    }

    // .ABKAIEKHKMA FBBEDMHJAGJ = 1;

    pub fn FBBEDMHJAGJ(&self) -> &super::ABKAIEKHKMA::ABKAIEKHKMA {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(ref v)) => v,
            _ => <super::ABKAIEKHKMA::ABKAIEKHKMA as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FBBEDMHJAGJ(&mut self) {
        self.BLJFHNCKLFK = ::std::option::Option::None;
    }

    pub fn has_FBBEDMHJAGJ(&self) -> bool {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FBBEDMHJAGJ(&mut self, v: super::ABKAIEKHKMA::ABKAIEKHKMA) {
        self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FBBEDMHJAGJ(&mut self) -> &mut super::ABKAIEKHKMA::ABKAIEKHKMA {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(_)) = self.BLJFHNCKLFK {
        } else {
            self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(super::ABKAIEKHKMA::ABKAIEKHKMA::new()));
        }
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FBBEDMHJAGJ(&mut self) -> super::ABKAIEKHKMA::ABKAIEKHKMA {
        if self.has_FBBEDMHJAGJ() {
            match self.BLJFHNCKLFK.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ABKAIEKHKMA::ABKAIEKHKMA::new()
        }
    }

    // .ECBKOMLFLNE JDFNKFOIAJH = 7;

    pub fn JDFNKFOIAJH(&self) -> &super::ECBKOMLFLNE::ECBKOMLFLNE {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(ref v)) => v,
            _ => <super::ECBKOMLFLNE::ECBKOMLFLNE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_JDFNKFOIAJH(&mut self) {
        self.BLJFHNCKLFK = ::std::option::Option::None;
    }

    pub fn has_JDFNKFOIAJH(&self) -> bool {
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_JDFNKFOIAJH(&mut self, v: super::ECBKOMLFLNE::ECBKOMLFLNE) {
        self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_JDFNKFOIAJH(&mut self) -> &mut super::ECBKOMLFLNE::ECBKOMLFLNE {
        if let ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(_)) = self.BLJFHNCKLFK {
        } else {
            self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(super::ECBKOMLFLNE::ECBKOMLFLNE::new()));
        }
        match self.BLJFHNCKLFK {
            ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_JDFNKFOIAJH(&mut self) -> super::ECBKOMLFLNE::ECBKOMLFLNE {
        if self.has_JDFNKFOIAJH() {
            match self.BLJFHNCKLFK.take() {
                ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ECBKOMLFLNE::ECBKOMLFLNE::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "IOPPGEGDHGL",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.IOPPGEGDHGL },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.IOPPGEGDHGL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "retcode",
            |m: &GetChallengeGroupStatisticsScRsp| { &m.retcode },
            |m: &mut GetChallengeGroupStatisticsScRsp| { &mut m.retcode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JOBPPMFDAKK::JOBPPMFDAKK>(
            "DEFFOFDENKF",
            GetChallengeGroupStatisticsScRsp::has_DEFFOFDENKF,
            GetChallengeGroupStatisticsScRsp::DEFFOFDENKF,
            GetChallengeGroupStatisticsScRsp::mut_DEFFOFDENKF,
            GetChallengeGroupStatisticsScRsp::set_DEFFOFDENKF,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ABKAIEKHKMA::ABKAIEKHKMA>(
            "FBBEDMHJAGJ",
            GetChallengeGroupStatisticsScRsp::has_FBBEDMHJAGJ,
            GetChallengeGroupStatisticsScRsp::FBBEDMHJAGJ,
            GetChallengeGroupStatisticsScRsp::mut_FBBEDMHJAGJ,
            GetChallengeGroupStatisticsScRsp::set_FBBEDMHJAGJ,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ECBKOMLFLNE::ECBKOMLFLNE>(
            "JDFNKFOIAJH",
            GetChallengeGroupStatisticsScRsp::has_JDFNKFOIAJH,
            GetChallengeGroupStatisticsScRsp::JDFNKFOIAJH,
            GetChallengeGroupStatisticsScRsp::mut_JDFNKFOIAJH,
            GetChallengeGroupStatisticsScRsp::set_JDFNKFOIAJH,
        ));
        oneofs.push(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<GetChallengeGroupStatisticsScRsp>(
            "GetChallengeGroupStatisticsScRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for GetChallengeGroupStatisticsScRsp {
    const NAME: &'static str = "GetChallengeGroupStatisticsScRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                80 => {
                    self.IOPPGEGDHGL = is.read_uint32()?;
                },
                64 => {
                    self.retcode = is.read_uint32()?;
                },
                98 => {
                    self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(is.read_message()?));
                },
                10 => {
                    self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(is.read_message()?));
                },
                58 => {
                    self.BLJFHNCKLFK = ::std::option::Option::Some(get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(is.read_message()?));
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
        if self.IOPPGEGDHGL != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.IOPPGEGDHGL);
        }
        if self.retcode != 0 {
            my_size += ::protobuf::rt::uint32_size(8, self.retcode);
        }
        if let ::std::option::Option::Some(ref v) = self.BLJFHNCKLFK {
            match v {
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.IOPPGEGDHGL != 0 {
            os.write_uint32(10, self.IOPPGEGDHGL)?;
        }
        if self.retcode != 0 {
            os.write_uint32(8, self.retcode)?;
        }
        if let ::std::option::Option::Some(ref v) = self.BLJFHNCKLFK {
            match v {
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::DEFFOFDENKF(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::FBBEDMHJAGJ(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &get_challenge_group_statistics_sc_rsp::BLJFHNCKLFK::JDFNKFOIAJH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(7, v, os)?;
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

    fn new() -> GetChallengeGroupStatisticsScRsp {
        GetChallengeGroupStatisticsScRsp::new()
    }

    fn clear(&mut self) {
        self.IOPPGEGDHGL = 0;
        self.retcode = 0;
        self.BLJFHNCKLFK = ::std::option::Option::None;
        self.BLJFHNCKLFK = ::std::option::Option::None;
        self.BLJFHNCKLFK = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static GetChallengeGroupStatisticsScRsp {
        static instance: GetChallengeGroupStatisticsScRsp = GetChallengeGroupStatisticsScRsp {
            IOPPGEGDHGL: 0,
            retcode: 0,
            BLJFHNCKLFK: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for GetChallengeGroupStatisticsScRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("GetChallengeGroupStatisticsScRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for GetChallengeGroupStatisticsScRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetChallengeGroupStatisticsScRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `GetChallengeGroupStatisticsScRsp`
pub mod get_challenge_group_statistics_sc_rsp {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:GetChallengeGroupStatisticsScRsp.BLJFHNCKLFK)
    pub enum BLJFHNCKLFK {
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.DEFFOFDENKF)
        DEFFOFDENKF(super::super::JOBPPMFDAKK::JOBPPMFDAKK),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.FBBEDMHJAGJ)
        FBBEDMHJAGJ(super::super::ABKAIEKHKMA::ABKAIEKHKMA),
        // @@protoc_insertion_point(oneof_field:GetChallengeGroupStatisticsScRsp.JDFNKFOIAJH)
        JDFNKFOIAJH(super::super::ECBKOMLFLNE::ECBKOMLFLNE),
    }

    impl ::protobuf::Oneof for BLJFHNCKLFK {
    }

    impl ::protobuf::OneofFull for BLJFHNCKLFK {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::GetChallengeGroupStatisticsScRsp as ::protobuf::MessageFull>::descriptor().oneof_by_name("BLJFHNCKLFK").unwrap()).clone()
        }
    }

    impl BLJFHNCKLFK {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<BLJFHNCKLFK>("BLJFHNCKLFK")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n&GetChallengeGroupStatisticsScRsp.proto\x1a\x11ABKAIEKHKMA.proto\x1a\
    \x11ECBKOMLFLNE.proto\x1a\x11JOBPPMFDAKK.proto\"\x83\x02\n\x20GetChallen\
    geGroupStatisticsScRsp\x12\x20\n\x0bIOPPGEGDHGL\x18\n\x20\x01(\rR\x0bIOP\
    PGEGDHGL\x12\x18\n\x07retcode\x18\x08\x20\x01(\rR\x07retcode\x120\n\x0bD\
    EFFOFDENKF\x18\x0c\x20\x01(\x0b2\x0c.JOBPPMFDAKKH\0R\x0bDEFFOFDENKF\x120\
    \n\x0bFBBEDMHJAGJ\x18\x01\x20\x01(\x0b2\x0c.ABKAIEKHKMAH\0R\x0bFBBEDMHJA\
    GJ\x120\n\x0bJDFNKFOIAJH\x18\x07\x20\x01(\x0b2\x0c.ECBKOMLFLNEH\0R\x0bJD\
    FNKFOIAJHB\r\n\x0bBLJFHNCKLFKb\x06proto3\
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
            deps.push(super::ABKAIEKHKMA::file_descriptor().clone());
            deps.push(super::ECBKOMLFLNE::file_descriptor().clone());
            deps.push(super::JOBPPMFDAKK::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(GetChallengeGroupStatisticsScRsp::generated_message_descriptor_data());
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
