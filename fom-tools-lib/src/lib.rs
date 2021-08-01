use std::io::Read;
use xmltree::{Element, ParseError, XMLNode};

/// Return the trimmed text content of the provided element
fn get_element_text(e: &Element) -> String {
    if let Some(text) = e.get_text() {
        text.into_owned().trim().to_string()
    } else {
        String::from("")
    }
}

/// Return the trimmed text content of the named child element of the provided
/// root element. Return None if the named child element does not exist.
fn get_text_of_child_element(root: &Element, child_element_name: &str) -> Option<String> {
    if let Some(e) = root.get_child(child_element_name) {
        Some(get_element_text(e))
    } else {
        None
    }
}

///
fn get_text_of_attribute(element: &Element, attribute_name: &str) -> Option<String> {
    if let Some(attribute_value) = element.attributes.get(attribute_name) {
        Some(attribute_value.clone())
    } else {
        None
    }
}

/// Return the trimmed text content of the named child element of the provided
/// root element. Panic, with the provided panic message, if the name child
/// element does not exist.
fn get_text_of_child_element_or_panic(
    root: &Element,
    child_element_name: &str,
    panic_message: &'static str,
) -> String {
    match get_text_of_child_element(root, child_element_name) {
        Some(text) => text,
        None => panic!("{}", panic_message),
    }
}

///
fn get_text_of_attribute_or_panic(
    element: &Element,
    attribute_name: &str,
    panic_message: &str,
) -> String {
    match get_text_of_attribute(element, attribute_name) {
        Some(attribute_value) => attribute_value,
        None => panic!("{}", panic_message),
    }
}

/// Return an instance of the generic type created from the named child
/// element of the provided root element. Return None if the named child
/// element does not exist.
fn get_child_element_as_type<'a, T: From<&'a Element>>(
    root: &'a Element,
    child_element_name: &'a str,
) -> Option<T> {
    if let Some(e) = root.get_child(child_element_name) {
        Some(T::from(e))
    } else {
        None
    }
}

/// Return an instance of the generic type created from the named attribute
/// of the provided element. Return None if the named attribute does not
/// exist
fn get_attribute_as_type<'a, T: From<&'a String>>(
    element: &'a Element,
    attribute_name: &'a str,
) -> Option<T> {
    if let Some(attribute_value) = element.attributes.get(attribute_name) {
        Some(T::from(attribute_value))
    } else {
        None
    }
}

/// Return an instance of the generic type created from the named child
/// element of the provided root element. Panic, with the provided panic
/// message, if the named child element does not exist.
fn get_child_element_as_type_or_panic<'a, T: From<&'a Element>>(
    root: &'a Element,
    child_element_name: &'a str,
    panic_message: &'static str,
) -> T {
    match get_child_element_as_type(root, child_element_name) {
        Some(val) => val,
        None => panic!("{}", panic_message),
    }
}

/// Return an instance of the generic type created from the named attribute
/// of the provided element. Panic, with the provided panic message, if the
/// named attribute does not exist
fn get_attribute_as_type_or_panic<'a, T: From<&'a String>>(
    element: &'a Element,
    attribute_name: &'a str,
    panic_message: &'static str,
) -> T {
    match get_attribute_as_type(element, attribute_name) {
        Some(attribute_value) => attribute_value,
        None => panic!("{}", panic_message),
    }
}

/// Return the trimmed text of all named child elements of the provided root element.
/// The returned vector will be empty if no such child elements exist.
fn get_text_of_child_elements(root: &Element, child_element_name: &str) -> Vec<String> {
    root.children
        .iter()
        .filter(|xml_node| match xml_node {
            XMLNode::Element(e) => e.name == child_element_name,
            _ => false,
        })
        .map(|xml_node| get_element_text(xml_node.as_element().unwrap()))
        .collect()
}

/// Return instances of the generic type created from each of the named child
/// elements of the provided root element. The returned vector will be empty
/// if no such child elements exist.
fn get_text_of_child_elements_as_type<'a, T: From<&'a Element>>(
    root: &'a Element,
    child_element_name: &str,
) -> Vec<T> {
    root.children
        .iter()
        .filter(|xml_node| match xml_node {
            XMLNode::Element(e) => e.name == child_element_name,
            _ => false,
        })
        .map(|xml_node| T::from(xml_node.as_element().unwrap()))
        .collect()
}

pub struct ObjectModelType {
    pub model_identification: ModelIdentificationType,
    pub service_utilization: Option<ServiceUtiliizationType>,
    pub objects: ObjectsType,
    pub interactions: InteractionsType,
    pub dimensions: DimensionsType,
    pub time: Option<TimeType>,
    pub tags: Option<TagsType>,
    pub synchronizations: Option<SynchronizationsType>,
    pub transportations: TransportationsType,
    pub switches: SwitchesType,
    pub update_rates: Option<UpdateRatesType>,
    pub data_types: DataTypesType,
    pub notes: Option<NotesType>,
}

