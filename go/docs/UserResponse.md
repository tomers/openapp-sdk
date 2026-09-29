# UserResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**Email** | **string** | Email address (used for login and provisioning). May be an internal placeholder when no verified email is available. |
**Id** | **string** | Unique identifier (ULID). |
**MergedIntoUserId** | Pointer to **string** | When set, this row was absorbed into another user. Follow for live identity. | [optional]
**Name** | [**LocalizedString**](LocalizedString.md) | Display name. |
**Phone** | Pointer to **string** | Verified E.164 phone when set. | [optional]
**PhoneVerifiedAt** | Pointer to **string** |  | [optional]
**EmailIsSynthetic** | **bool** | True when &#x60;email&#x60; is an internal placeholder (phone-only account or identity-provider merge claimant) and must not be shown as a mailbox. |
**Roles** | **map[string][]string** |  |

## Methods

### NewUserResponse

`func NewUserResponse(email string, id string, name LocalizedString, emailIsSynthetic bool, roles map[string][]string, ) *UserResponse`

NewUserResponse instantiates a new UserResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUserResponseWithDefaults

`func NewUserResponseWithDefaults() *UserResponse`

NewUserResponseWithDefaults instantiates a new UserResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *UserResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *UserResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *UserResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *UserResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *UserResponse) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *UserResponse) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *UserResponse) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *UserResponse) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *UserResponse) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *UserResponse) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *UserResponse) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *UserResponse) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *UserResponse) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *UserResponse) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *UserResponse) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *UserResponse) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *UserResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *UserResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *UserResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *UserResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetEmail

`func (o *UserResponse) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *UserResponse) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *UserResponse) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetId

`func (o *UserResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *UserResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *UserResponse) SetId(v string)`

SetId sets Id field to given value.


### GetMergedIntoUserId

`func (o *UserResponse) GetMergedIntoUserId() string`

GetMergedIntoUserId returns the MergedIntoUserId field if non-nil, zero value otherwise.

### GetMergedIntoUserIdOk

`func (o *UserResponse) GetMergedIntoUserIdOk() (*string, bool)`

GetMergedIntoUserIdOk returns a tuple with the MergedIntoUserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMergedIntoUserId

`func (o *UserResponse) SetMergedIntoUserId(v string)`

SetMergedIntoUserId sets MergedIntoUserId field to given value.

### HasMergedIntoUserId

`func (o *UserResponse) HasMergedIntoUserId() bool`

HasMergedIntoUserId returns a boolean if a field has been set.

### GetName

`func (o *UserResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UserResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UserResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPhone

`func (o *UserResponse) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *UserResponse) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *UserResponse) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *UserResponse) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### GetPhoneVerifiedAt

`func (o *UserResponse) GetPhoneVerifiedAt() string`

GetPhoneVerifiedAt returns the PhoneVerifiedAt field if non-nil, zero value otherwise.

### GetPhoneVerifiedAtOk

`func (o *UserResponse) GetPhoneVerifiedAtOk() (*string, bool)`

GetPhoneVerifiedAtOk returns a tuple with the PhoneVerifiedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneVerifiedAt

`func (o *UserResponse) SetPhoneVerifiedAt(v string)`

SetPhoneVerifiedAt sets PhoneVerifiedAt field to given value.

### HasPhoneVerifiedAt

`func (o *UserResponse) HasPhoneVerifiedAt() bool`

HasPhoneVerifiedAt returns a boolean if a field has been set.

### GetEmailIsSynthetic

`func (o *UserResponse) GetEmailIsSynthetic() bool`

GetEmailIsSynthetic returns the EmailIsSynthetic field if non-nil, zero value otherwise.

### GetEmailIsSyntheticOk

`func (o *UserResponse) GetEmailIsSyntheticOk() (*bool, bool)`

GetEmailIsSyntheticOk returns a tuple with the EmailIsSynthetic field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailIsSynthetic

`func (o *UserResponse) SetEmailIsSynthetic(v bool)`

SetEmailIsSynthetic sets EmailIsSynthetic field to given value.


### GetRoles

`func (o *UserResponse) GetRoles() map[string][]string`

GetRoles returns the Roles field if non-nil, zero value otherwise.

### GetRolesOk

`func (o *UserResponse) GetRolesOk() (*map[string][]string, bool)`

GetRolesOk returns a tuple with the Roles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoles

`func (o *UserResponse) SetRoles(v map[string][]string)`

SetRoles sets Roles field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
