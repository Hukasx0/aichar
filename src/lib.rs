/*
    MIT License

    Copyright (c) 2023-2025 Hubert Kasperek

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use base64::{Engine, engine::GeneralPurpose, engine::GeneralPurposeConfig, alphabet::STANDARD};
use png::Decoder;
use chrono::Utc;
use std::{fs, fs::File};
use std::io::{Read, Write, Cursor, BufReader, Seek, SeekFrom};


static PROGRAM_INFO: ProgramInfo = ProgramInfo {
    name: "aichar Python library",
    version: "1.0.5",
    url: "https://github.com/Hukasx0/aichar",
};


#[derive(Clone)]
#[pyclass]
struct CharacterClass {
    name: String,
    summary: String,
    personality: String,
    scenario: String,
    greeting_message: String,
    example_messages: String,
    image_path: Option<String>, 
    created_time: Option<i64>,
}

#[pymethods]
impl CharacterClass {
    #[getter]
    fn name(&self) -> &String {
        &self.name
    }

    #[getter]
    fn summary(&self) -> &String {
        &self.summary
    }

    #[getter]
    fn personality(&self) -> &String {
        &self.personality
    }

    #[getter]
    fn scenario(&self) -> &String {
        &self.scenario
    }

    #[getter]
    fn greeting_message(&self) -> &String {
        &self.greeting_message
    }

    #[getter]
    fn example_messages(&self) -> &String {
        &self.example_messages
    }

    #[getter]
    fn image_path(&self) -> Option<&String> {
        self.image_path.as_ref()
    }

    #[getter]
    fn data_summary(&self) -> String {
        let mut character_str = String::new();

        character_str.push_str(&format!("Name: {}\n", self.name));
        character_str.push_str(&format!("Summary: {}\n", self.summary));
        character_str.push_str(&format!("Personality: {}\n", self.personality));
        character_str.push_str(&format!("Scenario: {}\n", self.scenario));
        character_str.push_str(&format!("Greeting Message: {}\n", self.greeting_message));
        character_str.push_str(&format!("Example Messages: \n{}\n", self.example_messages));

        match &self.image_path {
            Some(path) => character_str.push_str(&format!("Image Path: {}", path)),
            None => character_str.push_str("Image Path: None"),
        }
        character_str
    }

    #[setter]
    fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    #[setter]
    fn set_summary(&mut self, summary: &str) {
        self.summary = summary.to_string();
    }

    #[setter]
    fn set_personality(&mut self, personality: &str) {
        self.personality = personality.to_string();
    }

    #[setter]
    fn set_scenario(&mut self, scenario: &str) {
        self.scenario = scenario.to_string();
    }

    #[setter]
    fn set_greeting_message(&mut self, greeting_message: &str) {
        self.greeting_message = greeting_message.to_string();
    }

    #[setter]
    fn set_example_messages(&mut self, example_messages: &str) {
        self.example_messages = example_messages.to_string();
    }

    #[setter]
    fn set_image_path(&mut self, image_path: &str) -> PyResult<()> {
        if image_path.to_lowercase().ends_with(".png") {
            if let Ok(metadata) = fs::metadata(image_path) {
                if metadata.is_file() {
                    self.image_path = Some(image_path.to_string());
                    return Ok(());
                } else {
                    return Err(pyo3::exceptions::PyValueError::new_err("Specified path is not a file"));
                }
            } else {
                return Err(pyo3::exceptions::PyValueError::new_err(format!("'{}' file does not exist", image_path)));
            }
        }
        Err(pyo3::exceptions::PyValueError::new_err("Invalid file extension. Image file must have a .png extension"))
    }

    fn export_json(&self, format_type: &str) -> PyResult<String> {
        export_as_json(self, format_type)
    }

    fn export_json_file(&self, format_type: &str, export_json_path: &str) -> PyResult<()> {
        let json_string = export_as_json(self, format_type)?;
        let mut file = File::create(export_json_path)?;
        file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        Ok(())
    }

    fn export_neutral_json(&self) -> PyResult<String> {
        export_as_neutral_json(self)
    }

    fn export_neutral_json_file(&self, export_json_path: &str) -> PyResult<()> {
        let json_string = export_as_neutral_json(self)?;
        let mut file = File::create(export_json_path)?;
        file.write_all(json_string.as_bytes())?;
        Ok(())
    }

    fn export_yaml(&self, format_type: &str) -> PyResult<String> {
        export_as_yaml(self, format_type)
    }

    fn export_yaml_file(&self, format_type: &str, export_yaml_path: &str) -> PyResult<()> {
        let yaml_string = export_as_yaml(self, format_type)?;
        let mut file = File::create(export_yaml_path)?;
        file.write_all(yaml_string.as_bytes()).expect("Error while writing to yaml file");
        Ok(())
    }

    fn export_neutral_yaml(&self) -> PyResult<String> {
        export_as_neutral_yaml(self)
    }

    fn export_neutral_yaml_file(&self, export_yaml_path: &str) -> PyResult<()> {
        let yaml_string = export_as_neutral_yaml(self)?;
        let mut file = File::create(export_yaml_path)?;
        file.write_all(yaml_string.as_bytes())?;
        Ok(())
    }

    fn export_card(&self, format_type: &str) -> PyResult<Vec<u8>> {
        export_as_card(self, format_type)
    }

    fn export_card_file(&self, format_type: &str, export_card_path: &str) -> PyResult<()> {
        let bytes = export_as_card(self, format_type)?;
        let mut file = File::create(export_card_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }

    fn export_neutral_card(&self) -> PyResult<Vec<u8>> {
        export_as_card(self, "neutral")
    }

    fn export_neutral_card_file(&self, export_card_path: &str) -> PyResult<()> {
        let bytes = export_as_card(self, "neutral")?;
        let mut file = File::create(export_card_path)?;
        file.write_all(&bytes)?;
        Ok(())
    }
}

fn export_as_yaml(character: &CharacterClass, format_type: &str) -> PyResult<String> {
    let current_time = Utc::now().timestamp_millis();
    match format_type.to_lowercase().as_str() {
        "tavernai" | "sillytavern" => {
            let export: ExportTavernAi = ExportTavernAi {
                name: &character.name,
                description: &character.summary,
                personality: &character.personality,
                scenario: &character.scenario,
                first_mes: &character.greeting_message,
                mes_example: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_yaml::to_string(&export).expect("Error while serializing YAML"))
        },
        "textgenerationwebui" | "pygmalion" => {
            let export: ExportTextGenerationWebuiPygmalion = ExportTextGenerationWebuiPygmalion {
                char_name: &character.name,
                char_persona: if character.personality.is_empty() {
                    &character.summary
                } else {
                    &character.personality
                },
                world_scenario: &character.scenario,
                char_greeting: &character.greeting_message,
                example_dialogue: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_yaml::to_string(&export).expect("Error while serializing YAML"))
        },
        "aicompanion" => {
            let export: ExportAiCompanion = ExportAiCompanion {
                name: &character.name,
                description: if character.personality.is_empty() {
                    &character.summary
                } else {
                    &character.personality
                },
                first_mes: &character.greeting_message,
                mes_example: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_yaml::to_string(&export).expect("Error while serializing YAML"))
        },
        _ => {
            Err(pyo3::exceptions::PyValueError::new_err("Format not supported, supported formats: 'tavernai', 'sillytavern', 'textgenerationwebui', 'pygmalion', 'aicompanion'"))
        }
    }
}

fn export_as_neutral_yaml(character: &CharacterClass) -> PyResult<String> {
    let current_time = Utc::now().timestamp_millis();
    let export_class: ExportAllCharacterClass = ExportAllCharacterClass {
        char_name: &character.name,
        char_persona: if character.personality.is_empty() {
            &character.summary
        } else {
            &character.personality
        },
        world_scenario: &character.scenario,
        char_greeting: &character.greeting_message,
        example_dialogue: &character.example_messages,
        name: &character.name,
        description: &character.summary,
        personality: &character.personality,
        scenario: &character.scenario,
        first_mes: &character.greeting_message,
        mes_example: &character.example_messages,
        metadata: Metadata {
            version: 1,
            created: &character.created_time.unwrap_or(current_time),
            modified: current_time,
            source: None,
            tool: Tooldata {
                name: PROGRAM_INFO.name,
                version: PROGRAM_INFO.version,
                url: PROGRAM_INFO.url,
            }
        },
    };
    Ok(serde_yaml::to_string(&export_class).expect("Error while serializing YAML"))
}

fn export_as_json(character: &CharacterClass, format_type: &str) -> PyResult<String> {
    let current_time = Utc::now().timestamp_millis();
    match format_type.to_lowercase().as_str() {
        "tavernai" | "sillytavern" => {
            let export: ExportTavernAi = ExportTavernAi {
                name: &character.name,
                description: &character.summary,
                personality: &character.personality,
                scenario: &character.scenario,
                first_mes: &character.greeting_message,
                mes_example: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
        },
        "textgenerationwebui" | "pygmalion" => {
            let export: ExportTextGenerationWebuiPygmalion = ExportTextGenerationWebuiPygmalion {
                char_name: &character.name,
                char_persona: if character.personality.is_empty() {
                    &character.summary
                } else {
                    &character.personality
                },
                world_scenario: &character.scenario,
                char_greeting: &character.greeting_message,
                example_dialogue: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
        },
        "aicompanion" => {
            let export: ExportAiCompanion = ExportAiCompanion {
                name: &character.name,
                description: if character.personality.is_empty() {
                    &character.summary
                } else {
                    &character.personality
                },
                first_mes: &character.greeting_message,
                mes_example: &character.example_messages,
                metadata: Metadata {
                    version: 1,
                    created: &character.created_time.unwrap_or(current_time),
                    modified: current_time,
                    source: None,
                    tool: Tooldata {
                        name: PROGRAM_INFO.name,
                        version: PROGRAM_INFO.version,
                        url: PROGRAM_INFO.url,
                    }
                },
            };
            Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
        },
        _ => {
            Err(pyo3::exceptions::PyValueError::new_err("Format not supported, supported formats: 'tavernai', 'sillytavern', 'textgenerationwebui', 'pygmalion', 'aicompanion'"))
        }
    }
}

fn export_as_neutral_json(character: &CharacterClass) -> PyResult<String> {
    let current_time = Utc::now().timestamp_millis();
    let export_class: ExportAllCharacterClass = ExportAllCharacterClass {
        char_name: &character.name,
        char_persona: if character.personality.is_empty() {
            &character.summary
        } else {
            &character.personality
        },
        world_scenario: &character.scenario,
        char_greeting: &character.greeting_message,
        example_dialogue: &character.example_messages,
        name: &character.name,
        description: &character.summary,
        personality: &character.personality,
        scenario: &character.scenario,
        first_mes: &character.greeting_message,
        mes_example: &character.example_messages,
        metadata: Metadata {
            version: 1,
            created: &character.created_time.unwrap_or(current_time),
            modified: current_time,
            source: None,
            tool: Tooldata {
                name: PROGRAM_INFO.name,
                version: PROGRAM_INFO.version,
                url: PROGRAM_INFO.url,
            }
        },
    };
    Ok(serde_json::to_string_pretty(&export_class).expect("Error while serializing JSON"))
}

fn export_as_card(character: &CharacterClass, format_type: &str) -> PyResult<Vec<u8>> {
    let character_image = match &character.image_path {
        Some(v) => v,
        None => {
            return Err(pyo3::exceptions::PyValueError::new_err("To export a character using the character card, you must provide a png file that will hold the encoded data. You can add the image path to the CharacterClass object using the property: .image_path = \"png/file/path\""));
        }
    };

    let file = File::open(character_image).map_err(|e| pyo3::exceptions::PyIOError::new_err(format!("Failed to open image file '{}': {}", character_image, e)))?;
    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info().map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read PNG info from file '{}' (Check if it is a valid, working PNG file): {}", character_image, e)))?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read PNG frame from file '{}' (Check if it is a valid, working PNG file): {}", character_image, e)))?;
    let bytes = &buf[..info.buffer_size()];

    let mut encoded_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut encoded_data, info.width, info.height);
        encoder.set_color(info.color_type);
        encoder.set_depth(info.bit_depth);
        let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());

        let character_base64 = if format_type == "neutral" {
            engine.encode(export_as_neutral_json(character)?)
        } else {
            engine.encode(export_as_json(character, format_type)?)
        };

        encoder.add_text_chunk(
            "chara".to_string(),
            character_base64,
        ).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to add text chunk: {}", e)))?;
        let mut writer = encoder.write_header().map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to write PNG header: {}", e)))?;
        writer.write_image_data(bytes).map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to write PNG image data: {}", e)))?;
    }

    Ok(encoded_data)
}

struct ProgramInfo {
    name: &'static str,
    version: &'static str,
    url: &'static str,
}

#[derive(Serialize)]
struct Metadata<'a> {
    version: u32,
    created: &'a i64,
    modified: i64,
    source: Option<String>,
    tool: Tooldata<'a>,
}

#[derive(Serialize)]
struct Tooldata<'a> {
    name: &'a str,
    version: &'a str,
    url: &'a str,
}

#[derive(Serialize)]
struct ExportAllCharacterClass<'a> {
    char_name: &'a str,
    char_persona: &'a str,
    world_scenario: &'a str,
    char_greeting: &'a str,
    example_dialogue: &'a str,
    name: &'a str,
    description: &'a str,
    personality: &'a str,
    scenario: &'a str,
    first_mes: &'a str,
    mes_example: &'a str,
    metadata: Metadata<'a>,
}

#[derive(Deserialize)]
struct LoadCharacterClass {
    char_name: Option<String>,
    char_persona: Option<String>,
    world_scenario: Option<String>,
    char_greeting: Option<String>,
    example_dialogue: Option<String>,
    name: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    personality: Option<String>,
    scenario: Option<String>,
    first_mes: Option<String>,
    mes_example: Option<String>,
    metadata: Option<TimeMetadata>
}

#[derive(Deserialize)]
struct TimeMetadata {
    created: Option<i64>,
}

#[derive(Serialize)]
struct ExportTavernAi<'a> {
    name: &'a str,
    description: &'a str,
    personality: &'a str,
    scenario: &'a str,
    first_mes: &'a str,
    mes_example: &'a str,
    metadata: Metadata<'a>,
}

#[derive(Serialize)]
struct ExportTextGenerationWebuiPygmalion<'a> {
    char_name: &'a str,
    char_persona: &'a str,
    world_scenario: &'a str,
    char_greeting: &'a str,
    example_dialogue: &'a str,
    metadata: Metadata<'a>,
}

#[derive(Serialize)]
struct ExportAiCompanion<'a> {
    name: &'a str,
    description: &'a str,
    first_mes: &'a str,
    mes_example: &'a str,
    metadata: Metadata<'a>,
}

#[pyfunction]
fn create_character(name: &str, summary: &str, personality: &str, scenario: &str, greeting_message: &str, example_messages: &str, image_path: &str) -> PyResult<CharacterClass> {
    Ok(CharacterClass {
        name: name.to_string(),
        summary: summary.to_string(),
        personality: personality.to_string(),
        scenario: scenario.to_string(),
        greeting_message: greeting_message.to_string(),
        example_messages: example_messages.to_string(),
        image_path: Some(image_path.to_string()),
        created_time: None,
    })
}

#[pyfunction]
fn load_character_json(json: &str) -> PyResult<CharacterClass> {
    let char_data: LoadCharacterClass = serde_json::from_str(json).expect("Error while parsing json string");
    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

#[pyfunction]
fn load_character_json_file(path: &str) -> PyResult<CharacterClass> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    load_character_json(&buf)
}

#[pyfunction]
fn load_character_yaml(json: &str) -> PyResult<CharacterClass> {
    let char_data: LoadCharacterClass = serde_yaml::from_str(json).expect("Error while parsing yaml string");
    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

#[pyfunction]
fn load_character_yaml_file(path: &str) -> PyResult<CharacterClass> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    load_character_yaml(&buf)
}

#[pyfunction]
fn load_character_card(bytes: &[u8]) -> PyResult<CharacterClass> {
    let decoder = png::Decoder::new(Cursor::new(bytes));
    let reader = decoder.read_info().map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read PNG info: {}", e)))?;
    let character_base64_option: Option<String> = reader.info().uncompressed_latin1_text.iter()
        .filter(|text_chunk| text_chunk.keyword == "chara")
        .map(|text_chunk| text_chunk.text.clone())
        .next();
    let character_base64: String = match character_base64_option {
        Some(v) => v,
        None => {
            let text_chunk_start = bytes.windows(9).position(|window| window == b"tEXtchara").ok_or_else(|| 
                pyo3::exceptions::PyValueError::new_err(
                    r#"Failed to find 'chara' metadata (tEXt chunk) in the PNG file. This may occur due to:
            1. The file is not a valid character card (Tavern Card V1 format).
            2. The file may be corrupted or incomplete.
            3. The character data might be stored in a different format.
            
            Please ensure that:
            - You are using a file created by a compatible character creation tool.
            - The file hasn't been modified or damaged.
            - You are using the correct file format for your character data.
            
            If the problem persists, try re-exporting the character from its original creation tool."#
                ))?;
            let text_chunk_end = bytes.windows(4).rposition(|window| window == b"IEND").ok_or_else(|| 
                pyo3::exceptions::PyValueError::new_err(
                    r#"Failed to find 'chara' metadata (tEXt chunk) in the PNG file. This may occur due to:
            1. The file is not a valid character card (Tavern Card V1 format).
            2. The file may be corrupted or incomplete.
            3. The character data might be stored in a different format.
            
            Please ensure that:
            - You are using a file created by a compatible character creation tool.
            - The file hasn't been modified or damaged.
            - You are using the correct file format for your character data.
            
            If the problem persists, try re-exporting the character from its original creation tool."#
                ))?;
            String::from_utf8_lossy(&bytes[text_chunk_start + 10..text_chunk_end - 8]).to_string()
        }
    };
    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let character_bytes = match engine.decode(character_base64) {
        Ok(b) => b,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error while decoding base64 character data from character card: {:?}",
                e
            )));
        }
    };
    let character_text: &str = match std::str::from_utf8(&character_bytes) {
        Ok(s) => s,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "Error while parsing decoded base64 bytes to utf8 string: {:?}",
                e
            )));
        }
    };
    let char_data: LoadCharacterClass =
        serde_json::from_str(character_text).expect("Your image file does not contain correct json data");

    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

#[pyfunction]
fn load_character_card_file(path: &str) -> PyResult<CharacterClass> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    let decoder = png::Decoder::new(&mut reader);
    let reader_info = decoder.read_info().map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to read PNG info: {}", e)))?;
    let character_base64 = reader_info.info().uncompressed_latin1_text.iter()
        .find(|text_chunk| text_chunk.keyword == "chara")
        .map(|text_chunk| text_chunk.text.clone());

    let character_base64 = if let Some(base64) = character_base64 {
        base64
    } else {
        reader.seek(SeekFrom::Start(0))?;
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        find_chara_chunk(&buffer)
            .ok_or_else(|| 
                pyo3::exceptions::PyValueError::new_err(
                    r#"Failed to find 'chara' metadata (tEXt chunk) in the PNG file. This may occur due to:
            1. The file is not a valid character card (Tavern Card V1 format).
            2. The file may be corrupted or incomplete.
            3. The character data might be stored in a different format.
            
            Please ensure that:
            - You are using a file created by a compatible character creation tool.
            - The file hasn't been modified or damaged.
            - You are using the correct file format for your character data.
            
            If the problem persists, try re-exporting the character from its original creation tool."#
                ))?
    };

    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let character_bytes = engine.decode(character_base64)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Error while decoding base64 character data: {:?}", e)))?;

    let character_text = std::str::from_utf8(&character_bytes)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Error while parsing decoded base64 bytes to utf8 string: {:?}", e)))?;

    let char_data: LoadCharacterClass = serde_json::from_str(character_text)
        .map_err(|_| pyo3::exceptions::PyValueError::new_err("Your image file does not contain correct json data"))?;

    Ok(CharacterClass {
        name: char_data.char_name.or(char_data.name).unwrap_or_default(),
        summary: char_data.summary.or(char_data.description).unwrap_or_default(),
        personality: char_data.char_persona.or(char_data.personality).unwrap_or_default(),
        scenario: char_data.world_scenario.or(char_data.scenario).unwrap_or_default(),
        greeting_message: char_data.char_greeting.or(char_data.first_mes).unwrap_or_default(),
        example_messages: char_data.example_dialogue.or(char_data.mes_example).unwrap_or_default(),
        image_path: Some(path.to_string()),
        created_time: char_data.metadata.and_then(|time_metadata| time_metadata.created),
    })
}

fn find_chara_chunk(buffer: &[u8]) -> Option<String> {
    let chara_marker = b"tEXtchara";
    let iend_marker = b"IEND";

    let start = buffer.windows(chara_marker.len())
        .position(|window| window == chara_marker)?;
    let end = buffer.windows(iend_marker.len())
        .rposition(|window| window == iend_marker)?;

    Some(String::from_utf8_lossy(&buffer[start + chara_marker.len() + 1..end - 8]).into_owned())
}

#[pyfunction]
fn license() -> &'static str {
    r#"
    MIT License

    Copyright (c) 2023-2025 Hubert Kasperek
    
    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"), to deal
    in the Software without restriction, including without limitation the rights
    to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
    copies of the Software, and to permit persons to whom the Software is
    furnished to do so, subject to the following conditions:
    
    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.
    
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
    "#
}

#[pymodule]
fn aichar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CharacterClass>()?;
    m.add_function(wrap_pyfunction!(create_character, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_yaml, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_yaml_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_card, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_card_file, m)?)?;
    m.add_function(wrap_pyfunction!(license, m)?)?;
    Ok(())
}
