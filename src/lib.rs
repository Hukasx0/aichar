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
fn change_character_name(mut character_data: CharacterClass, name: &str) -> PyResult<CharacterClass> {
    character_data.name = name.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_summary(mut character_data: CharacterClass, summary: &str) -> PyResult<CharacterClass> {
    character_data.summary = summary.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_personality(mut character_data: CharacterClass, personality: &str) -> PyResult<CharacterClass> {
    character_data.personality = personality.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_scenario(mut character_data: CharacterClass, scenario: &str) -> PyResult<CharacterClass> {
    character_data.scenario = scenario.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_greeting_message(mut character_data: CharacterClass, greeting_message: &str) -> PyResult<CharacterClass> {
    character_data.greeting_message = greeting_message.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_example_messages(mut character_data: CharacterClass, example_messages: &str) -> PyResult<CharacterClass> {
    character_data.example_messages = example_messages.to_string();
    Ok(character_data)
}

#[pyfunction]
fn change_character_image_path(mut character_data: CharacterClass, image_path: &str) -> PyResult<CharacterClass> {
    character_data.image_path = Some(image_path.to_string());
    Ok(character_data)
}

#[pyfunction]
fn print_character(character_data: CharacterClass) -> PyResult<()> {
    let mut character_str = String::new();

    character_str.push_str(&format!("Name: {}\n", character_data.name));
    character_str.push_str(&format!("Summary: {}\n", character_data.summary));
    character_str.push_str(&format!("Personality: {}\n", character_data.personality));
    character_str.push_str(&format!("Scenario: {}\n", character_data.scenario));
    character_str.push_str(&format!("Greeting Message: {}\n", character_data.greeting_message));
    character_str.push_str(&format!("Example Messages: \n{}\n", character_data.example_messages));

    match &character_data.image_path {
        Some(path) => character_str.push_str(&format!("Image Path: {}\n", path)),
        None => character_str.push_str("Image Path: None\n"),
    }
    
    println!("{}", character_str);
    Ok(())
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

#[pyfunction]
fn export_character_json(character_data: CharacterClass) -> PyResult<String> {
    let export_class: ExportAllCharacterClass = ExportAllCharacterClass {
        char_name: &character_data.name,
        char_persona: if (&character_data.personality).is_empty() {
            &character_data.summary
        } else {
            &character_data.personality
        },
        world_scenario: &character_data.scenario,
        char_greeting: &character_data.greeting_message,
        example_dialogue: &character_data.example_messages,
        name: &character_data.name,
        description: &character_data.summary,
        personality: &character_data.personality,
        scenario: &character_data.scenario,
        first_mes: &character_data.greeting_message,
        mes_example: &character_data.example_messages,
    };
    Ok(serde_json::to_string(&export_class).expect("Error while serializing JSON"))
}

#[pyfunction]
fn export_character_json_file(character_data: CharacterClass, export_json_path: &str) -> PyResult<()> {
    let export_class: ExportAllCharacterClass = ExportAllCharacterClass {
        char_name: &character_data.name,
        char_persona: if (&character_data.personality).is_empty() {
            &character_data.summary
        } else {
            &character_data.personality
        },
        world_scenario: &character_data.scenario,
        char_greeting: &character_data.greeting_message,
        example_dialogue: &character_data.example_messages,
        name: &character_data.name,
        description: &character_data.summary,
        personality: &character_data.personality,
        scenario: &character_data.scenario,
        first_mes: &character_data.greeting_message,
        mes_example: &character_data.example_messages,
    };
    let json_string = serde_json::to_string_pretty(&export_class).expect("Error while serializing JSON");
    let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
    file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
    Ok(())
}

/*
#[pyfunction]
fn export_character_card_file(character_data: CharacterClass, export_card_path: &str) -> PyResult<()> {
}
*/

#[pyfunction]
fn export_json(character_data: CharacterClass, format_type: &str) -> PyResult<String>  {
    match format_type.to_lowercase().as_str() {
        "tavernai" => {
            let export: ExportTavernAi = ExportTavernAi {
                name: &character_data.name,
                description: &character_data.summary,
                personality: &character_data.personality,
                scenario: &character_data.scenario,
                first_mes: &character_data.greeting_message,
                mes_example: &character_data.example_messages,
            };
            return Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"));
        },
        "textgenerationwebui" | "pygmalion" => {
            let export: ExportTextGenerationWebuiPygmalion = ExportTextGenerationWebuiPygmalion {
                char_name: &character_data.name,
                char_persona: if (&character_data.personality).is_empty() {
                    &character_data.summary
                } else {
                    &character_data.personality
                },
                world_scenario: &character_data.scenario,
                char_greeting: &character_data.greeting_message,
                example_dialogue: &character_data.example_messages,
            };
            return Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"));
        },
        "aicompanion" => {
            let export: ExportAiCompanion = ExportAiCompanion {
                name: &character_data.name,
                description: if (&character_data.personality).is_empty() {
                    &character_data.summary
                } else {
                    &character_data.personality
                },
                first_mes: &character_data.greeting_message,
                mes_example: &character_data.example_messages,
            };
            return Ok(serde_json::to_string_pretty(&export).expect("Error while serializing JSON"));
        },
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err("Format not supported, supported formats: 'tavernai', 'textgenerationwebui', 'pygmalion', 'aicompanion'"));
        }
    }
}

#[pyfunction]
fn export_json_file(character_data: CharacterClass, format_type: &str, export_json_path: &str) -> PyResult<()>  {
    match format_type.to_lowercase().as_str() {
        "tavernai" => {
            let export: ExportTavernAi = ExportTavernAi {
                name: &character_data.name,
                description: &character_data.summary,
                personality: &character_data.personality,
                scenario: &character_data.scenario,
                first_mes: &character_data.greeting_message,
                mes_example: &character_data.example_messages,
            };
            let json_string = serde_json::to_string_pretty(&export).expect("Error while serializing JSON");
            let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
            file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        },
        "textgenerationwebui" | "pygmalion" => {
            let export: ExportTextGenerationWebuiPygmalion = ExportTextGenerationWebuiPygmalion {
                char_name: &character_data.name,
                char_persona: if (&character_data.personality).is_empty() {
                    &character_data.summary
                } else {
                    &character_data.personality
                },
                world_scenario: &character_data.scenario,
                char_greeting: &character_data.greeting_message,
                example_dialogue: &character_data.example_messages,
            };
            let json_string = serde_json::to_string_pretty(&export).expect("Error while serializing JSON");
            let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
            file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        },
        "aicompanion" => {
            let export: ExportAiCompanion = ExportAiCompanion {
                name: &character_data.name,
                description: if (&character_data.personality).is_empty() {
                    &character_data.summary
                } else {
                    &character_data.personality
                },
                first_mes: &character_data.greeting_message,
                mes_example: &character_data.example_messages,
            };
            let json_string = serde_json::to_string_pretty(&export).expect("Error while serializing JSON");
            let mut file = File::create(export_json_path).expect(&format!("Cannot create file at path: {}", export_json_path));
            file.write_all(json_string.as_bytes()).expect("Error while writing to json file");
        },
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err("Format not supported, supported formats: 'tavernai', 'textgenerationwebui', 'pygmalion', 'aicompanion'"));
        }
    };
    Ok(())
}
/*
#[pyfunction]
fn export_card_file(character_data: CharacterClass, type: &str, export_card_path: &str) -> PyResult<()> {

}
*/

#[pymodule]
fn aichar(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_character, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_name, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_summary, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_personality, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_scenario, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_greeting_message, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_example_messages, m)?)?;
    m.add_function(wrap_pyfunction!(change_character_image_path, m)?)?;
    m.add_function(wrap_pyfunction!(print_character, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_json_file, m)?)?;
    m.add_function(wrap_pyfunction!(load_character_card_file, m)?)?;
    m.add_function(wrap_pyfunction!(export_character_json, m)?)?;
    m.add_function(wrap_pyfunction!(export_character_json_file, m)?)?;
//    m.add_function(wrap_pyfunction!(export_character_card_file, m)?)?;
    m.add_function(wrap_pyfunction!(export_json, m)?)?;
    m.add_function(wrap_pyfunction!(export_json_file, m)?)?;
    Ok(())
}
