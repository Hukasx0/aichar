/*
    MIT License

    Copyright (c) 2023 Hubert Kasperek

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
use serde_json;
use base64::{Engine, engine::GeneralPurpose, engine::GeneralPurposeConfig, alphabet::STANDARD};
use std::fs::File;
use std::io::{Read, Write};

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
    fn set_image_path(&mut self, image_path: &str) {
        self.image_path = Some(image_path.to_string());
    }

    fn print_summary(&self) -> PyResult<()> {
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

        println!("{}", character_str);
        Ok(())
    }

    fn export_neutral_json(&self) -> PyResult<String> {
        let export_class: ExportAllCharacterClass = ExportAllCharacterClass {
            char_name: &self.name,
            char_persona: if self.personality.is_empty() {
                &self.summary
            } else {
                &self.personality
            },
            world_scenario: &self.scenario,
            char_greeting: &self.greeting_message,
            example_dialogue: &self.example_messages,
            name: &self.name,
            description: &self.summary,
            personality: &self.personality,
            scenario: &self.scenario,
            first_mes: &self.greeting_message,
            mes_example: &self.example_messages,
        };
        Ok(serde_json::to_string_pretty(&export_class).expect("Error while serializing JSON"))
    }

    fn export_neutral_json_file(&self, export_json_path: &str) -> PyResult<()> {
        let json_string = self.export_neutral_json()?;
        let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
        file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        Ok(())
    }

    fn export_json(&self, format_type: &str) -> PyResult<String> {
        match format_type.to_lowercase().as_str() {
            "tavernai" | "sillytavern" => {
                let export: ExportTavernAi = ExportTavernAi {
                    name: &self.name,
                    description: &self.summary,
                    personality: &self.personality,
                    scenario: &self.scenario,
                    first_mes: &self.greeting_message,
                    mes_example: &self.example_messages,
                };
                Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
            },
            "textgenerationwebui" | "pygmalion" => {
                let export: ExportTextGenerationWebuiPygmalion = ExportTextGenerationWebuiPygmalion {
                    char_name: &self.name,
                    char_persona: if self.personality.is_empty() {
                        &self.summary
                    } else {
                        &self.personality
                    },
                    world_scenario: &self.scenario,
                    char_greeting: &self.greeting_message,
                    example_dialogue: &self.example_messages,
                };
                Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
            },
            "aicompanion" => {
                let export: ExportAiCompanion = ExportAiCompanion {
                    name: &self.name,
                    description: if self.personality.is_empty() {
                        &self.summary
                    } else {
                        &self.personality
                    },
                    first_mes: &self.greeting_message,
                    mes_example: &self.example_messages,
                };
                Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"))
            },
            _ => {
                Err(pyo3::exceptions::PyValueError::new_err("Format not supported, supported formats: 'tavernai', 'sillytavern', 'textgenerationwebui', 'pygmalion', 'aicompanion'"))
            }
        }
    }

    fn export_json_file(&self, format_type: &str, export_json_path: &str) -> PyResult<()> {
        let json_string = self.export_json(format_type)?;
        let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
        file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        Ok(())
    }

/*                  TODO: Add export method as character cards
    fn export_character_card_file(&self, export_card_path: &str) -> PyResult<()> {
    }

    fn export_card_file(&self, type: &str, export_card_path: &str) -> PyResult<()> {
    }
*/
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
}

#[derive(Serialize)]
struct ExportTavernAi<'a> {
    name: &'a str,
    description: &'a str,
    personality: &'a str,
    scenario: &'a str,
    first_mes: &'a str,
    mes_example: &'a str,
}

#[derive(Serialize)]
struct ExportTextGenerationWebuiPygmalion<'a> {
    char_name: &'a str,
    char_persona: &'a str,
    world_scenario: &'a str,
    char_greeting: &'a str,
    example_dialogue: &'a str,
}

#[derive(Serialize)]
struct ExportAiCompanion<'a> {
    name: &'a str,
    description: &'a str,
    first_mes: &'a str,
    mes_example: &'a str,
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
    })
}

#[pyfunction]
fn load_character_json_file(path: &str) -> PyResult<CharacterClass> {
    let mut file = File::open(path).expect(&format!("Cannot open file at path: {}", path));
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect(&format!("Error while reading file at path: {}", path));
    let char_data: LoadCharacterClass = serde_json::from_str(&buf).expect("Error while parsing json file");
    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
    })
}

#[pyfunction]
fn load_character_card_file(path: &str) -> PyResult<CharacterClass> {
    let mut card_file = File::open(path).expect("File at this path does not exist");
    let mut data = Vec::new();
    match card_file.read_to_end(&mut data) {
        Ok(_) => {},
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!("Error while reading character card file: {:?}", e)));
        }
    };
    let text_chunk_start = data.windows(9).position(|window| window == b"tEXtchara").expect("Looks like this image does not contain character data");
    let text_chunk_end = data.windows(4).rposition(|window| window == b"IEND").expect("Looks like this image does not contain character data");
    let character_base64 = &data[text_chunk_start + 10..text_chunk_end - 8];
    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let character_bytes = match engine.decode(character_base64) {
        Ok(b) => b,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!("Error while decoding base64 character data from character card: {:?}", e)));
        }
    };
    let character_text: &str = match std::str::from_utf8(&character_bytes) {
        Ok(s) => s,
        Err(e) => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!("Error while parsing decoded base64 bytes to utf8 string: {:?}", e)));
        }
    };
    let char_data: LoadCharacterClass = serde_json::from_str(character_text).expect("Your image file does not contain correct json data");
    Ok(CharacterClass {
        name: char_data.char_name.unwrap_or(char_data.name.unwrap_or(String::from(""))),
        summary: char_data.summary.unwrap_or(char_data.description.unwrap_or(String::from(""))),
        personality: char_data.char_persona.unwrap_or(char_data.personality.unwrap_or(String::from(""))),
        scenario: char_data.world_scenario.unwrap_or(char_data.scenario.unwrap_or(String::from(""))),
        greeting_message: char_data.char_greeting.unwrap_or(char_data.first_mes.unwrap_or(String::from(""))),
        example_messages: char_data.example_dialogue.unwrap_or(char_data.mes_example.unwrap_or(String::from(""))),
        image_path: None,
    })
}

#[pymodule]
fn aichar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_character, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_card_file, m)?)?;
    Ok(())
}
