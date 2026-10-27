# QuietHoursPolicyConfig

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AppliesTo** | Pointer to [**[]PolicyPrincipalKind**](PolicyPrincipalKind.md) | Principal kinds the row binds; unknown and empty values are rejected. Omitted binds the type&#39;s default principals. | [optional]
**End** | **string** | Inclusive local end of the window, in &#x60;HH:MM&#x60;; a later end wraps midnight. |
**Start** | **string** | Inclusive local start of the window, in &#x60;HH:MM&#x60;. |

## Methods

### NewQuietHoursPolicyConfig

`func NewQuietHoursPolicyConfig(end string, start string, ) *QuietHoursPolicyConfig`

NewQuietHoursPolicyConfig instantiates a new QuietHoursPolicyConfig object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQuietHoursPolicyConfigWithDefaults

`func NewQuietHoursPolicyConfigWithDefaults() *QuietHoursPolicyConfig`

NewQuietHoursPolicyConfigWithDefaults instantiates a new QuietHoursPolicyConfig object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAppliesTo

`func (o *QuietHoursPolicyConfig) GetAppliesTo() []PolicyPrincipalKind`

GetAppliesTo returns the AppliesTo field if non-nil, zero value otherwise.

### GetAppliesToOk

`func (o *QuietHoursPolicyConfig) GetAppliesToOk() (*[]PolicyPrincipalKind, bool)`

GetAppliesToOk returns a tuple with the AppliesTo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAppliesTo

`func (o *QuietHoursPolicyConfig) SetAppliesTo(v []PolicyPrincipalKind)`

SetAppliesTo sets AppliesTo field to given value.

### HasAppliesTo

`func (o *QuietHoursPolicyConfig) HasAppliesTo() bool`

HasAppliesTo returns a boolean if a field has been set.

### SetAppliesToNil

`func (o *QuietHoursPolicyConfig) SetAppliesToNil(b bool)`

 SetAppliesToNil sets the value for AppliesTo to be an explicit nil

### UnsetAppliesTo
`func (o *QuietHoursPolicyConfig) UnsetAppliesTo()`

UnsetAppliesTo ensures that no value is present for AppliesTo, not even an explicit nil
### GetEnd

`func (o *QuietHoursPolicyConfig) GetEnd() string`

GetEnd returns the End field if non-nil, zero value otherwise.

### GetEndOk

`func (o *QuietHoursPolicyConfig) GetEndOk() (*string, bool)`

GetEndOk returns a tuple with the End field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnd

`func (o *QuietHoursPolicyConfig) SetEnd(v string)`

SetEnd sets End field to given value.


### GetStart

`func (o *QuietHoursPolicyConfig) GetStart() string`

GetStart returns the Start field if non-nil, zero value otherwise.

### GetStartOk

`func (o *QuietHoursPolicyConfig) GetStartOk() (*string, bool)`

GetStartOk returns a tuple with the Start field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStart

`func (o *QuietHoursPolicyConfig) SetStart(v string)`

SetStart sets Start field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
