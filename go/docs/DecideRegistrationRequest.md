# DecideRegistrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Decision** | **string** | &#x60;approve&#x60; or &#x60;reject&#x60;. |
**DisplayName** | Pointer to [**NullableLocalizedString**](LocalizedString.md) | Optional localized display name for the new member&#39;s personal billing account. | [optional]

## Methods

### NewDecideRegistrationRequest

`func NewDecideRegistrationRequest(decision string, ) *DecideRegistrationRequest`

NewDecideRegistrationRequest instantiates a new DecideRegistrationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDecideRegistrationRequestWithDefaults

`func NewDecideRegistrationRequestWithDefaults() *DecideRegistrationRequest`

NewDecideRegistrationRequestWithDefaults instantiates a new DecideRegistrationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDecision

`func (o *DecideRegistrationRequest) GetDecision() string`

GetDecision returns the Decision field if non-nil, zero value otherwise.

### GetDecisionOk

`func (o *DecideRegistrationRequest) GetDecisionOk() (*string, bool)`

GetDecisionOk returns a tuple with the Decision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDecision

`func (o *DecideRegistrationRequest) SetDecision(v string)`

SetDecision sets Decision field to given value.


### GetDisplayName

`func (o *DecideRegistrationRequest) GetDisplayName() LocalizedString`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *DecideRegistrationRequest) GetDisplayNameOk() (*LocalizedString, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *DecideRegistrationRequest) SetDisplayName(v LocalizedString)`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *DecideRegistrationRequest) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *DecideRegistrationRequest) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *DecideRegistrationRequest) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
