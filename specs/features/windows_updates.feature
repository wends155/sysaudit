Feature: Windows Updates
  As a sysaudit user
  I want to list installed Windows Updates
  So that I can verify patch status

  Scenario: Collect updates
    Given a Windows machine
    When updates are collected
    Then result is a list of WindowsUpdate entries

  Scenario: Date format slash
    Given WMI date "01/15/2024"
    When WMI date is parsed
    Then it becomes 2024-01-15

  Scenario: Date format ISO
    Given WMI date "2024-01-15"
    When WMI date is parsed
    Then it becomes 2024-01-15

  Scenario: Date format compact
    Given WMI date "20240115"
    When WMI date is parsed
    Then it becomes 2024-01-15

  Scenario: Invalid date
    Given WMI date "not-a-date"
    Then the update date result is None

  Scenario: Empty hotfix ID skipped
    Given an update entry with blank HotFixID
    Then the update is excluded from results

  Scenario: Graceful WMI failure
    Given WMI is unavailable
    When updates are collected
    Then empty list is returned
    And a warning is logged
