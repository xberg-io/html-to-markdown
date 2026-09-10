"""Keep reporting recovery limited to the verified original publication."""

from __future__ import annotations

import unittest

import pytest

from validate_published_release import RELEASE_RUN, RELEASE_SHA, RELEASE_TAG, REPOSITORY, REQUIRED_JOBS, validate


class PublishedReleaseValidationTests(unittest.TestCase):
    """Reject source substitutions and incomplete target publication."""

    def setUp(self) -> None:
        """Construct a complete run with only its known reporting failure."""
        self.run = {
            "id": RELEASE_RUN,
            "repository": {"full_name": REPOSITORY},
            "event": "release",
            "path": ".github/workflows/publish.yaml",
            "head_sha": RELEASE_SHA,
            "head_branch": RELEASE_TAG,
            "status": "completed",
        }
        self.pages = [{"jobs": [{"name": name, "conclusion": "success"} for name in REQUIRED_JOBS]}]
        self.pages[0]["jobs"].append({"name": "Report release outcome", "conclusion": "failure"})
        self.release = {"tag_name": RELEASE_TAG, "draft": False, "published_at": "2026-09-09T18:40:09Z"}

    def test_complete_publication_accepts_only_reporting_failure(self) -> None:
        """A reporting failure does not invalidate proven publication."""
        validate(self.run, self.pages, self.release)

    def test_source_substitution_is_rejected(self) -> None:
        """A different source SHA cannot stand in for the original release."""
        self.run["head_sha"] = "0" * 40
        with pytest.raises(ValueError, match="original"):
            validate(self.run, self.pages, self.release)

    def test_skipped_required_publication_is_rejected(self) -> None:
        """Skipped required targets remain incomplete even without failures."""
        self.pages[0]["jobs"][0]["conclusion"] = "skipped"
        with pytest.raises(ValueError, match="did not succeed"):
            validate(self.run, self.pages, self.release)

    def test_other_failure_is_rejected(self) -> None:
        """Recovery cannot hide another failure in the release matrix."""
        self.pages[0]["jobs"].append({"name": "Build native", "conclusion": "failure"})
        with pytest.raises(ValueError, match="Unexpected"):
            validate(self.run, self.pages, self.release)

    def test_draft_release_is_rejected(self) -> None:
        """Published metadata must describe a public release."""
        self.release["draft"] = True
        with pytest.raises(ValueError, match="public"):
            validate(self.run, self.pages, self.release)
