# PolicyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Capability** | Pointer to **NullableString** |  | [optional]
**Config** | **map[string]interface{}** |  |
**ConflictReport** | Pointer to [**[]ConflictingInviteResponse**](ConflictingInviteResponse.md) | Informational: currently valid invites that this curfew will constrain (never mutated). | [optional]
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**CreatedBy** | Pointer to **NullableString** |  | [optional]
**Enabled** | **bool** |  |
**Enforcement** | **string** |  |
**Id** | **string** |  |
**IntegrationId** | Pointer to **NullableString** |  | [optional]
**OrgId** | Pointer to **NullableString** |  | [optional]
**PhysicalDeviceId** | Pointer to **NullableString** |  | [optional]
**PolicyType** | **string** |  |
**Scope** | **string** | &#x60;org&#x60; | &#x60;integration&#x60; | &#x60;device&#x60;. |
**UpdatedAt** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPolicyResponse

`func NewPolicyResponse(config map[string]interface{}, enabled bool, enforcement string, id string, policyType string, scope string, ) *PolicyResponse`

NewPolicyResponse instantiates a new PolicyResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPolicyResponseWithDefaults

`func NewPolicyResponseWithDefaults() *PolicyResponse`

NewPolicyResponseWithDefaults instantiates a new PolicyResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCapability

`func (o *PolicyResponse) GetCapability() string`

GetCapability returns the Capability field if non-nil, zero value otherwise.

### GetCapabilityOk

`func (o *PolicyResponse) GetCapabilityOk() (*string, bool)`

GetCapabilityOk returns a tuple with the Capability field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCapability

`func (o *PolicyResponse) SetCapability(v string)`

SetCapability sets Capability field to given value.

### HasCapability

`func (o *PolicyResponse) HasCapability() bool`

HasCapability returns a boolean if a field has been set.

### SetCapabilityNil

`func (o *PolicyResponse) SetCapabilityNil(b bool)`

 SetCapabilityNil sets the value for Capability to be an explicit nil

### UnsetCapability
`func (o *PolicyResponse) UnsetCapability()`

UnsetCapability ensures that no value is present for Capability, not even an explicit nil
### GetConfig

