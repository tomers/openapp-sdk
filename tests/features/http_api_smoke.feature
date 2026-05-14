# language: en
@mock_api @tier0 @smoke
Feature: HTTP API smoke against mocked transport
  Exercises the SDK HTTP bridge shape without a live backend.

  Scenario: Status GET returns JSON
    Given path "/status" responds to GET with status 200 and JSON
      """
      {"backend": "ok"}
      """
    And the async client is connected with status probe skipped
    When the async client fetches status
    Then the last JSON has key "backend"

  Scenario: Org creation POST returns resource
    Given path "/orgs" responds to POST with status 201 and JSON
      """
      {"id": "org_1", "name": "Acme"}
      """
    And the async client is connected with status probe skipped
    When the async client creates an org named "Acme"
    Then the last JSON has key "id"
    And the last JSON field "name" equals string "Acme"
