# Aichar
[![PyPI version](https://badge.fury.io/py/aichar.svg)](https://badge.fury.io/py/aichar)
[![Downloads](https://static.pepy.tech/badge/aichar)](https://pepy.tech/project/aichar)
[![Downloads per month](https://img.shields.io/pypi/dm/aichar.svg)]()

Python library for creating/editing/transporting AI characters between different frontends ([TavernAI](https://github.com/TavernAI/TavernAI), [SillyTavern](https://github.com/SillyTavern/SillyTavern), [TextGenerationWebUI](https://github.com/oobabooga/text-generation-webui), [AI-companion](https://github.com/Hukasx0/ai-companion), Pygmalion) and character generation tools like [Character Factory](https://github.com/Hukasx0/character-factory).

This library allows you to read JSON, Yaml and character card files, edit their data, create your characters from scratch and export them as JSON, Yaml or character cards compatible with the frontends mentioned above. It's also used by Character Factory - a tool for generating characters using LLM and Stable Diffusion.
## Installation
```py
pip install aichar
```
and you can follow the steps from [Usage](#usage) to use the library

## Building library
If you are only interested in downloading the library and using it, just follow the [installation step](#installation).

This option is only for building manually, code experiments, testing, or when the `pip install aichar` command cannot download the library for some reason.

This library uses PyO3 and Maturin for building, so we will follow the steps from [this documentation](https://pyo3.rs/v0.20.2/getting_started).

1. Install [Rust and Cargo](https://www.rust-lang.org/) and [Python](https://www.python.org/)
2. clone git repository
```sh
git clone https://github.com/Hukasx0/aichar
```
```
cd aichar/
```
3. Create a Virtual Environment
```sh
python -m venv venv
```
And activate the virtual environment
```sh
source venv/bin/activate
```
for windows use
```
venv\Scripts\activate
```
4. install Maturin (in a virtual environment)
```
pip install maturin
```
5. Build the Library

development:
```sh
maturin develop
```
production:
```sh
maturin build
```
6. Test in Python
```py
import aichar

print( aichar.license() ) # it should print the library license to the console
```

## Usage
### Creating a Character
To create a new character, you can use the create_character function. This function takes several parameters to initialize the character's attributes and returns a CharacterClass object.
```py
import aichar

character = aichar.create_character(
    name="Character Name",
    summary="Character Summary",
    personality="Character Personality",
    scenario="Character Scenario",
    greeting_message="Character Greeting Message",
    example_messages="Character Example Messages",
    image_path="Character Image Path"
)
```

### Loading a Character data from a PNG Character Card File
```py
character = aichar.load_character_card_file("character_card.png")
```

### Loading a Character data from a PNG Character Card Bytes
```py
character = aichar.load_character_card(data_bytes)
```

Where ***data_bytes*** can be e.g. bytes of the opened png file of the character card
```py
with open("character_card.png", 'rb') as file:
   data_bytes = file.read()
```

### Loading a Character data from a JSON File
```py
character = aichar.load_character_json_file("character.json")
```

### Loading a Character data from a JSON String
```py
character = aichar.load_character_json('{"char_name": "Character Name", "char_persona": "Character Personality", "world_scenario": "Character Scenario", "char_greeting": "Character Greeting Message", "example_dialogue": "Character Example Messages", "name": "Character Name", "description": "Character Summary", "personality": "Character Personality", "scenario": "Character Scenario", "first_mes": "Character Greeting Message", "mes_example": "Character Example Messages"}')
```

### Loading a Character data from a Yaml File
```py
character = aichar.load_character_yaml_file("character.yaml")
```

### Loading a Character data from a Yaml String
```py
character = aichar.load_character_yaml('char_name: Character Name\nchar_persona: Character Personality\nworld_scenario: Character Scenario\nchar_greeting: Character Greeting Message\nexample_dialogue: Character Example Messages\nname: Character Name\ndescription: Character Summary\npersonality: Character Personality\nscenario: Character Scenario\nfirst_mes: Character Greeting Message\nmes_example: Character Example Messages\nmetadata:\n  version: 1\n  created: 1696945481977\n  modified: 1696945481977\n  source: null\n  tool:\n    name: aichar Python library\n    version: 0.5.0\n    url: https://github.com/Hukasx0/aichar\n')
```

### Modifying Character Attributes
You can modify the attributes of a character. Here are some examples:
```py
# Load a character card from a JSON file
character = aichar.load_character_json_file("character_data.json")

# Change character name
character.name = "New Name"

# Change character summary
character.summary = "New Summary"

# Change character personality
character.personality = "New Personality"

# Change character scenario
character.scenario = "New Scenario"

# Change character greeting message
character.greeting_message = "New Greeting Message"

# Change character example messages
character.example_messages = "New Example Messages"

# Change character image path (needed if you want to export character as character png card)
character.image_path = "New Image Path"
```

### Printing Character Information Summary
You can get character's information summary by using the data_summary attribute:
```py
print(character.data_summary)
```

### Accessing Character Attributes
You can access character's attributes using the provided getter methods. For example:
```py
print("Character Name: ", character.name)
print("Character Summary: ", character.summary)
print("Character Personality: ", character.personality)
image_path = character.image_path
```

### Exporting Character Data
You can export the character's data in different formats using the export_card_file, export_json, export_json_file, export_yaml and export_yaml_file function. Supported export formats include "tavernai" (or "sillytavern"), "textgenerationwebui" (or "pygmalion"), and "aicompanion". 

exporting data as character card png:
```py
# Export character card in "tavernai" format
character.export_card_file("tavernai", "tavernai_character_card.png")

# Export character card in "sillytavern" format
character.export_card_file("sillytavern", "sillytavern_character_card.png")

# Export character card in "textgenerationwebui" format
character.export_card_file("textgenerationwebui", "textgenerationwebui_character_card.png")

# Export character card in "pygmalion" format
character.export_card_file("pygmalion", "pygmalion_character_card.png")

# Export character card in "aicompanion" format
character.export_card_file("aicompanion", "aicompanion_character_card.png")
```

exporting data as json string or file:
```py
# Export character data in "tavernai" format
tavernai_json_string = character.export_json("tavernai")
# or to file
character.export_json_file("tavernai", "tavernai_character_data.json")

# Export character data in "sillytavern" format
sillytavern_json_string = character.export_json("sillytavern")
# or to file
character.export_json_file("sillytavern", "sillytavern_character_data.json")

# Export character data in "textgenerationwebui" format
textgenerationwebui_json_string = character.export_json("textgenerationwebui")
# or to file
character.export_json_file("textgenerationwebui", "textgenerationwebui_character_data.json")

# Export character data in "pygmalion" format
pygmalion_json_string = character.export_json("pygmalion")
# or to file
character.export_json_file("pygmalion", "pygmalion_character_data.json")

# Export character data in "aicompanion" format
aicompanion_json_string = character.export_json("aicompanion")
# or to file
character.export_json_file("aicompanion", "companion_character_data.json")
```

exporting data as yaml string or file:
```py
# Export character data in "tavernai" format
tavernai_yaml_string = character.export_yaml("tavernai")
# or to file
character.export_yaml_file("tavernai", "tavernai_character_data.yml")

# Export character data in "sillytavern" format
sillytavern_yaml_string = character.export_yaml("sillytavern")
# or to file
character.export_yaml_file("sillytavern", "sillytavern_character_data.yml")

# Export character data in "textgenerationwebui" format
textgenerationwebui_yaml_string = character.export_yaml("textgenerationwebui")
# or to file
character.export_yaml_file("textgenerationwebui", "textgenerationwebui_character_data.yml")

# Export character data in "pygmalion" format
pygmalion_yaml_string = character.export_yaml("pygmalion")
# or to file
character.export_yaml_file("pygmalion", "pygmalion_character_data.yml")

# Export character data in "aicompanion" format
aicompanion_yaml_string = character.export_yaml("aicompanion")
# or to file
character.export_yaml_file("aicompanion", "companion_character_data.yml")
```

Or you can export it in neutral format for those frontends:
```py
neutral_json_string = character.export_neutral_json()
neutral_yaml_string = character.export_neutral_yaml()
character.export_neutral_json_file("neutral_character_data.json")
character.export_neutral_yaml_file("neutral_character_data.yml")
character.export_neutral_card_file("neutral_card_name.png")
```

## Related Projects

### Character Factory
[Character Factory](https://github.com/Hukasx0/character-factory) is a powerful tool for generating AI characters using Large Language Models (LLM) and Stable Diffusion. It uses this aichar library to create character cards compatible with various AI frontends.

**Features:**
- Generate characters with LLM (Mistral, Zephyr)
- Create character avatars with Stable Diffusion
- Web interface for easy character generation
- Direct export to aichar-compatible formats
- Support for custom prompts and topics
- Uses neutral export format (JSON/card) for maximum compatibility across all frontends

**Usage with aichar:**
```python
# Character Factory generates characters using neutral format for universal compatibility
character = aichar.load_character_card_file("generated_character.card.png")
print(f"Generated character: {character.name}")

# Character Factory uses neutral export, equivalent to:
# character.export_neutral_json()  # Works with all frontends
# character.export_neutral_card()  # Universal character card format
```

## Exporting character cards as bytes
```py
character_neutral_bytes_list = character.export_neutral_card()
# you can also export in any format you choose
character_sillytavern_bytes_list = character.export_card("sillytavern")
```
Why bytes_list and not just bytes?

Both .export_neutral_card() and .export_card() methods return 'bytes': 'list', if you need bytes then you can use the python function bytes() to convert the data to 'PyBytes'.
For example, you will get an error like this: 
```TypeError: argument 'bytes': 'list' object cannot be converted to 'PyBytes'```

Example of a solution to a problem:
```py
character_neutral_bytes = bytes(character.export_neutral_card())
# then you can perform the same operations on it as you would on bytes
new_character = aichar.load_character_card(character_neutral_bytes)
```

## License
2023-2025 Hubert Kasperek

At any time when using the library, you can read the content of the license by calling the ***.license()*** method
```py
print( aichar.license() )
```

This library is distributed under the [MIT License](https://github.com/Hukasx0/aichar/blob/main/LICENSE).
