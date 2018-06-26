extern crate csv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

const LANGUAGE_DATA: &'static [u8] = include_bytes!("../assets/full-language-codes.csv");
#[derive(Debug, Deserialize)]
pub struct Language {
  #[serde(rename = "alpha3-b")]
  pub alpha3b: String,
  #[serde(rename = "alpha3-t")]
  pub alpha3t: String,
  pub alpha2: String,
  #[serde(rename = "English")]
  pub english: String,
  #[serde(rename = "French")]
  pub french: String,
}

lazy_static! {
  pub static ref LANGUAGES: Vec<Language> = {
    csv::Reader::from_reader(LANGUAGE_DATA)
      .deserialize()
      .map(|result| result.expect("Language data is corrupted"))
      .collect()
  };
}

pub fn languages() -> impl Iterator<Item = &'static Language> {
  LANGUAGES.iter()
}

const SCRIPT_DATA: &'static [u8] = include_bytes!("../assets/iso15924.csv");
#[derive(Debug, Deserialize)]
pub struct Script {
  #[serde(rename = "Code")]
  pub code: String,
  #[serde(rename = "N°")]
  pub number: String,
  #[serde(rename = "English Name")]
  pub english: String,
  #[serde(rename = "Nom français")]
  pub french: String,
  #[serde(rename = "PVA")]
  pub pva: String,
  #[serde(rename = "Unicode Version")]
  pub version: String,
  #[serde(rename = "Date")]
  pub date: String,
}
lazy_static! {
  pub static ref SCRIPTS: Vec<Script> = {
    csv::ReaderBuilder::new()
      .delimiter(b';')
      .from_reader(SCRIPT_DATA)
      .deserialize()
      .map(|result| result.expect("Script data is corrupted"))
      .collect()
  };
}

pub fn scripts() -> impl Iterator<Item = &'static Script> {
  SCRIPTS.iter()
}
