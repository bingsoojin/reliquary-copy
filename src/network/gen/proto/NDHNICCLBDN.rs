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

//! Generated file from `NDHNICCLBDN.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:NDHNICCLBDN)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct NDHNICCLBDN {
    // message fields
    // @@protoc_insertion_point(field:NDHNICCLBDN.EPJFPDPHMIJ)
    pub EPJFPDPHMIJ: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.JPFPJHOPBHH)
    pub JPFPJHOPBHH: ::std::vec::Vec<super::PDJDKKJCPOF::PDJDKKJCPOF>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.CCCEDBIGCDG)
    pub CCCEDBIGCDG: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.PHBPBGIJEKG)
    pub PHBPBGIJEKG: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.DJBLNJDMLBL)
    pub DJBLNJDMLBL: ::std::vec::Vec<super::MDLAAPIEBLF::MDLAAPIEBLF>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.KEDLBOOKGKK)
    pub KEDLBOOKGKK: ::std::vec::Vec<super::SceneEntityInfo::SceneEntityInfo>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.LLJPNNFLKON)
    pub LLJPNNFLKON: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.BHONGJMFKIM)
    pub BHONGJMFKIM: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.LBLNFDDBNCM)
    pub LBLNFDDBNCM: ::std::vec::Vec<super::ANGOENJIKBF::ANGOENJIKBF>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.MGAPDCENGLP)
    pub MGAPDCENGLP: ::std::vec::Vec<super::DEIFMNDOBEB::DEIFMNDOBEB>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.OKJPNBHNNMK)
    pub OKJPNBHNNMK: ::protobuf::MessageField<super::LMOJNCJPACC::LMOJNCJPACC>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.AKCMBCMDMBC)
    pub AKCMBCMDMBC: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.OLGCJIKGFFC)
    pub OLGCJIKGFFC: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.MOFJMDEEADD)
    pub MOFJMDEEADD: ::std::collections::HashMap<::std::string::String, i32>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.NHKHGDCBHJD)
    pub NHKHGDCBHJD: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.FHLAMIHOOJA)
    pub FHLAMIHOOJA: ::std::collections::HashMap<u32, super::DLIBJAFBHEA::DLIBJAFBHEA>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.AACPJNOFIEF)
    pub AACPJNOFIEF: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.LLLHPFLFKPP)
    pub LLLHPFLFKPP: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.BJNKEEFKAIK)
    pub BJNKEEFKAIK: u32,
    // @@protoc_insertion_point(field:NDHNICCLBDN.LECGLNPFACA)
    pub LECGLNPFACA: ::std::vec::Vec<super::ACGLLJPJCHK::ACGLLJPJCHK>,
    // @@protoc_insertion_point(field:NDHNICCLBDN.MGLHEBHJABE)
    pub MGLHEBHJABE: ::std::vec::Vec<u32>,
    // special fields
    // @@protoc_insertion_point(special_field:NDHNICCLBDN.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a NDHNICCLBDN {
    fn default() -> &'a NDHNICCLBDN {
        <NDHNICCLBDN as ::protobuf::Message>::default_instance()
    }
}