impl From<&Element> for ObjectModelType {
    fn from(e: &Element) -> Self {
        Self {
            model_identification: if let Some(e) = e.get_child("modelIdentification") {
                ModelIdentificationType::from(e)
            } else {
                panic!("No 'modelIdentification' element")
            },
            service_utilization: if let Some(e) = e.get_child("serviceUtilization") {
                Some(ServiceUtiliizationType::from(e))
            } else {
                None
            },
            objects: if let Some(e) = e.get_child("objects") {
                ObjectsType::from(e)
            } else {
                panic!("No 'objects' element")
            },
            interactions: if let Some(e) = e.get_child("interactions") {
                InteractionsType::from(e)
            } else {
                panic!("No 'interactions' element")
            },
            dimensions: if let Some(e) = e.get_child("dimensions") {
                DimensionsType::from(e)
            } else {
                panic!("No 'dimensions' element")
            },
            time: if let Some(e) = e.get_child("time") {
                Some(TimeType::from(e))
            } else {
                None
            },
            tags: if let Some(e) = e.get_child("tags") {
                Some(TagsType::from(e))
            } else {
                None
            },
            synchronizations: if let Some(e) = e.get_child("synchronizations") {
                Some(SynchronizationsType::from(e))
            } else {
                None
            },
            transportations: if let Some(e) = e.get_child("transportations") {
                TransportationsType::from(e)
            } else {
                panic!("No 'transportations' element")
            },
            switches: if let Some(e) = e.get_child("switches") {
                SwitchesType::from(e)
            } else {
                panic!("No 'switches' element")
            },
            update_rates: if let Some(e) = e.get_child("updateRates") {
                Some(UpdateRatesType::from(e))
            } else {
                None
            },
            data_types: if let Some(e) = e.get_child("dataTypes") {
                DataTypesType::from(e)
            } else {
                panic!("No 'dataTypes' element")
            },
            notes: if let Some(e) = e.get_child("notes") {
                Some(NotesType::from(e))
            } else {
                None
            },
        }
    }
}

pub struct ModelIdentificationType {
    pub name: String,
    pub model_type: ModelType,
    pub version: String,
    pub modification_date: String,
    pub security_classification: SecurityClassificationType,
    pub release_restriction: Option<Vec<String>>,
    pub purpose: Option<String>,
    pub application_domain: Option<ApplicationDomainType>,
    pub description: String,
    pub use_limitation: Option<String>,
    pub use_history: Option<Vec<String>>,
    pub keywords: Option<Vec<KeywordType>>,
    pub poc: Vec<PocType>,
    pub references: Option<Vec<IdReferenceType>>,
    pub other: Option<String>,
    pub glyph: Option<GlyphType>,
}

impl From<&Element> for ModelIdentificationType {
    fn from(e: &Element) -> Self {
        Self {
            name: if let Some(e) = e.get_child("name") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> name' element")
            },
            model_type: if let Some(e) = e.get_child("type") {
                ModelType::from(e)
            } else {
                panic!("No 'modelIdentification -> type' element")
            },
            version: if let Some(e) = e.get_child("version") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> version' element")
            },
            modification_date: if let Some(e) = e.get_child("modificationDate") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> modificationDate' element")
            },
            security_classification: if let Some(e) = e.get_child("securityClassification") {
                SecurityClassificationType::from(e)
            } else {
                panic!("No 'modelIdentification -> securityClassification' element")
            },
            release_restriction: {
                let release_restrictions = get_text_of_child_elements(e, "releaseRestriction");
                if release_restrictions.len() == 0 {
                    None
                } else {
                    Some(release_restrictions)
                }
            },
            purpose: if let Some(e) = e.get_child("purpose") {
                Some(get_element_text(e))
            } else {
                None
            },
            application_domain: if let Some(e) = e.get_child("applicationDomain") {
                Some(ApplicationDomainType::from(e))
            } else {
                None
            },
            description: if let Some(e) = e.get_child("description") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> description' element")
            },
            use_limitation: get_text_of_child_element(e, "useLimitation"),
            use_history: {
                let use_history = get_text_of_child_elements(e, "useHistory");
                if use_history.len() == 0 {
                    None
                } else {
                    Some(use_history)
                }
            },
            keywords: {
                let keywords = get_text_of_child_elements_as_type(e, "keyword");
                if keywords.len() == 0 {
                    None
                } else {
                    Some(keywords)
                }
            },
            poc: get_text_of_child_elements_as_type(e, "poc"),
            references: {
                let references = get_text_of_child_elements_as_type(e, "reference");
                if references.len() == 0 {
                    None
                } else {
                    Some(references)
                }
            },
            other: if let Some(e) = e.get_child("other") {
                Some(get_element_text(e))
            } else {
                None
            },
            glyph: if let Some(e) = e.get_child("glyph") {
                Some(GlyphType::from(e))
            } else {
                None
            },
        }
    }
}

pub enum ModelType {
    FOM,
    SOM,
}

impl From<&Element> for ModelType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "FOM" => ModelType::FOM,
            "SOM" => ModelType::SOM,
            _ => panic!("Unknown 'modelidentification -> type': {}", text),
        }
    }
}

pub enum SecurityClassificationType {
    Unclassified,
    Confidential,
    Secret,
    TopSecret,
    Other(String),
}

impl From<&Element> for SecurityClassificationType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Unclassified" => SecurityClassificationType::Unclassified,
            "Confidential" => SecurityClassificationType::Confidential,
            "Secret" => SecurityClassificationType::Secret,
            "Top Secret" => SecurityClassificationType::TopSecret,
            _ => SecurityClassificationType::Other(text),
        }
    }
}

pub enum ApplicationDomainType {
    Analysis,
    Training,
    TestAndEvaluation,
    Engineering,
    Acquisition,
    Other(String),
}

impl From<&Element> for ApplicationDomainType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Analysis" => ApplicationDomainType::Analysis,
            "Training" => ApplicationDomainType::Training,
            "Test and Evaluation" => ApplicationDomainType::TestAndEvaluation,
            "Engineering" => ApplicationDomainType::Engineering,
            "Acquisition" => ApplicationDomainType::Acquisition,
            _ => ApplicationDomainType::Other(text),
        }
    }
}

pub struct KeywordType {
    pub taxonomy: Option<String>,
    pub keyword_value: String,
}

impl From<&Element> for KeywordType {
    fn from(e: &Element) -> Self {
        Self {
            taxonomy: if let Some(e) = e.get_child("taxonomy") {
                Some(get_element_text(e))
            } else {
                None
            },
            keyword_value: if let Some(e) = e.get_child("keywordValue") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> keyword -> keywordValue' element")
            },
        }
    }
}

pub struct PocType {
    pub poc_type: PocTypeType,
    pub poc_name: Option<String>,
    pub poc_org: Option<String>,
    pub poc_telephones: Option<Vec<String>>,
    pub poc_emails: Option<Vec<String>>,
}

