Feature: Output Formats
  As a sysaudit user
  I want different output formats
  So that I can consume data in various ways

  Scenario: Console table for system
    Given system info data
    When formatted as table
    Then output contains "Computer Name" header

  Scenario: JSON for system
    Given system info data
    When formatted as JSON
    Then output is valid JSON object
    And contains "computer_name" field

  Scenario: CSV for software
    Given software list data
    When exported to CSV
    Then header contains "Name,Version,Publisher"
    And rows contain software details

  Scenario: CSV for industrial
    Given industrial software data
    When exported to CSV
    Then header contains "Vendor,Product,Version"

  Scenario: CSV for updates
    Given updates list data
    When exported to CSV
    Then header contains "HotFixID,Description"

  Scenario: JSON for software
    Given software list data
    When software is formatted as JSON
    Then output is valid JSON array
    And each entry contains "name" field
