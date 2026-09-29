# PostMeDeviceTokenPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Platform** | **string** | One of: &#x60;ios_voip&#x60;, &#x60;ios_alert&#x60;, &#x60;android&#x60;. |
**Token** | **string** |  |

## Methods

### NewPostMeDeviceTokenPayload

`func NewPostMeDeviceTokenPayload(platform string, token string, ) *PostMeDeviceTokenPayload`

NewPostMeDeviceTokenPayload instantiates a new PostMeDeviceTokenPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPostMeDeviceTokenPayloadWithDefaults

`func NewPostMeDeviceTokenPayloadWithDefaults() *PostMeDeviceTokenPayload`

NewPostMeDeviceTokenPayloadWithDefaults instantiates a new PostMeDeviceTokenPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPlatform

`func (o *PostMeDeviceTokenPayload) GetPlatform() string`

GetPlatform returns the Platform field if non-nil, zero value otherwise.

### GetPlatformOk

`func (o *PostMeDeviceTokenPayload) GetPlatformOk() (*string, bool)`

GetPlatformOk returns a tuple with the Platform field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlatform

`func (o *PostMeDeviceTokenPayload) SetPlatform(v string)`

SetPlatform sets Platform field to given value.


### GetToken

`func (o *PostMeDeviceTokenPayload) GetToken() string`

GetToken returns the Token field if non-nil, zero value otherwise.

### GetTokenOk

`func (o *PostMeDeviceTokenPayload) GetTokenOk() (*string, bool)`

GetTokenOk returns a tuple with the Token field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToken

`func (o *PostMeDeviceTokenPayload) SetToken(v string)`

SetToken sets Token field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