impl From<&Element> for PocType {
    fn from(e: &Element) -> Self {
        Self {
            poc_type: if let Some(e) = e.get_child("pocType") {
                PocTypeType::from(e)
            } else {
                panic!("No 'modelIdentification -> poc -> pocType' found")
            },
            poc_name: if let Some(e) = e.get_child("pocName") {
                Some(get_element_text(e))
            } else {
                None
            },
            poc_org: if let Some(e) = e.get_child("pocOrg") {
                Some(get_element_text(e))
            } else {
                None
            },
            poc_telephones: {
                let telephones = get_text_of_child_elements(e, "pocTelephone");
                if telephones.is_empty() {
                    None
                } else {
                    Some(telephones)
                }
            },
            poc_emails: {
                let emails = get_text_of_child_elements(e, "pocEmail");
                if emails.is_empty() {
                    None
                } else {
                    Some(emails)
                }
            },
        }
    }
}

pub enum PocTypeType {
    PrimaryAuthor,
    Contributor,
    Proponent,
    Sponsor,
    ReleaseAuthority,
    TechnicalPoc,
}

impl From<&Element> for PocTypeType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Primary author" => PocTypeType::PrimaryAuthor,
            "Contributor" => PocTypeType::Contributor,
            "Proponent" => PocTypeType::Proponent,
            "Sponsor" => PocTypeType::Sponsor,
            "Release authority" => PocTypeType::ReleaseAuthority,
            "Technical POC" => PocTypeType::TechnicalPoc,
            _ => panic!("Unknown 'modelIdentification -> poc -> pocType': {}", text),
        }
    }
}

pub struct IdReferenceType {
    pub reference_type: String,
    pub identification: String,
}

impl From<&Element> for IdReferenceType {
    fn from(e: &Element) -> Self {
        Self {
            reference_type: if let Some(e) = e.get_child("type") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> reference -> type' found")
            },
            identification: if let Some(e) = e.get_child("identification") {
                get_element_text(e)
            } else {
                panic!("No 'modelIdentification -> reference -> identification' found")
            },
        }
    }
}

pub struct GlyphType {
    pub href: Option<String>,
    pub glyph_type: GlyphTypeType,
    pub height: u16,
    pub width: u16,
    pub alt: String,
}

impl From<&Element> for GlyphType {
    fn from(e: &Element) -> Self {
        Self {
            href: get_text_of_child_element(e, "href"),
            glyph_type: get_attribute_as_type_or_panic(
                e,
                "type",
                "No 'modelIdentification -> glyph[type]' found",
            ),
            height: get_text_of_attribute_or_panic(
                e,
                "height",
                "No 'modelIdentification -> glyph[height]' found",
            )
            .parse()
            .unwrap(),
            width: get_text_of_attribute_or_panic(
                e,
                "width",
                "No 'modelIdentification -> glyph[width]' found",
            )
            .parse()
            .unwrap(),
            alt: get_text_of_attribute_or_panic(
                e,
                "alt",
                "No 'modelIdentification -> glyph[alt]' found",
            ),
        }
    }
}

pub enum GlyphTypeType {
    Bitmap,
    Jpg,
    Gif,
    Png,
    Tiff,
}

impl From<&String> for GlyphTypeType {
    fn from(attribute: &String) -> Self {
        match attribute.to_uppercase().as_str() {
            "BITMAP" => GlyphTypeType::Bitmap,
            "JPG" => GlyphTypeType::Jpg,
            "GIF" => GlyphTypeType::Gif,
            "PNG" => GlyphTypeType::Png,
            "TIFF" => GlyphTypeType::Tiff,
            _ => panic!(
                "Unknown 'modelIdentification -> glyph -> type': {}",
                attribute
            ),
        }
    }
}

pub struct ServiceUtiliizationType {
    pub connect: ServiceInfoType,
    pub disconnect: ServiceInfoType,
    // ... and the rest
}

impl From<&Element> for ServiceUtiliizationType {
    fn from(e: &Element) -> Self {
        Self {
            connect: if let Some(e) = e.get_child("connect") {
                ServiceInfoType::from(e)
            } else {
                panic!("No 'objectModel -> serviceUtilization -> connect' found")
            },
            disconnect: if let Some(e) = e.get_child("disconnect") {
                ServiceInfoType::from(e)
            } else {
                panic!("No 'objectModel -> serviceUtilization -> connect' found")
            },
        }
    }
}

pub struct ServiceInfoType {
    pub section: String,
    pub is_callback: bool,
    pub is_used: bool,
}

impl From<&Element> for ServiceInfoType {
    fn from(e: &Element) -> Self {
        Self {
            section: if let Some(a) = e.attributes.get("section") {
                a.clone()
            } else {
                panic!("No 'objectModel -> serviceUtilization -> section' found")
            },
            is_callback: if let Some(a) = e.attributes.get("isCallback") {
                a.parse().unwrap()
            } else {
                panic!("No 'objectModel -> serviceUtilization -> isCallback' found")
            },
            is_used: if let Some(a) = e.attributes.get("isUsed") {
                a.parse().unwrap()
            } else {
                panic!("No 'objectModel -> serviceUtilization -> isUsed' found")
            },
        }
    }
}

pub struct ObjectsType {
    pub root_object_class: ObjectClassType,
}

impl From<&Element> for ObjectsType {
    fn from(e: &Element) -> Self {
        Self {
            root_object_class: if let Some(e) = e.get_child("objectClass") {
                ObjectClassType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass' found")
            },
        }
    }
}

pub struct ObjectClassType {
    pub name: String,
    pub sharing: SharingType,
    pub semantics: Option<String>,
    pub attributes: Option<Vec<AttributeType>>,
    pub object_classes: Option<Vec<ObjectClassType>>,
}

