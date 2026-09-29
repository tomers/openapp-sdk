# TransferIntegrationResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**DeletedAt** | Pointer to **time.Time** |  | [optional]
**HardDeleteAt** | Pointer to **time.Time** |  | [optional]
**PurgeAt** | Pointer to **time.Time** |  | [optional]
**UpdatedAt** | Pointer to **time.Time** |  | [optional]
**Config** | Pointer to **interface{}** |  | [optional]
**Enabled** | **bool** | Pause switch for hardware connectors. Always true for site providers. |
**Health** | [**IntegrationHealth**](IntegrationHealth.md) | Backend-controlled health marker (ok/reduced/error). |
**Id** | **string** | Unique identifier (ULID). |
**Name** | [**LocalizedString**](LocalizedString.md) | Human-friendly name for this integration (distinguishes multiple integrations of same provider). |
**OrgId** | **string** | Organization that owns this integration. |
**ProviderType** | [**ProviderType**](ProviderType.md) | Provider type (e.g. homeassistant, shelly_cloud). |
**ProviderTypeName** | [**LocalizedString**](LocalizedString.md) | Human-friendly provider type name(s), keyed by locale. |
**Affected** | [**AffectedCounts**](AffectedCounts.md) |  |
**Mode** | **string** |  |
**SourceOrgId** | **string** |  |
**TargetOrgId** | **string** |  |
**Warnings** | **[]string** | Side-effect warning codes that applied to this transfer. |

## Methods

### NewTransferIntegrationResponse

`func NewTransferIntegrationResponse(enabled bool, health IntegrationHealth, id string, name LocalizedString, orgId string, providerType ProviderType, providerTypeName LocalizedString, affected AffectedCounts, mode string, sourceOrgId string, targetOrgId string, warnings []string, ) *TransferIntegrationResponse`

NewTransferIntegrationResponse instantiates a new TransferIntegrationResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTransferIntegrationResponseWithDefaults

`func NewTransferIntegrationResponseWithDefaults() *TransferIntegrationResponse`

NewTransferIntegrationResponseWithDefaults instantiates a new TransferIntegrationResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *TransferIntegrationResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *TransferIntegrationResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *TransferIntegrationResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *TransferIntegrationResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetDeletedAt

`func (o *TransferIntegrationResponse) GetDeletedAt() time.Time`

GetDeletedAt returns the DeletedAt field if non-nil, zero value otherwise.

### GetDeletedAtOk

`func (o *TransferIntegrationResponse) GetDeletedAtOk() (*time.Time, bool)`

GetDeletedAtOk returns a tuple with the DeletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeletedAt

`func (o *TransferIntegrationResponse) SetDeletedAt(v time.Time)`

SetDeletedAt sets DeletedAt field to given value.

### HasDeletedAt

`func (o *TransferIntegrationResponse) HasDeletedAt() bool`

HasDeletedAt returns a boolean if a field has been set.

### GetHardDeleteAt

`func (o *TransferIntegrationResponse) GetHardDeleteAt() time.Time`

GetHardDeleteAt returns the HardDeleteAt field if non-nil, zero value otherwise.

### GetHardDeleteAtOk

`func (o *TransferIntegrationResponse) GetHardDeleteAtOk() (*time.Time, bool)`

GetHardDeleteAtOk returns a tuple with the HardDeleteAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardDeleteAt

`func (o *TransferIntegrationResponse) SetHardDeleteAt(v time.Time)`

SetHardDeleteAt sets HardDeleteAt field to given value.

### HasHardDeleteAt

`func (o *TransferIntegrationResponse) HasHardDeleteAt() bool`

HasHardDeleteAt returns a boolean if a field has been set.

### GetPurgeAt

`func (o *TransferIntegrationResponse) GetPurgeAt() time.Time`

GetPurgeAt returns the PurgeAt field if non-nil, zero value otherwise.

### GetPurgeAtOk

`func (o *TransferIntegrationResponse) GetPurgeAtOk() (*time.Time, bool)`

GetPurgeAtOk returns a tuple with the PurgeAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPurgeAt

`func (o *TransferIntegrationResponse) SetPurgeAt(v time.Time)`

SetPurgeAt sets PurgeAt field to given value.

### HasPurgeAt

`func (o *TransferIntegrationResponse) HasPurgeAt() bool`

HasPurgeAt returns a boolean if a field has been set.

### GetUpdatedAt

