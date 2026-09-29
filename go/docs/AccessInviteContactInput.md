# AccessInviteContactInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ContactKind** | **string** | &#x60;email&#x60;, &#x60;phone&#x60;, &#x60;user&#x60;, or &#x60;group&#x60;. |
**DisplayName** | Pointer to **NullableString** |  | [optional]
**Email** | Pointer to **NullableString** |  | [optional]
**GroupId** | Pointer to **NullableString** |  | [optional]
**Phone** | Pointer to **NullableString** | E.164, or national format when &#x60;phone_region&#x60; is supplied. | [optional]
**PhoneRegion** | Pointer to **NullableString** | ISO 3166-1 alpha-2 region used to normalize a national-format phone. | [optional]
**UserId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAccessInviteContactInput

`func NewAccessInviteContactInput(contactKind string, ) *AccessInviteContactInput`

NewAccessInviteContactInput instantiates a new AccessInviteContactInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAccessInviteContactInputWithDefaults

`func NewAccessInviteContactInputWithDefaults() *AccessInviteContactInput`

NewAccessInviteContactInputWithDefaults instantiates a new AccessInviteContactInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetContactKind

`func (o *AccessInviteContactInput) GetContactKind() string`

GetContactKind returns the ContactKind field if non-nil, zero value otherwise.

### GetContactKindOk

`func (o *AccessInviteContactInput) GetContactKindOk() (*string, bool)`

GetContactKindOk returns a tuple with the ContactKind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactKind

`func (o *AccessInviteContactInput) SetContactKind(v string)`

SetContactKind sets ContactKind field to given value.


### GetDisplayName

`func (o *AccessInviteContactInput) GetDisplayName() string`

GetDisplayName returns the DisplayName field if non-nil, zero value otherwise.

### GetDisplayNameOk

`func (o *AccessInviteContactInput) GetDisplayNameOk() (*string, bool)`

GetDisplayNameOk returns a tuple with the DisplayName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDisplayName

`func (o *AccessInviteContactInput) SetDisplayName(v string)`

SetDisplayName sets DisplayName field to given value.

### HasDisplayName

`func (o *AccessInviteContactInput) HasDisplayName() bool`

HasDisplayName returns a boolean if a field has been set.

### SetDisplayNameNil

`func (o *AccessInviteContactInput) SetDisplayNameNil(b bool)`

 SetDisplayNameNil sets the value for DisplayName to be an explicit nil

### UnsetDisplayName
`func (o *AccessInviteContactInput) UnsetDisplayName()`

UnsetDisplayName ensures that no value is present for DisplayName, not even an explicit nil
### GetEmail

`func (o *AccessInviteContactInput) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *AccessInviteContactInput) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *AccessInviteContactInput) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *AccessInviteContactInput) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *AccessInviteContactInput) SetEmailNil(b bool)`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *AccessInviteContactInput) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil
### GetGroupId

`func (o *AccessInviteContactInput) GetGroupId() string`

GetGroupId returns the GroupId field if non-nil, zero value otherwise.

### GetGroupIdOk

`func (o *AccessInviteContactInput) GetGroupIdOk() (*string, bool)`

GetGroupIdOk returns a tuple with the GroupId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroupId

`func (o *AccessInviteContactInput) SetGroupId(v string)`

SetGroupId sets GroupId field to given value.

### HasGroupId

`func (o *AccessInviteContactInput) HasGroupId() bool`

HasGroupId returns a boolean if a field has been set.

### SetGroupIdNil

`func (o *AccessInviteContactInput) SetGroupIdNil(b bool)`

 SetGroupIdNil sets the value for GroupId to be an explicit nil

### UnsetGroupId
`func (o *AccessInviteContactInput) UnsetGroupId()`

UnsetGroupId ensures that no value is present for GroupId, not even an explicit nil
### GetPhone

`func (o *AccessInviteContactInput) GetPhone() string`

GetPhone returns the Phone field if non-nil, zero value otherwise.

### GetPhoneOk

`func (o *AccessInviteContactInput) GetPhoneOk() (*string, bool)`

GetPhoneOk returns a tuple with the Phone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhone

`func (o *AccessInviteContactInput) SetPhone(v string)`

SetPhone sets Phone field to given value.

### HasPhone

`func (o *AccessInviteContactInput) HasPhone() bool`

HasPhone returns a boolean if a field has been set.

### SetPhoneNil

`func (o *AccessInviteContactInput) SetPhoneNil(b bool)`

 SetPhoneNil sets the value for Phone to be an explicit nil

### UnsetPhone
`func (o *AccessInviteContactInput) UnsetPhone()`

UnsetPhone ensures that no value is present for Phone, not even an explicit nil
### GetPhoneRegion

`func (o *AccessInviteContactInput) GetPhoneRegion() string`

GetPhoneRegion returns the PhoneRegion field if non-nil, zero value otherwise.

### GetPhoneRegionOk

`func (o *AccessInviteContactInput) GetPhoneRegionOk() (*string, bool)`

GetPhoneRegionOk returns a tuple with the PhoneRegion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhoneRegion

`func (o *AccessInviteContactInput) SetPhoneRegion(v string)`

SetPhoneRegion sets PhoneRegion field to given value.

### HasPhoneRegion

`func (o *AccessInviteContactInput) HasPhoneRegion() bool`

HasPhoneRegion returns a boolean if a field has been set.

### SetPhoneRegionNil

`func (o *AccessInviteContactInput) SetPhoneRegionNil(b bool)`

 SetPhoneRegionNil sets the value for PhoneRegion to be an explicit nil

### UnsetPhoneRegion
`func (o *AccessInviteContactInput) UnsetPhoneRegion()`

UnsetPhoneRegion ensures that no value is present for PhoneRegion, not even an explicit nil
### GetUserId

`func (o *AccessInviteContactInput) GetUserId() string`

GetUserId returns the UserId field if non-nil, zero value otherwise.

### GetUserIdOk

`func (o *AccessInviteContactInput) GetUserIdOk() (*string, bool)`

GetUserIdOk returns a tuple with the UserId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUserId

`func (o *AccessInviteContactInput) SetUserId(v string)`

SetUserId sets UserId field to given value.

### HasUserId

`func (o *AccessInviteContactInput) HasUserId() bool`

HasUserId returns a boolean if a field has been set.

### SetUserIdNil

`func (o *AccessInviteContactInput) SetUserIdNil(b bool)`

 SetUserIdNil sets the value for UserId to be an explicit nil

### UnsetUserId
`func (o *AccessInviteContactInput) UnsetUserId()`

UnsetUserId ensures that no value is present for UserId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
