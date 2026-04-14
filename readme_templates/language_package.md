# html-to-markdown

{% include 'partials/_badges.md' %}

{{ description }}

## Installation

{% include 'partials/_installation.md' %}

{% if migration_guide %}
{{ migration_guide }}
{% endif %}

{% if performance %}

## Performance Snapshot

{{ performance | render_performance_table(name) }}

{% endif %}

## Quick Start

{% include 'partials/_quick_start.md' %}

## API Reference

{% include 'partials/_api_reference.md' %}

{% include 'partials/_djot_output.md' %}

{% include 'partials/_plain_text_output.md' %}

{% if features.metadata_extraction %}

## Metadata Extraction

{% include 'partials/_metadata_extraction.md' %}
{% endif %}

{% if features.visitor_pattern %}

## Visitor Pattern

{% include 'partials/_visitor_pattern.md' %}
{% endif %}

## Examples


{% include 'partials/_footer.md' %}