`func (o *TransferIntegrationResponse) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *TransferIntegrationResponse) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *TransferIntegrationResponse) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.

### HasUpdatedAt

`func (o *TransferIntegrationResponse) HasUpdatedAt() bool`

HasUpdatedAt returns a boolean if a field has been set.

### GetConfig

`func (o *TransferIntegrationResponse) GetConfig() interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *TransferIntegrationResponse) GetConfigOk() (*interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *TransferIntegrationResponse) SetConfig(v interface{})`

SetConfig sets Config field to given value.

### HasConfig

`func (o *TransferIntegrationResponse) HasConfig() bool`

HasConfig returns a boolean if a field has been set.

### SetConfigNil

`func (o *TransferIntegrationResponse) SetConfigNil(b bool)`

 SetConfigNil sets the value for Config to be an explicit nil

### UnsetConfig
`func (o *TransferIntegrationResponse) UnsetConfig()`

UnsetConfig ensures that no value is present for Config, not even an explicit nil
### GetEnabled

`func (o *TransferIntegrationResponse) GetEnabled() bool`

GetEnabled returns the Enabled field if non-nil, zero value otherwise.

### GetEnabledOk

`func (o *TransferIntegrationResponse) GetEnabledOk() (*bool, bool)`

GetEnabledOk returns a tuple with the Enabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEnabled

`func (o *TransferIntegrationResponse) SetEnabled(v bool)`

SetEnabled sets Enabled field to given value.


### GetHealth

`func (o *TransferIntegrationResponse) GetHealth() IntegrationHealth`

GetHealth returns the Health field if non-nil, zero value otherwise.

### GetHealthOk

`func (o *TransferIntegrationResponse) GetHealthOk() (*IntegrationHealth, bool)`

GetHealthOk returns a tuple with the Health field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHealth

`func (o *TransferIntegrationResponse) SetHealth(v IntegrationHealth)`

SetHealth sets Health field to given value.


### GetId

`func (o *TransferIntegrationResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *TransferIntegrationResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *TransferIntegrationResponse) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *TransferIntegrationResponse) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *TransferIntegrationResponse) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *TransferIntegrationResponse) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetOrgId

`func (o *TransferIntegrationResponse) GetOrgId() string`

GetOrgId returns the OrgId field if non-nil, zero value otherwise.

### GetOrgIdOk

`func (o *TransferIntegrationResponse) GetOrgIdOk() (*string, bool)`

GetOrgIdOk returns a tuple with the OrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOrgId

`func (o *TransferIntegrationResponse) SetOrgId(v string)`

SetOrgId sets OrgId field to given value.


### GetProviderType

`func (o *TransferIntegrationResponse) GetProviderType() ProviderType`

GetProviderType returns the ProviderType field if non-nil, zero value otherwise.

### GetProviderTypeOk

`func (o *TransferIntegrationResponse) GetProviderTypeOk() (*ProviderType, bool)`

GetProviderTypeOk returns a tuple with the ProviderType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderType

`func (o *TransferIntegrationResponse) SetProviderType(v ProviderType)`

SetProviderType sets ProviderType field to given value.


### GetProviderTypeName

`func (o *TransferIntegrationResponse) GetProviderTypeName() LocalizedString`

GetProviderTypeName returns the ProviderTypeName field if non-nil, zero value otherwise.

### GetProviderTypeNameOk

`func (o *TransferIntegrationResponse) GetProviderTypeNameOk() (*LocalizedString, bool)`

GetProviderTypeNameOk returns a tuple with the ProviderTypeName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderTypeName

`func (o *TransferIntegrationResponse) SetProviderTypeName(v LocalizedString)`

SetProviderTypeName sets ProviderTypeName field to given value.


### GetAffected

`func (o *TransferIntegrationResponse) GetAffected() AffectedCounts`

GetAffected returns the Affected field if non-nil, zero value otherwise.

### GetAffectedOk

`func (o *TransferIntegrationResponse) GetAffectedOk() (*AffectedCounts, bool)`

GetAffectedOk returns a tuple with the Affected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAffected

`func (o *TransferIntegrationResponse) SetAffected(v AffectedCounts)`

SetAffected sets Affected field to given value.


### GetMode

`func (o *TransferIntegrationResponse) GetMode() string`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *TransferIntegrationResponse) GetModeOk() (*string, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *TransferIntegrationResponse) SetMode(v string)`

SetMode sets Mode field to given value.


### GetSourceOrgId

`func (o *TransferIntegrationResponse) GetSourceOrgId() string`

GetSourceOrgId returns the SourceOrgId field if non-nil, zero value otherwise.

### GetSourceOrgIdOk

`func (o *TransferIntegrationResponse) GetSourceOrgIdOk() (*string, bool)`

GetSourceOrgIdOk returns a tuple with the SourceOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceOrgId

`func (o *TransferIntegrationResponse) SetSourceOrgId(v string)`

SetSourceOrgId sets SourceOrgId field to given value.


### GetTargetOrgId

`func (o *TransferIntegrationResponse) GetTargetOrgId() string`

GetTargetOrgId returns the TargetOrgId field if non-nil, zero value otherwise.

### GetTargetOrgIdOk

`func (o *TransferIntegrationResponse) GetTargetOrgIdOk() (*string, bool)`

GetTargetOrgIdOk returns a tuple with the TargetOrgId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTargetOrgId

`func (o *TransferIntegrationResponse) SetTargetOrgId(v string)`

SetTargetOrgId sets TargetOrgId field to given value.


### GetWarnings

`func (o *TransferIntegrationResponse) GetWarnings() []string`

GetWarnings returns the Warnings field if non-nil, zero value otherwise.

### GetWarningsOk

`func (o *TransferIntegrationResponse) GetWarningsOk() (*[]string, bool)`

GetWarningsOk returns a tuple with the Warnings field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWarnings

`func (o *TransferIntegrationResponse) SetWarnings(v []string)`

SetWarnings sets Warnings field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
