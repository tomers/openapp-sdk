# GroupResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Description** | Pointer to **interface{}** |  | [optional]
**HomeOrgId** | **string** |  |
**Id** | **string** |  |
**Kind** | **string** |  |
**Name** | **interface{}** |  |
**Shareable** | **bool** |  |

## Methods

### NewGroupResponse

`func NewGroupResponse(homeOrgId string, id string, kind string, name interface{}, shareable bool, ) *GroupResponse`

NewGroupResponse instantiates a new GroupResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGroupResponseWithDefaults

`func NewGroupResponseWithDefaults() *GroupResponse`

NewGroupResponseWithDefaults instantiates a new GroupResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDescription

`func (o *GroupResponse) GetDescription() interface{}`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *GroupResponse) GetDescriptionOk() (*interface{}, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *GroupResponse) SetDescription(v interface{})`

SetDescription sets Description field to given value.

### HasDescription

`func (o *GroupResponse) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### SetDescriptionNil

`func (o *GroupResponse) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *GroupResponse) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetHomeOrgId

`func (o *GroupResponse) GetHomeOrgId() string`

GetHomeOrgId returns the HomeOrgId field if non-nil, zero value otherwise.

### GetHomeOrgIdOk

`func (o *GroupResponse) GetHomeOrgIdOk() (*string, bool)`

GetHomeOrgIdOk returns a tuple with the HomeOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHomeOrgId

`func (o *GroupResponse) SetHomeOrgId(v string)`

SetHomeOrgId sets HomeOrgId field to given value.


### GetId

`func (o *GroupResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *GroupResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *GroupResponse) SetId(v string)`

SetId sets Id field to given value.


### GetKind

`func (o *GroupResponse) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *GroupResponse) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *GroupResponse) SetKind(v string)`

SetKind sets Kind field to given value.


### GetName

`func (o *GroupResponse) GetName() interface{}`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *GroupResponse) GetNameOk() (*interface{}, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *GroupResponse) SetName(v interface{})`

SetName sets Name field to given value.


### SetNameNil

`func (o *GroupResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *GroupResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetShareable

`func (o *GroupResponse) GetShareable() bool`

GetShareable returns the Shareable field if non-nil, zero value otherwise.

### GetShareableOk

`func (o *GroupResponse) GetShareableOk() (*bool, bool)`

GetShareableOk returns a tuple with the Shareable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareable

`func (o *GroupResponse) SetShareable(v bool)`

SetShareable sets Shareable field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
