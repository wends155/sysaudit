Feature: Error Handling
  As a developer
  I want distinct error types
  So that I can handle failures gracefully

  Scenario: Registry error type
    Given a registry failure
    When converted to Error
    Then it matches Error::Registry variant

  Scenario: WMI error type
    Given a WMI failure
    When converted to Error
    Then it matches Error::Wmi variant

  Scenario: IO error type
    Given an IO failure
    When converted to Error
    Then it matches Error::Io variant

  Scenario: No panics in library
    Given a library function
    When it fails
    Then it returns Result::Err
    And it does not panic

  Scenario: CLI error exit
    Given the CLI application
    When a command fails
    Then stderr contains error message
    And exit code is 1

  Scenario: WMI graceful degradation
    Given the WMI service is unavailable
    Then empty vec + warning, no error propagated
