import typing

class CharacterClass:
    """Character card representation

    :ivar name: Character name
    :ivar summary: Short character description
    :ivar personality: Personality traits
    :ivar scenario: Current situation context
    :ivar greeting_message: Initial greeting
    :ivar example_messages: Sample conversation history
    :ivar image_path: Optional PNG image path
    """

    @property
    def name(self) -> str: ...
    @name.setter
    def name(self, name: str) -> None: ...
    @property
    def summary(self) -> str: ...
    @summary.setter
    def summary(self, summary: str) -> None: ...
    @property
    def personality(self) -> str: ...
    @personality.setter
    def personality(self, personality: str) -> None: ...
    @property
    def scenario(self) -> str: ...
    @scenario.setter
    def scenario(self, scenario: str) -> None: ...
    @property
    def greeting_message(self) -> str: ...
    @greeting_message.setter
    def greeting_message(self, greeting_message: str) -> None: ...
    @property
    def example_messages(self) -> str: ...
    @example_messages.setter
    def example_messages(self, example_messages: str) -> None: ...
    @property
    def image_path(self) -> typing.Optional[str]: ...
    @image_path.setter
    def image_path(self, image_path: str) -> typing.Optional[Exception]: ...
    @property
    def data_summary(self) -> str:
        """Get all character data as formatted string"""

    def export_json(self, format_type: str) -> str:
        """Export character as JSON string

        :param format_type: Target format ('tavernai', 'sillytavern',
                            'textgenerationwebui', 'pygmalion', 'aicompanion')
        """

    def export_json_file(self, format_type: str, export_json_path: str) -> None:
        """Export character to JSON file

        :param format_type: Target format
        :param export_json_path: Output file path
        """

    def export_neutral_json(self) -> str:
        """Export as cross-compatible JSON"""

    def export_neutral_json_file(self, export_json_path: str) -> None:
        """Export cross-compatible JSON to file"""

    def export_yaml(self, format_type: str) -> str:
        """Export character as YAML string

        :param format_type: Target format
        """

    def export_yaml_file(self, format_type: str, export_yaml_path: str) -> None:
        """Export character to YAML file"""

    def export_neutral_yaml(self) -> str:
        """Export as cross-compatible YAML"""

    def export_neutral_yaml_file(self, export_yaml_path: str) -> None:
        """Export cross-compatible YAML to file"""

    def export_card(self, format_type: str) -> bytes:
        """Export as character card (PNG bytes)

        :param format_type: Target format or 'neutral'
        :raises ValueError: Missing PNG image path
        """

    def export_card_file(self, format_type: str, export_card_path: str) -> None:
        """Export character card to PNG file"""

    def export_neutral_card(self) -> bytes:
        """Export cross-compatible character card"""

    def export_neutral_card_file(self, export_card_path: str) -> None:
        """Export cross-compatible card to file"""

def create_character(
    name: str,
    summary: str,
    personality: str,
    scenario: str,
    greeting_message: str,
    example_messages: str,
    image_path: str,
) -> CharacterClass:
    """Create new character instance

    :param name: Character name
    :param summary: Short description
    :param personality: Personality traits
    :param scenario: Situation context
    :param greeting_message: Initial greeting
    :param example_messages: Sample conversation
    :param image_path: PNG image path
    """

def load_character_json(json: str) -> CharacterClass:
    """Load character from JSON string"""

def load_character_json_file(path: str) -> CharacterClass:
    """Load character from JSON file"""

def load_character_yaml(yaml: str) -> CharacterClass:
    """Load character from YAML string"""

def load_character_yaml_file(path: str) -> CharacterClass:
    """Load character from YAML file"""

def load_character_card(bytes: bytes) -> CharacterClass:
    """Load character from card bytes
    :raises ValueError: Invalid character card format
    """

def load_character_card_file(path: str) -> CharacterClass:
    """Load character from PNG card file
    :raises ValueError: Invalid character card format
    """

def license() -> str:
    """Get license text"""
