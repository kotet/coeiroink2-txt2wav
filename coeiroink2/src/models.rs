#![allow(unused_qualifications)]

use validator::Validate;

use crate::models;
#[cfg(any(feature = "client", feature = "server"))]
use crate::header;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AccentPhrase {
    #[serde(rename = "moras")]
    pub moras: Vec<models::CoeirocoreVUtilMora>,

    #[serde(rename = "accent")]
    pub accent: i32,

    #[serde(rename = "pauseMora")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pause_mora: Option<models::CoeirocoreVUtilMora>,

    #[serde(rename = "isInterrogative")]
    pub is_interrogative: bool,

}


impl AccentPhrase {
    #[allow(clippy::new_without_default)]
    pub fn new(moras: Vec<models::CoeirocoreVUtilMora>, accent: i32, is_interrogative: bool, ) -> AccentPhrase {
        AccentPhrase {
            moras,
            accent,
            pause_mora: None,
            is_interrogative,
        }
    }
}

/// Converts the AccentPhrase value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AccentPhrase {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping moras in query parameter serialization


            Some("accent".to_string()),
            Some(self.accent.to_string()),

            // Skipping pauseMora in query parameter serialization


            Some("isInterrogative".to_string()),
            Some(self.is_interrogative.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AccentPhrase value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AccentPhrase {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub moras: Vec<Vec<models::CoeirocoreVUtilMora>>,
            pub accent: Vec<i32>,
            pub pause_mora: Vec<models::CoeirocoreVUtilMora>,
            pub is_interrogative: Vec<bool>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AccentPhrase".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "moras" => return std::result::Result::Err("Parsing a container in this style is not supported in AccentPhrase".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "accent" => intermediate_rep.accent.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pauseMora" => intermediate_rep.pause_mora.push(<models::CoeirocoreVUtilMora as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "isInterrogative" => intermediate_rep.is_interrogative.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AccentPhrase".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AccentPhrase {
            moras: intermediate_rep.moras.into_iter().next().ok_or_else(|| "moras missing in AccentPhrase".to_string())?,
            accent: intermediate_rep.accent.into_iter().next().ok_or_else(|| "accent missing in AccentPhrase".to_string())?,
            pause_mora: intermediate_rep.pause_mora.into_iter().next(),
            is_interrogative: intermediate_rep.is_interrogative.into_iter().next().ok_or_else(|| "isInterrogative missing in AccentPhrase".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AccentPhrase> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AccentPhrase>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AccentPhrase>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AccentPhrase - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AccentPhrase> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AccentPhrase as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AccentPhrase - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct AudioQuery {
    #[serde(rename = "accentPhrases")]
    pub accent_phrases: Vec<models::AccentPhrase>,

    #[serde(rename = "speedScale")]
    pub speed_scale: f64,

    #[serde(rename = "pitchScale")]
    pub pitch_scale: f64,

    #[serde(rename = "intonationScale")]
    pub intonation_scale: f64,

    #[serde(rename = "volumeScale")]
    pub volume_scale: f64,

    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f64,

    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f64,

    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: i32,

    #[serde(rename = "outputStereo")]
    pub output_stereo: bool,

    #[serde(rename = "kana")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kana: Option<String>,

}


impl AudioQuery {
    #[allow(clippy::new_without_default)]
    pub fn new(accent_phrases: Vec<models::AccentPhrase>, speed_scale: f64, pitch_scale: f64, intonation_scale: f64, volume_scale: f64, pre_phoneme_length: f64, post_phoneme_length: f64, output_sampling_rate: i32, output_stereo: bool, ) -> AudioQuery {
        AudioQuery {
            accent_phrases,
            speed_scale,
            pitch_scale,
            intonation_scale,
            volume_scale,
            pre_phoneme_length,
            post_phoneme_length,
            output_sampling_rate,
            output_stereo,
            kana: None,
        }
    }
}

/// Converts the AudioQuery value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for AudioQuery {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping accentPhrases in query parameter serialization


            Some("speedScale".to_string()),
            Some(self.speed_scale.to_string()),


            Some("pitchScale".to_string()),
            Some(self.pitch_scale.to_string()),


            Some("intonationScale".to_string()),
            Some(self.intonation_scale.to_string()),


            Some("volumeScale".to_string()),
            Some(self.volume_scale.to_string()),


            Some("prePhonemeLength".to_string()),
            Some(self.pre_phoneme_length.to_string()),


            Some("postPhonemeLength".to_string()),
            Some(self.post_phoneme_length.to_string()),


            Some("outputSamplingRate".to_string()),
            Some(self.output_sampling_rate.to_string()),


            Some("outputStereo".to_string()),
            Some(self.output_stereo.to_string()),


            self.kana.as_ref().map(|kana| {
                [
                    "kana".to_string(),
                    kana.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a AudioQuery value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for AudioQuery {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub accent_phrases: Vec<Vec<models::AccentPhrase>>,
            pub speed_scale: Vec<f64>,
            pub pitch_scale: Vec<f64>,
            pub intonation_scale: Vec<f64>,
            pub volume_scale: Vec<f64>,
            pub pre_phoneme_length: Vec<f64>,
            pub post_phoneme_length: Vec<f64>,
            pub output_sampling_rate: Vec<i32>,
            pub output_stereo: Vec<bool>,
            pub kana: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing AudioQuery".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "accentPhrases" => return std::result::Result::Err("Parsing a container in this style is not supported in AudioQuery".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "speedScale" => intermediate_rep.speed_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pitchScale" => intermediate_rep.pitch_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "intonationScale" => intermediate_rep.intonation_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volumeScale" => intermediate_rep.volume_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prePhonemeLength" => intermediate_rep.pre_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postPhonemeLength" => intermediate_rep.post_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputSamplingRate" => intermediate_rep.output_sampling_rate.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputStereo" => intermediate_rep.output_stereo.push(<bool as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "kana" => intermediate_rep.kana.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing AudioQuery".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(AudioQuery {
            accent_phrases: intermediate_rep.accent_phrases.into_iter().next().ok_or_else(|| "accentPhrases missing in AudioQuery".to_string())?,
            speed_scale: intermediate_rep.speed_scale.into_iter().next().ok_or_else(|| "speedScale missing in AudioQuery".to_string())?,
            pitch_scale: intermediate_rep.pitch_scale.into_iter().next().ok_or_else(|| "pitchScale missing in AudioQuery".to_string())?,
            intonation_scale: intermediate_rep.intonation_scale.into_iter().next().ok_or_else(|| "intonationScale missing in AudioQuery".to_string())?,
            volume_scale: intermediate_rep.volume_scale.into_iter().next().ok_or_else(|| "volumeScale missing in AudioQuery".to_string())?,
            pre_phoneme_length: intermediate_rep.pre_phoneme_length.into_iter().next().ok_or_else(|| "prePhonemeLength missing in AudioQuery".to_string())?,
            post_phoneme_length: intermediate_rep.post_phoneme_length.into_iter().next().ok_or_else(|| "postPhonemeLength missing in AudioQuery".to_string())?,
            output_sampling_rate: intermediate_rep.output_sampling_rate.into_iter().next().ok_or_else(|| "outputSamplingRate missing in AudioQuery".to_string())?,
            output_stereo: intermediate_rep.output_stereo.into_iter().next().ok_or_else(|| "outputStereo missing in AudioQuery".to_string())?,
            kana: intermediate_rep.kana.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<AudioQuery> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<AudioQuery>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<AudioQuery>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for AudioQuery - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<AudioQuery> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <AudioQuery as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into AudioQuery - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CoeirocoreMetaStyle {
    #[serde(rename = "styleName")]
    pub style_name: String,

    #[serde(rename = "styleId")]
    pub style_id: i32,

    #[serde(rename = "base64Icon")]
    pub base64_icon: String,

    #[serde(rename = "base64Portrait")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub base64_portrait: Option<String>,

}


impl CoeirocoreMetaStyle {
    #[allow(clippy::new_without_default)]
    pub fn new(style_name: String, style_id: i32, base64_icon: String, ) -> CoeirocoreMetaStyle {
        CoeirocoreMetaStyle {
            style_name,
            style_id,
            base64_icon,
            base64_portrait: None,
        }
    }
}

/// Converts the CoeirocoreMetaStyle value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CoeirocoreMetaStyle {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("styleName".to_string()),
            Some(self.style_name.to_string()),


            Some("styleId".to_string()),
            Some(self.style_id.to_string()),


            Some("base64Icon".to_string()),
            Some(self.base64_icon.to_string()),


            self.base64_portrait.as_ref().map(|base64_portrait| {
                [
                    "base64Portrait".to_string(),
                    base64_portrait.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CoeirocoreMetaStyle value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CoeirocoreMetaStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub style_name: Vec<String>,
            pub style_id: Vec<i32>,
            pub base64_icon: Vec<String>,
            pub base64_portrait: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CoeirocoreMetaStyle".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "styleName" => intermediate_rep.style_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "styleId" => intermediate_rep.style_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "base64Icon" => intermediate_rep.base64_icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "base64Portrait" => intermediate_rep.base64_portrait.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CoeirocoreMetaStyle".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CoeirocoreMetaStyle {
            style_name: intermediate_rep.style_name.into_iter().next().ok_or_else(|| "styleName missing in CoeirocoreMetaStyle".to_string())?,
            style_id: intermediate_rep.style_id.into_iter().next().ok_or_else(|| "styleId missing in CoeirocoreMetaStyle".to_string())?,
            base64_icon: intermediate_rep.base64_icon.into_iter().next().ok_or_else(|| "base64Icon missing in CoeirocoreMetaStyle".to_string())?,
            base64_portrait: intermediate_rep.base64_portrait.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CoeirocoreMetaStyle> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CoeirocoreMetaStyle>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CoeirocoreMetaStyle>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CoeirocoreMetaStyle - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CoeirocoreMetaStyle> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CoeirocoreMetaStyle as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CoeirocoreMetaStyle - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CoeirocoreModelStyle {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "id")]
    pub id: i32,

}


impl CoeirocoreModelStyle {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, id: i32, ) -> CoeirocoreModelStyle {
        CoeirocoreModelStyle {
            name,
            id,
        }
    }
}

/// Converts the CoeirocoreModelStyle value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CoeirocoreModelStyle {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("id".to_string()),
            Some(self.id.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CoeirocoreModelStyle value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CoeirocoreModelStyle {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub id: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CoeirocoreModelStyle".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CoeirocoreModelStyle".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CoeirocoreModelStyle {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in CoeirocoreModelStyle".to_string())?,
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in CoeirocoreModelStyle".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CoeirocoreModelStyle> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CoeirocoreModelStyle>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CoeirocoreModelStyle>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CoeirocoreModelStyle - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CoeirocoreModelStyle> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CoeirocoreModelStyle as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CoeirocoreModelStyle - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CoeirocoreMoraMora {
    #[serde(rename = "phoneme")]
    pub phoneme: String,

    #[serde(rename = "hira")]
    pub hira: String,

    #[serde(rename = "accent")]
    pub accent: i32,

}


impl CoeirocoreMoraMora {
    #[allow(clippy::new_without_default)]
    pub fn new(phoneme: String, hira: String, accent: i32, ) -> CoeirocoreMoraMora {
        CoeirocoreMoraMora {
            phoneme,
            hira,
            accent,
        }
    }
}

/// Converts the CoeirocoreMoraMora value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CoeirocoreMoraMora {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("phoneme".to_string()),
            Some(self.phoneme.to_string()),


            Some("hira".to_string()),
            Some(self.hira.to_string()),


            Some("accent".to_string()),
            Some(self.accent.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CoeirocoreMoraMora value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CoeirocoreMoraMora {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub phoneme: Vec<String>,
            pub hira: Vec<String>,
            pub accent: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CoeirocoreMoraMora".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "phoneme" => intermediate_rep.phoneme.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hira" => intermediate_rep.hira.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "accent" => intermediate_rep.accent.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CoeirocoreMoraMora".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CoeirocoreMoraMora {
            phoneme: intermediate_rep.phoneme.into_iter().next().ok_or_else(|| "phoneme missing in CoeirocoreMoraMora".to_string())?,
            hira: intermediate_rep.hira.into_iter().next().ok_or_else(|| "hira missing in CoeirocoreMoraMora".to_string())?,
            accent: intermediate_rep.accent.into_iter().next().ok_or_else(|| "accent missing in CoeirocoreMoraMora".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CoeirocoreMoraMora> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CoeirocoreMoraMora>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CoeirocoreMoraMora>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CoeirocoreMoraMora - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CoeirocoreMoraMora> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CoeirocoreMoraMora as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CoeirocoreMoraMora - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct CoeirocoreVUtilMora {
    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "consonant")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consonant: Option<String>,

    #[serde(rename = "consonantLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub consonant_length: Option<f64>,

    #[serde(rename = "vowel")]
    pub vowel: String,

    #[serde(rename = "vowelLength")]
    pub vowel_length: f64,

    #[serde(rename = "pitch")]
    pub pitch: f64,

}


impl CoeirocoreVUtilMora {
    #[allow(clippy::new_without_default)]
    pub fn new(text: String, vowel: String, vowel_length: f64, pitch: f64, ) -> CoeirocoreVUtilMora {
        CoeirocoreVUtilMora {
            text,
            consonant: None,
            consonant_length: None,
            vowel,
            vowel_length,
            pitch,
        }
    }
}

/// Converts the CoeirocoreVUtilMora value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for CoeirocoreVUtilMora {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("text".to_string()),
            Some(self.text.to_string()),


            self.consonant.as_ref().map(|consonant| {
                [
                    "consonant".to_string(),
                    consonant.to_string(),
                ].join(",")
            }),


            self.consonant_length.as_ref().map(|consonant_length| {
                [
                    "consonantLength".to_string(),
                    consonant_length.to_string(),
                ].join(",")
            }),


            Some("vowel".to_string()),
            Some(self.vowel.to_string()),


            Some("vowelLength".to_string()),
            Some(self.vowel_length.to_string()),


            Some("pitch".to_string()),
            Some(self.pitch.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a CoeirocoreVUtilMora value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for CoeirocoreVUtilMora {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
            pub consonant: Vec<String>,
            pub consonant_length: Vec<f64>,
            pub vowel: Vec<String>,
            pub vowel_length: Vec<f64>,
            pub pitch: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing CoeirocoreVUtilMora".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "consonant" => intermediate_rep.consonant.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "consonantLength" => intermediate_rep.consonant_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vowel" => intermediate_rep.vowel.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "vowelLength" => intermediate_rep.vowel_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pitch" => intermediate_rep.pitch.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing CoeirocoreVUtilMora".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(CoeirocoreVUtilMora {
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in CoeirocoreVUtilMora".to_string())?,
            consonant: intermediate_rep.consonant.into_iter().next(),
            consonant_length: intermediate_rep.consonant_length.into_iter().next(),
            vowel: intermediate_rep.vowel.into_iter().next().ok_or_else(|| "vowel missing in CoeirocoreVUtilMora".to_string())?,
            vowel_length: intermediate_rep.vowel_length.into_iter().next().ok_or_else(|| "vowelLength missing in CoeirocoreVUtilMora".to_string())?,
            pitch: intermediate_rep.pitch.into_iter().next().ok_or_else(|| "pitch missing in CoeirocoreVUtilMora".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<CoeirocoreVUtilMora> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<CoeirocoreVUtilMora>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<CoeirocoreVUtilMora>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for CoeirocoreVUtilMora - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<CoeirocoreVUtilMora> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <CoeirocoreVUtilMora as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into CoeirocoreVUtilMora - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DictionaryWord {
    #[serde(rename = "word")]
    pub word: String,

    #[serde(rename = "yomi")]
    pub yomi: String,

    #[serde(rename = "accent")]
    pub accent: i32,

    #[serde(rename = "numMoras")]
    pub num_moras: i32,

}


impl DictionaryWord {
    #[allow(clippy::new_without_default)]
    pub fn new(word: String, yomi: String, accent: i32, num_moras: i32, ) -> DictionaryWord {
        DictionaryWord {
            word,
            yomi,
            accent,
            num_moras,
        }
    }
}

/// Converts the DictionaryWord value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DictionaryWord {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("word".to_string()),
            Some(self.word.to_string()),


            Some("yomi".to_string()),
            Some(self.yomi.to_string()),


            Some("accent".to_string()),
            Some(self.accent.to_string()),


            Some("numMoras".to_string()),
            Some(self.num_moras.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DictionaryWord value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DictionaryWord {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub word: Vec<String>,
            pub yomi: Vec<String>,
            pub accent: Vec<i32>,
            pub num_moras: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DictionaryWord".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "word" => intermediate_rep.word.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "yomi" => intermediate_rep.yomi.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "accent" => intermediate_rep.accent.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "numMoras" => intermediate_rep.num_moras.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DictionaryWord".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DictionaryWord {
            word: intermediate_rep.word.into_iter().next().ok_or_else(|| "word missing in DictionaryWord".to_string())?,
            yomi: intermediate_rep.yomi.into_iter().next().ok_or_else(|| "yomi missing in DictionaryWord".to_string())?,
            accent: intermediate_rep.accent.into_iter().next().ok_or_else(|| "accent missing in DictionaryWord".to_string())?,
            num_moras: intermediate_rep.num_moras.into_iter().next().ok_or_else(|| "numMoras missing in DictionaryWord".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DictionaryWord> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DictionaryWord>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DictionaryWord>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DictionaryWord - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DictionaryWord> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DictionaryWord as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DictionaryWord - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DictionaryWords {
    #[serde(rename = "dictionaryWords")]
    pub dictionary_words: Vec<models::DictionaryWord>,

}


impl DictionaryWords {
    #[allow(clippy::new_without_default)]
    pub fn new(dictionary_words: Vec<models::DictionaryWord>, ) -> DictionaryWords {
        DictionaryWords {
            dictionary_words,
        }
    }
}

/// Converts the DictionaryWords value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DictionaryWords {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping dictionaryWords in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DictionaryWords value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DictionaryWords {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub dictionary_words: Vec<Vec<models::DictionaryWord>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DictionaryWords".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "dictionaryWords" => return std::result::Result::Err("Parsing a container in this style is not supported in DictionaryWords".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing DictionaryWords".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DictionaryWords {
            dictionary_words: intermediate_rep.dictionary_words.into_iter().next().ok_or_else(|| "dictionaryWords missing in DictionaryWords".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DictionaryWords> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DictionaryWords>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DictionaryWords>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DictionaryWords - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DictionaryWords> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DictionaryWords as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DictionaryWords - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct DownloadableModel {
    #[serde(rename = "download_path")]
    pub download_path: String,

    #[serde(rename = "volume")]
    pub volume: String,

    #[serde(rename = "speaker")]
    pub speaker: models::Speaker,

    #[serde(rename = "speaker_info")]
    pub speaker_info: models::SpeakerInfo,

}


impl DownloadableModel {
    #[allow(clippy::new_without_default)]
    pub fn new(download_path: String, volume: String, speaker: models::Speaker, speaker_info: models::SpeakerInfo, ) -> DownloadableModel {
        DownloadableModel {
            download_path,
            volume,
            speaker,
            speaker_info,
        }
    }
}

/// Converts the DownloadableModel value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for DownloadableModel {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("download_path".to_string()),
            Some(self.download_path.to_string()),


            Some("volume".to_string()),
            Some(self.volume.to_string()),

            // Skipping speaker in query parameter serialization

            // Skipping speaker_info in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a DownloadableModel value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for DownloadableModel {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub download_path: Vec<String>,
            pub volume: Vec<String>,
            pub speaker: Vec<models::Speaker>,
            pub speaker_info: Vec<models::SpeakerInfo>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing DownloadableModel".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "download_path" => intermediate_rep.download_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volume" => intermediate_rep.volume.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speaker" => intermediate_rep.speaker.push(<models::Speaker as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speaker_info" => intermediate_rep.speaker_info.push(<models::SpeakerInfo as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing DownloadableModel".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(DownloadableModel {
            download_path: intermediate_rep.download_path.into_iter().next().ok_or_else(|| "download_path missing in DownloadableModel".to_string())?,
            volume: intermediate_rep.volume.into_iter().next().ok_or_else(|| "volume missing in DownloadableModel".to_string())?,
            speaker: intermediate_rep.speaker.into_iter().next().ok_or_else(|| "speaker missing in DownloadableModel".to_string())?,
            speaker_info: intermediate_rep.speaker_info.into_iter().next().ok_or_else(|| "speaker_info missing in DownloadableModel".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<DownloadableModel> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<DownloadableModel>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<DownloadableModel>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for DownloadableModel - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<DownloadableModel> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <DownloadableModel as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into DownloadableModel - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct EngineInfo {
    #[serde(rename = "device")]
    pub device: String,

    #[serde(rename = "version")]
    pub version: String,

}


impl EngineInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(device: String, version: String, ) -> EngineInfo {
        EngineInfo {
            device,
            version,
        }
    }
}

/// Converts the EngineInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for EngineInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("device".to_string()),
            Some(self.device.to_string()),


            Some("version".to_string()),
            Some(self.version.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a EngineInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for EngineInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub device: Vec<String>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing EngineInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "device" => intermediate_rep.device.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing EngineInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(EngineInfo {
            device: intermediate_rep.device.into_iter().next().ok_or_else(|| "device missing in EngineInfo".to_string())?,
            version: intermediate_rep.version.into_iter().next().ok_or_else(|| "version missing in EngineInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<EngineInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<EngineInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<EngineInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for EngineInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<EngineInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <EngineInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into EngineInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct HttpValidationError {
    #[serde(rename = "detail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub detail: Option<Vec<models::ValidationError>>,

}


impl HttpValidationError {
    #[allow(clippy::new_without_default)]
    pub fn new() -> HttpValidationError {
        HttpValidationError {
            detail: None,
        }
    }
}

/// Converts the HttpValidationError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for HttpValidationError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping detail in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a HttpValidationError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for HttpValidationError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub detail: Vec<Vec<models::ValidationError>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing HttpValidationError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "detail" => return std::result::Result::Err("Parsing a container in this style is not supported in HttpValidationError".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing HttpValidationError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(HttpValidationError {
            detail: intermediate_rep.detail.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<HttpValidationError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<HttpValidationError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<HttpValidationError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for HttpValidationError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<HttpValidationError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <HttpValidationError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into HttpValidationError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct LocationInner {
}


impl LocationInner {
    #[allow(clippy::new_without_default)]
    pub fn new() -> LocationInner {
        LocationInner {
        }
    }
}

/// Converts the LocationInner value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for LocationInner {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a LocationInner value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for LocationInner {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing LocationInner".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    _ => return std::result::Result::Err("Unexpected key while parsing LocationInner".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(LocationInner {
        })
    }
}

// Methods for converting between header::IntoHeaderValue<LocationInner> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<LocationInner>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<LocationInner>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for LocationInner - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<LocationInner> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <LocationInner as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into LocationInner - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct MoraDuration {
    #[serde(rename = "mora")]
    pub mora: String,

    #[serde(rename = "hira")]
    pub hira: String,

    #[serde(rename = "phonemePitches")]
    pub phoneme_pitches: Vec<models::PhonemeDuration>,

    #[serde(rename = "wavRange")]
    pub wav_range: models::TimeRange,

}


impl MoraDuration {
    #[allow(clippy::new_without_default)]
    pub fn new(mora: String, hira: String, phoneme_pitches: Vec<models::PhonemeDuration>, wav_range: models::TimeRange, ) -> MoraDuration {
        MoraDuration {
            mora,
            hira,
            phoneme_pitches,
            wav_range,
        }
    }
}

/// Converts the MoraDuration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for MoraDuration {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("mora".to_string()),
            Some(self.mora.to_string()),


            Some("hira".to_string()),
            Some(self.hira.to_string()),

            // Skipping phonemePitches in query parameter serialization

            // Skipping wavRange in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a MoraDuration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for MoraDuration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub mora: Vec<String>,
            pub hira: Vec<String>,
            pub phoneme_pitches: Vec<Vec<models::PhonemeDuration>>,
            pub wav_range: Vec<models::TimeRange>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing MoraDuration".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "mora" => intermediate_rep.mora.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "hira" => intermediate_rep.hira.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "phonemePitches" => return std::result::Result::Err("Parsing a container in this style is not supported in MoraDuration".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "wavRange" => intermediate_rep.wav_range.push(<models::TimeRange as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing MoraDuration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(MoraDuration {
            mora: intermediate_rep.mora.into_iter().next().ok_or_else(|| "mora missing in MoraDuration".to_string())?,
            hira: intermediate_rep.hira.into_iter().next().ok_or_else(|| "hira missing in MoraDuration".to_string())?,
            phoneme_pitches: intermediate_rep.phoneme_pitches.into_iter().next().ok_or_else(|| "phonemePitches missing in MoraDuration".to_string())?,
            wav_range: intermediate_rep.wav_range.into_iter().next().ok_or_else(|| "wavRange missing in MoraDuration".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<MoraDuration> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<MoraDuration>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<MoraDuration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for MoraDuration - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<MoraDuration> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <MoraDuration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into MoraDuration - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct PhonemeDuration {
    #[serde(rename = "phoneme")]
    pub phoneme: String,

    #[serde(rename = "wavRange")]
    pub wav_range: models::TimeRange,

}


impl PhonemeDuration {
    #[allow(clippy::new_without_default)]
    pub fn new(phoneme: String, wav_range: models::TimeRange, ) -> PhonemeDuration {
        PhonemeDuration {
            phoneme,
            wav_range,
        }
    }
}

/// Converts the PhonemeDuration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for PhonemeDuration {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("phoneme".to_string()),
            Some(self.phoneme.to_string()),

            // Skipping wavRange in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a PhonemeDuration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for PhonemeDuration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub phoneme: Vec<String>,
            pub wav_range: Vec<models::TimeRange>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing PhonemeDuration".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "phoneme" => intermediate_rep.phoneme.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "wavRange" => intermediate_rep.wav_range.push(<models::TimeRange as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing PhonemeDuration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(PhonemeDuration {
            phoneme: intermediate_rep.phoneme.into_iter().next().ok_or_else(|| "phoneme missing in PhonemeDuration".to_string())?,
            wav_range: intermediate_rep.wav_range.into_iter().next().ok_or_else(|| "wavRange missing in PhonemeDuration".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<PhonemeDuration> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<PhonemeDuration>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<PhonemeDuration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for PhonemeDuration - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<PhonemeDuration> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <PhonemeDuration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into PhonemeDuration - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Prosody {
    #[serde(rename = "plain")]
    pub plain: Vec<String>,

    #[serde(rename = "detail")]
    pub detail: Vec<Vec<models::CoeirocoreMoraMora>>,

}


impl Prosody {
    #[allow(clippy::new_without_default)]
    pub fn new(plain: Vec<String>, detail: Vec<Vec<models::CoeirocoreMoraMora>>, ) -> Prosody {
        Prosody {
            plain,
            detail,
        }
    }
}

/// Converts the Prosody value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Prosody {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("plain".to_string()),
            Some(self.plain.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping detail in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Prosody value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Prosody {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub plain: Vec<Vec<String>>,
            pub detail: Vec<Vec<Vec<models::CoeirocoreMoraMora>>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Prosody".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "plain" => return std::result::Result::Err("Parsing a container in this style is not supported in Prosody".to_string()),
                    "detail" => return std::result::Result::Err("Parsing a container in this style is not supported in Prosody".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing Prosody".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Prosody {
            plain: intermediate_rep.plain.into_iter().next().ok_or_else(|| "plain missing in Prosody".to_string())?,
            detail: intermediate_rep.detail.into_iter().next().ok_or_else(|| "detail missing in Prosody".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Prosody> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Prosody>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Prosody>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Prosody - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Prosody> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Prosody as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Prosody - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ProsodyMakingParam {
    #[serde(rename = "text")]
    pub text: String,

}


impl ProsodyMakingParam {
    #[allow(clippy::new_without_default)]
    pub fn new(text: String, ) -> ProsodyMakingParam {
        ProsodyMakingParam {
            text,
        }
    }
}

/// Converts the ProsodyMakingParam value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ProsodyMakingParam {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("text".to_string()),
            Some(self.text.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ProsodyMakingParam value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ProsodyMakingParam {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub text: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ProsodyMakingParam".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ProsodyMakingParam".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ProsodyMakingParam {
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in ProsodyMakingParam".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ProsodyMakingParam> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ProsodyMakingParam>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ProsodyMakingParam>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ProsodyMakingParam - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ProsodyMakingParam> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ProsodyMakingParam as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ProsodyMakingParam - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Speaker {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "speaker_uuid")]
    pub speaker_uuid: String,

    #[serde(rename = "styles")]
    pub styles: Vec<models::CoeirocoreModelStyle>,

    #[serde(rename = "version")]
    pub version: String,

}


impl Speaker {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String, speaker_uuid: String, styles: Vec<models::CoeirocoreModelStyle>, version: String, ) -> Speaker {
        Speaker {
            name,
            speaker_uuid,
            styles,
            version,
        }
    }
}

/// Converts the Speaker value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Speaker {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("name".to_string()),
            Some(self.name.to_string()),


            Some("speaker_uuid".to_string()),
            Some(self.speaker_uuid.to_string()),

            // Skipping styles in query parameter serialization


            Some("version".to_string()),
            Some(self.version.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Speaker value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Speaker {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub name: Vec<String>,
            pub speaker_uuid: Vec<String>,
            pub styles: Vec<Vec<models::CoeirocoreModelStyle>>,
            pub version: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Speaker".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "name" => intermediate_rep.name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speaker_uuid" => intermediate_rep.speaker_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "styles" => return std::result::Result::Err("Parsing a container in this style is not supported in Speaker".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Speaker".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Speaker {
            name: intermediate_rep.name.into_iter().next().ok_or_else(|| "name missing in Speaker".to_string())?,
            speaker_uuid: intermediate_rep.speaker_uuid.into_iter().next().ok_or_else(|| "speaker_uuid missing in Speaker".to_string())?,
            styles: intermediate_rep.styles.into_iter().next().ok_or_else(|| "styles missing in Speaker".to_string())?,
            version: intermediate_rep.version.into_iter().next().ok_or_else(|| "version missing in Speaker".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Speaker> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Speaker>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Speaker>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Speaker - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Speaker> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Speaker as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Speaker - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SpeakerFolderPath {
    #[serde(rename = "speakerFolderPath")]
    pub speaker_folder_path: String,

}


impl SpeakerFolderPath {
    #[allow(clippy::new_without_default)]
    pub fn new(speaker_folder_path: String, ) -> SpeakerFolderPath {
        SpeakerFolderPath {
            speaker_folder_path,
        }
    }
}

/// Converts the SpeakerFolderPath value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SpeakerFolderPath {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("speakerFolderPath".to_string()),
            Some(self.speaker_folder_path.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SpeakerFolderPath value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SpeakerFolderPath {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub speaker_folder_path: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SpeakerFolderPath".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "speakerFolderPath" => intermediate_rep.speaker_folder_path.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SpeakerFolderPath".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SpeakerFolderPath {
            speaker_folder_path: intermediate_rep.speaker_folder_path.into_iter().next().ok_or_else(|| "speakerFolderPath missing in SpeakerFolderPath".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SpeakerFolderPath> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SpeakerFolderPath>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SpeakerFolderPath>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SpeakerFolderPath - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SpeakerFolderPath> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SpeakerFolderPath as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SpeakerFolderPath - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SpeakerInfo {
    #[serde(rename = "policy")]
    pub policy: String,

    #[serde(rename = "portrait")]
    pub portrait: String,

    #[serde(rename = "style_infos")]
    pub style_infos: Vec<models::StyleInfo>,

}


impl SpeakerInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(policy: String, portrait: String, style_infos: Vec<models::StyleInfo>, ) -> SpeakerInfo {
        SpeakerInfo {
            policy,
            portrait,
            style_infos,
        }
    }
}

/// Converts the SpeakerInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SpeakerInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("policy".to_string()),
            Some(self.policy.to_string()),


            Some("portrait".to_string()),
            Some(self.portrait.to_string()),

            // Skipping style_infos in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SpeakerInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SpeakerInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub policy: Vec<String>,
            pub portrait: Vec<String>,
            pub style_infos: Vec<Vec<models::StyleInfo>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SpeakerInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "policy" => intermediate_rep.policy.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "portrait" => intermediate_rep.portrait.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "style_infos" => return std::result::Result::Err("Parsing a container in this style is not supported in SpeakerInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing SpeakerInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SpeakerInfo {
            policy: intermediate_rep.policy.into_iter().next().ok_or_else(|| "policy missing in SpeakerInfo".to_string())?,
            portrait: intermediate_rep.portrait.into_iter().next().ok_or_else(|| "portrait missing in SpeakerInfo".to_string())?,
            style_infos: intermediate_rep.style_infos.into_iter().next().ok_or_else(|| "style_infos missing in SpeakerInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SpeakerInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SpeakerInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SpeakerInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SpeakerInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SpeakerInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SpeakerInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SpeakerInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SpeakerMeta {
    #[serde(rename = "speakerName")]
    pub speaker_name: String,

    #[serde(rename = "speakerUuid")]
    pub speaker_uuid: String,

    #[serde(rename = "styles")]
    pub styles: Vec<models::CoeirocoreMetaStyle>,

    #[serde(rename = "version")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub version: Option<String>,

    #[serde(rename = "base64Portrait")]
    pub base64_portrait: String,

}


impl SpeakerMeta {
    #[allow(clippy::new_without_default)]
    pub fn new(speaker_name: String, speaker_uuid: String, styles: Vec<models::CoeirocoreMetaStyle>, base64_portrait: String, ) -> SpeakerMeta {
        SpeakerMeta {
            speaker_name,
            speaker_uuid,
            styles,
            version: Some("0.0.1".to_string()),
            base64_portrait,
        }
    }
}

/// Converts the SpeakerMeta value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SpeakerMeta {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("speakerName".to_string()),
            Some(self.speaker_name.to_string()),


            Some("speakerUuid".to_string()),
            Some(self.speaker_uuid.to_string()),

            // Skipping styles in query parameter serialization


            self.version.as_ref().map(|version| {
                [
                    "version".to_string(),
                    version.to_string(),
                ].join(",")
            }),


            Some("base64Portrait".to_string()),
            Some(self.base64_portrait.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SpeakerMeta value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SpeakerMeta {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub speaker_name: Vec<String>,
            pub speaker_uuid: Vec<String>,
            pub styles: Vec<Vec<models::CoeirocoreMetaStyle>>,
            pub version: Vec<String>,
            pub base64_portrait: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SpeakerMeta".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "speakerName" => intermediate_rep.speaker_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speakerUuid" => intermediate_rep.speaker_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "styles" => return std::result::Result::Err("Parsing a container in this style is not supported in SpeakerMeta".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "version" => intermediate_rep.version.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "base64Portrait" => intermediate_rep.base64_portrait.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SpeakerMeta".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SpeakerMeta {
            speaker_name: intermediate_rep.speaker_name.into_iter().next().ok_or_else(|| "speakerName missing in SpeakerMeta".to_string())?,
            speaker_uuid: intermediate_rep.speaker_uuid.into_iter().next().ok_or_else(|| "speakerUuid missing in SpeakerMeta".to_string())?,
            styles: intermediate_rep.styles.into_iter().next().ok_or_else(|| "styles missing in SpeakerMeta".to_string())?,
            version: intermediate_rep.version.into_iter().next(),
            base64_portrait: intermediate_rep.base64_portrait.into_iter().next().ok_or_else(|| "base64Portrait missing in SpeakerMeta".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SpeakerMeta> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SpeakerMeta>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SpeakerMeta>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SpeakerMeta - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SpeakerMeta> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SpeakerMeta as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SpeakerMeta - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SpeakerMetaForTextBox {
    #[serde(rename = "speakerUuid")]
    pub speaker_uuid: String,

    #[serde(rename = "styleId")]
    pub style_id: i32,

    #[serde(rename = "speakerName")]
    pub speaker_name: String,

    #[serde(rename = "styleName")]
    pub style_name: String,

}


impl SpeakerMetaForTextBox {
    #[allow(clippy::new_without_default)]
    pub fn new(speaker_uuid: String, style_id: i32, speaker_name: String, style_name: String, ) -> SpeakerMetaForTextBox {
        SpeakerMetaForTextBox {
            speaker_uuid,
            style_id,
            speaker_name,
            style_name,
        }
    }
}

/// Converts the SpeakerMetaForTextBox value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SpeakerMetaForTextBox {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("speakerUuid".to_string()),
            Some(self.speaker_uuid.to_string()),


            Some("styleId".to_string()),
            Some(self.style_id.to_string()),


            Some("speakerName".to_string()),
            Some(self.speaker_name.to_string()),


            Some("styleName".to_string()),
            Some(self.style_name.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SpeakerMetaForTextBox value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SpeakerMetaForTextBox {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub speaker_uuid: Vec<String>,
            pub style_id: Vec<i32>,
            pub speaker_name: Vec<String>,
            pub style_name: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SpeakerMetaForTextBox".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "speakerUuid" => intermediate_rep.speaker_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "styleId" => intermediate_rep.style_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "speakerName" => intermediate_rep.speaker_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "styleName" => intermediate_rep.style_name.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SpeakerMetaForTextBox".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SpeakerMetaForTextBox {
            speaker_uuid: intermediate_rep.speaker_uuid.into_iter().next().ok_or_else(|| "speakerUuid missing in SpeakerMetaForTextBox".to_string())?,
            style_id: intermediate_rep.style_id.into_iter().next().ok_or_else(|| "styleId missing in SpeakerMetaForTextBox".to_string())?,
            speaker_name: intermediate_rep.speaker_name.into_iter().next().ok_or_else(|| "speakerName missing in SpeakerMetaForTextBox".to_string())?,
            style_name: intermediate_rep.style_name.into_iter().next().ok_or_else(|| "styleName missing in SpeakerMetaForTextBox".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SpeakerMetaForTextBox> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SpeakerMetaForTextBox>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SpeakerMetaForTextBox>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SpeakerMetaForTextBox - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SpeakerMetaForTextBox> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SpeakerMetaForTextBox as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SpeakerMetaForTextBox - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SpeakerPolicy {
    #[serde(rename = "policy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub policy: Option<String>,

    #[serde(rename = "license")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub license: Option<String>,

}


impl SpeakerPolicy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> SpeakerPolicy {
        SpeakerPolicy {
            policy: None,
            license: None,
        }
    }
}

/// Converts the SpeakerPolicy value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SpeakerPolicy {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            self.policy.as_ref().map(|policy| {
                [
                    "policy".to_string(),
                    policy.to_string(),
                ].join(",")
            }),


            self.license.as_ref().map(|license| {
                [
                    "license".to_string(),
                    license.to_string(),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SpeakerPolicy value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SpeakerPolicy {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub policy: Vec<String>,
            pub license: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SpeakerPolicy".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "policy" => intermediate_rep.policy.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "license" => intermediate_rep.license.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SpeakerPolicy".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SpeakerPolicy {
            policy: intermediate_rep.policy.into_iter().next(),
            license: intermediate_rep.license.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SpeakerPolicy> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SpeakerPolicy>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SpeakerPolicy>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SpeakerPolicy - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SpeakerPolicy> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SpeakerPolicy as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SpeakerPolicy - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct Status {
    #[serde(rename = "status")]
    pub status: String,

}


impl Status {
    #[allow(clippy::new_without_default)]
    pub fn new(status: String, ) -> Status {
        Status {
            status,
        }
    }
}

/// Converts the Status value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for Status {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("status".to_string()),
            Some(self.status.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a Status value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub status: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing Status".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "status" => intermediate_rep.status.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing Status".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(Status {
            status: intermediate_rep.status.into_iter().next().ok_or_else(|| "status missing in Status".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<Status> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<Status>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<Status>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for Status - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<Status> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <Status as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into Status - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct StyleInfo {
    #[serde(rename = "id")]
    pub id: i32,

    #[serde(rename = "icon")]
    pub icon: String,

    #[serde(rename = "voice_samples")]
    pub voice_samples: Vec<String>,

}


impl StyleInfo {
    #[allow(clippy::new_without_default)]
    pub fn new(id: i32, icon: String, voice_samples: Vec<String>, ) -> StyleInfo {
        StyleInfo {
            id,
            icon,
            voice_samples,
        }
    }
}

/// Converts the StyleInfo value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for StyleInfo {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("id".to_string()),
            Some(self.id.to_string()),


            Some("icon".to_string()),
            Some(self.icon.to_string()),


            Some("voice_samples".to_string()),
            Some(self.voice_samples.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a StyleInfo value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for StyleInfo {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub id: Vec<i32>,
            pub icon: Vec<String>,
            pub voice_samples: Vec<Vec<String>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing StyleInfo".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "id" => intermediate_rep.id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "icon" => intermediate_rep.icon.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "voice_samples" => return std::result::Result::Err("Parsing a container in this style is not supported in StyleInfo".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing StyleInfo".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(StyleInfo {
            id: intermediate_rep.id.into_iter().next().ok_or_else(|| "id missing in StyleInfo".to_string())?,
            icon: intermediate_rep.icon.into_iter().next().ok_or_else(|| "icon missing in StyleInfo".to_string())?,
            voice_samples: intermediate_rep.voice_samples.into_iter().next().ok_or_else(|| "voice_samples missing in StyleInfo".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<StyleInfo> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<StyleInfo>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<StyleInfo>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for StyleInfo - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<StyleInfo> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <StyleInfo as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into StyleInfo - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct SynthesisParam {
    #[serde(rename = "speakerUuid")]
    pub speaker_uuid: String,

    #[serde(rename = "styleId")]
    pub style_id: i32,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "prosodyDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prosody_detail: Option<Vec<Vec<models::CoeirocoreMoraMora>>>,

    #[serde(rename = "speedScale")]
    pub speed_scale: f64,

    #[serde(rename = "volumeScale")]
    pub volume_scale: f64,

    #[serde(rename = "pitchScale")]
    pub pitch_scale: f64,

    #[serde(rename = "intonationScale")]
    pub intonation_scale: f64,

    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f64,

    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f64,

    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: i32,

}


impl SynthesisParam {
    #[allow(clippy::new_without_default)]
    pub fn new(speaker_uuid: String, style_id: i32, text: String, speed_scale: f64, volume_scale: f64, pitch_scale: f64, intonation_scale: f64, pre_phoneme_length: f64, post_phoneme_length: f64, output_sampling_rate: i32, ) -> SynthesisParam {
        SynthesisParam {
            speaker_uuid,
            style_id,
            text,
            prosody_detail: None,
            speed_scale,
            volume_scale,
            pitch_scale,
            intonation_scale,
            pre_phoneme_length,
            post_phoneme_length,
            output_sampling_rate,
        }
    }
}

/// Converts the SynthesisParam value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for SynthesisParam {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("speakerUuid".to_string()),
            Some(self.speaker_uuid.to_string()),


            Some("styleId".to_string()),
            Some(self.style_id.to_string()),


            Some("text".to_string()),
            Some(self.text.to_string()),

            // Skipping prosodyDetail in query parameter serialization


            Some("speedScale".to_string()),
            Some(self.speed_scale.to_string()),


            Some("volumeScale".to_string()),
            Some(self.volume_scale.to_string()),


            Some("pitchScale".to_string()),
            Some(self.pitch_scale.to_string()),


            Some("intonationScale".to_string()),
            Some(self.intonation_scale.to_string()),


            Some("prePhonemeLength".to_string()),
            Some(self.pre_phoneme_length.to_string()),


            Some("postPhonemeLength".to_string()),
            Some(self.post_phoneme_length.to_string()),


            Some("outputSamplingRate".to_string()),
            Some(self.output_sampling_rate.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a SynthesisParam value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for SynthesisParam {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub speaker_uuid: Vec<String>,
            pub style_id: Vec<i32>,
            pub text: Vec<String>,
            pub prosody_detail: Vec<Vec<Vec<models::CoeirocoreMoraMora>>>,
            pub speed_scale: Vec<f64>,
            pub volume_scale: Vec<f64>,
            pub pitch_scale: Vec<f64>,
            pub intonation_scale: Vec<f64>,
            pub pre_phoneme_length: Vec<f64>,
            pub post_phoneme_length: Vec<f64>,
            pub output_sampling_rate: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing SynthesisParam".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "speakerUuid" => intermediate_rep.speaker_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "styleId" => intermediate_rep.style_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "prosodyDetail" => return std::result::Result::Err("Parsing a container in this style is not supported in SynthesisParam".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "speedScale" => intermediate_rep.speed_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volumeScale" => intermediate_rep.volume_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pitchScale" => intermediate_rep.pitch_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "intonationScale" => intermediate_rep.intonation_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prePhonemeLength" => intermediate_rep.pre_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postPhonemeLength" => intermediate_rep.post_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputSamplingRate" => intermediate_rep.output_sampling_rate.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing SynthesisParam".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(SynthesisParam {
            speaker_uuid: intermediate_rep.speaker_uuid.into_iter().next().ok_or_else(|| "speakerUuid missing in SynthesisParam".to_string())?,
            style_id: intermediate_rep.style_id.into_iter().next().ok_or_else(|| "styleId missing in SynthesisParam".to_string())?,
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in SynthesisParam".to_string())?,
            prosody_detail: intermediate_rep.prosody_detail.into_iter().next(),
            speed_scale: intermediate_rep.speed_scale.into_iter().next().ok_or_else(|| "speedScale missing in SynthesisParam".to_string())?,
            volume_scale: intermediate_rep.volume_scale.into_iter().next().ok_or_else(|| "volumeScale missing in SynthesisParam".to_string())?,
            pitch_scale: intermediate_rep.pitch_scale.into_iter().next().ok_or_else(|| "pitchScale missing in SynthesisParam".to_string())?,
            intonation_scale: intermediate_rep.intonation_scale.into_iter().next().ok_or_else(|| "intonationScale missing in SynthesisParam".to_string())?,
            pre_phoneme_length: intermediate_rep.pre_phoneme_length.into_iter().next().ok_or_else(|| "prePhonemeLength missing in SynthesisParam".to_string())?,
            post_phoneme_length: intermediate_rep.post_phoneme_length.into_iter().next().ok_or_else(|| "postPhonemeLength missing in SynthesisParam".to_string())?,
            output_sampling_rate: intermediate_rep.output_sampling_rate.into_iter().next().ok_or_else(|| "outputSamplingRate missing in SynthesisParam".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<SynthesisParam> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<SynthesisParam>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<SynthesisParam>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for SynthesisParam - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<SynthesisParam> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <SynthesisParam as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into SynthesisParam - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct TimeRange {
    #[serde(rename = "start")]
    pub start: i32,

    #[serde(rename = "end")]
    pub end: i32,

}


impl TimeRange {
    #[allow(clippy::new_without_default)]
    pub fn new(start: i32, end: i32, ) -> TimeRange {
        TimeRange {
            start,
            end,
        }
    }
}

/// Converts the TimeRange value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for TimeRange {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("start".to_string()),
            Some(self.start.to_string()),


            Some("end".to_string()),
            Some(self.end.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a TimeRange value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for TimeRange {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub start: Vec<i32>,
            pub end: Vec<i32>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing TimeRange".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "start" => intermediate_rep.start.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "end" => intermediate_rep.end.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing TimeRange".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(TimeRange {
            start: intermediate_rep.start.into_iter().next().ok_or_else(|| "start missing in TimeRange".to_string())?,
            end: intermediate_rep.end.into_iter().next().ok_or_else(|| "end missing in TimeRange".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<TimeRange> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<TimeRange>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<TimeRange>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for TimeRange - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<TimeRange> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <TimeRange as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into TimeRange - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<models::LocationInner>,

    #[serde(rename = "msg")]
    pub msg: String,

    #[serde(rename = "type")]
    pub r#type: String,

}


impl ValidationError {
    #[allow(clippy::new_without_default)]
    pub fn new(loc: Vec<models::LocationInner>, msg: String, r#type: String, ) -> ValidationError {
        ValidationError {
            loc,
            msg,
            r#type,
        }
    }
}

/// Converts the ValidationError value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for ValidationError {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![
            // Skipping loc in query parameter serialization


            Some("msg".to_string()),
            Some(self.msg.to_string()),


            Some("type".to_string()),
            Some(self.r#type.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a ValidationError value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for ValidationError {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub loc: Vec<Vec<models::LocationInner>>,
            pub msg: Vec<String>,
            pub r#type: Vec<String>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing ValidationError".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "loc" => return std::result::Result::Err("Parsing a container in this style is not supported in ValidationError".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "msg" => intermediate_rep.msg.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "type" => intermediate_rep.r#type.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing ValidationError".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(ValidationError {
            loc: intermediate_rep.loc.into_iter().next().ok_or_else(|| "loc missing in ValidationError".to_string())?,
            msg: intermediate_rep.msg.into_iter().next().ok_or_else(|| "msg missing in ValidationError".to_string())?,
            r#type: intermediate_rep.r#type.into_iter().next().ok_or_else(|| "type missing in ValidationError".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<ValidationError> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<ValidationError>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<ValidationError>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for ValidationError - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<ValidationError> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <ValidationError as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into ValidationError - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WavMakingParam {
    #[serde(rename = "speakerUuid")]
    pub speaker_uuid: String,

    #[serde(rename = "styleId")]
    pub style_id: i32,

    #[serde(rename = "text")]
    pub text: String,

    #[serde(rename = "prosodyDetail")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prosody_detail: Option<Vec<Vec<models::CoeirocoreMoraMora>>>,

    #[serde(rename = "speedScale")]
    pub speed_scale: f64,

}


impl WavMakingParam {
    #[allow(clippy::new_without_default)]
    pub fn new(speaker_uuid: String, style_id: i32, text: String, speed_scale: f64, ) -> WavMakingParam {
        WavMakingParam {
            speaker_uuid,
            style_id,
            text,
            prosody_detail: None,
            speed_scale,
        }
    }
}

/// Converts the WavMakingParam value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WavMakingParam {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("speakerUuid".to_string()),
            Some(self.speaker_uuid.to_string()),


            Some("styleId".to_string()),
            Some(self.style_id.to_string()),


            Some("text".to_string()),
            Some(self.text.to_string()),

            // Skipping prosodyDetail in query parameter serialization


            Some("speedScale".to_string()),
            Some(self.speed_scale.to_string()),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WavMakingParam value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WavMakingParam {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub speaker_uuid: Vec<String>,
            pub style_id: Vec<i32>,
            pub text: Vec<String>,
            pub prosody_detail: Vec<Vec<Vec<models::CoeirocoreMoraMora>>>,
            pub speed_scale: Vec<f64>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WavMakingParam".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "speakerUuid" => intermediate_rep.speaker_uuid.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "styleId" => intermediate_rep.style_id.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "text" => intermediate_rep.text.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "prosodyDetail" => return std::result::Result::Err("Parsing a container in this style is not supported in WavMakingParam".to_string()),
                    #[allow(clippy::redundant_clone)]
                    "speedScale" => intermediate_rep.speed_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    _ => return std::result::Result::Err("Unexpected key while parsing WavMakingParam".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WavMakingParam {
            speaker_uuid: intermediate_rep.speaker_uuid.into_iter().next().ok_or_else(|| "speakerUuid missing in WavMakingParam".to_string())?,
            style_id: intermediate_rep.style_id.into_iter().next().ok_or_else(|| "styleId missing in WavMakingParam".to_string())?,
            text: intermediate_rep.text.into_iter().next().ok_or_else(|| "text missing in WavMakingParam".to_string())?,
            prosody_detail: intermediate_rep.prosody_detail.into_iter().next(),
            speed_scale: intermediate_rep.speed_scale.into_iter().next().ok_or_else(|| "speedScale missing in WavMakingParam".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WavMakingParam> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WavMakingParam>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WavMakingParam>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WavMakingParam - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WavMakingParam> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WavMakingParam as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WavMakingParam - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WavProcessingParam {
    #[serde(rename = "wavBase64")]
    pub wav_base64: String,

    #[serde(rename = "volumeScale")]
    pub volume_scale: f64,

    #[serde(rename = "pitchScale")]
    pub pitch_scale: f64,

    #[serde(rename = "intonationScale")]
    pub intonation_scale: f64,

    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f64,

    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f64,

    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: i32,

    #[serde(rename = "adjustedF0")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub adjusted_f0: Option<Vec<f64>>,

}


impl WavProcessingParam {
    #[allow(clippy::new_without_default)]
    pub fn new(wav_base64: String, volume_scale: f64, pitch_scale: f64, intonation_scale: f64, pre_phoneme_length: f64, post_phoneme_length: f64, output_sampling_rate: i32, ) -> WavProcessingParam {
        WavProcessingParam {
            wav_base64,
            volume_scale,
            pitch_scale,
            intonation_scale,
            pre_phoneme_length,
            post_phoneme_length,
            output_sampling_rate,
            adjusted_f0: None,
        }
    }
}

/// Converts the WavProcessingParam value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WavProcessingParam {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("wavBase64".to_string()),
            Some(self.wav_base64.to_string()),


            Some("volumeScale".to_string()),
            Some(self.volume_scale.to_string()),


            Some("pitchScale".to_string()),
            Some(self.pitch_scale.to_string()),


            Some("intonationScale".to_string()),
            Some(self.intonation_scale.to_string()),


            Some("prePhonemeLength".to_string()),
            Some(self.pre_phoneme_length.to_string()),


            Some("postPhonemeLength".to_string()),
            Some(self.post_phoneme_length.to_string()),


            Some("outputSamplingRate".to_string()),
            Some(self.output_sampling_rate.to_string()),


            self.adjusted_f0.as_ref().map(|adjusted_f0| {
                [
                    "adjustedF0".to_string(),
                    adjusted_f0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(","),
                ].join(",")
            }),

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WavProcessingParam value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WavProcessingParam {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub wav_base64: Vec<String>,
            pub volume_scale: Vec<f64>,
            pub pitch_scale: Vec<f64>,
            pub intonation_scale: Vec<f64>,
            pub pre_phoneme_length: Vec<f64>,
            pub post_phoneme_length: Vec<f64>,
            pub output_sampling_rate: Vec<i32>,
            pub adjusted_f0: Vec<Vec<f64>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WavProcessingParam".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "wavBase64" => intermediate_rep.wav_base64.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "volumeScale" => intermediate_rep.volume_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "pitchScale" => intermediate_rep.pitch_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "intonationScale" => intermediate_rep.intonation_scale.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "prePhonemeLength" => intermediate_rep.pre_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "postPhonemeLength" => intermediate_rep.post_phoneme_length.push(<f64 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    #[allow(clippy::redundant_clone)]
                    "outputSamplingRate" => intermediate_rep.output_sampling_rate.push(<i32 as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "adjustedF0" => return std::result::Result::Err("Parsing a container in this style is not supported in WavProcessingParam".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing WavProcessingParam".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WavProcessingParam {
            wav_base64: intermediate_rep.wav_base64.into_iter().next().ok_or_else(|| "wavBase64 missing in WavProcessingParam".to_string())?,
            volume_scale: intermediate_rep.volume_scale.into_iter().next().ok_or_else(|| "volumeScale missing in WavProcessingParam".to_string())?,
            pitch_scale: intermediate_rep.pitch_scale.into_iter().next().ok_or_else(|| "pitchScale missing in WavProcessingParam".to_string())?,
            intonation_scale: intermediate_rep.intonation_scale.into_iter().next().ok_or_else(|| "intonationScale missing in WavProcessingParam".to_string())?,
            pre_phoneme_length: intermediate_rep.pre_phoneme_length.into_iter().next().ok_or_else(|| "prePhonemeLength missing in WavProcessingParam".to_string())?,
            post_phoneme_length: intermediate_rep.post_phoneme_length.into_iter().next().ok_or_else(|| "postPhonemeLength missing in WavProcessingParam".to_string())?,
            output_sampling_rate: intermediate_rep.output_sampling_rate.into_iter().next().ok_or_else(|| "outputSamplingRate missing in WavProcessingParam".to_string())?,
            adjusted_f0: intermediate_rep.adjusted_f0.into_iter().next(),
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WavProcessingParam> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WavProcessingParam>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WavProcessingParam>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WavProcessingParam - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WavProcessingParam> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WavProcessingParam as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WavProcessingParam - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WavWithDuration {
    #[serde(rename = "wavBase64")]
    pub wav_base64: String,

    #[serde(rename = "moraDurations")]
    pub mora_durations: Vec<models::MoraDuration>,

}


impl WavWithDuration {
    #[allow(clippy::new_without_default)]
    pub fn new(wav_base64: String, mora_durations: Vec<models::MoraDuration>, ) -> WavWithDuration {
        WavWithDuration {
            wav_base64,
            mora_durations,
        }
    }
}

/// Converts the WavWithDuration value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WavWithDuration {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("wavBase64".to_string()),
            Some(self.wav_base64.to_string()),

            // Skipping moraDurations in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WavWithDuration value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WavWithDuration {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub wav_base64: Vec<String>,
            pub mora_durations: Vec<Vec<models::MoraDuration>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WavWithDuration".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    #[allow(clippy::redundant_clone)]
                    "wavBase64" => intermediate_rep.wav_base64.push(<String as std::str::FromStr>::from_str(val).map_err(|x| x.to_string())?),
                    "moraDurations" => return std::result::Result::Err("Parsing a container in this style is not supported in WavWithDuration".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing WavWithDuration".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WavWithDuration {
            wav_base64: intermediate_rep.wav_base64.into_iter().next().ok_or_else(|| "wavBase64 missing in WavWithDuration".to_string())?,
            mora_durations: intermediate_rep.mora_durations.into_iter().next().ok_or_else(|| "moraDurations missing in WavWithDuration".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WavWithDuration> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WavWithDuration>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WavWithDuration>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WavWithDuration - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WavWithDuration> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WavWithDuration as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WavWithDuration - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}


#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, validator::Validate)]
#[cfg_attr(feature = "conversion", derive(frunk::LabelledGeneric))]
pub struct WorldF0 {
    #[serde(rename = "f0")]
    pub f0: Vec<f64>,

    #[serde(rename = "moraDurations")]
    pub mora_durations: Vec<models::MoraDuration>,

}


impl WorldF0 {
    #[allow(clippy::new_without_default)]
    pub fn new(f0: Vec<f64>, mora_durations: Vec<models::MoraDuration>, ) -> WorldF0 {
        WorldF0 {
            f0,
            mora_durations,
        }
    }
}

/// Converts the WorldF0 value to the Query Parameters representation (style=form, explode=false)
/// specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde serializer
impl std::string::ToString for WorldF0 {
    fn to_string(&self) -> String {
        let params: Vec<Option<String>> = vec![

            Some("f0".to_string()),
            Some(self.f0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",")),

            // Skipping moraDurations in query parameter serialization

        ];

        params.into_iter().flatten().collect::<Vec<_>>().join(",")
    }
}

/// Converts Query Parameters representation (style=form, explode=false) to a WorldF0 value
/// as specified in https://swagger.io/docs/specification/serialization/
/// Should be implemented in a serde deserializer
impl std::str::FromStr for WorldF0 {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        /// An intermediate representation of the struct to use for parsing.
        #[derive(Default)]
        #[allow(dead_code)]
        struct IntermediateRep {
            pub f0: Vec<Vec<f64>>,
            pub mora_durations: Vec<Vec<models::MoraDuration>>,
        }

        let mut intermediate_rep = IntermediateRep::default();

        // Parse into intermediate representation
        let mut string_iter = s.split(',');
        let mut key_result = string_iter.next();

        while key_result.is_some() {
            let val = match string_iter.next() {
                Some(x) => x,
                None => return std::result::Result::Err("Missing value while parsing WorldF0".to_string())
            };

            if let Some(key) = key_result {
                #[allow(clippy::match_single_binding)]
                match key {
                    "f0" => return std::result::Result::Err("Parsing a container in this style is not supported in WorldF0".to_string()),
                    "moraDurations" => return std::result::Result::Err("Parsing a container in this style is not supported in WorldF0".to_string()),
                    _ => return std::result::Result::Err("Unexpected key while parsing WorldF0".to_string())
                }
            }

            // Get the next key
            key_result = string_iter.next();
        }

        // Use the intermediate representation to return the struct
        std::result::Result::Ok(WorldF0 {
            f0: intermediate_rep.f0.into_iter().next().ok_or_else(|| "f0 missing in WorldF0".to_string())?,
            mora_durations: intermediate_rep.mora_durations.into_iter().next().ok_or_else(|| "moraDurations missing in WorldF0".to_string())?,
        })
    }
}

// Methods for converting between header::IntoHeaderValue<WorldF0> and hyper::header::HeaderValue

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<header::IntoHeaderValue<WorldF0>> for hyper::header::HeaderValue {
    type Error = String;

    fn try_from(hdr_value: header::IntoHeaderValue<WorldF0>) -> std::result::Result<Self, Self::Error> {
        let hdr_value = hdr_value.to_string();
        match hyper::header::HeaderValue::from_str(&hdr_value) {
             std::result::Result::Ok(value) => std::result::Result::Ok(value),
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Invalid header value for WorldF0 - value: {} is invalid {}",
                     hdr_value, e))
        }
    }
}

#[cfg(any(feature = "client", feature = "server"))]
impl std::convert::TryFrom<hyper::header::HeaderValue> for header::IntoHeaderValue<WorldF0> {
    type Error = String;

    fn try_from(hdr_value: hyper::header::HeaderValue) -> std::result::Result<Self, Self::Error> {
        match hdr_value.to_str() {
             std::result::Result::Ok(value) => {
                    match <WorldF0 as std::str::FromStr>::from_str(value) {
                        std::result::Result::Ok(value) => std::result::Result::Ok(header::IntoHeaderValue(value)),
                        std::result::Result::Err(err) => std::result::Result::Err(
                            format!("Unable to convert header value '{}' into WorldF0 - {}",
                                value, err))
                    }
             },
             std::result::Result::Err(e) => std::result::Result::Err(
                 format!("Unable to convert header: {:?} to string: {}",
                     hdr_value, e))
        }
    }
}

