# CreateGroupShareOfferPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ExpiresInHours** | Pointer to **NullableInt64** | Hours until the offer expires. Bounded so an offer cannot outlive the reason for it. | [optional]

## Methods

### NewCreateGroupShareOfferPayload

`func NewCreateGroupShareOfferPayload() *CreateGroupShareOfferPayload`

NewCreateGroupShareOfferPayload instantiates a new CreateGroupShareOfferPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateGroupShareOfferPayloadWithDefaults

`func NewCreateGroupShareOfferPayloadWithDefaults() *CreateGroupShareOfferPayload`

NewCreateGroupShareOfferPayloadWithDefaults instantiates a new CreateGroupShareOfferPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetExpiresInHours

`func (o *CreateGroupShareOfferPayload) GetExpiresInHours() int64`

GetExpiresInHours returns the ExpiresInHours field if non-nil, zero value otherwise.

### GetExpiresInHoursOk

`func (o *CreateGroupShareOfferPayload) GetExpiresInHoursOk() (*int64, bool)`

GetExpiresInHoursOk returns a tuple with the ExpiresInHours field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresInHours

`func (o *CreateGroupShareOfferPayload) SetExpiresInHours(v int64)`

SetExpiresInHours sets ExpiresInHours field to given value.

### HasExpiresInHours

`func (o *CreateGroupShareOfferPayload) HasExpiresInHours() bool`

HasExpiresInHours returns a boolean if a field has been set.

### SetExpiresInHoursNil

`func (o *CreateGroupShareOfferPayload) SetExpiresInHoursNil(b bool)`

 SetExpiresInHoursNil sets the value for ExpiresInHours to be an explicit nil

### UnsetExpiresInHours
`func (o *CreateGroupShareOfferPayload) UnsetExpiresInHours()`

UnsetExpiresInHours ensures that no value is present for ExpiresInHours, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
