# MyGroupResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Description** | Pointer to **interface{}** |  | [optional]
**HomeOrgId** | **string** |  |
**Id** | **string** |  |
**Kind** | **string** |  |
**Name** | **interface{}** |  |
**Shareable** | **bool** |  |
**ManagerTransfer** | Pointer to [**NullableHouseholdManagerTransferView**](HouseholdManagerTransferView.md) | A managerless, read-only period while a nominated successor decides whether to take over. | [optional]
**MyRole** | **string** | The caller&#39;s role within the group. |
**Sites** | [**[]GroupSiteRef**](GroupSiteRef.md) | Sites where this group is linked and the caller is admitted. |

## Methods

### NewMyGroupResponse

`func NewMyGroupResponse(homeOrgId string, id string, kind string, name interface{}, shareable bool, myRole string, sites []GroupSiteRef, ) *MyGroupResponse`

NewMyGroupResponse instantiates a new MyGroupResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMyGroupResponseWithDefaults

`func NewMyGroupResponseWithDefaults() *MyGroupResponse`

NewMyGroupResponseWithDefaults instantiates a new MyGroupResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDescription

`func (o *MyGroupResponse) GetDescription() interface{}`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *MyGroupResponse) GetDescriptionOk() (*interface{}, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *MyGroupResponse) SetDescription(v interface{})`

SetDescription sets Description field to given value.

### HasDescription

`func (o *MyGroupResponse) HasDescription() bool`

HasDescription returns a boolean if a field has been set.

### SetDescriptionNil

`func (o *MyGroupResponse) SetDescriptionNil(b bool)`

 SetDescriptionNil sets the value for Description to be an explicit nil

### UnsetDescription
`func (o *MyGroupResponse) UnsetDescription()`

UnsetDescription ensures that no value is present for Description, not even an explicit nil
### GetHomeOrgId

`func (o *MyGroupResponse) GetHomeOrgId() string`

GetHomeOrgId returns the HomeOrgId field if non-nil, zero value otherwise.

### GetHomeOrgIdOk

`func (o *MyGroupResponse) GetHomeOrgIdOk() (*string, bool)`

GetHomeOrgIdOk returns a tuple with the HomeOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHomeOrgId

`func (o *MyGroupResponse) SetHomeOrgId(v string)`

SetHomeOrgId sets HomeOrgId field to given value.


### GetId

`func (o *MyGroupResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *MyGroupResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *MyGroupResponse) SetId(v string)`

SetId sets Id field to given value.


### GetKind

`func (o *MyGroupResponse) GetKind() string`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *MyGroupResponse) GetKindOk() (*string, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *MyGroupResponse) SetKind(v string)`

SetKind sets Kind field to given value.


### GetName

`func (o *MyGroupResponse) GetName() interface{}`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *MyGroupResponse) GetNameOk() (*interface{}, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *MyGroupResponse) SetName(v interface{})`

SetName sets Name field to given value.


### SetNameNil

`func (o *MyGroupResponse) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *MyGroupResponse) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil
### GetShareable

`func (o *MyGroupResponse) GetShareable() bool`

GetShareable returns the Shareable field if non-nil, zero value otherwise.

### GetShareableOk

`func (o *MyGroupResponse) GetShareableOk() (*bool, bool)`

GetShareableOk returns a tuple with the Shareable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetShareable

`func (o *MyGroupResponse) SetShareable(v bool)`

SetShareable sets Shareable field to given value.


### GetManagerTransfer

`func (o *MyGroupResponse) GetManagerTransfer() HouseholdManagerTransferView`

GetManagerTransfer returns the ManagerTransfer field if non-nil, zero value otherwise.

### GetManagerTransferOk

`func (o *MyGroupResponse) GetManagerTransferOk() (*HouseholdManagerTransferView, bool)`

GetManagerTransferOk returns a tuple with the ManagerTransfer field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetManagerTransfer

`func (o *MyGroupResponse) SetManagerTransfer(v HouseholdManagerTransferView)`

SetManagerTransfer sets ManagerTransfer field to given value.

### HasManagerTransfer

`func (o *MyGroupResponse) HasManagerTransfer() bool`

HasManagerTransfer returns a boolean if a field has been set.

### SetManagerTransferNil

`func (o *MyGroupResponse) SetManagerTransferNil(b bool)`

 SetManagerTransferNil sets the value for ManagerTransfer to be an explicit nil

### UnsetManagerTransfer
`func (o *MyGroupResponse) UnsetManagerTransfer()`

UnsetManagerTransfer ensures that no value is present for ManagerTransfer, not even an explicit nil
### GetMyRole

`func (o *MyGroupResponse) GetMyRole() string`

GetMyRole returns the MyRole field if non-nil, zero value otherwise.

### GetMyRoleOk

`func (o *MyGroupResponse) GetMyRoleOk() (*string, bool)`

GetMyRoleOk returns a tuple with the MyRole field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMyRole

`func (o *MyGroupResponse) SetMyRole(v string)`

SetMyRole sets MyRole field to given value.


### GetSites

`func (o *MyGroupResponse) GetSites() []GroupSiteRef`

GetSites returns the Sites field if non-nil, zero value otherwise.

### GetSitesOk

`func (o *MyGroupResponse) GetSitesOk() (*[]GroupSiteRef, bool)`

GetSitesOk returns a tuple with the Sites field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSites

`func (o *MyGroupResponse) SetSites(v []GroupSiteRef)`

SetSites sets Sites field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
