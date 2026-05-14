# language: en
@tier0 @smoke @example
Feature: SDK smoke placeholder
  This is a structural placeholder for the shared Gherkin corpus.
  Replace or extend with real scenarios (CRUD, auth, invitations) as adapters are implemented per language.

  Scenario: SDK is importable and version is defined
    Given the SDK is installed
    When the client library is imported
    Then a non-empty version string is available