impl From<&Element> for ObjectClassType {
    fn from(e: &Element) -> Self {
        Self {
            name: if let Some(e) = e.get_child("name") {
                get_element_text(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> name' found")
            },
            sharing: if let Some(e) = e.get_child("sharing") {
                SharingType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> sharing' found")
            },
            semantics: if let Some(e) = e.get_child("semantics") {
                Some(get_element_text(e))
            } else {
                None
            },
            attributes: {
                let attributes = get_text_of_child_elements_as_type(e, "attribute");
                if attributes.is_empty() {
                    None
                } else {
                    Some(attributes)
                }
            },
            object_classes: {
                let object_classes = get_text_of_child_elements_as_type(e, "objectClasses");
                if object_classes.is_empty() {
                    None
                } else {
                    Some(object_classes)
                }
            },
        }
    }
}

pub enum SharingType {
    Publish,
    Subscribe,
    PublishSubscribe,
    Neither,
}

impl From<&Element> for SharingType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Publish" => SharingType::Publish,
            "Subscribe" => SharingType::Subscribe,
            "PublishSubscribe" => SharingType::PublishSubscribe,
            "Neither" => SharingType::Neither,
            _ => panic!("Unknown sharing type: {}", text),
        }
    }
}

pub struct AttributeType {
    pub name: String,
    pub data_type: ReferenceType,
    pub update_type: UpdateType,
    pub update_condition: Option<String>,
    pub onwership: OwnershipType,
    pub sharing: SharingType,
    pub dimensions: Option<Vec<ReferenceType>>,
    pub transportation: ReferenceType,
    pub order: OrderType,
    pub semantics: Option<String>,
}

impl From<&Element> for AttributeType {
    fn from(e: &Element) -> Self {
        Self {
            name: if let Some(e) = e.get_child("name") {
                get_element_text(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> name' found")
            },
            data_type: if let Some(e) = e.get_child("dataType") {
                ReferenceType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> dataType' found")
            },
            update_type: if let Some(e) = e.get_child("updateType") {
                UpdateType::from(e)
            } else {
                panic!(
                    "No 'objectModel -> objects -> objectClass -> attribute -> updateType' found"
                )
            },
            update_condition: if let Some(e) = e.get_child("updateCondition") {
                Some(get_element_text(e))
            } else {
                None
            },
            onwership: if let Some(e) = e.get_child("ownership") {
                OwnershipType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> ownership' found")
            },
            sharing: if let Some(e) = e.get_child("sharing") {
                SharingType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> sharing' found")
            },
            dimensions: if let Some(e) = e.get_child("dimensions") {
                Some(get_text_of_child_elements_as_type(e, "dimension"))
            } else {
                None
            },
            transportation: if let Some(e) = e.get_child("transportation") {
                ReferenceType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> transportation' found")
            },
            order: if let Some(e) = e.get_child("order") {
                OrderType::from(e)
            } else {
                panic!("No 'objectModel -> objects -> objectClass -> attribute -> order' found")
            },
            semantics: if let Some(e) = e.get_child("semantics") {
                Some(get_element_text(e))
            } else {
                None
            },
        }
    }
}

pub struct ReferenceType {
    pub value: String,
}

impl From<&Element> for ReferenceType {
    fn from(e: &Element) -> Self {
        Self {
            value: get_element_text(e),
        }
    }
}

pub enum UpdateType {
    Static,
    Periodic,
    Conditional,
    Na,
}

impl From<&Element> for UpdateType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Static" => UpdateType::Static,
            "Periodic" => UpdateType::Periodic,
            "Conditional" => UpdateType::Conditional,
            "NA" => UpdateType::Na,
            _ => panic!("Unknown UpdateType: {}", text),
        }
    }
}

pub enum OwnershipType {
    Divest,
    Acquire,
    DivestAcquire,
    NoTransfer,
}

impl From<&Element> for OwnershipType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Divest" => OwnershipType::Divest,
            "Acquire" => OwnershipType::Acquire,
            "DivestAcquire" => OwnershipType::DivestAcquire,
            "NoTransfer" => OwnershipType::NoTransfer,
            _ => panic!("Unknown OwnershipType: {}", text),
        }
    }
}

pub enum OrderType {
    Receive,
    TimeStamp,
}

impl From<&Element> for OrderType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Receive" => OrderType::Receive,
            "TimeStamp" => OrderType::TimeStamp,
            _ => panic!("Unknown OrderType: {}", text),
        }
    }
}

pub struct InteractionsType {
    pub interactions: InteractionClassType,
}

impl From<&Element> for InteractionsType {
    fn from(e: &Element) -> Self {
        Self {
            interactions: get_child_element_as_type_or_panic(
                e,
                "interactionClass",
                "No 'objectModel -> interactions -> interactionClass' found",
            ),
        }
    }
}

pub struct InteractionClassType {
    pub name: String,
    pub sharing: SharingType,
    pub dimensions: Option<Vec<ReferenceType>>,
    pub transportation: ReferenceType,
    pub order: OrderType,
    pub semantics: Option<String>,
    pub parameters: Option<Vec<ParameterType>>,
    pub interaction_classes: Option<Vec<InteractionClassType>>,
}

impl From<&Element> for InteractionClassType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'objectModel -> interactions -> interactionClass -> name' found",
            ),
            sharing: get_child_element_as_type_or_panic(
                e,
                "sharing",
                "No 'objectModel -> interactions -> interactionClass -> sharing' found",
            ),
            dimensions: {
                let dimensions = get_text_of_child_elements_as_type(e, "dimension");
                if dimensions.is_empty() {
                    None
                } else {
                    Some(dimensions)
                }
            },
            transportation: get_child_element_as_type_or_panic(
                e,
                "transportation",
                "No 'objectModel -> interactions -> interactionClass -> transportation' found",
            ),
            order: get_child_element_as_type_or_panic(
                e,
                "order",
                "No 'objectModel -> interactions -> interactionClass -> order",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
            parameters: {
                let parameters = get_text_of_child_elements_as_type(e, "parameter");
                if parameters.is_empty() {
                    None
                } else {
                    Some(parameters)
                }
            },
            interaction_classes: {
                let interaction_classes = get_text_of_child_elements_as_type(e, "interactionClass");
                if interaction_classes.is_empty() {
                    None
                } else {
                    Some(interaction_classes)
                }
            },
        }
    }
}

