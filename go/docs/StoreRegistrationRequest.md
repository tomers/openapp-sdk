# StoreRegistrationRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**DecidedAt** | Pointer to **NullableString** |  | [optional]
**DecidedBy** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  |
**Note** | Pointer to **NullableString** |  | [optional]
**OrgId** | **string** |  |
**Status** | [**RegistrationStatus**](RegistrationStatus.md) |  |
**StoreId** | **string** |  |
**UserId** | **string** |  |

## Methods

### NewStoreRegistrationRequest

`func NewStoreRegistrationRequest(id string, orgId string, status RegistrationStatus, storeId string, userId string, ) *StoreRegistrationRequest`

NewStoreRegistrationRequest instantiates a new StoreRegistrationRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewStoreRegistrationRequestWithDefaults

`func NewStoreRegistrationRequestWithDefaults() *StoreRegistrationRequest`

NewStoreRegistrationRequestWithDefaults instantiates a new StoreRegistrationRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *StoreRegistrationRequest) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *StoreRegistrationRequest) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *StoreRegistrationRequest) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *StoreRegistrationRequest) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *StoreRegistrationRequest) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *StoreRegistrationRequest) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetDecidedAt

`func (o *StoreRegistrationRequest) GetDecidedAt() string`

GetDecidedAt returns the DecidedAt field if non-nil, zero value otherwise.

### GetDecidedAtOk

`func (o *StoreRegistrationRequest) GetDecidedAtOk() (*string, bool)`

GetDecidedAtOk returns a tuple with the DecidedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDecidedAt

`func (o *StoreRegistrationRequest) SetDecidedAt(v string)`

SetDecidedAt sets DecidedAt field to given value.

### HasDecidedAt

`func (o *StoreRegistrationRequest) HasDecidedAt() bool`

HasDecidedAt returns a boolean if a field has been set.

### SetDecidedAtNil

`func (o *StoreRegistrationRequest) SetDecidedAtNil(b bool)`

 SetDecidedAtNil sets the value for DecidedAt to be an explicit nil

### UnsetDecidedAt
`func (o *StoreRegistrationRequest) UnsetDecidedAt()`

UnsetDecidedAt ensures that no value is present for DecidedAt, not even an explicit nil
### GetDecidedBy

`func (o *StoreRegistrationRequest) GetDecidedBy() string`

GetDecidedBy returns the DecidedBy field if non-nil, zero value otherwise.

### GetDecidedByOk

`func (o *StoreRegistrationRequest) GetDecidedByOk() (*string, bool)`

GetDecidedByOk returns a tuple with the DecidedBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDecidedBy

`func (o *StoreRegistrationRequest) SetDecidedBy(v string)`

SetDecidedBy sets DecidedBy field to given value.

### HasDecidedBy

`func (o *StoreRegistrationRequest) HasDecidedBy() bool`

HasDecidedBy returns a boolean if a field has been set.

### SetDecidedByNil

`func (o *StoreRegistrationRequest) SetDecidedByNil(b bool)`

 SetDecidedByNil sets the value for DecidedBy to be an explicit nil

### UnsetDecidedBy
`func (o *StoreRegistrationRequest) UnsetDecidedBy()`

UnsetDecidedBy ensures that no value is present for DecidedBy, not even an explicit nil
### GetId

`func (o *StoreRegistrationRequest) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *StoreRegistrationRequest) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *StoreRegistrationRequest) SetId(v string)`

SetId sets Id field to given value.


### GetNote

`func (o *StoreRegistrationRequest) GetNote() string`

GetNote returns the Note field if non-nil, zero value otherwise.

### GetNoteOk

`func (o *StoreRegistrationRequest) GetNoteOk() (*string, bool)`

GetNoteOk returns a tuple with the Note field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNote

`func (o *StoreRegistrationRequest) SetNote(v string)`

SetNote sets Note field to given value.

### HasNote

`func (o *StoreRegistrationRequest) HasNote() bool`

HasNote returns a boolean if a field has been set.

### SetNoteNil

`func (o *StoreRegistrationRequest) SetNoteNil(b bool)`

 SetNoteNil sets the value for Note to be an explicit nil

### UnsetNote
`func (o *StoreRegistrationRequest) UnsetNote()`

UnsetNote ensures that no value is present for Note, not even an explicit nil
### GetOrgId

`func (o *StoreRegistrationRequest) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *StoreRegistrationRequest) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *StoreRegistrationRequest) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetStatus

`func (o *StoreRegistrationRequest) GetStatus() RegistrationStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *StoreRegistrationRequest) GetStatusOk() (*RegistrationStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *StoreRegistrationRequest) SetStatus(v RegistrationStatus)`

SetStatus sets Status field to given value.


### GetStoreId

`func (o *StoreRegistrationRequest) GetStoreId() string`

GetStoreId returns the StoreId field if non-nil, zero value otherwise.

### GetStoreIdOk

`func (o *StoreRegistrationRequest) GetStoreIdOk() (*string, bool)`

GetStoreIdOk returns a tuple with the StoreId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStoreId

`func (o *StoreRegistrationRequest) SetStoreId(v string)`

SetStoreId sets StoreId field to given value.


### GetUserId

`func (o *StoreRegistrationRequest) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *StoreRegistrationRequest) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *StoreRegistrationRequest) SetUserId(v string)`

SetUserId sets UserId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
