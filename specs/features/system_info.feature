Feature: System Information
  As a sysaudit user
  I want to collect detailed system information
  So that I can audit the machine's configuration

  Scenario: Collect basic system information
    Given a Windows machine
    When system info is collected
    Then OS name is non-empty
    And OS version is non-empty
    And build number is non-empty
    And computer name is non-empty

  Scenario: Build number format
    Given a Windows machine
    When build number is read
    Then it contains digits
    And it optionally has a UBR suffix

  Scenario: CPU information
    Given a Windows machine
    When system info is collected
    Then CPU brand is populated
    And physical core count is populated
    And logical core count is populated
    And CPU frequency is populated

  Scenario: Memory metrics
    Given a Windows machine
    When system info is collected
    Then total memory is non-zero
    And used memory is non-zero
    And free memory is non-zero

  Scenario: Network interfaces
    Given a Windows machine
    When system info is collected
    Then at least one network interface is found
    And each interface has a name
    And each interface has an IP address
    And each interface has a valid MAC address

  Scenario: WMI manufacturer and model
    Given a Windows machine
    When system info is collected
    Then manufacturer is populated if WMI is available
    And model is populated if WMI is available