impl NDHNICCLBDN {
    pub fn new() -> NDHNICCLBDN {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(21);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "EPJFPDPHMIJ",
            |m: &NDHNICCLBDN| { &m.EPJFPDPHMIJ },
            |m: &mut NDHNICCLBDN| { &mut m.EPJFPDPHMIJ },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "JPFPJHOPBHH",
            |m: &NDHNICCLBDN| { &m.JPFPJHOPBHH },
            |m: &mut NDHNICCLBDN| { &mut m.JPFPJHOPBHH },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "CCCEDBIGCDG",
            |m: &NDHNICCLBDN| { &m.CCCEDBIGCDG },
            |m: &mut NDHNICCLBDN| { &mut m.CCCEDBIGCDG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "PHBPBGIJEKG",
            |m: &NDHNICCLBDN| { &m.PHBPBGIJEKG },
            |m: &mut NDHNICCLBDN| { &mut m.PHBPBGIJEKG },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "DJBLNJDMLBL",
            |m: &NDHNICCLBDN| { &m.DJBLNJDMLBL },
            |m: &mut NDHNICCLBDN| { &mut m.DJBLNJDMLBL },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "KEDLBOOKGKK",
            |m: &NDHNICCLBDN| { &m.KEDLBOOKGKK },
            |m: &mut NDHNICCLBDN| { &mut m.KEDLBOOKGKK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLJPNNFLKON",
            |m: &NDHNICCLBDN| { &m.LLJPNNFLKON },
            |m: &mut NDHNICCLBDN| { &mut m.LLJPNNFLKON },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BHONGJMFKIM",
            |m: &NDHNICCLBDN| { &m.BHONGJMFKIM },
            |m: &mut NDHNICCLBDN| { &mut m.BHONGJMFKIM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LBLNFDDBNCM",
            |m: &NDHNICCLBDN| { &m.LBLNFDDBNCM },
            |m: &mut NDHNICCLBDN| { &mut m.LBLNFDDBNCM },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MGAPDCENGLP",
            |m: &NDHNICCLBDN| { &m.MGAPDCENGLP },
            |m: &mut NDHNICCLBDN| { &mut m.MGAPDCENGLP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, super::LMOJNCJPACC::LMOJNCJPACC>(
            "OKJPNBHNNMK",
            |m: &NDHNICCLBDN| { &m.OKJPNBHNNMK },
            |m: &mut NDHNICCLBDN| { &mut m.OKJPNBHNNMK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "AKCMBCMDMBC",
            |m: &NDHNICCLBDN| { &m.AKCMBCMDMBC },
            |m: &mut NDHNICCLBDN| { &mut m.AKCMBCMDMBC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "OLGCJIKGFFC",
            |m: &NDHNICCLBDN| { &m.OLGCJIKGFFC },
            |m: &mut NDHNICCLBDN| { &mut m.OLGCJIKGFFC },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "MOFJMDEEADD",
            |m: &NDHNICCLBDN| { &m.MOFJMDEEADD },
            |m: &mut NDHNICCLBDN| { &mut m.MOFJMDEEADD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "NHKHGDCBHJD",
            |m: &NDHNICCLBDN| { &m.NHKHGDCBHJD },
            |m: &mut NDHNICCLBDN| { &mut m.NHKHGDCBHJD },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_map_simpler_accessor::<_, _, _>(
            "FHLAMIHOOJA",
            |m: &NDHNICCLBDN| { &m.FHLAMIHOOJA },
            |m: &mut NDHNICCLBDN| { &mut m.FHLAMIHOOJA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "AACPJNOFIEF",
            |m: &NDHNICCLBDN| { &m.AACPJNOFIEF },
            |m: &mut NDHNICCLBDN| { &mut m.AACPJNOFIEF },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "LLLHPFLFKPP",
            |m: &NDHNICCLBDN| { &m.LLLHPFLFKPP },
            |m: &mut NDHNICCLBDN| { &mut m.LLLHPFLFKPP },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "BJNKEEFKAIK",
            |m: &NDHNICCLBDN| { &m.BJNKEEFKAIK },
            |m: &mut NDHNICCLBDN| { &mut m.BJNKEEFKAIK },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "LECGLNPFACA",
            |m: &NDHNICCLBDN| { &m.LECGLNPFACA },
            |m: &mut NDHNICCLBDN| { &mut m.LECGLNPFACA },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "MGLHEBHJABE",
            |m: &NDHNICCLBDN| { &m.MGLHEBHJABE },
            |m: &mut NDHNICCLBDN| { &mut m.MGLHEBHJABE },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<NDHNICCLBDN>(
            "NDHNICCLBDN",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for NDHNICCLBDN {
    const NAME: &'static str = "NDHNICCLBDN";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.EPJFPDPHMIJ = is.read_uint32()?;
                },
                10410 => {
                    self.JPFPJHOPBHH.push(is.read_message()?);
                },
                216 => {
                    self.CCCEDBIGCDG = is.read_uint32()?;
                },
                112 => {
                    self.PHBPBGIJEKG = is.read_uint32()?;
                },
                66 => {
                    self.DJBLNJDMLBL.push(is.read_message()?);
                },
                34 => {
                    self.KEDLBOOKGKK.push(is.read_message()?);
                },
                88 => {
                    self.LLJPNNFLKON = is.read_uint32()?;
                },
                80 => {
                    self.BHONGJMFKIM = is.read_uint32()?;
                },
                106 => {
                    self.LBLNFDDBNCM.push(is.read_message()?);
                },
                18 => {
                    self.MGAPDCENGLP.push(is.read_message()?);
                },
                8314 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.OKJPNBHNNMK)?;
                },
                10784 => {
                    self.AKCMBCMDMBC = is.read_uint32()?;
                },
                9040 => {
                    self.OLGCJIKGFFC = is.read_uint32()?;
                },
                10506 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            10 => key = is.read_string()?,
                            16 => value = is.read_int32()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.MOFJMDEEADD.insert(key, value);
                },
                72 => {
                    self.NHKHGDCBHJD = is.read_uint32()?;
                },
                122 => {
                    let len = is.read_raw_varint32()?;
                    let old_limit = is.push_limit(len as u64)?;
                    let mut key = ::std::default::Default::default();
                    let mut value = ::std::default::Default::default();
                    while let Some(tag) = is.read_raw_tag_or_eof()? {
                        match tag {
                            8 => key = is.read_uint32()?,
                            18 => value = is.read_message()?,
                            _ => ::protobuf::rt::skip_field_for_tag(tag, is)?,
                        };
                    }
                    is.pop_limit(old_limit);
                    self.FHLAMIHOOJA.insert(key, value);
                },
                10 => {
                    is.read_repeated_packed_uint32_into(&mut self.AACPJNOFIEF)?;
                },
                8 => {
                    self.AACPJNOFIEF.push(is.read_uint32()?);
                },
                48 => {
                    self.LLLHPFLFKPP = is.read_uint32()?;
                },
                56 => {
                    self.BJNKEEFKAIK = is.read_uint32()?;
                },
                12098 => {
                    self.LECGLNPFACA.push(is.read_message()?);
                },
                8514 => {
                    is.read_repeated_packed_uint32_into(&mut self.MGLHEBHJABE)?;
                },
                8512 => {
                    self.MGLHEBHJABE.push(is.read_uint32()?);
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
        if self.EPJFPDPHMIJ != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.EPJFPDPHMIJ);
        }
        for value in &self.JPFPJHOPBHH {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.CCCEDBIGCDG != 0 {
            my_size += ::protobuf::rt::uint32_size(27, self.CCCEDBIGCDG);
        }
        if self.PHBPBGIJEKG != 0 {
            my_size += ::protobuf::rt::uint32_size(14, self.PHBPBGIJEKG);
        }
        for value in &self.DJBLNJDMLBL {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.KEDLBOOKGKK {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.LLJPNNFLKON != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.LLJPNNFLKON);
        }
        if self.BHONGJMFKIM != 0 {
            my_size += ::protobuf::rt::uint32_size(10, self.BHONGJMFKIM);
        }
        for value in &self.LBLNFDDBNCM {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MGAPDCENGLP {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if let Some(v) = self.OKJPNBHNNMK.as_ref() {
            let len = v.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        if self.AKCMBCMDMBC != 0 {
            my_size += ::protobuf::rt::uint32_size(1348, self.AKCMBCMDMBC);
        }
        if self.OLGCJIKGFFC != 0 {
            my_size += ::protobuf::rt::uint32_size(1130, self.OLGCJIKGFFC);
        }
        for (k, v) in &self.MOFJMDEEADD {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        if self.NHKHGDCBHJD != 0 {
            my_size += ::protobuf::rt::uint32_size(9, self.NHKHGDCBHJD);
        }
        for (k, v) in &self.FHLAMIHOOJA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.compute_size();
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(entry_size) + entry_size
        };
        for value in &self.AACPJNOFIEF {
            my_size += ::protobuf::rt::uint32_size(1, *value);
        };
        if self.LLLHPFLFKPP != 0 {
            my_size += ::protobuf::rt::uint32_size(6, self.LLLHPFLFKPP);
        }
        if self.BJNKEEFKAIK != 0 {
            my_size += ::protobuf::rt::uint32_size(7, self.BJNKEEFKAIK);
        }
        for value in &self.LECGLNPFACA {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        for value in &self.MGLHEBHJABE {
            my_size += ::protobuf::rt::uint32_size(1064, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.EPJFPDPHMIJ != 0 {
            os.write_uint32(5, self.EPJFPDPHMIJ)?;
        }
        for v in &self.JPFPJHOPBHH {
            ::protobuf::rt::write_message_field_with_cached_size(1301, v, os)?;
        };
        if self.CCCEDBIGCDG != 0 {
            os.write_uint32(27, self.CCCEDBIGCDG)?;
        }
        if self.PHBPBGIJEKG != 0 {
            os.write_uint32(14, self.PHBPBGIJEKG)?;
        }
        for v in &self.DJBLNJDMLBL {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        for v in &self.KEDLBOOKGKK {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        if self.LLJPNNFLKON != 0 {
            os.write_uint32(11, self.LLJPNNFLKON)?;
        }
        if self.BHONGJMFKIM != 0 {
            os.write_uint32(10, self.BHONGJMFKIM)?;
        }
        for v in &self.LBLNFDDBNCM {
            ::protobuf::rt::write_message_field_with_cached_size(13, v, os)?;
        };
        for v in &self.MGAPDCENGLP {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        if let Some(v) = self.OKJPNBHNNMK.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(1039, v, os)?;
        }
        if self.AKCMBCMDMBC != 0 {
            os.write_uint32(1348, self.AKCMBCMDMBC)?;
        }
        if self.OLGCJIKGFFC != 0 {
            os.write_uint32(1130, self.OLGCJIKGFFC)?;
        }
        for (k, v) in &self.MOFJMDEEADD {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::string_size(1, &k);
            entry_size += ::protobuf::rt::int32_size(2, *v);
            os.write_raw_varint32(10506)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_string(1, &k)?;
            os.write_int32(2, *v)?;
        };
        if self.NHKHGDCBHJD != 0 {
            os.write_uint32(9, self.NHKHGDCBHJD)?;
        }
        for (k, v) in &self.FHLAMIHOOJA {
            let mut entry_size = 0;
            entry_size += ::protobuf::rt::uint32_size(1, *k);
            let len = v.cached_size() as u64;
            entry_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
            os.write_raw_varint32(122)?; // Tag.
            os.write_raw_varint32(entry_size as u32)?;
            os.write_uint32(1, *k)?;
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        };
        for v in &self.AACPJNOFIEF {
            os.write_uint32(1, *v)?;
        };
        if self.LLLHPFLFKPP != 0 {
            os.write_uint32(6, self.LLLHPFLFKPP)?;
        }
        if self.BJNKEEFKAIK != 0 {
            os.write_uint32(7, self.BJNKEEFKAIK)?;
        }
        for v in &self.LECGLNPFACA {
            ::protobuf::rt::write_message_field_with_cached_size(1512, v, os)?;
        };
        for v in &self.MGLHEBHJABE {
            os.write_uint32(1064, *v)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> NDHNICCLBDN {
        NDHNICCLBDN::new()
    }

    fn clear(&mut self) {
        self.EPJFPDPHMIJ = 0;
        self.JPFPJHOPBHH.clear();
        self.CCCEDBIGCDG = 0;
        self.PHBPBGIJEKG = 0;
        self.DJBLNJDMLBL.clear();
        self.KEDLBOOKGKK.clear();
        self.LLJPNNFLKON = 0;
        self.BHONGJMFKIM = 0;
        self.LBLNFDDBNCM.clear();
        self.MGAPDCENGLP.clear();
        self.OKJPNBHNNMK.clear();
        self.AKCMBCMDMBC = 0;
        self.OLGCJIKGFFC = 0;
        self.MOFJMDEEADD.clear();
        self.NHKHGDCBHJD = 0;
        self.FHLAMIHOOJA.clear();
        self.AACPJNOFIEF.clear();
        self.LLLHPFLFKPP = 0;
        self.BJNKEEFKAIK = 0;
        self.LECGLNPFACA.clear();
        self.MGLHEBHJABE.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static NDHNICCLBDN {
        static instance: ::protobuf::rt::Lazy<NDHNICCLBDN> = ::protobuf::rt::Lazy::new();
        instance.get(NDHNICCLBDN::new)
    }
}

impl ::protobuf::MessageFull for NDHNICCLBDN {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("NDHNICCLBDN").unwrap()).clone()
    }
}

impl ::std::fmt::Display for NDHNICCLBDN {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NDHNICCLBDN {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11NDHNICCLBDN.proto\x1a\x11ACGLLJPJCHK.proto\x1a\x11ANGOENJIKBF.prot\
    o\x1a\x11DEIFMNDOBEB.proto\x1a\x11DLIBJAFBHEA.proto\x1a\x11LMOJNCJPACC.p\
    roto\x1a\x11MDLAAPIEBLF.proto\x1a\x11PDJDKKJCPOF.proto\x1a\x15SceneEntit\
    yInfo.proto\"\x90\x08\n\x0bNDHNICCLBDN\x12\x20\n\x0bEPJFPDPHMIJ\x18\x05\
    \x20\x01(\rR\x0bEPJFPDPHMIJ\x12/\n\x0bJPFPJHOPBHH\x18\x95\n\x20\x03(\x0b\
    2\x0c.PDJDKKJCPOFR\x0bJPFPJHOPBHH\x12\x20\n\x0bCCCEDBIGCDG\x18\x1b\x20\
    \x01(\rR\x0bCCCEDBIGCDG\x12\x20\n\x0bPHBPBGIJEKG\x18\x0e\x20\x01(\rR\x0b\
    PHBPBGIJEKG\x12.\n\x0bDJBLNJDMLBL\x18\x08\x20\x03(\x0b2\x0c.MDLAAPIEBLFR\
    \x0bDJBLNJDMLBL\x122\n\x0bKEDLBOOKGKK\x18\x04\x20\x03(\x0b2\x10.SceneEnt\
    ityInfoR\x0bKEDLBOOKGKK\x12\x20\n\x0bLLJPNNFLKON\x18\x0b\x20\x01(\rR\x0b\
    LLJPNNFLKON\x12\x20\n\x0bBHONGJMFKIM\x18\n\x20\x01(\rR\x0bBHONGJMFKIM\
    \x12.\n\x0bLBLNFDDBNCM\x18\r\x20\x03(\x0b2\x0c.ANGOENJIKBFR\x0bLBLNFDDBN\
    CM\x12.\n\x0bMGAPDCENGLP\x18\x02\x20\x03(\x0b2\x0c.DEIFMNDOBEBR\x0bMGAPD\
    CENGLP\x12/\n\x0bOKJPNBHNNMK\x18\x8f\x08\x20\x01(\x0b2\x0c.LMOJNCJPACCR\
    \x0bOKJPNBHNNMK\x12!\n\x0bAKCMBCMDMBC\x18\xc4\n\x20\x01(\rR\x0bAKCMBCMDM\
    BC\x12!\n\x0bOLGCJIKGFFC\x18\xea\x08\x20\x01(\rR\x0bOLGCJIKGFFC\x12@\n\
    \x0bMOFJMDEEADD\x18\xa1\n\x20\x03(\x0b2\x1d.NDHNICCLBDN.MOFJMDEEADDEntry\
    R\x0bMOFJMDEEADD\x12\x20\n\x0bNHKHGDCBHJD\x18\t\x20\x01(\rR\x0bNHKHGDCBH\
    JD\x12?\n\x0bFHLAMIHOOJA\x18\x0f\x20\x03(\x0b2\x1d.NDHNICCLBDN.FHLAMIHOO\
    JAEntryR\x0bFHLAMIHOOJA\x12\x20\n\x0bAACPJNOFIEF\x18\x01\x20\x03(\rR\x0b\
    AACPJNOFIEF\x12\x20\n\x0bLLLHPFLFKPP\x18\x06\x20\x01(\rR\x0bLLLHPFLFKPP\
    \x12\x20\n\x0bBJNKEEFKAIK\x18\x07\x20\x01(\rR\x0bBJNKEEFKAIK\x12/\n\x0bL\
    ECGLNPFACA\x18\xe8\x0b\x20\x03(\x0b2\x0c.ACGLLJPJCHKR\x0bLECGLNPFACA\x12\
    !\n\x0bMGLHEBHJABE\x18\xa8\x08\x20\x03(\rR\x0bMGLHEBHJABE\x1a>\n\x10MOFJ\
    MDEEADDEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05va\
    lue\x18\x02\x20\x01(\x05R\x05value:\x028\x01\x1aL\n\x10FHLAMIHOOJAEntry\
    \x12\x10\n\x03key\x18\x01\x20\x01(\rR\x03key\x12\"\n\x05value\x18\x02\
    \x20\x01(\x0b2\x0c.DLIBJAFBHEAR\x05value:\x028\x01b\x06proto3\
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
            let mut deps = ::std::vec::Vec::with_capacity(8);
            deps.push(super::ACGLLJPJCHK::file_descriptor().clone());
            deps.push(super::ANGOENJIKBF::file_descriptor().clone());
            deps.push(super::DEIFMNDOBEB::file_descriptor().clone());
            deps.push(super::DLIBJAFBHEA::file_descriptor().clone());
            deps.push(super::LMOJNCJPACC::file_descriptor().clone());
            deps.push(super::MDLAAPIEBLF::file_descriptor().clone());
            deps.push(super::PDJDKKJCPOF::file_descriptor().clone());
            deps.push(super::SceneEntityInfo::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(NDHNICCLBDN::generated_message_descriptor_data());
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
