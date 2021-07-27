use std::io::Read;
use xmltree::{Element, ParseError};

pub struct ObjectModelType {
    model_identification: ModelIdentificationType,
    service_utilization: Option<ServiceUtiliizationType>,
    objects: ObjectsType,
    interactions: InteractionsType,
    dimensions: DimensionsType,
    time: Option<TimeType>,
    tags: Option<TagsType>,
    synchronizations: Option<SynchronizationsType>,
    transportations: TransportationsType,
    switches: SwitchesType,
    update_rates: Option<UpdateRatesType>,
    data_types: DataTypesType,
    notes: Option<NotesType>,
}

struct ModelIdentificationType {
    name: String,
    model_type: ModelType,
    version: String,
    modification_date: String,
    security_classification: SecurityClassificationType,
    release_restriction: Option<Vec<String>>,
    purpose: Option<String>,
    application_domain: Option<ApplicationDomainType>,
    description: String,
    use_limitation: Option<String>,
    use_history: Option<Vec<String>>,
    keywords: Option<Vec<KeywordType>>,
    poc: Vec<PocType>,
    references: Option<Vec<ReferenceType>>,
    other: Option<String>,
    glyph: Option<GlyphType>,
}

enum ModelType {
    FOM,
    SOM,
}

enum SecurityClassificationType {
    Unclassified,
    Confidential,
    Secret,
    TopSecret,
}

enum ApplicationDomainType {
    Analysis,
    Training,
    TestAndEvaluation,
    Engineering,
    Acquisition,
}

struct KeywordType {
    taxonomy: Option<String>,
    keyword_value: String,
}

struct PocType {
    poc_type: PocTypeType,
    poc_name: Option<String>,
    poc_org: Option<String>,
    poc_telephones: Option<Vec<String>>,
    poc_emails: Option<Vec<String>>,
}

enum PocTypeType {
    PrimaryAuthor,
    Contributor,
    Proponent,
    Sponsor,
    ReleaseAuthority,
    TechnicalPoc,
}

struct ReferenceType {
    reference_type: String,
    identification: String,
}

struct GlyphType {
    href: String,
    glyph_type: GlyphTypeType,
    height: u16,
    width: u16,
    alt: String,
}

enum GlyphTypeType {
    Bitmap,
    Jpg,
    Gif,
    Png,
    Tiff,
}

struct ServiceUtiliizationType {
    connect: ServiceInfoType,
    disconnect: ServiceInfoType,
    // ... and the rest
}

struct ServiceInfoType {
    section: String,
    is_callback: bool,
    is_used: bool,
}

struct ObjectsType {
    root_object_class: ObjectClassType,
}

struct ObjectClassType {
    name: String,
    sharing: SharingType,
    semantics: Option<String>,
    attributes: Option<Vec<AttributeType>>,
    object_classes: Option<Vec<ObjectClassType>>,
}

enum SharingType {
    Neither,
}

struct AttributeType {}
struct InteractionsType {}
struct DimensionsType {}
struct TimeType {}
struct TagsType {}
struct SynchronizationsType {}
struct TransportationsType {}
struct SwitchesType {}
struct UpdateRatesType {}
struct DataTypesType {}
struct NotesType {}

pub fn parse<R: Read>(r: R) -> Result<(), ParseError> {
    let fom_as_xml = Element::parse(r)?;

    if let Some(model_identification_element) = fom_as_xml.get_child("modelIdentification") {
        if let Some(model_id_name_element) = model_identification_element.get_child("name") {
            if let Some(model_id_name_text) = model_id_name_element.get_text() {
                println!("{:?}", model_id_name_text.trim());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
