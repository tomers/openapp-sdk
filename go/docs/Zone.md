# Zone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**Id** | **string** | Unique identifier (ULID). |
**Name** | [**LocalizedString**](LocalizedString.md) | Zone name. |
**OrgId** | **string** | Organization that owns this zone. |
**ParentZoneId** | Pointer to **NullableString** | Parent zone (same org) for hierarchy. | [optional]

## Methods

### NewZone

`func NewZone(id string, name LocalizedString, orgId string, ) *Zone`

NewZone instantiates a new Zone object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneWithDefaults

`func NewZoneWithDefaults() *Zone`

NewZoneWithDefaults instantiates a new Zone object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *Zone) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *Zone) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *Zone) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *Zone) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *Zone) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *Zone) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *Zone) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *Zone) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *Zone) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *Zone) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *Zone) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *Zone) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *Zone) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *Zone) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *Zone) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *Zone) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *Zone) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *Zone) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *Zone) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *Zone) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetId

`func (o *Zone) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *Zone) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *Zone) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *Zone) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *Zone) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *Zone) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *Zone) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *Zone) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *Zone) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetParentZoneId

`func (o *Zone) GetParentZoneId() string`

GetParentZoneId returns the ParentZoneId field if non-nil, zero value otherwise.

### GetParentZoneIdOk

`func (o *Zone) GetParentZoneIdOk() (*string, bool)`

GetParentZoneIdOk returns a tuple with the ParentZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetParentZoneId

`func (o *Zone) SetParentZoneId(v string)`

SetParentZoneId sets ParentZoneId field to given value.

### HasParentZoneId

`func (o *Zone) HasParentZoneId() bool`

HasParentZoneId returns a boolean if a field has been set.

### SetParentZoneIdNil

`func (o *Zone) SetParentZoneIdNil(b bool)`

 SetParentZoneIdNil sets the value for ParentZoneId to be an explicit nil

### UnsetParentZoneId
`func (o *Zone) UnsetParentZoneId()`

UnsetParentZoneId ensures that no value is present for ParentZoneId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
