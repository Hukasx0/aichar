# Aichar
[![PyPI version](https://badge.fury.io/py/aichar.svg)](https://badge.fury.io/py/aichar)

Python library for creating/transporting AI characters between different frontends ([TavernAI](https://github.com/TavernAI/TavernAI), [TextGenerationWebUI](https://github.com/oobabooga/text-generation-webui), [AI-companion](https://github.com/Hukasx0/ai-companion), Pygmalion) 

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
You can modify the attributes of a character using various functions provided by the aichar library. Here are some examples:
```py
# Load a character card from a JSON file
character = aichar.load_character_json_file("character_data.json")

# Change character name
character = aichar.change_character_name(character, "New Name")

# Change character summary
character = aichar.change_character_summary(character, "New Summary")

# Change character personality
character = aichar.change_character_personality(character, "New Personality")

# Change character scenario
character = aichar.change_character_scenario(character, "New Scenario")

# Change character greeting message
character = aichar.change_character_greeting_message(character, "New Greeting Message")

# Change character example messages
character = aichar.change_character_example_messages(character, "New Example Messages")

# Change character image path (needed if you want to export character as character png card)
character = aichar.change_character_image_path(character, "New Image Path")
```

### Printing Character Information
You can print the character's information to the console using the print_character function:
```py
aichar.print_character(character)
```

### Exporting Character Data
You can export the character's data in different formats using the export_json function. Supported export formats include "tavernai," "textgenerationwebui" (or "pygmalion"), and "aicompanion."
```py
# Export character data in "tavernai" format
tavernai_data = aichar.export_json(character, "tavernai")
# or to file
aichar.export_json_file(character, "tavernai", "tavernai_character_data.json")

# Export character data in "textgenerationwebui" format
textgenerationwebui_data = aichar.export_json(character, "textgenerationwebui")
# or to file
aichar.export_json_file(character, "textgenerationwebui", "textgenerationwebui_character_data.json")

# Export character data in "pygmalion" format
pygmalion_data = aichar.export_json(character, "pygmalion")
# or to file
aichar.export_json_file(character, "pygmalion", "pygmalion_character_data.json")

# Export character data in "aicompanion" format
aicompanion_data = aichar.export_json(character, "aicompanion")
# or to file
aichar.export_json_file(character, "aicompanion", "companion_character_data.json")

```
Or you can export it in neutral format for those frontends:
```py
neutral_data = aichar.export_character_json(character)
aichar.export_character_json_file(character, "neutral_character_data.json")
```

## License
This library is distributed under the MIT License.
