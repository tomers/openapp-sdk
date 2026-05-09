# LanAgentMetaResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**LanProtocolVersion** | **int32** |  |
**LatestAgentVersion** | Pointer to **NullableString** |  | [optional]
**MinSupportedAgentVersion** | Pointer to **NullableString** |  | [optional]
**TasksCatalog** | **interface{}** |  |
**UpgradeDownloadUrl** | Pointer to **NullableString** |  | [optional]

## Methods

### NewLanAgentMetaResponse

`func NewLanAgentMetaResponse(lanProtocolVersion int32, tasksCatalog interface{}, ) *LanAgentMetaResponse`

NewLanAgentMetaResponse instantiates a new LanAgentMetaResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLanAgentMetaResponseWithDefaults

`func NewLanAgentMetaResponseWithDefaults() *LanAgentMetaResponse`

NewLanAgentMetaResponseWithDefaults instantiates a new LanAgentMetaResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLanProtocolVersion

`func (o *LanAgentMetaResponse) GetLanProtocolVersion() int32`

GetLanProtocolVersion returns the LanProtocolVersion field if non-nil, zero value otherwise.

### GetLanProtocolVersionOk

`func (o *LanAgentMetaResponse) GetLanProtocolVersionOk() (*int32, bool)`

GetLanProtocolVersionOk returns a tuple with the LanProtocolVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLanProtocolVersion

`func (o *LanAgentMetaResponse) SetLanProtocolVersion(v int32)`

SetLanProtocolVersion sets LanProtocolVersion field to given value.


### GetLatestAgentVersion

`func (o *LanAgentMetaResponse) GetLatestAgentVersion() string`

GetLatestAgentVersion returns the LatestAgentVersion field if non-nil, zero value otherwise.

### GetLatestAgentVersionOk

`func (o *LanAgentMetaResponse) GetLatestAgentVersionOk() (*string, bool)`

GetLatestAgentVersionOk returns a tuple with the LatestAgentVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLatestAgentVersion

`func (o *LanAgentMetaResponse) SetLatestAgentVersion(v string)`

SetLatestAgentVersion sets LatestAgentVersion field to given value.

### HasLatestAgentVersion

`func (o *LanAgentMetaResponse) HasLatestAgentVersion() bool`

HasLatestAgentVersion returns a boolean if a field has been set.

### SetLatestAgentVersionNil

`func (o *LanAgentMetaResponse) SetLatestAgentVersionNil(b bool)`

 SetLatestAgentVersionNil sets the value for LatestAgentVersion to be an explicit nil

### UnsetLatestAgentVersion
`func (o *LanAgentMetaResponse) UnsetLatestAgentVersion()`

UnsetLatestAgentVersion ensures that no value is present for LatestAgentVersion, not even an explicit nil
### GetMinSupportedAgentVersion

`func (o *LanAgentMetaResponse) GetMinSupportedAgentVersion() string`

GetMinSupportedAgentVersion returns the MinSupportedAgentVersion field if non-nil, zero value otherwise.

### GetMinSupportedAgentVersionOk

`func (o *LanAgentMetaResponse) GetMinSupportedAgentVersionOk() (*string, bool)`

GetMinSupportedAgentVersionOk returns a tuple with the MinSupportedAgentVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMinSupportedAgentVersion

`func (o *LanAgentMetaResponse) SetMinSupportedAgentVersion(v string)`

SetMinSupportedAgentVersion sets MinSupportedAgentVersion field to given value.

### HasMinSupportedAgentVersion

`func (o *LanAgentMetaResponse) HasMinSupportedAgentVersion() bool`

HasMinSupportedAgentVersion returns a boolean if a field has been set.

### SetMinSupportedAgentVersionNil

`func (o *LanAgentMetaResponse) SetMinSupportedAgentVersionNil(b bool)`

 SetMinSupportedAgentVersionNil sets the value for MinSupportedAgentVersion to be an explicit nil

### UnsetMinSupportedAgentVersion
`func (o *LanAgentMetaResponse) UnsetMinSupportedAgentVersion()`

UnsetMinSupportedAgentVersion ensures that no value is present for MinSupportedAgentVersion, not even an explicit nil
### GetTasksCatalog

`func (o *LanAgentMetaResponse) GetTasksCatalog() interface{}`

GetTasksCatalog returns the TasksCatalog field if non-nil, zero value otherwise.

### GetTasksCatalogOk

`func (o *LanAgentMetaResponse) GetTasksCatalogOk() (*interface{}, bool)`

GetTasksCatalogOk returns a tuple with the TasksCatalog field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTasksCatalog

`func (o *LanAgentMetaResponse) SetTasksCatalog(v interface{})`

SetTasksCatalog sets TasksCatalog field to given value.


### SetTasksCatalogNil

`func (o *LanAgentMetaResponse) SetTasksCatalogNil(b bool)`

 SetTasksCatalogNil sets the value for TasksCatalog to be an explicit nil

### UnsetTasksCatalog
`func (o *LanAgentMetaResponse) UnsetTasksCatalog()`

UnsetTasksCatalog ensures that no value is present for TasksCatalog, not even an explicit nil
### GetUpgradeDownloadUrl

`func (o *LanAgentMetaResponse) GetUpgradeDownloadUrl() string`

GetUpgradeDownloadUrl returns the UpgradeDownloadUrl field if non-nil, zero value otherwise.

### GetUpgradeDownloadUrlOk

`func (o *LanAgentMetaResponse) GetUpgradeDownloadUrlOk() (*string, bool)`

GetUpgradeDownloadUrlOk returns a tuple with the UpgradeDownloadUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpgradeDownloadUrl

`func (o *LanAgentMetaResponse) SetUpgradeDownloadUrl(v string)`

SetUpgradeDownloadUrl sets UpgradeDownloadUrl field to given value.

### HasUpgradeDownloadUrl

`func (o *LanAgentMetaResponse) HasUpgradeDownloadUrl() bool`

HasUpgradeDownloadUrl returns a boolean if a field has been set.

### SetUpgradeDownloadUrlNil

`func (o *LanAgentMetaResponse) SetUpgradeDownloadUrlNil(b bool)`

 SetUpgradeDownloadUrlNil sets the value for UpgradeDownloadUrl to be an explicit nil

### UnsetUpgradeDownloadUrl
`func (o *LanAgentMetaResponse) UnsetUpgradeDownloadUrl()`

UnsetUpgradeDownloadUrl ensures that no value is present for UpgradeDownloadUrl, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
