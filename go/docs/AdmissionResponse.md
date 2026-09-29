# AdmissionResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AdmissionCount** | **int32** |  |
**AdmittedAt** | Pointer to **NullableString** |  | [optional]
**DeniedAt** | Pointer to **NullableString** |  | [optional]
**DisplayName** | Pointer to **interface{}** |  | [optional]
**FirstSeenAt** | Pointer to **NullableString** |  | [optional]
**MemberRef** | **string** |  |
**RemovedAt** | Pointer to **NullableString** |  | [optional]
**Status** | **string** |  |

## Methods

### NewAdmissionResponse

`func NewAdmissionResponse(admissionCount int32, memberRef string, status string, ) *AdmissionResponse`

NewAdmissionResponse instantiates a new AdmissionResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdmissionResponseWithDefaults

`func NewAdmissionResponseWithDefaults() *AdmissionResponse`

NewAdmissionResponseWithDefaults instantiates a new AdmissionResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAdmissionCount

`func (o *AdmissionResponse) GetAdmissionCount() int32`

GetAdmissionCount returns the AdmissionCount field if non-nil, zero value otherwise.

### GetAdmissionCountOk

`func (o *AdmissionResponse) GetAdmissionCountOk() (*int32, bool)`

GetAdmissionCountOk returns a tuple with the AdmissionCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAdmissionCount

`func (o *AdmissionResponse) SetAdmissionCount(v int32)`

SetAdmissionCount sets AdmissionCount field to given value.


### GetAdmittedAt

`func (o *AdmissionResponse) GetAdmittedAt() string`

GetAdmittedAt returns the AdmittedAt field if non-nil, zero value otherwise.

### GetAdmittedAtOk

`func (o *AdmissionResponse) GetAdmittedAtOk() (*string, bool)`

GetAdmittedAtOk returns a tuple with the AdmittedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAdmittedAt

`func (o *AdmissionResponse) SetAdmittedAt(v string)`

SetAdmittedAt sets AdmittedAt field to given value.

### HasAdmittedAt

`func (o *AdmissionResponse) HasAdmittedAt() bool`

HasAdmittedAt returns a boolean if a field has been set.

### SetAdmittedAtNil

`func (o *AdmissionResponse) SetAdmittedAtNil(b bool)`

 SetAdmittedAtNil sets the value for AdmittedAt to be an explicit nil

### UnsetAdmittedAt
`func (o *AdmissionResponse) UnsetAdmittedAt()`

UnsetAdmittedAt ensures that no value is present for AdmittedAt, not even an explicit nil
### GetDeniedAt

`func (o *AdmissionResponse) GetDeniedAt() string`

GetDeniedAt returns the DeniedAt field if non-nil, zero value otherwise.

### GetDeniedAtOk

`func (o *AdmissionResponse) GetDeniedAtOk() (*string, bool)`

GetDeniedAtOk returns a tuple with the DeniedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeniedAt

`func (o *AdmissionResponse) SetDeniedAt(v string)`

SetDeniedAt sets DeniedAt field to given value.

### HasDeniedAt

`func (o *AdmissionResponse) HasDeniedAt() bool`

HasDeniedAt returns a boolean if a field has been set.

### SetDeniedAtNil

`func (o *AdmissionResponse) SetDeniedAtNil(b bool)`

 SetDeniedAtNil sets the value for DeniedAt to be an explicit nil

### UnsetDeniedAt
`func (o *AdmissionResponse) UnsetDeniedAt()`

UnsetDeniedAt ensures that no value is present for DeniedAt, not even an explicit nil
### GetDisplayName

`func (o *AdmissionResponse) GetDisplayName() interface{}`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *AdmissionResponse) GetDisplayNameOk() (*interface{}, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *AdmissionResponse) SetDisplayName(v interface{})`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *AdmissionResponse) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *AdmissionResponse) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *AdmissionResponse) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetFirstSeenAt

`func (o *AdmissionResponse) GetFirstSeenAt() string`

GetFirstSeenAt returns the FirstSeenAt field if non-nil, zero value otherwise.

### GetFirstSeenAtOk

`func (o *AdmissionResponse) GetFirstSeenAtOk() (*string, bool)`

GetFirstSeenAtOk returns a tuple with the FirstSeenAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFirstSeenAt

`func (o *AdmissionResponse) SetFirstSeenAt(v string)`

SetFirstSeenAt sets FirstSeenAt field to given value.

### HasFirstSeenAt

`func (o *AdmissionResponse) HasFirstSeenAt() bool`

HasFirstSeenAt returns a boolean if a field has been set.

### SetFirstSeenAtNil

`func (o *AdmissionResponse) SetFirstSeenAtNil(b bool)`

 SetFirstSeenAtNil sets the value for FirstSeenAt to be an explicit nil

### UnsetFirstSeenAt
`func (o *AdmissionResponse) UnsetFirstSeenAt()`

UnsetFirstSeenAt ensures that no value is present for FirstSeenAt, not even an explicit nil
### GetMemberRef

`func (o *AdmissionResponse) GetMemberRef() string`

GetMemberRef returns the MemberRef field if non-nil, zero value otherwise.

### GetMemberRefOk

`func (o *AdmissionResponse) GetMemberRefOk() (*string, bool)`

GetMemberRefOk returns a tuple with the MemberRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemberRef

`func (o *AdmissionResponse) SetMemberRef(v string)`

SetMemberRef sets MemberRef field to given value.


### GetRemovedAt

`func (o *AdmissionResponse) GetRemovedAt() string`

GetRemovedAt returns the RemovedAt field if non-nil, zero value otherwise.

### GetRemovedAtOk

`func (o *AdmissionResponse) GetRemovedAtOk() (*string, bool)`

GetRemovedAtOk returns a tuple with the RemovedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRemovedAt

`func (o *AdmissionResponse) SetRemovedAt(v string)`

SetRemovedAt sets RemovedAt field to given value.

### HasRemovedAt

`func (o *AdmissionResponse) HasRemovedAt() bool`

HasRemovedAt returns a boolean if a field has been set.

### SetRemovedAtNil

`func (o *AdmissionResponse) SetRemovedAtNil(b bool)`

 SetRemovedAtNil sets the value for RemovedAt to be an explicit nil

### UnsetRemovedAt
`func (o *AdmissionResponse) UnsetRemovedAt()`

UnsetRemovedAt ensures that no value is present for RemovedAt, not even an explicit nil
### GetStatus

`func (o *AdmissionResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AdmissionResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AdmissionResponse) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
