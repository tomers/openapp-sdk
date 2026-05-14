# language: en
@mock_api @tier1 @story_crud
Feature: CRUD read paths (mocked HTTP)
  Repository reads beyond single writes: org detail, collections, device scope.

  Scenario: Fetch a single organization by id
    Given path "/orgs/org_9" responds to GET with status 200 and JSON
      """
      {"id": "org_9", "name": "Regional HQ"}
      """
    And the async client is connected with status probe skipped
    When the async client gets org "org_9"
    Then the last JSON field "id" equals string "org_9"
    And the last JSON field "name" equals string "Regional HQ"

  Scenario: List devices scoped to an organization
    Given path "/devices" responds to GET with status 200 and JSON
      """
      {"items": [{"id": "dev_1", "name": "Gate A"}], "next_cursor": null}
      """
    And the async client is connected with status probe skipped
    When the async client lists devices for org "org_1"
    Then the last JSON list at key "items" has length 1

  Scenario: Org collection list returns an envelope
    Given path "/orgs" responds to GET with status 200 and JSON
      """
      {"items": [{"id": "o1", "name": "Acme"}], "next_cursor": null}
      """
    And the async client is connected with status probe skipped
    When the async client lists orgs
    Then the last JSON list at key "items" has length 1
