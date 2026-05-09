# QuotaOverrideRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | Pointer to **NullableString** |  | [optional]
**LimitValue** | **int64** |  |
**Period** | Pointer to **NullableString** |  | [optional]
**QuotaKey** | **string** |  |
**Reason** | Pointer to **NullableString** |  | [optional]
**SubjectId** | **string** |  |
**SubjectType** | **string** |  |

## Methods

### NewQuotaOverrideRequest

`func NewQuotaOverrideRequest(limitValue int64, quotaKey string, subjectId string, subjectType string, ) *QuotaOverrideRequest`

NewQuotaOverrideRequest instantiates a new QuotaOverrideRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQuotaOverrideRequestWithDefaults

`func NewQuotaOverrideRequestWithDefaults() *QuotaOverrideRequest`

NewQuotaOverrideRequestWithDefaults instantiates a new QuotaOverrideRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *QuotaOverrideRequest) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *QuotaOverrideRequest) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *QuotaOverrideRequest) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *QuotaOverrideRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *QuotaOverrideRequest) SetExpiresAtNil(b bool)`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *QuotaOverrideRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil
### GetLimitValue

`func (o *QuotaOverrideRequest) GetLimitValue() int64`

GetLimitValue returns the LimitValue field if non-nil, zero value otherwise.

### GetLimitValueOk

`func (o *QuotaOverrideRequest) GetLimitValueOk() (*int64, bool)`

GetLimitValueOk returns a tuple with the LimitValue field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLimitValue

`func (o *QuotaOverrideRequest) SetLimitValue(v int64)`

SetLimitValue sets LimitValue field to given value.


### GetPeriod

`func (o *QuotaOverrideRequest) GetPeriod() string`

GetPeriod returns the Period field if non-nil, zero value otherwise.

### GetPeriodOk

`func (o *QuotaOverrideRequest) GetPeriodOk() (*string, bool)`

GetPeriodOk returns a tuple with the Period field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPeriod

`func (o *QuotaOverrideRequest) SetPeriod(v string)`

SetPeriod sets Period field to given value.

### HasPeriod

`func (o *QuotaOverrideRequest) HasPeriod() bool`

HasPeriod returns a boolean if a field has been set.

### SetPeriodNil

`func (o *QuotaOverrideRequest) SetPeriodNil(b bool)`

 SetPeriodNil sets the value for Period to be an explicit nil

### UnsetPeriod
`func (o *QuotaOverrideRequest) UnsetPeriod()`

UnsetPeriod ensures that no value is present for Period, not even an explicit nil
### GetQuotaKey

`func (o *QuotaOverrideRequest) GetQuotaKey() string`

GetQuotaKey returns the QuotaKey field if non-nil, zero value otherwise.

### GetQuotaKeyOk

`func (o *QuotaOverrideRequest) GetQuotaKeyOk() (*string, bool)`

GetQuotaKeyOk returns a tuple with the QuotaKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQuotaKey

`func (o *QuotaOverrideRequest) SetQuotaKey(v string)`

SetQuotaKey sets QuotaKey field to given value.


### GetReason

`func (o *QuotaOverrideRequest) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *QuotaOverrideRequest) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *QuotaOverrideRequest) SetReason(v string)`

SetReason sets Reason field to given value.

### HasReason

`func (o *QuotaOverrideRequest) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *QuotaOverrideRequest) SetReasonNil(b bool)`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *QuotaOverrideRequest) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil
### GetSubjectId

`func (o *QuotaOverrideRequest) GetSubjectId() string`

GetSubjectId returns the SubjectId field if non-nil, zero value otherwise.

### GetSubjectIdOk

`func (o *QuotaOverrideRequest) GetSubjectIdOk() (*string, bool)`

GetSubjectIdOk returns a tuple with the SubjectId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubjectId

`func (o *QuotaOverrideRequest) SetSubjectId(v string)`

SetSubjectId sets SubjectId field to given value.


### GetSubjectType

`func (o *QuotaOverrideRequest) GetSubjectType() string`

GetSubjectType returns the SubjectType field if non-nil, zero value otherwise.

### GetSubjectTypeOk

`func (o *QuotaOverrideRequest) GetSubjectTypeOk() (*string, bool)`

GetSubjectTypeOk returns a tuple with the SubjectType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubjectType

`func (o *QuotaOverrideRequest) SetSubjectType(v string)`

SetSubjectType sets SubjectType field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
