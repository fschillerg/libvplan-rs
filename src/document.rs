use serde_derive::*;

#[derive(Deserialize)]
pub struct Value {
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Deserialize)]
pub struct Vplan {
    #[serde(rename = "kopf")]
    pub header: Header,
    #[serde(rename = "freietage")]
    pub days_off: DaysOff,
    #[serde(rename = "haupt")]
    pub main: Option<Main>,
    #[serde(rename = "fuss")]
    pub footer: Option<Footer>
}

#[derive(Deserialize)]
pub struct Header {
    #[serde(rename = "datei")]
    pub file: Value,
    #[serde(rename = "titel")]
    pub title: Value,
    #[serde(rename = "schulname")]
    pub schoolname: Value,
    #[serde(rename = "datum")]
    pub date: Value,
    #[serde(rename = "kopfinfo")]
    pub info: HeaderInfo
}

#[derive(Deserialize)]
pub struct HeaderInfo {
    #[serde(rename = "abwesendk")]
    pub naforms: Option<Value>,
    #[serde(rename = "abwesendr")]
    pub narooms: Option<Value>,
    #[serde(rename = "aenderungk")]
    pub changed: Option<Value>
}

#[derive(Deserialize)]
pub struct DaysOff {
    #[serde(rename = "ft")]
    pub items: Vec<DayOff>
}

#[derive(Deserialize)]
pub struct DayOff {
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Deserialize)]
pub struct Main {
    #[serde(rename = "aktion")]
    pub items: Vec<Action>
}

#[derive(Deserialize)]
pub struct Action {
    #[serde(rename = "klasse")]
    pub form: ActionInner,
    #[serde(rename = "stunde")]
    pub lesson: ActionInner,
    #[serde(rename = "fach")]
    pub subject: ActionInner,
    #[serde(rename = "lehrer")]
    pub teacher: ActionInner,
    #[serde(rename = "raum")]
    pub room: ActionInner,
    pub info: ActionInner
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct ActionInner {
    fageaendert: Option<String>,
    legeaendert: Option<String>,
    rageaendert: Option<String>,
    #[serde(rename = "$value")]
    pub value: Option<String>
}

#[derive(Deserialize)]
pub struct Footer {
    #[serde(rename = "fusszeile")]
    pub items: Vec<FooterLine>
}

#[derive(Deserialize)]
pub struct FooterLine {
    #[serde(rename = "fussinfo")]
    pub inner: Value
}
