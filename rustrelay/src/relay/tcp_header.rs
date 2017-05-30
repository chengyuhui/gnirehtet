use byteorder::{BigEndian, ByteOrder};
use super::ipv4_header::IPv4Header;
use super::source_destination::SourceDestination;

pub struct TCPHeader {
    source_port: u16,
    destination_port: u16,
    sequence_number: u32,
    acknowledgement_number: u32,
    header_length: u8,
    flags: u16,
    window: u16,
}

impl TCPHeader {
    pub fn parse(raw: &[u8]) -> TCPHeader {
        let data_offset_and_flags = BigEndian::read_u16(&raw[12..14]);
        TCPHeader {
            source_port: BigEndian::read_u16(&raw[0..2]),
            destination_port: BigEndian::read_u16(&raw[2..4]),
            sequence_number: BigEndian::read_u32(&raw[4..8]),
            acknowledgement_number: BigEndian::read_u32(&raw[8..12]),
            header_length: (data_offset_and_flags & 0xF000 >> 10) as u8,
            flags: data_offset_and_flags & 0x1FF,
            window: BigEndian::read_u16(&raw[14..16]),
        }
    }

    pub fn set_source_port(&mut self, raw: &mut [u8], source_port: u16) {
        self.source_port = source_port;
        BigEndian::write_u16(&mut raw[0..2], source_port);
    }

    pub fn set_destination_port(&mut self, raw: &mut [u8], destination_port: u16) {
        self.destination_port = destination_port;
        BigEndian::write_u16(&mut raw[2..4], destination_port);
    }

    pub fn set_sequence_number(&mut self, raw: &mut [u8], sequence_number: u32) {
        self.sequence_number = sequence_number;
        BigEndian::write_u32(&mut raw[4..8], sequence_number);
    }

    pub fn set_acknowledgment_number(&mut self, raw: &mut [u8], acknowledgement_number: u32) {
        self.acknowledgement_number = acknowledgement_number;
        BigEndian::write_u32(&mut raw[8..12], acknowledgement_number);
    }

    pub fn set_flags(&mut self, raw: &mut [u8], flags: u16) {
        self.flags = flags;
        let mut data_offset_and_flags = BigEndian::read_u16(&mut raw[12..14]);
        data_offset_and_flags = data_offset_and_flags & 0xFE00 | flags & 0x1FF;
        BigEndian::write_u16(&mut raw[12..14], data_offset_and_flags);
    }

    pub fn shrink_options(&mut self, raw: &mut [u8]) {
        self.set_data_offset(raw, 5);
    }

    fn set_data_offset(&mut self, raw: &mut [u8], data_offset: u8) {
        let mut data_offset_and_flags = BigEndian::read_u16(&mut raw[12..14]);
        data_offset_and_flags = data_offset_and_flags & 0x0FFF | ((data_offset as u16) << 12);
        BigEndian::write_u16(&mut raw[12..14], data_offset_and_flags);
        self.header_length = data_offset << 2;
    }

    pub fn compute_checksum(&self, raw: &mut [u8], ipv4_header: &IPv4Header) {
    }

    pub fn set_checksum(&mut self, raw: &mut [u8], checksum: u16) {
        BigEndian::write_u16(&mut raw[16..18], checksum);
    }
}

impl SourceDestination<u16> for TCPHeader {
    fn get_source(&self, _: &[u8]) -> u16 {
        self.source_port
    }

    fn get_destination(&self, _: &[u8]) -> u16 {
        self.destination_port
    }

    fn set_source(&mut self, raw: &mut [u8], source: u16) {
        self.set_source_port(raw, source);
    }

    fn set_destination(&mut self, raw: &mut [u8], destination: u16) {
        self.set_destination_port(raw, destination);
    }
}