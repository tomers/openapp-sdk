# CreateGroupPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Description** | Pointer to **interface{}** |  | [optional]
**Kind** | Pointer to **NullableString** |  | [optional]
**Name** | **interface{}** |  |

## Methods

### NewCreateGroupPayload

`func NewCreateGroupPayload(name interface{}, ) *CreateGroupPayload`

NewCreateGroupPayload instantiates a new CreateGroupPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateGroupPayloadWithDefaults

`func NewCreateGroupPayloadWithDefaults() *CreateGroupPayload`

NewCreateGroupPayloadWithDefaults instantiates a new CreateGroupPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDescription

`func (o *CreateGroupPayload) GetDescription() interface{}`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *CreateGroupPayload) GetDescriptionOk() (*interface{}, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *CreateGroupPayload) SetDescription(v interface{})`

SetDescription sets Description field to given value.

### HasDescription

`func (o *CreateGroupPayload) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### SetDescriptionNil

`func (o *CreateGroupPayload) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *CreateGroupPayload) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetKind

`func (o *CreateGroupPayload) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *CreateGroupPayload) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *CreateGroupPayload) SetKind(v string)`

SetKind sets Kind field to given value.

### HasKind

`func (o *CreateGroupPayload) HasKind() bool`

HasKind returns a boolean if a field has been set.

### SetKindNil

`func (o *CreateGroupPayload) SetKindNil(b bool)`

 SetKindNil sets the value for Kind to be an explicit nil

### UnsetKind
`func (o *CreateGroupPayload) UnsetKind()`

UnsetKind ensures that no value is present for Kind, not even an explicit nil
### GetName

`func (o *CreateGroupPayload) GetName() interface{}`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateGroupPayload) GetNameOk() (*interface{}, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateGroupPayload) SetName(v interface{})`

SetName sets Name field to given value.


### SetNameNil

`func (o *CreateGroupPayload) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *CreateGroupPayload) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
