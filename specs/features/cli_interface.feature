Feature: CLI Interface
  As a sysaudit user
  I want to use the command line interface
  So that I can run audits from scripts

  Scenario: Help output
    Given the CLI application
    When run with "--help" flag
    Then stdout contains "Windows System & Software Auditor"

  Scenario: Invalid arguments
    Given the CLI application
    When run with "--invalid-flag"
    Then exit code is non-zero
    And stderr contains "unexpected argument"

  Scenario: System command
    Given the CLI application
    When run with "system" command
    Then stdout contains "SYSTEM INFORMATION"

  Scenario: System JSON
    Given the CLI application
    When run with "system --format json"
    Then output is valid JSON

  Scenario: Software filter
    Given the CLI application
    When run with "software --filter Microsoft --format json"
    Then output is valid JSON array

  Scenario: Software CSV export
    Given the CLI application
    When run with "software --format csv -o test_software.csv"
    Then file "test_software.csv" is created
    And file content starts with "Name,Version,Publisher"

  Scenario: Industrial vendors
    Given the CLI application
    When run with "industrial --vendors citect,abb"
    Then output contains "Citect" or "ABB"

  Scenario: Updates command
    Given the CLI application
    When run with "updates" command
    Then stdout contains "Description"

  Scenario: Full audit
    Given the CLI application
    When run with "all" command
    Then output contains system info
    And output contains software list
    And output contains updates list
