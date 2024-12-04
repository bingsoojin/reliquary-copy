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

//! Generated file from `BGPOKPPMEBI.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_4_0;

// @@protoc_insertion_point(message:BGPOKPPMEBI)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct BGPOKPPMEBI {
    // message oneof groups
    pub NGKHDEHJCHG: ::std::option::Option<bgpokppmebi::NGKHDEHJCHG>,
    // special fields
    // @@protoc_insertion_point(special_field:BGPOKPPMEBI.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BGPOKPPMEBI {
    fn default() -> &'a BGPOKPPMEBI {
        <BGPOKPPMEBI as ::protobuf::Message>::default_instance()
    }
}

impl BGPOKPPMEBI {
    pub fn new() -> BGPOKPPMEBI {
        ::std::default::Default::default()
    }

    // .ICNPHJAADDE FHEODGILEEP = 12;

    pub fn FHEODGILEEP(&self) -> &super::ICNPHJAADDE::ICNPHJAADDE {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(ref v)) => v,
            _ => <super::ICNPHJAADDE::ICNPHJAADDE as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FHEODGILEEP(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_FHEODGILEEP(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FHEODGILEEP(&mut self, v: super::ICNPHJAADDE::ICNPHJAADDE) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FHEODGILEEP(&mut self) -> &mut super::ICNPHJAADDE::ICNPHJAADDE {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(super::ICNPHJAADDE::ICNPHJAADDE::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FHEODGILEEP(&mut self) -> super::ICNPHJAADDE::ICNPHJAADDE {
        if self.has_FHEODGILEEP() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ICNPHJAADDE::ICNPHJAADDE::new()
        }
    }

    // .FIIDPGGKANO NCOAIBHNPOH = 8;

    pub fn NCOAIBHNPOH(&self) -> &super::FIIDPGGKANO::FIIDPGGKANO {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(ref v)) => v,
            _ => <super::FIIDPGGKANO::FIIDPGGKANO as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NCOAIBHNPOH(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_NCOAIBHNPOH(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NCOAIBHNPOH(&mut self, v: super::FIIDPGGKANO::FIIDPGGKANO) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NCOAIBHNPOH(&mut self) -> &mut super::FIIDPGGKANO::FIIDPGGKANO {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(super::FIIDPGGKANO::FIIDPGGKANO::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NCOAIBHNPOH(&mut self) -> super::FIIDPGGKANO::FIIDPGGKANO {
        if self.has_NCOAIBHNPOH() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(v)) => v,
                _ => panic!(),
            }
        } else {
            super::FIIDPGGKANO::FIIDPGGKANO::new()
        }
    }

    // .JHGGMOKNOJB HLIKPFEHOEI = 1;

    pub fn HLIKPFEHOEI(&self) -> &super::JHGGMOKNOJB::JHGGMOKNOJB {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(ref v)) => v,
            _ => <super::JHGGMOKNOJB::JHGGMOKNOJB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_HLIKPFEHOEI(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_HLIKPFEHOEI(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_HLIKPFEHOEI(&mut self, v: super::JHGGMOKNOJB::JHGGMOKNOJB) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(v))
    }

    // Mutable pointer to the field.
    pub fn mut_HLIKPFEHOEI(&mut self) -> &mut super::JHGGMOKNOJB::JHGGMOKNOJB {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(super::JHGGMOKNOJB::JHGGMOKNOJB::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_HLIKPFEHOEI(&mut self) -> super::JHGGMOKNOJB::JHGGMOKNOJB {
        if self.has_HLIKPFEHOEI() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(v)) => v,
                _ => panic!(),
            }
        } else {
            super::JHGGMOKNOJB::JHGGMOKNOJB::new()
        }
    }

    // .ENCEHJOGEAB KACLLKAKKKO = 4;

    pub fn KACLLKAKKKO(&self) -> &super::ENCEHJOGEAB::ENCEHJOGEAB {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(ref v)) => v,
            _ => <super::ENCEHJOGEAB::ENCEHJOGEAB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_KACLLKAKKKO(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_KACLLKAKKKO(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_KACLLKAKKKO(&mut self, v: super::ENCEHJOGEAB::ENCEHJOGEAB) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_KACLLKAKKKO(&mut self) -> &mut super::ENCEHJOGEAB::ENCEHJOGEAB {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(super::ENCEHJOGEAB::ENCEHJOGEAB::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_KACLLKAKKKO(&mut self) -> super::ENCEHJOGEAB::ENCEHJOGEAB {
        if self.has_KACLLKAKKKO() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::ENCEHJOGEAB::ENCEHJOGEAB::new()
        }
    }

    // .IPLEMMPCECB NNDPGGJCPOG = 15;

    pub fn NNDPGGJCPOG(&self) -> &super::IPLEMMPCECB::IPLEMMPCECB {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(ref v)) => v,
            _ => <super::IPLEMMPCECB::IPLEMMPCECB as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NNDPGGJCPOG(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_NNDPGGJCPOG(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NNDPGGJCPOG(&mut self, v: super::IPLEMMPCECB::IPLEMMPCECB) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NNDPGGJCPOG(&mut self) -> &mut super::IPLEMMPCECB::IPLEMMPCECB {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(super::IPLEMMPCECB::IPLEMMPCECB::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NNDPGGJCPOG(&mut self) -> super::IPLEMMPCECB::IPLEMMPCECB {
        if self.has_NNDPGGJCPOG() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(v)) => v,
                _ => panic!(),
            }
        } else {
            super::IPLEMMPCECB::IPLEMMPCECB::new()
        }
    }

    // .LABLHOAFNBJ IFKDNKJJMAA = 10;

    pub fn IFKDNKJJMAA(&self) -> &super::LABLHOAFNBJ::LABLHOAFNBJ {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(ref v)) => v,
            _ => <super::LABLHOAFNBJ::LABLHOAFNBJ as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_IFKDNKJJMAA(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_IFKDNKJJMAA(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_IFKDNKJJMAA(&mut self, v: super::LABLHOAFNBJ::LABLHOAFNBJ) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(v))
    }

    // Mutable pointer to the field.
    pub fn mut_IFKDNKJJMAA(&mut self) -> &mut super::LABLHOAFNBJ::LABLHOAFNBJ {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(super::LABLHOAFNBJ::LABLHOAFNBJ::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_IFKDNKJJMAA(&mut self) -> super::LABLHOAFNBJ::LABLHOAFNBJ {
        if self.has_IFKDNKJJMAA() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(v)) => v,
                _ => panic!(),
            }
        } else {
            super::LABLHOAFNBJ::LABLHOAFNBJ::new()
        }
    }

    // .POOAECFIIGH FMELHOHDHGP = 11;

    pub fn FMELHOHDHGP(&self) -> &super::POOAECFIIGH::POOAECFIIGH {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(ref v)) => v,
            _ => <super::POOAECFIIGH::POOAECFIIGH as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_FMELHOHDHGP(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_FMELHOHDHGP(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_FMELHOHDHGP(&mut self, v: super::POOAECFIIGH::POOAECFIIGH) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(v))
    }

    // Mutable pointer to the field.
    pub fn mut_FMELHOHDHGP(&mut self) -> &mut super::POOAECFIIGH::POOAECFIIGH {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(super::POOAECFIIGH::POOAECFIIGH::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_FMELHOHDHGP(&mut self) -> super::POOAECFIIGH::POOAECFIIGH {
        if self.has_FMELHOHDHGP() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(v)) => v,
                _ => panic!(),
            }
        } else {
            super::POOAECFIIGH::POOAECFIIGH::new()
        }
    }

    // .PKANLCNEDOC NEMGEOPCBAO = 5;

    pub fn NEMGEOPCBAO(&self) -> &super::PKANLCNEDOC::PKANLCNEDOC {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(ref v)) => v,
            _ => <super::PKANLCNEDOC::PKANLCNEDOC as ::protobuf::Message>::default_instance(),
        }
    }

    pub fn clear_NEMGEOPCBAO(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
    }

    pub fn has_NEMGEOPCBAO(&self) -> bool {
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_NEMGEOPCBAO(&mut self, v: super::PKANLCNEDOC::PKANLCNEDOC) {
        self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(v))
    }

    // Mutable pointer to the field.
    pub fn mut_NEMGEOPCBAO(&mut self) -> &mut super::PKANLCNEDOC::PKANLCNEDOC {
        if let ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(_)) = self.NGKHDEHJCHG {
        } else {
            self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(super::PKANLCNEDOC::PKANLCNEDOC::new()));
        }
        match self.NGKHDEHJCHG {
            ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_NEMGEOPCBAO(&mut self) -> super::PKANLCNEDOC::PKANLCNEDOC {
        if self.has_NEMGEOPCBAO() {
            match self.NGKHDEHJCHG.take() {
                ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(v)) => v,
                _ => panic!(),
            }
        } else {
            super::PKANLCNEDOC::PKANLCNEDOC::new()
        }
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(8);
        let mut oneofs = ::std::vec::Vec::with_capacity(1);
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ICNPHJAADDE::ICNPHJAADDE>(
            "FHEODGILEEP",
            BGPOKPPMEBI::has_FHEODGILEEP,
            BGPOKPPMEBI::FHEODGILEEP,
            BGPOKPPMEBI::mut_FHEODGILEEP,
            BGPOKPPMEBI::set_FHEODGILEEP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::FIIDPGGKANO::FIIDPGGKANO>(
            "NCOAIBHNPOH",
            BGPOKPPMEBI::has_NCOAIBHNPOH,
            BGPOKPPMEBI::NCOAIBHNPOH,
            BGPOKPPMEBI::mut_NCOAIBHNPOH,
            BGPOKPPMEBI::set_NCOAIBHNPOH,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::JHGGMOKNOJB::JHGGMOKNOJB>(
            "HLIKPFEHOEI",
            BGPOKPPMEBI::has_HLIKPFEHOEI,
            BGPOKPPMEBI::HLIKPFEHOEI,
            BGPOKPPMEBI::mut_HLIKPFEHOEI,
            BGPOKPPMEBI::set_HLIKPFEHOEI,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::ENCEHJOGEAB::ENCEHJOGEAB>(
            "KACLLKAKKKO",
            BGPOKPPMEBI::has_KACLLKAKKKO,
            BGPOKPPMEBI::KACLLKAKKKO,
            BGPOKPPMEBI::mut_KACLLKAKKKO,
            BGPOKPPMEBI::set_KACLLKAKKKO,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::IPLEMMPCECB::IPLEMMPCECB>(
            "NNDPGGJCPOG",
            BGPOKPPMEBI::has_NNDPGGJCPOG,
            BGPOKPPMEBI::NNDPGGJCPOG,
            BGPOKPPMEBI::mut_NNDPGGJCPOG,
            BGPOKPPMEBI::set_NNDPGGJCPOG,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::LABLHOAFNBJ::LABLHOAFNBJ>(
            "IFKDNKJJMAA",
            BGPOKPPMEBI::has_IFKDNKJJMAA,
            BGPOKPPMEBI::IFKDNKJJMAA,
            BGPOKPPMEBI::mut_IFKDNKJJMAA,
            BGPOKPPMEBI::set_IFKDNKJJMAA,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::POOAECFIIGH::POOAECFIIGH>(
            "FMELHOHDHGP",
            BGPOKPPMEBI::has_FMELHOHDHGP,
            BGPOKPPMEBI::FMELHOHDHGP,
            BGPOKPPMEBI::mut_FMELHOHDHGP,
            BGPOKPPMEBI::set_FMELHOHDHGP,
        ));
        fields.push(::protobuf::reflect::rt::v2::make_oneof_message_has_get_mut_set_accessor::<_, super::PKANLCNEDOC::PKANLCNEDOC>(
            "NEMGEOPCBAO",
            BGPOKPPMEBI::has_NEMGEOPCBAO,
            BGPOKPPMEBI::NEMGEOPCBAO,
            BGPOKPPMEBI::mut_NEMGEOPCBAO,
            BGPOKPPMEBI::set_NEMGEOPCBAO,
        ));
        oneofs.push(bgpokppmebi::NGKHDEHJCHG::generated_oneof_descriptor_data());
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BGPOKPPMEBI>(
            "BGPOKPPMEBI",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BGPOKPPMEBI {
    const NAME: &'static str = "BGPOKPPMEBI";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                98 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(is.read_message()?));
                },
                66 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(is.read_message()?));
                },
                10 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(is.read_message()?));
                },
                34 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(is.read_message()?));
                },
                122 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(is.read_message()?));
                },
                82 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(is.read_message()?));
                },
                90 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(is.read_message()?));
                },
                42 => {
                    self.NGKHDEHJCHG = ::std::option::Option::Some(bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.NGKHDEHJCHG {
            match v {
                &bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
                },
                &bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(ref v) => {
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
        if let ::std::option::Option::Some(ref v) = self.NGKHDEHJCHG {
            match v {
                &bgpokppmebi::NGKHDEHJCHG::FHEODGILEEP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(12, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::NCOAIBHNPOH(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::HLIKPFEHOEI(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(1, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::KACLLKAKKKO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::NNDPGGJCPOG(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(15, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::IFKDNKJJMAA(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(10, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::FMELHOHDHGP(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(11, v, os)?;
                },
                &bgpokppmebi::NGKHDEHJCHG::NEMGEOPCBAO(ref v) => {
                    ::protobuf::rt::write_message_field_with_cached_size(5, v, os)?;
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

    fn new() -> BGPOKPPMEBI {
        BGPOKPPMEBI::new()
    }

    fn clear(&mut self) {
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.NGKHDEHJCHG = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BGPOKPPMEBI {
        static instance: BGPOKPPMEBI = BGPOKPPMEBI {
            NGKHDEHJCHG: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BGPOKPPMEBI {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BGPOKPPMEBI").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BGPOKPPMEBI {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BGPOKPPMEBI {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `BGPOKPPMEBI`
pub mod bgpokppmebi {

    #[derive(Clone,PartialEq,Debug)]
    #[non_exhaustive]
    // @@protoc_insertion_point(oneof:BGPOKPPMEBI.NGKHDEHJCHG)
    pub enum NGKHDEHJCHG {
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.FHEODGILEEP)
        FHEODGILEEP(super::super::ICNPHJAADDE::ICNPHJAADDE),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.NCOAIBHNPOH)
        NCOAIBHNPOH(super::super::FIIDPGGKANO::FIIDPGGKANO),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.HLIKPFEHOEI)
        HLIKPFEHOEI(super::super::JHGGMOKNOJB::JHGGMOKNOJB),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.KACLLKAKKKO)
        KACLLKAKKKO(super::super::ENCEHJOGEAB::ENCEHJOGEAB),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.NNDPGGJCPOG)
        NNDPGGJCPOG(super::super::IPLEMMPCECB::IPLEMMPCECB),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.IFKDNKJJMAA)
        IFKDNKJJMAA(super::super::LABLHOAFNBJ::LABLHOAFNBJ),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.FMELHOHDHGP)
        FMELHOHDHGP(super::super::POOAECFIIGH::POOAECFIIGH),
        // @@protoc_insertion_point(oneof_field:BGPOKPPMEBI.NEMGEOPCBAO)
        NEMGEOPCBAO(super::super::PKANLCNEDOC::PKANLCNEDOC),
    }

    impl ::protobuf::Oneof for NGKHDEHJCHG {
    }

    impl ::protobuf::OneofFull for NGKHDEHJCHG {
        fn descriptor() -> ::protobuf::reflect::OneofDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::OneofDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| <super::BGPOKPPMEBI as ::protobuf::MessageFull>::descriptor().oneof_by_name("NGKHDEHJCHG").unwrap()).clone()
        }
    }

    impl NGKHDEHJCHG {
        pub(in super) fn generated_oneof_descriptor_data() -> ::protobuf::reflect::GeneratedOneofDescriptorData {
            ::protobuf::reflect::GeneratedOneofDescriptorData::new::<NGKHDEHJCHG>("NGKHDEHJCHG")
        }
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11BGPOKPPMEBI.proto\x1a\x11ENCEHJOGEAB.proto\x1a\x11FIIDPGGKANO.prot\
    o\x1a\x11ICNPHJAADDE.proto\x1a\x11IPLEMMPCECB.proto\x1a\x11JHGGMOKNOJB.p\
    roto\x1a\x11LABLHOAFNBJ.proto\x1a\x11PKANLCNEDOC.proto\x1a\x11POOAECFIIG\
    H.proto\"\xac\x03\n\x0bBGPOKPPMEBI\x120\n\x0bFHEODGILEEP\x18\x0c\x20\x01\
    (\x0b2\x0c.ICNPHJAADDEH\0R\x0bFHEODGILEEP\x120\n\x0bNCOAIBHNPOH\x18\x08\
    \x20\x01(\x0b2\x0c.FIIDPGGKANOH\0R\x0bNCOAIBHNPOH\x120\n\x0bHLIKPFEHOEI\
    \x18\x01\x20\x01(\x0b2\x0c.JHGGMOKNOJBH\0R\x0bHLIKPFEHOEI\x120\n\x0bKACL\
    LKAKKKO\x18\x04\x20\x01(\x0b2\x0c.ENCEHJOGEABH\0R\x0bKACLLKAKKKO\x120\n\
    \x0bNNDPGGJCPOG\x18\x0f\x20\x01(\x0b2\x0c.IPLEMMPCECBH\0R\x0bNNDPGGJCPOG\
    \x120\n\x0bIFKDNKJJMAA\x18\n\x20\x01(\x0b2\x0c.LABLHOAFNBJH\0R\x0bIFKDNK\
    JJMAA\x120\n\x0bFMELHOHDHGP\x18\x0b\x20\x01(\x0b2\x0c.POOAECFIIGHH\0R\
    \x0bFMELHOHDHGP\x120\n\x0bNEMGEOPCBAO\x18\x05\x20\x01(\x0b2\x0c.PKANLCNE\
    DOCH\0R\x0bNEMGEOPCBAOB\r\n\x0bNGKHDEHJCHGb\x06proto3\
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
            deps.push(super::ENCEHJOGEAB::file_descriptor().clone());
            deps.push(super::FIIDPGGKANO::file_descriptor().clone());
            deps.push(super::ICNPHJAADDE::file_descriptor().clone());
            deps.push(super::IPLEMMPCECB::file_descriptor().clone());
            deps.push(super::JHGGMOKNOJB::file_descriptor().clone());
            deps.push(super::LABLHOAFNBJ::file_descriptor().clone());
            deps.push(super::PKANLCNEDOC::file_descriptor().clone());
            deps.push(super::POOAECFIIGH::file_descriptor().clone());
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(BGPOKPPMEBI::generated_message_descriptor_data());
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