pub struct ParameterType {
    pub name: String,
    pub data_type: ReferenceType,
    pub semantics: Option<String>,
}

impl From<&Element> for ParameterType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'interactions -> interactionClass -> parameter -> name' found",
            ),
            data_type: get_child_element_as_type_or_panic(
                e,
                "dataType",
                "No 'interactions -> interactionClass -> parameter -> dataType' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct DimensionsType {}
impl From<&Element> for DimensionsType {
    fn from(_e: &Element) -> Self {
        Self {}
    }
}

pub struct TimeType {
    pub time_stamp: Option<TimeTypeType>,
    pub lookahead: Option<TimeTypeType>,
}

impl From<&Element> for TimeType {
    fn from(e: &Element) -> Self {
        Self {
            time_stamp: get_child_element_as_type(e, "timeStamp"),
            lookahead: get_child_element_as_type(e, "lookahead"),
        }
    }
}

pub struct TimeTypeType {
    pub data_type: ReferenceType,
    pub semantics: Option<String>,
}

impl From<&Element> for TimeTypeType {
    fn from(e: &Element) -> Self {
        Self {
            data_type: get_child_element_as_type_or_panic(
                e,
                "dataType",
                "No 'time type -> dataType' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct TagsType {
    pub update_reflect_tag: Option<TagType>,
    pub send_receive_tag: Option<TagType>,
    pub delete_remove_tag: Option<TagType>,
    pub divestiture_request_tag: Option<TagType>,
    pub divestiture_completion_tag: Option<TagType>,
    pub acquisition_request_tag: Option<TagType>,
    pub request_update_tag: Option<TagType>,
}

impl From<&Element> for TagsType {
    fn from(e: &Element) -> Self {
        Self {
            update_reflect_tag: get_child_element_as_type(e, "update_reflect_tag"),
            send_receive_tag: get_child_element_as_type(e, "send_receive_tag"),
            delete_remove_tag: get_child_element_as_type(e, "delete_remove_tag"),
            divestiture_request_tag: get_child_element_as_type(e, "divestiture_request_tag"),
            divestiture_completion_tag: get_child_element_as_type(e, "divestiture_completion_tag"),
            acquisition_request_tag: get_child_element_as_type(e, "acquisition_request_tag"),
            request_update_tag: get_child_element_as_type(e, "request_update_tag"),
        }
    }
}

pub struct TagType {
    pub data_type: ReferenceType,
    pub semantics: Option<String>,
}

impl From<&Element> for TagType {
    fn from(e: &Element) -> Self {
        Self {
            data_type: get_child_element_as_type_or_panic(
                e,
                "dataType",
                "No 'tag type -> dataType' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct SynchronizationsType {
    pub synchronization_points: Option<Vec<SynchronizationPointType>>,
}

impl From<&Element> for SynchronizationsType {
    fn from(e: &Element) -> Self {
        Self {
            synchronization_points: {
                let synchronization_points =
                    get_text_of_child_elements_as_type(e, "synchronizationPoint");
                if synchronization_points.is_empty() {
                    None
                } else {
                    Some(synchronization_points)
                }
            },
        }
    }
}

pub struct SynchronizationPointType {
    pub label: String,
    pub data_type: Option<ReferenceType>,
    pub capability: CapabilityType,
    pub semantics: Option<String>,
}

impl From<&Element> for SynchronizationPointType {
    fn from(e: &Element) -> Self {
        Self {
            label: get_text_of_child_element_or_panic(
                e,
                "label",
                "No 'synchronizationPoint -> label' found",
            ),
            data_type: get_child_element_as_type(e, "dataType"),
            capability: get_child_element_as_type_or_panic(
                e,
                "capability",
                "No 'synchronizationPoint -> capability' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub enum CapabilityType {
    Register,
    Achieve,
    RegisterAchieve,
    NoSynch,
    Na,
}

impl From<&Element> for CapabilityType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Register" => CapabilityType::Register,
            "Achieve" => CapabilityType::Achieve,
            "RegisterAchieve" => CapabilityType::RegisterAchieve,
            "NoSynch" => CapabilityType::NoSynch,
            "NA" => CapabilityType::Na,
            _ => panic!("Unknown capability: {}", text),
        }
    }
}

pub struct TransportationsType {
    pub transportations: Option<Vec<TransportationType>>,
}

impl From<&Element> for TransportationsType {
    fn from(e: &Element) -> Self {
        Self {
            transportations: {
                let transportations = get_text_of_child_elements_as_type(e, "transportation");
                if transportations.is_empty() {
                    None
                } else {
                    Some(transportations)
                }
            },
        }
    }
}

pub struct TransportationType {
    pub name: String,
    pub reliable: ReliableType,
    pub semantics: Option<String>,
}

impl From<&Element> for TransportationType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'transportation -> name' found",
            ),
            reliable: get_child_element_as_type_or_panic(
                e,
                "reliable",
                "No 'transportation -> reliable' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub enum ReliableType {
    Yes,
    No,
}

impl From<&Element> for ReliableType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Yes" => ReliableType::Yes,
            "No" => ReliableType::No,
            _ => panic!("Unexpected reliable type: {}", text),
        }
    }
}

pub struct SwitchesType {
    pub auto_provide: SwitchType,
    pub convey_region_designator_sets: SwitchType,
    pub convey_producing_federate: SwitchType,
    pub attribute_scope_advisory: SwitchType,
    pub attribute_relevance_advisory: SwitchType,
    pub object_class_relevance_advisory: SwitchType,
    pub interaction_relevance_advisory: SwitchType,
    pub service_reporting: SwitchType,
    pub exception_reporting: SwitchType,
    pub delay_subscription_evaluation: SwitchType,
    pub automatic_resign_action: ResignSwitchType,
}

impl From<&Element> for SwitchesType {
    fn from(e: &Element) -> Self {
        Self {
            auto_provide: get_attribute_as_type_or_panic(
                e,
                "auto_provide",
                "No 'switch -> auto_provide' found",
            ),
            convey_region_designator_sets: get_attribute_as_type_or_panic(
                e,
                "convey_region_designator_sets",
                "No 'switch -> convey_region_designator_sets' found",
            ),
            convey_producing_federate: get_attribute_as_type_or_panic(
                e,
                "convey_producing_federate",
                "No 'switch -> convey_producing_federate' found",
            ),
            attribute_scope_advisory: get_attribute_as_type_or_panic(
                e,
                "attribute_scope_advisory",
                "No 'switch -> attribute_scope_advisory' found",
            ),
            attribute_relevance_advisory: get_attribute_as_type_or_panic(
                e,
                "attribute_relevance_advisory",
                "No 'switch -> attribute_relevance_advisory' found",
            ),
            object_class_relevance_advisory: get_attribute_as_type_or_panic(
                e,
                "object_class_relevance_advisory",
                "No 'switch -> object_class_relevance_advisory' found",
            ),
            interaction_relevance_advisory: get_attribute_as_type_or_panic(
                e,
                "interaction_relevance_advisory",
                "No 'switch -> interaction_relevance_advisory' found",
            ),
            service_reporting: get_attribute_as_type_or_panic(
                e,
                "service_reporting",
                "No 'switch -> service_reporting' found",
            ),
            exception_reporting: get_attribute_as_type_or_panic(
                e,
                "exception_reporting",
                "No 'switch -> exception_reporting' found",
            ),
            delay_subscription_evaluation: get_attribute_as_type_or_panic(
                e,
                "delay_subscription_evaluation",
                "No 'switch -> delay_subscription_evaluation' found",
            ),
            automatic_resign_action: get_attribute_as_type_or_panic(
                e,
                "automatic_resign_action",
                "No 'switch -> automatic_resign_action' found",
            ),
        }
    }
}

pub struct SwitchType {
    pub is_enabled: bool,
}

impl From<&String> for SwitchType {
    fn from(attribute: &String) -> Self {
        Self {
            is_enabled: attribute.parse().unwrap(),
        }
    }
}

pub enum ResignSwitchType {
    UnconditionallyDivestAttributes,
    DeleteObjects,
    CancelPendingOwnershipAcquisitions,
    DeleteObjectsThenDivest,
    CancelThenDeleteThenDivest,
    NoAction,
}

impl From<&String> for ResignSwitchType {
    fn from(attribute: &String) -> Self {
        match attribute.as_str() {
            "UnconditionallyDivestAttributes" => ResignSwitchType::UnconditionallyDivestAttributes,
            "DeleteObjects" => ResignSwitchType::DeleteObjects,
            "CancelPendingOwnershipAcquisitions" => {
                ResignSwitchType::CancelPendingOwnershipAcquisitions
            }
            "DeleteObjectsThenDivest" => ResignSwitchType::DeleteObjectsThenDivest,
            "CancelThenDeleteThenDivest" => ResignSwitchType::CancelThenDeleteThenDivest,
            "NoAction" => ResignSwitchType::NoAction,
            _ => panic!("Unknown resign switch type: {}", attribute),
        }
    }
}

pub struct UpdateRatesType {
    pub update_rates: Option<Vec<UpdateRateType>>,
}

impl From<&Element> for UpdateRatesType {
    fn from(e: &Element) -> Self {
        Self {
            update_rates: {
                let update_rates = get_text_of_child_elements_as_type(e, "updateRate");
                if update_rates.is_empty() {
                    None
                } else {
                    Some(update_rates)
                }
            },
        }
    }
}

pub struct UpdateRateType {
    pub name: String,
    pub rate: RateType,
    pub semantics: Option<String>,
}

impl From<&Element> for UpdateRateType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(e, "name", "No 'updateRate -> name' found"),
            rate: get_child_element_as_type_or_panic(e, "rate", "No 'updateRate -> rate' found"),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct RateType {
    pub value: String,
}

impl From<&Element> for RateType {
    fn from(e: &Element) -> Self {
        Self {
            value: get_element_text(e),
        }
    }
}

pub struct DataTypesType {
    pub basic_data_representations: BasicDataRepresentationsType,
    pub simple_data_types: SimpleDataTypesType,
    pub enumerated_data_types: EnumeratedDataTypesType,
    pub array_data_types: ArrayDataTypesType,
    pub fixed_record_data_types: FixedRecordDataTypesType,
    pub variand_record_data_types: VariantRecordDataTypesType,
}

impl From<&Element> for DataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            basic_data_representations: get_child_element_as_type_or_panic(
                e,
                "basicDataRepresentations",
                "No 'dataTypes -> basicDataRepresentations' found",
            ),
            simple_data_types: get_child_element_as_type_or_panic(
                e,
                "simpleDataTypes",
                "No 'dataTypes -> simpleDataTypes' found",
            ),
            enumerated_data_types: get_child_element_as_type_or_panic(
                e,
                "enumeratedDataTypes",
                "No 'dataTypes -> enumeratedDataTypes' found",
            ),
            array_data_types: get_child_element_as_type_or_panic(
                e,
                "arrayDataTypes",
                "No 'dataTypes -> arrayDataTypes' found",
            ),
            fixed_record_data_types: get_child_element_as_type_or_panic(
                e,
                "fixedRecordDataTypes",
                "No 'dataTypes -> fixedRecordDataTypes' found",
            ),
            variand_record_data_types: get_child_element_as_type_or_panic(
                e,
                "variantRecordDataTypes",
                "No 'dataTypes -> variantRecordDataTypes' found",
            ),
        }
    }
}

