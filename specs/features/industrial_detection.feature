Feature: Industrial Detection
  As a sysaudit user
  I want to detect industrial software
  So that I can identify SCADA and automation systems

  Scenario: All vendor scan
    Given a scanner with all vendors
    When industrial scan runs
    Then it checks all known vendors

  Scenario: Specific vendor filter
    Given a scanner for "Rockwell" and "ABB"
    When industrial scan runs
    Then only those vendors are scanned

  Scenario: Citect detection
    Given software named "Citect SCADA 2018"
    When classified
    Then it is identified as Citect

  Scenario: AVEVA detection
    Given software named "AVEVA InTouch"
    When classified
    Then it is identified as Citect

  Scenario: AVEVA without SCADA keyword
    Given software named "AVEVA Accounting"
    When classified
    Then it is NOT identified as industrial

  Scenario: Rockwell detection
    Given software named "Studio 5000 Logix Designer"
    When classified
    Then it is identified as Rockwell

  Scenario: Siemens detection
    Given software named "Siemens TIA Portal V17"
    When classified
    Then it is identified as Siemens

  Scenario: ABB detection
    Given software named "ABB Ability System 800xA"
    When classified
    Then it is identified as ABB

  Scenario: Digifort detection
    Given software named "Digifort Enterprise"
    When classified
    Then it is identified as Digifort

  Scenario: Generic software ignored
    Given software named "Microsoft Word"
    When classified
    Then it is NOT identified as industrial
