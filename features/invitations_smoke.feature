# language: en
@mock_api @tier0 @smoke
Feature: Invitation list (mocked HTTP)
  Minimal invitation read path for cross-SDK parity.

  Scenario: Current user invitations are listed
    Given path "/me/invitations" responds to GET with status 200 and JSON
      """
      [{"id": "inv_1", "status": "pending"}]
      """
    And the async client is connected with status probe skipped
    When the async client lists invitations
    Then the last JSON is a non-empty list
