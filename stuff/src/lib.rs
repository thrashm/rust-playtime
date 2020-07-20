pub trait XSerialize {
    fn xml_serialize(&self) -> String;
}