pub struct BasicDataRepresentationsType {
    pub basic_datas: Option<Vec<BasicDataType>>,
}

impl From<&Element> for BasicDataRepresentationsType {
    fn from(e: &Element) -> Self {
        Self {
            basic_datas: {
                let basic_datas = get_text_of_child_elements_as_type(e, "basicData");
                if basic_datas.is_empty() {
                    None
                } else {
                    Some(basic_datas)
                }
            },
        }
    }
}

pub struct BasicDataType {
    pub name: String,
    pub size: SizeType,
    pub interpretation: String,
    pub endian: EndianType,
    pub encoding: String,
}

impl From<&Element> for BasicDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(e, "name", "No 'basicData -> name' found"),
            size: get_child_element_as_type_or_panic(e, "size", "No 'basicData -> size' found"),
            interpretation: get_text_of_child_element_or_panic(
                e,
                "interpretation",
                "No 'basicData -> interpretation' found",
            ),
            endian: get_child_element_as_type_or_panic(
                e,
                "endian",
                "No 'basicData -> endian' found",
            ),
            encoding: get_text_of_child_element_or_panic(
                e,
                "encoding",
                "No 'basicData -> encoding' found",
            ),
        }
    }
}

pub struct SizeType {
    pub size: u8,
}

impl From<&Element> for SizeType {
    fn from(e: &Element) -> Self {
        Self {
            size: get_element_text(e).parse().unwrap(),
        }
    }
}

