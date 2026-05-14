# language: en
@mock_api @tier1 @story_errors
Feature: Auth and validation errors (mocked HTTP)
  SDK-visible failures for credentials and request validation.

  Scenario: Revoked token surfaces as AuthError on org list
    Given path "/orgs" responds to GET with status 401 and JSON
      """
      {"message": "token revoked"}
      """
    And the async client is connected with status probe skipped
    When the async client lists orgs and records any error
    Then an AuthError was raised
    And the AuthError message contains "token revoked"

  Scenario: Empty org name surfaces as ApiError
    Given path "/orgs" responds to POST with status 400 and JSON
      """
      {"code": "validation_error", "message": "name is required"}
      """
    And the async client is connected with status probe skipped
    When the async client creates an org with empty name and records any error
    Then an ApiError was raised
    And the ApiError has code "validation_error"
