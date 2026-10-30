"""Check that the governance generator is never pinned (org ADR-0015)."""

from pathlib import Path
import re
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]


def ai_rulez_revision(sources: list[dict]) -> str | None:
    """Return the revision of the single ai-rulez hook source."""
    found = [source for source in sources if source.get("id") == "ai-rulez"]
    assert len(found) == 1, found
    return found[0].get("revision")


class AiRulezVersionTests(unittest.TestCase):
    """ai-rulez runs its latest release: no mise pin, no literal CI version, hook source on v5."""

    def test_should_not_pin_in_any_mise_config(self) -> None:
        """No mise config carries an ai-rulez tool entry."""
        configs = sorted(ROOT.glob("mise*.toml"))
        assert configs, ROOT
        pinned = [
            path.name
            for path in configs
            if re.search(r'^\s*"github:Goldziher/ai-rulez"\s*=', path.read_text(), re.MULTILINE)
        ]
        assert pinned == [], pinned

    def test_should_install_latest_in_all_ci_generator_installations(self) -> None:
        """Every CI generator installation takes the latest release, never a literal version or setup-mise."""
        workflows = sorted((ROOT / ".github/workflows").glob("*.y*ml"))
        assert workflows, ROOT
        versions = []
        mise_installs = []
        for path in workflows:
            text = path.read_text()
            versions.extend(re.findall(r'ai-rulez-version: "?([^"\s#]+)', text))
            if re.search(r"^\s*install-args:.*github:Goldziher/ai-rulez", text, re.MULTILINE):
                mise_installs.append(path.name)
        assert set(versions) <= {"latest"}, versions
        assert mise_installs == [], mise_installs

    def test_should_follow_the_floating_major_tag_for_the_hook_source(self) -> None:
        """alef.toml and the poly.toml it renders take the ai-rulez hooks from v5."""
        alef = tomllib.loads((ROOT / "alef.toml").read_text())
        poly = tomllib.loads((ROOT / "poly.toml").read_text())
        assert ai_rulez_revision(alef["workspace"]["poly"]["hooks-sources"]) == "v5"
        assert ai_rulez_revision(poly["hooks"]["sources"]) == "v5"


if __name__ == "__main__":
    unittest.main()
