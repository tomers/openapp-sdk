# language: en
@mock_api @tier0 @smoke
Feature: CRUD smoke (mocked HTTP)
  SDK-visible repository actions mapped to HTTP.

  Scenario: Entity action maps to POST
    Given path "/entities/g1/actions/open" responds to POST with status 200 and JSON
      """
      {"ok": true}
      """
    And the async client is connected with status probe skipped
    When the async client runs entity action "open" on entity "g1"
    Then the last JSON field "ok" is boolean true
