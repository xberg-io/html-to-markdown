"""Check how CI installs the governance generator."""

from pathlib import Path
import re
import unittest


class AiRulezVersionTests(unittest.TestCase):
    """mise.toml pins the local generator exactly; CI installations take the latest release."""

    def test_should_install_latest_in_all_ci_generator_installations(self) -> None:
        """Every explicit CI generator installation takes the latest release."""
        root = Path(__file__).resolve().parents[2]
        versions = []
        for path in (root / ".github/workflows").iterdir():
            versions.extend(re.findall(r'ai-rulez-version: "([^"]+)"', path.read_text()))
        assert len(versions) == 6, versions
        assert set(versions) == {"latest"}, versions


if __name__ == "__main__":
    unittest.main()
