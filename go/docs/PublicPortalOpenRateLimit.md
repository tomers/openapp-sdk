# PublicPortalOpenRateLimit

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IntegrationId** | **NullableString** |  |
**MinIntervalMs** | **int64** | Enforced minimum milliseconds between &#x60;switchable.open&#x60; calls on this integration. |

## Methods

### NewPublicPortalOpenRateLimit

`func NewPublicPortalOpenRateLimit(integrationId NullableString, minIntervalMs int64, ) *PublicPortalOpenRateLimit`

NewPublicPortalOpenRateLimit instantiates a new PublicPortalOpenRateLimit object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPublicPortalOpenRateLimitWithDefaults

`func NewPublicPortalOpenRateLimitWithDefaults() *PublicPortalOpenRateLimit`

NewPublicPortalOpenRateLimitWithDefaults instantiates a new PublicPortalOpenRateLimit object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIntegrationId

`func (o *PublicPortalOpenRateLimit) GetIntegrationId() string`

GetIntegrationId returns the IntegrationId field if non-nil, zero value otherwise.

### GetIntegrationIdOk

`func (o *PublicPortalOpenRateLimit) GetIntegrationIdOk() (*string, bool)`

GetIntegrationIdOk returns a tuple with the IntegrationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIntegrationId

`func (o *PublicPortalOpenRateLimit) SetIntegrationId(v string)`

SetIntegrationId sets IntegrationId field to given value.


### SetIntegrationIdNil

`func (o *PublicPortalOpenRateLimit) SetIntegrationIdNil(b bool)`

 SetIntegrationIdNil sets the value for IntegrationId to be an explicit nil

### UnsetIntegrationId
`func (o *PublicPortalOpenRateLimit) UnsetIntegrationId()`

UnsetIntegrationId ensures that no value is present for IntegrationId, not even an explicit nil
### GetMinIntervalMs

`func (o *PublicPortalOpenRateLimit) GetMinIntervalMs() int64`

GetMinIntervalMs returns the MinIntervalMs field if non-nil, zero value otherwise.

### GetMinIntervalMsOk

`func (o *PublicPortalOpenRateLimit) GetMinIntervalMsOk() (*int64, bool)`

GetMinIntervalMsOk returns a tuple with the MinIntervalMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMinIntervalMs

`func (o *PublicPortalOpenRateLimit) SetMinIntervalMs(v int64)`

SetMinIntervalMs sets MinIntervalMs field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