pub enum EndianType {
    Big,
    Little,
}

impl From<&Element> for EndianType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "Big" => EndianType::Big,
            "Little" => EndianType::Little,
            _ => panic!("Unknown endian type: {}", text),
        }
    }
}

pub struct SimpleDataTypesType {
    pub simple_datas: Option<Vec<SimpleDataType>>,
}

impl From<&Element> for SimpleDataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            simple_datas: {
                let simple_datas = get_text_of_child_elements_as_type(e, "simpleData");
                if simple_datas.is_empty() {
                    None
                } else {
                    Some(simple_datas)
                }
            },
        }
    }
}

pub struct SimpleDataType {
    pub name: String,
    pub representation: ReferenceType,
    pub units: Option<String>,
    pub resolution: Option<String>,
    pub accuracy: Option<String>,
    pub semantics: Option<String>,
}

impl From<&Element> for SimpleDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(e, "name", "No 'simpleData -> name' found"),
            representation: get_child_element_as_type_or_panic(
                e,
                "representation",
                "No 'simpleData -> representation' found",
            ),
            units: get_text_of_child_element(e, "units"),
            resolution: get_text_of_child_element(e, "resolution"),
            accuracy: get_text_of_child_element(e, "accuracy"),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct EnumeratedDataTypesType {
    pub enumerated_datas: Option<Vec<EnumeratedDataType>>,
}

impl From<&Element> for EnumeratedDataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            enumerated_datas: {
                let enumerated_datas = get_text_of_child_elements_as_type(e, "enumeratedData");
                if enumerated_datas.is_empty() {
                    None
                } else {
                    Some(enumerated_datas)
                }
            },
        }
    }
}

pub struct EnumeratedDataType {
    pub name: String,
    pub representation: ReferenceType,
    pub semantics: Option<String>,
    pub enumerators: Option<Vec<EnumeratorType>>,
}

impl From<&Element> for EnumeratedDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'enumeratedData -> name' found",
            ),
            representation: get_child_element_as_type_or_panic(
                e,
                "representation",
                "No 'enumeratedData -> representation' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
            enumerators: {
                let enumerators = get_text_of_child_elements_as_type(e, "enumerator");
                if enumerators.is_empty() {
                    None
                } else {
                    Some(enumerators)
                }
            },
        }
    }
}

pub struct EnumeratorType {
    pub name: String,
    pub value: Vec<String>,
}

impl From<&Element> for EnumeratorType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'enumeratedData -> enumerator -> name' found",
            ),
            value: get_text_of_child_elements(e, "value"),
        }
    }
}

pub struct ArrayDataTypesType {
    pub array_datas: Option<Vec<ArrayDataType>>,
}

impl From<&Element> for ArrayDataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            array_datas: {
                let array_datas = get_text_of_child_elements_as_type(e, "arrayData");
                if array_datas.is_empty() {
                    None
                } else {
                    Some(array_datas)
                }
            },
        }
    }
}

pub struct ArrayDataType {
    pub name: String,
    pub data_type: ReferenceType,
    pub cardinality: String, // needs to match a pattern
    pub encoding: ArrayDataTypeEncodingType,
    pub semantics: Option<String>,
}

impl From<&Element> for ArrayDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(e, "name", "No 'arrayData -> name' found"),
            data_type: get_child_element_as_type_or_panic(
                e,
                "representation",
                "No 'arrayData -> representation' found",
            ),
            cardinality: get_text_of_child_element_or_panic(
                e,
                "cardinality",
                "No 'arrayData -> cardinality' found",
            ), // needs to match a pattern
            encoding: get_child_element_as_type_or_panic(
                e,
                "encoding",
                "No 'arrayData -> encoding' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub enum ArrayDataTypeEncodingType {
    HlaFixedArray,
    HlaVariableArray,
}

impl From<&Element> for ArrayDataTypeEncodingType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "HLAfixedArray" => ArrayDataTypeEncodingType::HlaFixedArray,
            "HLAvariableArray" => ArrayDataTypeEncodingType::HlaVariableArray,
            _ => panic!("Unknown array data type encoding: {}", text),
        }
    }
}

pub struct FixedRecordDataTypesType {
    pub fixed_record_datas: Option<Vec<FixedRecordDataType>>,
}

impl From<&Element> for FixedRecordDataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            fixed_record_datas: {
                let fixed_record_datas = get_text_of_child_elements_as_type(e, "fixedRecordData");
                if fixed_record_datas.is_empty() {
                    None
                } else {
                    Some(fixed_record_datas)
                }
            },
        }
    }
}

