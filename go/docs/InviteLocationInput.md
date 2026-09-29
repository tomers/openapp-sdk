# InviteLocationInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | Pointer to [**NullableLocalizedString**](LocalizedString.md) |  | [optional]
**Country** | Pointer to [**NullableInviteLocationInputCountry**](InviteLocationInputCountry.md) |  | [optional]
**FormattedAddress** | Pointer to [**NullableInviteLocationInputCountry**](InviteLocationInputCountry.md) |  | [optional]
**Kind** | Pointer to [**NullableLocationKind**](LocationKind.md) |  | [optional]
**Lat** | Pointer to **NullableFloat64** |  | [optional]
**Lng** | Pointer to **NullableFloat64** |  | [optional]
**Notes** | Pointer to [**NullableInviteLocationInputCountry**](InviteLocationInputCountry.md) |  | [optional]
**PlaceId** | Pointer to **NullableString** |  | [optional]

## Methods

### NewInviteLocationInput

`func NewInviteLocationInput() *InviteLocationInput`

NewInviteLocationInput instantiates a new InviteLocationInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInviteLocationInputWithDefaults

`func NewInviteLocationInputWithDefaults() *InviteLocationInput`

NewInviteLocationInputWithDefaults instantiates a new InviteLocationInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *InviteLocationInput) GetCity() LocalizedString`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *InviteLocationInput) GetCityOk() (*LocalizedString, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *InviteLocationInput) SetCity(v LocalizedString)`

SetCity sets City field to given value.

### HasCity

`func (o *InviteLocationInput) HasCity() bool`

HasCity returns a boolean if a field has been set.

### SetCityNil

`func (o *InviteLocationInput) SetCityNil(b bool)`

 SetCityNil sets the value for City to be an explicit nil

### UnsetCity
`func (o *InviteLocationInput) UnsetCity()`

UnsetCity ensures that no value is present for City, not even an explicit nil
### GetCountry

`func (o *InviteLocationInput) GetCountry() InviteLocationInputCountry`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *InviteLocationInput) GetCountryOk() (*InviteLocationInputCountry, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *InviteLocationInput) SetCountry(v InviteLocationInputCountry)`

SetCountry sets Country field to given value.

### HasCountry

`func (o *InviteLocationInput) HasCountry() bool`

HasCountry returns a boolean if a field has been set.

### SetCountryNil

`func (o *InviteLocationInput) SetCountryNil(b bool)`

 SetCountryNil sets the value for Country to be an explicit nil

### UnsetCountry
`func (o *InviteLocationInput) UnsetCountry()`

UnsetCountry ensures that no value is present for Country, not even an explicit nil
### GetFormattedAddress

`func (o *InviteLocationInput) GetFormattedAddress() InviteLocationInputCountry`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *InviteLocationInput) GetFormattedAddressOk() (*InviteLocationInputCountry, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *InviteLocationInput) SetFormattedAddress(v InviteLocationInputCountry)`

SetFormattedAddress sets FormattedAddress field to given value.

### HasFormattedAddress

`func (o *InviteLocationInput) HasFormattedAddress() bool`

HasFormattedAddress returns a boolean if a field has been set.

### SetFormattedAddressNil

`func (o *InviteLocationInput) SetFormattedAddressNil(b bool)`

 SetFormattedAddressNil sets the value for FormattedAddress to be an explicit nil

### UnsetFormattedAddress
`func (o *InviteLocationInput) UnsetFormattedAddress()`

UnsetFormattedAddress ensures that no value is present for FormattedAddress, not even an explicit nil
### GetKind

`func (o *InviteLocationInput) GetKind() LocationKind`

GetKind returns the Kind field if non-nil, zero value otherwise.

### GetKindOk

`func (o *InviteLocationInput) GetKindOk() (*LocationKind, bool)`

GetKindOk returns a tuple with the Kind field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKind

`func (o *InviteLocationInput) SetKind(v LocationKind)`

SetKind sets Kind field to given value.

### HasKind

`func (o *InviteLocationInput) HasKind() bool`

HasKind returns a boolean if a field has been set.

### SetKindNil

`func (o *InviteLocationInput) SetKindNil(b bool)`

 SetKindNil sets the value for Kind to be an explicit nil

### UnsetKind
`func (o *InviteLocationInput) UnsetKind()`

UnsetKind ensures that no value is present for Kind, not even an explicit nil
### GetLat

`func (o *InviteLocationInput) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *InviteLocationInput) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *InviteLocationInput) SetLat(v float64)`

SetLat sets Lat field to given value.

### HasLat

`func (o *InviteLocationInput) HasLat() bool`

HasLat returns a boolean if a field has been set.

### SetLatNil

`func (o *InviteLocationInput) SetLatNil(b bool)`

 SetLatNil sets the value for Lat to be an explicit nil

### UnsetLat
`func (o *InviteLocationInput) UnsetLat()`

UnsetLat ensures that no value is present for Lat, not even an explicit nil
### GetLng

`func (o *InviteLocationInput) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *InviteLocationInput) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *InviteLocationInput) SetLng(v float64)`

SetLng sets Lng field to given value.

### HasLng

`func (o *InviteLocationInput) HasLng() bool`

HasLng returns a boolean if a field has been set.

### SetLngNil

`func (o *InviteLocationInput) SetLngNil(b bool)`

 SetLngNil sets the value for Lng to be an explicit nil

### UnsetLng
`func (o *InviteLocationInput) UnsetLng()`

UnsetLng ensures that no value is present for Lng, not even an explicit nil
### GetNotes

`func (o *InviteLocationInput) GetNotes() InviteLocationInputCountry`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *InviteLocationInput) GetNotesOk() (*InviteLocationInputCountry, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *InviteLocationInput) SetNotes(v InviteLocationInputCountry)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *InviteLocationInput) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *InviteLocationInput) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *InviteLocationInput) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetPlaceId

`func (o *InviteLocationInput) GetPlaceId() string`

GetPlaceId returns the PlaceId field if non-nil, zero value otherwise.

### GetPlaceIdOk

`func (o *InviteLocationInput) GetPlaceIdOk() (*string, bool)`

GetPlaceIdOk returns a tuple with the PlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlaceId

`func (o *InviteLocationInput) SetPlaceId(v string)`

SetPlaceId sets PlaceId field to given value.

### HasPlaceId

`func (o *InviteLocationInput) HasPlaceId() bool`

HasPlaceId returns a boolean if a field has been set.

### SetPlaceIdNil

`func (o *InviteLocationInput) SetPlaceIdNil(b bool)`

 SetPlaceIdNil sets the value for PlaceId to be an explicit nil

### UnsetPlaceId
`func (o *InviteLocationInput) UnsetPlaceId()`

UnsetPlaceId ensures that no value is present for PlaceId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
