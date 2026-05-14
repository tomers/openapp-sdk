# language: en
@mock_api @tier1 @story_retries
Feature: Transport retries (mocked HTTP)
  The SDK retries transient HTTP responses before returning success or a terminal error.

  Scenario: Org list succeeds after one transient 503
    Given path "/orgs" responds to GET with status 503 once then status 200 and JSON
      """
      {"items": [], "next_cursor": null}
      """
    And the async client is connected with short retries and status probe skipped
    When the async client lists orgs
    Then the last JSON list at key "items" has length 0