pub struct FixedRecordDataType {
    pub name: String,
    pub encoding: FixedRecordEncodingType,
    pub semantics: Option<String>,
    pub fields: Option<Vec<FieldType>>,
}

impl From<&Element> for FixedRecordDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'fixedRecordData -> name' found",
            ),
            encoding: get_child_element_as_type_or_panic(
                e,
                "encoding",
                "No 'fixedRecordData -> encoding' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
            fields: {
                let fields = get_text_of_child_elements_as_type(e, "field");
                if fields.is_empty() {
                    None
                } else {
                    Some(fields)
                }
            },
        }
    }
}

pub enum FixedRecordEncodingType {
    HlaFixedRecord,
}

impl From<&Element> for FixedRecordEncodingType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "HLAfixedRecord" => FixedRecordEncodingType::HlaFixedRecord,
            _ => panic!("Unknown fixed record encoding: {}", text),
        }
    }
}

pub struct FieldType {
    pub name: String,
    pub data_type: ReferenceType,
    pub semantics: Option<String>,
}

impl From<&Element> for FieldType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'fixedRecrodData -> field -> name' found",
            ),
            data_type: get_child_element_as_type_or_panic(
                e,
                "dataType",
                "No 'fixedRecordData -> field -> dataType' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct VariantRecordDataTypesType {
    pub variant_record_datas: Option<Vec<VariantRecordDataType>>,
}

impl From<&Element> for VariantRecordDataTypesType {
    fn from(e: &Element) -> Self {
        Self {
            variant_record_datas: {
                let variant_record_datas =
                    get_text_of_child_elements_as_type(e, "variantRecordData");
                if variant_record_datas.is_empty() {
                    None
                } else {
                    Some(variant_record_datas)
                }
            },
        }
    }
}

pub struct VariantRecordDataType {
    pub name: String,
    pub discriminant: String,
    pub data_type: ReferenceType,
    pub alternatives: Option<Vec<AlternativeType>>,
    pub encoding: VariantRecordEncodingType,
    pub semantics: Option<String>,
}

impl From<&Element> for VariantRecordDataType {
    fn from(e: &Element) -> Self {
        Self {
            name: get_text_of_child_element_or_panic(
                e,
                "name",
                "No 'variantRecordData -> name' found",
            ),
            discriminant: get_text_of_child_element_or_panic(
                e,
                "discriminant",
                "No 'variantRecordData -> discriminant' found",
            ),
            data_type: get_child_element_as_type_or_panic(
                e,
                "dataType",
                "No 'variantRecordData -> dataType' found",
            ),
            alternatives: {
                let alternatives = get_text_of_child_elements_as_type(e, "alternative");
                if alternatives.is_empty() {
                    None
                } else {
                    Some(alternatives)
                }
            },
            encoding: get_child_element_as_type_or_panic(
                e,
                "encoding",
                "No 'variantRecordData -> encoding' found",
            ),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub struct AlternativeType {
    pub enumerator: String,
    pub name: Option<String>,
    pub data_type: Option<ReferenceType>,
    pub semantics: Option<String>,
}

impl From<&Element> for AlternativeType {
    fn from(e: &Element) -> Self {
        Self {
            enumerator: get_text_of_child_element_or_panic(
                e,
                "enumerator",
                "No 'variantRecordData -> alternative -> enumerator' found",
            ),
            name: get_text_of_child_element(e, "name"),
            data_type: get_child_element_as_type(e, "dataType"),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub enum VariantRecordEncodingType {
    HlaVariantRecord,
}

impl From<&Element> for VariantRecordEncodingType {
    fn from(e: &Element) -> Self {
        let text = get_element_text(e);
        match text.as_str() {
            "HLAvariantRecord" => VariantRecordEncodingType::HlaVariantRecord,
            _ => panic!("Unknown variant record encoding: {}", text),
        }
    }
}

pub struct NotesType {
    pub notes: Option<Vec<NoteType>>,
}

impl From<&Element> for NotesType {
    fn from(e: &Element) -> Self {
        Self {
            notes: {
                let notes = get_text_of_child_elements_as_type(e, "note");
                if notes.is_empty() {
                    None
                } else {
                    Some(notes)
                }
            },
        }
    }
}

pub struct NoteType {
    pub label: String,
    pub semantics: Option<String>,
}

impl From<&Element> for NoteType {
    fn from(e: &Element) -> Self {
        Self {
            label: get_text_of_child_element_or_panic(e, "label", "No 'note -> label' found"),
            semantics: get_text_of_child_element(e, "semantics"),
        }
    }
}

pub fn parse<R: Read>(r: R) -> Result<(), ParseError> {
    let fom_as_xml = Element::parse(r)?;
    let fom = ObjectModelType::from(&fom_as_xml);

    println!("{:?}", fom.model_identification.name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_element_text() {
        let mut el = Element::new("root");
        let expected_text = String::from("text");
        el.children.push(XMLNode::Text(expected_text.clone()));
        let extracted_text = get_element_text(&el);
        assert_eq!(extracted_text, expected_text);
    }

    #[test]
    fn test_get_text_of_child_element() {
        let mut root = Element::new("root");
        let mut child = Element::new("child");
        let expected_text = String::from("text");
        child.children.push(XMLNode::Text(expected_text.clone()));
        root.children.push(XMLNode::Element(child));

        let extracted_text = get_text_of_child_element(&root, "child");
        assert_eq!(Some(expected_text), extracted_text);
        assert_eq!(None, get_text_of_child_element(&root, "non-child"));
    }

    #[test]
    #[should_panic]
    fn test_get_text_of_child_element_or_panic() {
        let mut root = Element::new("root");
        let mut child = Element::new("child");
        let expected_text = String::from("text");
        child.children.push(XMLNode::Text(expected_text.clone()));
        root.children.push(XMLNode::Element(child));

        let _ = get_text_of_child_element_or_panic(&root, "non-child", "panic message");
    }
}
