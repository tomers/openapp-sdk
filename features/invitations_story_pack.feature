# language: en
@mock_api @tier1 @story_invitations
Feature: Invitation and public-access stories (mocked HTTP)
  Me-scoped invitations plus opaque public invite flows.

  Scenario: Invitations list can be empty
    Given path "/me/invitations" responds to GET with status 200 and JSON
      """
      []
      """
    And the async client is connected with status probe skipped
    When the async client lists invitations
    Then the last JSON is an empty list

  Scenario: Pending and expired invites appear together
    Given path "/me/invitations" responds to GET with status 200 and JSON
      """
      [
        {"id": "inv_p", "status": "pending"},
        {"id": "inv_e", "status": "expired"}
      ]
      """
    And the async client is connected with status probe skipped
    When the async client lists invitations
    Then the last JSON list has length 2
    And the invitations list includes status "pending"
    And the invitations list includes status "expired"

  Scenario: Public invite metadata is readable by token
    Given path "/public/access/invites/tok_abcd" responds to GET with status 200 and JSON
      """
      {
        "invite_token": "tok_abcd",
        "status": "active",
        "valid_from": "2026-01-01T00:00:00Z"
      }
      """
    And the async client is connected with status probe skipped
    When the async client fetches public invite "tok_abcd"
    Then the last JSON has key "status"
    And the last JSON field "status" equals string "active"

  Scenario: Public invite claim returns a session envelope
    Given path "/public/access/invites/tok_claim/claim" responds to POST with status 200 and JSON
      """
      {"session_id": "sess_1", "status": "created"}
      """
    And the async client is connected with status probe skipped
    When the async client claims public invite "tok_claim" with empty body
    Then the last JSON has key "session_id"
    And the last JSON field "session_id" equals string "sess_1"
