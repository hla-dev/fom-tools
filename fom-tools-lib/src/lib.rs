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
    Publish,
    Subscribe,
    PublishSubscribe,
    Neither,
}

struct AttributeType {
    name: String,
    data_type: ReferenceType,
    update_type: UpdateType,
    update_condition: Option<String>,
    onwership: OwnershipType,
    sharing: SharingType,
    dimensions: Option<Vec<ReferenceType>>,
    transportation: ReferenceType,
    order: OrderType,
    semantics: Option<String>,
}

enum UpdateType {
    Static,
    Periodic,
    Conditional,
    NA,
}

enum OwnershipType {
    Divest,
    Acquire,
    DivestAcquire,
    NoTransfer,
}

enum OrderType {
    Receive,
    TimeStamp,
}

struct InteractionsType {
    interactions: InteractionClassType,
}

struct InteractionClassType {
    name: String,
    sharing: SharingType,
    dimensions: Option<Vec<ReferenceType>>,
    transportation: ReferenceType,
    order: OrderType,
    semantics: Option<String>,
    parameters: Option<Vec<ParameterType>>,
    interaction_classes: Option<Vec<InteractionClassType>>,
}

struct ParameterType {
    name: String,
    data_type: ReferenceType,
    semantics: Option<String>,
}

struct DimensionsType {}
struct TimeType {
    time_stamp: Option<TimeTypeType>,
    lookahead: Option<TimeTypeType>,
}

struct TimeTypeType {
    data_type: ReferenceType,
    semantics: Option<String>,
}

struct TagsType {
    update_reflect_tag: Option<TagType>,
    send_receive_tag: Option<TagType>,
    delete_remove_tag: Option<TagType>,
    divestiture_request_tag: Option<TagType>,
    divestiture_completion_tag: Option<TagType>,
    acquisition_request_tag: Option<TagType>,
    request_update_tag: Option<TagType>,
}

struct TagType {
    data_type: ReferenceType,
    semantics: Option<String>,
}

struct SynchronizationsType {
    synchronization_points: Option<Vec<SynchronizationPointType>>,
}

struct SynchronizationPointType {
    label: String,
    data_type: Option<ReferenceType>,
    capability: CapabilityType,
    semantics: Option<String>,
}

enum CapabilityType {
    Register,
    Achieve,
    RegisterAchieve,
    NoSynch,
    Na,
}

struct TransportationsType {
    transportations: Option<Vec<TransportationType>>,
}

struct TransportationType {
    name: String,
    reliable: ReliableType,
    semantics: Option<String>,
}

enum ReliableType {
    Yes,
    No,
}

struct SwitchesType {
    auto_provide: SwitchType,
    convey_region_designator_sets: SwitchType,
    convey_producing_federate: SwitchType,
    attribute_scope_advisory: SwitchType,
    attribute_relevance_advisory: SwitchType,
    object_class_relevance_advisory: SwitchType,
    interaction_relevance_advisory: SwitchType,
    service_reporting: SwitchType,
    exception_reporting: SwitchType,
    delay_subscription_evaluation: SwitchType,
    automatic_resign_action: ResignSwitchType,
}

struct SwitchType {
    is_enabled: bool,
}

enum ResignSwitchType {
    UnconditionallyDivestAttributes,
    DeleteObjects,
    CancelPendingOwnershipAcquisitions,
    DeleteObjectsThenDivest,
    CancelThenDeleteThenDivest,
    NoAction,
}

struct UpdateRatesType {
    update_rates: Option<Vec<UpdateRateType>>,
}

struct UpdateRateType {
    name: String,
    rate: RateType,
    semantics: Option<String>,
}

struct RateType {
    value: String,
}

struct DataTypesType {
    basic_data_representations: BasicDataRepresentationsType,
    simple_data_types: SimpleDataTypesType,
    enumerated_data_types: EnumeratedDataTypesType,
    array_data_types: ArrayDataTypesType,
    fixed_record_data_types: FixedRecordDataTypesType,
    variand_record_data_types: VariantRecordDataTypesType,
}

struct BasicDataRepresentationsType {
    basic_datas: Option<Vec<BasicDataType>>,
}

struct BasicDataType {
    name: String,
    size: SizeType,
    interpretation: String,
    endian: EndianType,
    encoding: String,
}

struct SizeType {
    size: u8,
}

enum EndianType {
    Big,
    Little,
}

struct SimpleDataTypesType {
    simple_datas: Option<Vec<SimpleDataType>>,
}

struct SimpleDataType {
    name: String,
    representation: ReferenceType,
    units: String,
    resolution: Option<String>,
    accuracy: Option<String>,
    semantics: Option<String>,
}

struct EnumeratedDataTypesType {}
struct ArrayDataTypesType {}
struct FixedRecordDataTypesType {}
struct VariantRecordDataTypesType {}

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
