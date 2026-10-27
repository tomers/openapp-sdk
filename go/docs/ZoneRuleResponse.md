# ZoneRuleResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Config** | **map[string]interface{}** |  |
**PolicyId** | **string** |  |
**PolicyType** | **string** |  |
**ZoneId** | **string** |  |
**ZoneName** | [**LocalizedString**](LocalizedString.md) |  |
**ZonePath** | [**[]LocalizedString**](LocalizedString.md) | Zone names from the root to this zone (inclusive). |

## Methods

### NewZoneRuleResponse

`func NewZoneRuleResponse(config map[string]interface{}, policyId string, policyType string, zoneId string, zoneName LocalizedString, zonePath []LocalizedString, ) *ZoneRuleResponse`

NewZoneRuleResponse instantiates a new ZoneRuleResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZoneRuleResponseWithDefaults

`func NewZoneRuleResponseWithDefaults() *ZoneRuleResponse`

NewZoneRuleResponseWithDefaults instantiates a new ZoneRuleResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfig

`func (o *ZoneRuleResponse) GetConfig() map[string]interface{}`

GetConfig returns the Config field if non-nil, zero value otherwise.

### GetConfigOk

`func (o *ZoneRuleResponse) GetConfigOk() (*map[string]interface{}, bool)`

GetConfigOk returns a tuple with the Config field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfig

`func (o *ZoneRuleResponse) SetConfig(v map[string]interface{})`

SetConfig sets Config field to given value.


### GetPolicyId

`func (o *ZoneRuleResponse) GetPolicyId() string`

GetPolicyId returns the PolicyId field if non-nil, zero value otherwise.

### GetPolicyIdOk

`func (o *ZoneRuleResponse) GetPolicyIdOk() (*string, bool)`

GetPolicyIdOk returns a tuple with the PolicyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyId

`func (o *ZoneRuleResponse) SetPolicyId(v string)`

SetPolicyId sets PolicyId field to given value.


### GetPolicyType

`func (o *ZoneRuleResponse) GetPolicyType() string`

GetPolicyType returns the PolicyType field if non-nil, zero value otherwise.

### GetPolicyTypeOk

`func (o *ZoneRuleResponse) GetPolicyTypeOk() (*string, bool)`

GetPolicyTypeOk returns a tuple with the PolicyType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyType

`func (o *ZoneRuleResponse) SetPolicyType(v string)`

SetPolicyType sets PolicyType field to given value.


### GetZoneId

`func (o *ZoneRuleResponse) GetZoneId() string`

GetZoneId returns the ZoneId field if non-nil, zero value otherwise.

### GetZoneIdOk

`func (o *ZoneRuleResponse) GetZoneIdOk() (*string, bool)`

GetZoneIdOk returns a tuple with the ZoneId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneId

`func (o *ZoneRuleResponse) SetZoneId(v string)`

SetZoneId sets ZoneId field to given value.


### GetZoneName

`func (o *ZoneRuleResponse) GetZoneName() LocalizedString`

GetZoneName returns the ZoneName field if non-nil, zero value otherwise.

### GetZoneNameOk

`func (o *ZoneRuleResponse) GetZoneNameOk() (*LocalizedString, bool)`

GetZoneNameOk returns a tuple with the ZoneName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZoneName

`func (o *ZoneRuleResponse) SetZoneName(v LocalizedString)`

SetZoneName sets ZoneName field to given value.


### GetZonePath

`func (o *ZoneRuleResponse) GetZonePath() []LocalizedString`

GetZonePath returns the ZonePath field if non-nil, zero value otherwise.

### GetZonePathOk

`func (o *ZoneRuleResponse) GetZonePathOk() (*[]LocalizedString, bool)`

GetZonePathOk returns a tuple with the ZonePath field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetZonePath

`func (o *ZoneRuleResponse) SetZonePath(v []LocalizedString)`

SetZonePath sets ZonePath field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
