# InvitationPreviewListing

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Detail** | Pointer to **interface{}** |  | [optional]
**Label** | Pointer to **interface{}** |  | [optional]
**Name** | Pointer to **NullableString** | Listing title (display name or entity name). | [optional]
**Role** | **string** | &#x60;admin&#x60; or &#x60;member&#x60;. |

## Methods

### NewInvitationPreviewListing

`func NewInvitationPreviewListing(role string, ) *InvitationPreviewListing`

NewInvitationPreviewListing instantiates a new InvitationPreviewListing object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInvitationPreviewListingWithDefaults

`func NewInvitationPreviewListingWithDefaults() *InvitationPreviewListing`

NewInvitationPreviewListingWithDefaults instantiates a new InvitationPreviewListing object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDetail

`func (o *InvitationPreviewListing) GetDetail() interface{}`

GetDetail returns the Detail field if non-nil, zero value otherwise.

### GetDetailOk

`func (o *InvitationPreviewListing) GetDetailOk() (*interface{}, bool)`

GetDetailOk returns a tuple with the Detail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDetail

`func (o *InvitationPreviewListing) SetDetail(v interface{})`

SetDetail sets Detail field to given value.

### HasDetail

`func (o *InvitationPreviewListing) HasDetail() bool`

HasDetail returns a boolean if a field has been set.

### SetDetailNil

`func (o *InvitationPreviewListing) SetDetailNil(b bool)`

 SetDetailNil sets the value for Detail to be an explicit nil

### UnsetDetail
`func (o *InvitationPreviewListing) UnsetDetail()`

UnsetDetail ensures that no value is present for Detail, not even an explicit nil
### GetLabel

`func (o *InvitationPreviewListing) GetLabel() interface{}`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *InvitationPreviewListing) GetLabelOk() (*interface{}, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *InvitationPreviewListing) SetLabel(v interface{})`

SetLabel sets Label field to given value.

### HasLabel

`func (o *InvitationPreviewListing) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### SetLabelNil

`func (o *InvitationPreviewListing) SetLabelNil(b bool)`

 SetLabelNil sets the value for Label to be an explicit nil

### UnsetLabel
`func (o *InvitationPreviewListing) UnsetLabel()`

UnsetLabel ensures that no value is present for Label, not even an explicit nil
### GetName

`func (o *InvitationPreviewListing) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *InvitationPreviewListing) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *InvitationPreviewListing) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *InvitationPreviewListing) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *InvitationPreviewListing) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *InvitationPreviewListing) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetRole

`func (o *InvitationPreviewListing) GetRole() string`

GetRole returns the Role field if non-nil, zero value otherwise.

### GetRoleOk

`func (o *InvitationPreviewListing) GetRoleOk() (*string, bool)`

GetRoleOk returns a tuple with the Role field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRole

`func (o *InvitationPreviewListing) SetRole(v string)`

SetRole sets Role field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
