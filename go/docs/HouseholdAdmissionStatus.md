# HouseholdAdmissionStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**MemberName** | Pointer to **interface{}** |  | [optional]
**MemberRef** | Pointer to **NullableString** |  | [optional]
**OrgName** | **interface{}** |  |
**Status** | **string** |  |

## Methods

### NewHouseholdAdmissionStatus

`func NewHouseholdAdmissionStatus(orgName interface{}, status string, ) *HouseholdAdmissionStatus`

NewHouseholdAdmissionStatus instantiates a new HouseholdAdmissionStatus object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHouseholdAdmissionStatusWithDefaults

`func NewHouseholdAdmissionStatusWithDefaults() *HouseholdAdmissionStatus`

NewHouseholdAdmissionStatusWithDefaults instantiates a new HouseholdAdmissionStatus object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMemberName

`func (o *HouseholdAdmissionStatus) GetMemberName() interface{}`

GetMemberName returns the MemberName field if non-nil, zero value otherwise.

### GetMemberNameOk

`func (o *HouseholdAdmissionStatus) GetMemberNameOk() (*interface{}, bool)`

GetMemberNameOk returns a tuple with the MemberName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemberName

`func (o *HouseholdAdmissionStatus) SetMemberName(v interface{})`

SetMemberName sets MemberName field to given value.

### HasMemberName

`func (o *HouseholdAdmissionStatus) HasMemberName() bool`

HasMemberName returns a boolean if a field has been set.

### SetMemberNameNil

`func (o *HouseholdAdmissionStatus) SetMemberNameNil(b bool)`

 SetMemberNameNil sets the value for MemberName to be an explicit nil

### UnsetMemberName
`func (o *HouseholdAdmissionStatus) UnsetMemberName()`

UnsetMemberName ensures that no value is present for MemberName, not even an explicit nil
### GetMemberRef

`func (o *HouseholdAdmissionStatus) GetMemberRef() string`

GetMemberRef returns the MemberRef field if non-nil, zero value otherwise.

### GetMemberRefOk

`func (o *HouseholdAdmissionStatus) GetMemberRefOk() (*string, bool)`

GetMemberRefOk returns a tuple with the MemberRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMemberRef

`func (o *HouseholdAdmissionStatus) SetMemberRef(v string)`

SetMemberRef sets MemberRef field to given value.

### HasMemberRef

`func (o *HouseholdAdmissionStatus) HasMemberRef() bool`

HasMemberRef returns a boolean if a field has been set.

### SetMemberRefNil

`func (o *HouseholdAdmissionStatus) SetMemberRefNil(b bool)`

 SetMemberRefNil sets the value for MemberRef to be an explicit nil

### UnsetMemberRef
`func (o *HouseholdAdmissionStatus) UnsetMemberRef()`

UnsetMemberRef ensures that no value is present for MemberRef, not even an explicit nil
### GetOrgName

`func (o *HouseholdAdmissionStatus) GetOrgName() interface{}`

GetOrgName returns the OrgName field if non-nil, zero value otherwise.

### GetOrgNameOk

`func (o *HouseholdAdmissionStatus) GetOrgNameOk() (*interface{}, bool)`

GetOrgNameOk returns a tuple with the OrgName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgName

`func (o *HouseholdAdmissionStatus) SetOrgName(v interface{})`

SetOrgName sets OrgName field to given value.


### SetOrgNameNil

`func (o *HouseholdAdmissionStatus) SetOrgNameNil(b bool)`

 SetOrgNameNil sets the value for OrgName to be an explicit nil

### UnsetOrgName
`func (o *HouseholdAdmissionStatus) UnsetOrgName()`

UnsetOrgName ensures that no value is present for OrgName, not even an explicit nil
### GetStatus

`func (o *HouseholdAdmissionStatus) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *HouseholdAdmissionStatus) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *HouseholdAdmissionStatus) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