`func (o *PolicyResponse) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *PolicyResponse) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *PolicyResponse) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.


### GetConflictReport

`func (o *PolicyResponse) GetConflictReport() []ConflictingInviteResponse`

GetConflictReport returns the ConflictReport field if non-nil, zero value otherwise.

### GetConflictReportOk

`func (o *PolicyResponse) GetConflictReportOk() (*[]ConflictingInviteResponse, bool)`

GetConflictReportOk returns a tuple with the ConflictReport field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConflictReport

`func (o *PolicyResponse) SetConflictReport(v []ConflictingInviteResponse)`

SetConflictReport sets ConflictReport field to given value.

### HasConflictReport

`func (o *PolicyResponse) HasConflictReport() bool`

HasConflictReport returns a boolean if a field has been set.

### SetConflictReportNil

`func (o *PolicyResponse) SetConflictReportNil(b bool)`

 SetConflictReportNil sets the value for ConflictReport to be an explicit nil

### UnsetConflictReport
`func (o *PolicyResponse) UnsetConflictReport()`

UnsetConflictReport ensures that no value is present for ConflictReport, not even an explicit nil
### GetCreatedAt

`func (o *PolicyResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PolicyResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PolicyResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *PolicyResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *PolicyResponse) SetCreatedAtNil(b bool)`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *PolicyResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil
### GetCreatedBy

`func (o *PolicyResponse) GetCreatedBy() string`

GetCreatedBy returns the CreatedBy field if non-nil, zero value otherwise.

### GetCreatedByOk

`func (o *PolicyResponse) GetCreatedByOk() (*string, bool)`

GetCreatedByOk returns a tuple with the CreatedBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedBy

`func (o *PolicyResponse) SetCreatedBy(v string)`

SetCreatedBy sets CreatedBy field to given value.

### HasCreatedBy

`func (o *PolicyResponse) HasCreatedBy() bool`

HasCreatedBy returns a boolean if a field has been set.

### SetCreatedByNil

`func (o *PolicyResponse) SetCreatedByNil(b bool)`

 SetCreatedByNil sets the value for CreatedBy to be an explicit nil

### UnsetCreatedBy
`func (o *PolicyResponse) UnsetCreatedBy()`

UnsetCreatedBy ensures that no value is present for CreatedBy, not even an explicit nil
### GetEnabled

`func (o *PolicyResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *PolicyResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *PolicyResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetEnforcement

`func (o *PolicyResponse) GetEnforcement() string`

GetEnforcement returns the Enforcement field if non-nil, zero value otherwise.

### GetEnforcementOk

`func (o *PolicyResponse) GetEnforcementOk() (*string, bool)`

GetEnforcementOk returns a tuple with the Enforcement field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnforcement

`func (o *PolicyResponse) SetEnforcement(v string)`

SetEnforcement sets Enforcement field to given value.


### GetId

`func (o *PolicyResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PolicyResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PolicyResponse) SetId(v string)`

SetId sets Id field to given value.


### GetIntegrationId

`func (o *PolicyResponse) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *PolicyResponse) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *PolicyResponse) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.

### HasIntegrationId

`func (o *PolicyResponse) HasIntegrationId() bool`

HasIntegrationId returns a boolean if a field has been set.

### SetIntegrationIdNil

`func (o *PolicyResponse) SetIntegrationIdNil(b bool)`

 SetIntegrationIdNil sets the value for IntegrationId to be an explicit nil

### UnsetIntegrationId
`func (o *PolicyResponse) UnsetIntegrationId()`

UnsetIntegrationId ensures that no value is present for IntegrationId, not even an explicit nil
### GetOrgId

`func (o *PolicyResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *PolicyResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *PolicyResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.

### HasOrgId

`func (o *PolicyResponse) HasOrgId() bool`

HasOrgId returns a boolean if a field has been set.

### SetOrgIdNil

`func (o *PolicyResponse) SetOrgIdNil(b bool)`

 SetOrgIdNil sets the value for OrgId to be an explicit nil

### UnsetOrgId
`func (o *PolicyResponse) UnsetOrgId()`

UnsetOrgId ensures that no value is present for OrgId, not even an explicit nil
### GetPhysicalDeviceId

`func (o *PolicyResponse) GetPhysicalDeviceId() string`

GetPhysicalDeviceId returns the PhysicalDeviceId field if non-nil, zero value otherwise.

### GetPhysicalDeviceIdOk

`func (o *PolicyResponse) GetPhysicalDeviceIdOk() (*string, bool)`

GetPhysicalDeviceIdOk returns a tuple with the PhysicalDeviceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPhysicalDeviceId

`func (o *PolicyResponse) SetPhysicalDeviceId(v string)`

SetPhysicalDeviceId sets PhysicalDeviceId field to given value.

### HasPhysicalDeviceId

`func (o *PolicyResponse) HasPhysicalDeviceId() bool`

HasPhysicalDeviceId returns a boolean if a field has been set.

### SetPhysicalDeviceIdNil

`func (o *PolicyResponse) SetPhysicalDeviceIdNil(b bool)`

 SetPhysicalDeviceIdNil sets the value for PhysicalDeviceId to be an explicit nil

### UnsetPhysicalDeviceId
`func (o *PolicyResponse) UnsetPhysicalDeviceId()`

UnsetPhysicalDeviceId ensures that no value is present for PhysicalDeviceId, not even an explicit nil
### GetPolicyType

`func (o *PolicyResponse) GetPolicyType() string`

GetPolicyType returns the PolicyType field if non-nil, zero value otherwise.

### GetPolicyTypeOk

`func (o *PolicyResponse) GetPolicyTypeOk() (*string, bool)`

GetPolicyTypeOk returns a tuple with the PolicyType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyType

`func (o *PolicyResponse) SetPolicyType(v string)`

SetPolicyType sets PolicyType field to given value.


### GetScope

`func (o *PolicyResponse) GetScope() string`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *PolicyResponse) GetScopeOk() (*string, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *PolicyResponse) SetScope(v string)`

SetScope sets Scope field to given value.


### GetUpdatedAt

`func (o *PolicyResponse) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PolicyResponse) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PolicyResponse) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *PolicyResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### SetUpdatedAtNil

`func (o *PolicyResponse) SetUpdatedAtNil(b bool)`

 SetUpdatedAtNil sets the value for UpdatedAt to be an explicit nil

### UnsetUpdatedAt
`func (o *PolicyResponse) UnsetUpdatedAt()`

UnsetUpdatedAt ensures that no value is present for UpdatedAt, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
