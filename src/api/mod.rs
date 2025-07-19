use serde::Deserialize;

use reqwest::blocking::Client;
use serde::Serialize;

const ENDPOINT: &str = "https://api.softcatala.org/corrector-dev/v2/check";

#[derive(Debug)]
pub struct APICorrector {}

impl APICorrector {
    pub fn run_query(text: &str) -> Result<APICorrectorResponse, reqwest::Error> {
        let http_client = Client::new();
        let request = RequestBody::default(text);
        let http_result = http_client.post(ENDPOINT).form(&request).send()?;
        let response_json: APICorrectorResponse = http_result.json()?;
        Ok(response_json)
    }
}

// REQUEST
#[derive(Debug, Serialize)]
struct RequestBody<'a> {
    level: &'a str,
    language: &'a str,
    enabledRules: &'a str,
    disabledRules: &'a str,
    disabledCategories: &'a str,
    text: &'a str,
}

impl<'a> RequestBody<'a> {
    fn default(text: &'a str) -> RequestBody<'a> {
        RequestBody {
            level: "picky",
            language: "ca-ES",
            enabledRules: "CA_SIMPLEREPLACE_DIACRITICS_IEC,EVITA_EXCLAMACIO_INICIAL,PERCENT_SENSE_ESPAI",
            disabledRules: "ESPAI_FI,ADVERBIS_MENT,TOO_LONG_SENTENCE,WHITESPACE_RULE,DIACRITICS_TRADITIONAL_RULES,CA_SIMPLEREPLACE_DIACRITICS_TRADITIONAL,PERCENT_AMB_ESPAI,DOSPUNTS_EN_HORES,PUNT_EN_HORES",
            disabledCategories: "DIACRITICS_TRADITIONAL",
            text,
        }
    }
}

// RESPONSE
#[derive(Debug, Deserialize, Serialize)]
pub struct APICorrectorResponse {
    software: Software,
    warnings: Warnings,
    language: Language,
    matches: Vec<Match>,
    sentenceRanges: Vec<Vec<usize>>,
    extendedSentenceRanges: Vec<ExtendedSentenceRange>,
}

impl APICorrectorResponse {
    pub fn matches(&self) -> &[Match] {
        &self.matches
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Software {
    name: String,
    version: String,
    buildDate: String,
    apiVersion: u32,
    premium: bool,
    premiumHint: String,
    status: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Warnings {
    incompleteResults: bool,
}

#[derive(Debug, Deserialize, Serialize)]
struct Language {
    name: String,
    code: String,
    detectedLanguage: DetectedLanguage,
}

#[derive(Debug, Deserialize, Serialize)]
struct DetectedLanguage {
    name: String,
    code: String,
    confidence: f64,
    source: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Match {
    message: String,
    shortMessage: String,
    replacements: Vec<Replacement>,
    offset: usize,
    length: usize,
    context: Context,
    sentence: String,
    #[serde(rename = "type")]
    match_type: MatchType,
    rule: Rule,
    ignoreForIncompleteSentence: bool,
    contextForSureMatch: i32,
}

impl Match {
    pub fn replacements(&self) -> &[Replacement] {
        &self.replacements
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn length(&self) -> usize {
        self.length
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Replacement {
    value: String,
}

impl Replacement {
    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Context {
    text: String,
    offset: usize,
    length: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct MatchType {
    typeName: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Rule {
    id: String,
    description: String,
    issueType: String,
    category: Category,
}

#[derive(Debug, Deserialize, Serialize)]
struct Category {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExtendedSentenceRange {
    from: usize,
    to: usize,
    detectedLanguages: Vec<DetectedLanguageRange>,
}

#[derive(Debug, Deserialize, Serialize)]
struct DetectedLanguageRange {
    language: String,
    rate: f64,
}
