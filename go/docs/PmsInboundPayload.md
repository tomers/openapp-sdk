# PmsInboundPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Event** | **string** |  |
**ReservationId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPmsInboundPayload

`func NewPmsInboundPayload(event string, ) *PmsInboundPayload`

NewPmsInboundPayload instantiates a new PmsInboundPayload object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPmsInboundPayloadWithDefaults

`func NewPmsInboundPayloadWithDefaults() *PmsInboundPayload`

NewPmsInboundPayloadWithDefaults instantiates a new PmsInboundPayload object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEvent

`func (o *PmsInboundPayload) GetEvent() string`

GetEvent returns the Event field if non-nil, zero value otherwise.

### GetEventOk

`func (o *PmsInboundPayload) GetEventOk() (*string, bool)`

GetEventOk returns a tuple with the Event field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvent

`func (o *PmsInboundPayload) SetEvent(v string)`

SetEvent sets Event field to given value.


### GetReservationId

`func (o *PmsInboundPayload) GetReservationId() string`

GetReservationId returns the ReservationId field if non-nil, zero value otherwise.

### GetReservationIdOk

`func (o *PmsInboundPayload) GetReservationIdOk() (*string, bool)`

GetReservationIdOk returns a tuple with the ReservationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReservationId

`func (o *PmsInboundPayload) SetReservationId(v string)`

SetReservationId sets ReservationId field to given value.

### HasReservationId

`func (o *PmsInboundPayload) HasReservationId() bool`

HasReservationId returns a boolean if a field has been set.

### SetReservationIdNil

`func (o *PmsInboundPayload) SetReservationIdNil(b bool)`

 SetReservationIdNil sets the value for ReservationId to be an explicit nil

### UnsetReservationId
`func (o *PmsInboundPayload) UnsetReservationId()`

UnsetReservationId ensures that no value is present for ReservationId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
