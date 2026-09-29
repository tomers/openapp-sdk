# IdpLinkResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsPrimary** | **bool** |  |
**Provider** | **string** |  |

## Methods

### NewIdpLinkResponse

`func NewIdpLinkResponse(isPrimary bool, provider string, ) *IdpLinkResponse`

NewIdpLinkResponse instantiates a new IdpLinkResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIdpLinkResponseWithDefaults

`func NewIdpLinkResponseWithDefaults() *IdpLinkResponse`

NewIdpLinkResponseWithDefaults instantiates a new IdpLinkResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsPrimary

`func (o *IdpLinkResponse) GetIsPrimary() bool`

GetIsPrimary returns the IsPrimary field if non-nil, zero value otherwise.

### GetIsPrimaryOk

`func (o *IdpLinkResponse) GetIsPrimaryOk() (*bool, bool)`

GetIsPrimaryOk returns a tuple with the IsPrimary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsPrimary

`func (o *IdpLinkResponse) SetIsPrimary(v bool)`

SetIsPrimary sets IsPrimary field to given value.


### GetProvider

`func (o *IdpLinkResponse) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *IdpLinkResponse) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *IdpLinkResponse) SetProvider(v string)`

SetProvider sets Provider field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
