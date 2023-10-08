# Aichar
[![PyPI version](https://badge.fury.io/py/aichar.svg)](https://badge.fury.io/py/aichar)

Python library for creating/transporting AI characters between different frontends ([TavernAI](https://github.com/TavernAI/TavernAI), [SillyTavern](https://github.com/SillyTavern/SillyTavern), [TextGenerationWebUI](https://github.com/oobabooga/text-generation-webui), [AI-companion](https://github.com/Hukasx0/ai-companion), Pygmalion) 

## Installation
```py
pip install aichar
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

### Loading a Character Card from a PNG Character Card File
```py
character = aichar.load_character_card_file("character_card.png")
```

### Loading a Character Card from a JSON File
```py
character = aichar.load_character_json_file("character.json")
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
You can print the character's information to the console using the print_summary function:
```py
character.print_summary()
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
You can export the character's data in different formats using the export_json function. Supported export formats include "tavernai" (or "sillytavern"), "textgenerationwebui" (or "pygmalion"), and "aicompanion."
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
Or you can export it in neutral format for those frontends:
```py
neutral_json_string = character.export_neutral_json()
character.export_neutral_json_file("neutral_character_data.json")
```

## License
2023 Hubert Kasperek

This library is distributed under the MIT License.
