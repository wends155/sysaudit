Feature: Software Scan
  As a sysaudit user
  I want to scan installed software
  So that I can audit installed applications

  Scenario: Scan all sources
    Given a default software scanner
    When scan runs
    Then results include 64-bit entries
    And results include 32-bit entries
    And results include user-specific entries

  Scenario: Required fields
    Given a default software scanner
    When scan runs
    Then every entry has a non-empty name

  Scenario: Empty names rejected
    Given a software entry with empty name
    When it is processed
    Then the software is excluded from results

  Scenario: Date parsing for software
    Given a date string "20240115"
    When it is parsed
    Then the parsed software date is 2024-01-15

  Scenario: Invalid date for software
    Given a date string "not-a-date"
    Then the software date result is None

  Scenario: Filter by name
    Given a list of software
    When filtered by "Microsoft"
    Then only matching entries remain

  Scenario: Exclude user installs
    Given a scanner with user installs disabled
    When scan runs
    Then no HKCU entries are returned
