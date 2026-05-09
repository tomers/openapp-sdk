# LanAgentBootstrapTokenResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BootstrapKey** | **string** | ULID for &#x60;GET /lan-agent/cli/bootstrap.sh?key&#x3D;…&#x60;. |
**ExpiresIn** | **int64** |  |

## Methods

### NewLanAgentBootstrapTokenResponse

`func NewLanAgentBootstrapTokenResponse(bootstrapKey string, expiresIn int64, ) *LanAgentBootstrapTokenResponse`

NewLanAgentBootstrapTokenResponse instantiates a new LanAgentBootstrapTokenResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewLanAgentBootstrapTokenResponseWithDefaults

`func NewLanAgentBootstrapTokenResponseWithDefaults() *LanAgentBootstrapTokenResponse`

NewLanAgentBootstrapTokenResponseWithDefaults instantiates a new LanAgentBootstrapTokenResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBootstrapKey

`func (o *LanAgentBootstrapTokenResponse) GetBootstrapKey() string`

GetBootstrapKey returns the BootstrapKey field if non-nil, zero value otherwise.

### GetBootstrapKeyOk

`func (o *LanAgentBootstrapTokenResponse) GetBootstrapKeyOk() (*string, bool)`

GetBootstrapKeyOk returns a tuple with the BootstrapKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBootstrapKey

`func (o *LanAgentBootstrapTokenResponse) SetBootstrapKey(v string)`

SetBootstrapKey sets BootstrapKey field to given value.


### GetExpiresIn

`func (o *LanAgentBootstrapTokenResponse) GetExpiresIn() int64`

GetExpiresIn returns the ExpiresIn field if non-nil, zero value otherwise.

### GetExpiresInOk

`func (o *LanAgentBootstrapTokenResponse) GetExpiresInOk() (*int64, bool)`

GetExpiresInOk returns a tuple with the ExpiresIn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresIn

`func (o *LanAgentBootstrapTokenResponse) SetExpiresIn(v int64)`

SetExpiresIn sets ExpiresIn field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
