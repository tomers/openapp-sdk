# MeOrgMembership

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsPersonal** | **bool** |  |
**OrgId** | **string** |  |
**OrgName** | [**LocalizedString**](LocalizedString.md) |  |
**Roles** | **[]string** |  |

## Methods

### NewMeOrgMembership

`func NewMeOrgMembership(isPersonal bool, orgId string, orgName LocalizedString, roles []string, ) *MeOrgMembership`

NewMeOrgMembership instantiates a new MeOrgMembership object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeOrgMembershipWithDefaults

`func NewMeOrgMembershipWithDefaults() *MeOrgMembership`

NewMeOrgMembershipWithDefaults instantiates a new MeOrgMembership object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsPersonal

`func (o *MeOrgMembership) GetIsPersonal() bool`

GetIsPersonal returns the IsPersonal field if non-nil, zero value otherwise.

### GetIsPersonalOk

`func (o *MeOrgMembership) GetIsPersonalOk() (*bool, bool)`

GetIsPersonalOk returns a tuple with the IsPersonal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsPersonal

`func (o *MeOrgMembership) SetIsPersonal(v bool)`

SetIsPersonal sets IsPersonal field to given value.


### GetOrgId

`func (o *MeOrgMembership) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *MeOrgMembership) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *MeOrgMembership) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetOrgName

`func (o *MeOrgMembership) GetOrgName() LocalizedString`

GetOrgName returns the OrgName field if non-nil, zero value otherwise.

### GetOrgNameOk

`func (o *MeOrgMembership) GetOrgNameOk() (*LocalizedString, bool)`

GetOrgNameOk returns a tuple with the OrgName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgName

`func (o *MeOrgMembership) SetOrgName(v LocalizedString)`

SetOrgName sets OrgName field to given value.


### GetRoles

`func (o *MeOrgMembership) GetRoles() []string`

GetRoles returns the Roles field if non-nil, zero value otherwise.

### GetRolesOk

`func (o *MeOrgMembership) GetRolesOk() (*[]string, bool)`

GetRolesOk returns a tuple with the Roles field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRoles

`func (o *MeOrgMembership) SetRoles(v []string)`

SetRoles sets Roles field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
