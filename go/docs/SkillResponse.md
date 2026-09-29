# SkillResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AgentId** | **string** |  |
**Enabled** | **bool** |  |
**Id** | **string** |  |
**Name** | **string** |  |
**OrgId** | **string** |  |
**Steps** | [**[]SkillStep**](SkillStep.md) |  |
**TriggerEventTypes** | **[]string** |  |
**Version** | **int32** |  |

## Methods

### NewSkillResponse

`func NewSkillResponse(agentId string, enabled bool, id string, name string, orgId string, steps []SkillStep, triggerEventTypes []string, version int32, ) *SkillResponse`

NewSkillResponse instantiates a new SkillResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSkillResponseWithDefaults

`func NewSkillResponseWithDefaults() *SkillResponse`

NewSkillResponseWithDefaults instantiates a new SkillResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAgentId

`func (o *SkillResponse) GetAgentId() string`

GetAgentId returns the AgentId field if non-nil, zero value otherwise.

### GetAgentIdOk

`func (o *SkillResponse) GetAgentIdOk() (*string, bool)`

GetAgentIdOk returns a tuple with the AgentId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAgentId

`func (o *SkillResponse) SetAgentId(v string)`

SetAgentId sets AgentId field to given value.


### GetEnabled

`func (o *SkillResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *SkillResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *SkillResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetId

`func (o *SkillResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SkillResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SkillResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *SkillResponse) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SkillResponse) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SkillResponse) SetName(v string)`

SetName sets Name field to given value.


### GetOrgId

`func (o *SkillResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *SkillResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *SkillResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetSteps

`func (o *SkillResponse) GetSteps() []SkillStep`

GetSteps returns the Steps field if non-nil, zero value otherwise.

### GetStepsOk

`func (o *SkillResponse) GetStepsOk() (*[]SkillStep, bool)`

GetStepsOk returns a tuple with the Steps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSteps

`func (o *SkillResponse) SetSteps(v []SkillStep)`

SetSteps sets Steps field to given value.


### GetTriggerEventTypes

`func (o *SkillResponse) GetTriggerEventTypes() []string`

GetTriggerEventTypes returns the TriggerEventTypes field if non-nil, zero value otherwise.

### GetTriggerEventTypesOk

`func (o *SkillResponse) GetTriggerEventTypesOk() (*[]string, bool)`

GetTriggerEventTypesOk returns a tuple with the TriggerEventTypes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerEventTypes

`func (o *SkillResponse) SetTriggerEventTypes(v []string)`

SetTriggerEventTypes sets TriggerEventTypes field to given value.


### GetVersion

`func (o *SkillResponse) GetVersion() int32`

GetVersion returns the Version field if non-nil, zero value otherwise.

### GetVersionOk

`func (o *SkillResponse) GetVersionOk() (*int32, bool)`

GetVersionOk returns a tuple with the Version field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVersion

`func (o *SkillResponse) SetVersion(v int32)`

SetVersion sets Version field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
