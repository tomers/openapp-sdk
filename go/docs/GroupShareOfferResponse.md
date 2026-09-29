# GroupShareOfferResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresAt** | **string** |  |
**GroupId** | **string** |  |
**OfferId** | **string** |  |
**Token** | Pointer to **NullableString** | The one-time token. Returned only by create; redemption never echoes it back. | [optional]

## Methods

### NewGroupShareOfferResponse

`func NewGroupShareOfferResponse(expiresAt string, groupId string, offerId string, ) *GroupShareOfferResponse`

NewGroupShareOfferResponse instantiates a new GroupShareOfferResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewGroupShareOfferResponseWithDefaults

`func NewGroupShareOfferResponseWithDefaults() *GroupShareOfferResponse`

NewGroupShareOfferResponseWithDefaults instantiates a new GroupShareOfferResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresAt

`func (o *GroupShareOfferResponse) GetExpiresAt() string`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *GroupShareOfferResponse) GetExpiresAtOk() (*string, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *GroupShareOfferResponse) SetExpiresAt(v string)`

SetExpiresAt sets ExpiresAt field to given value.


### GetGroupId

`func (o *GroupShareOfferResponse) GetGroupId() string`

GetGroupId returns the GroupId field if non-nil, zero value otherwise.

### GetGroupIdOk

`func (o *GroupShareOfferResponse) GetGroupIdOk() (*string, bool)`

GetGroupIdOk returns a tuple with the GroupId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGroupId

`func (o *GroupShareOfferResponse) SetGroupId(v string)`

SetGroupId sets GroupId field to given value.


### GetOfferId

`func (o *GroupShareOfferResponse) GetOfferId() string`

GetOfferId returns the OfferId field if non-nil, zero value otherwise.

### GetOfferIdOk

`func (o *GroupShareOfferResponse) GetOfferIdOk() (*string, bool)`

GetOfferIdOk returns a tuple with the OfferId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOfferId

`func (o *GroupShareOfferResponse) SetOfferId(v string)`

SetOfferId sets OfferId field to given value.


### GetToken

`func (o *GroupShareOfferResponse) GetToken() string`

GetToken returns the Token field if non-nil, zero value otherwise.

### GetTokenOk

`func (o *GroupShareOfferResponse) GetTokenOk() (*string, bool)`

GetTokenOk returns a tuple with the Token field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetToken

`func (o *GroupShareOfferResponse) SetToken(v string)`

SetToken sets Token field to given value.

### HasToken

`func (o *GroupShareOfferResponse) HasToken() bool`

HasToken returns a boolean if a field has been set.

### SetTokenNil

`func (o *GroupShareOfferResponse) SetTokenNil(b bool)`

 SetTokenNil sets the value for Token to be an explicit nil

### UnsetToken
`func (o *GroupShareOfferResponse) UnsetToken()`

UnsetToken ensures that no value is present for Token, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
