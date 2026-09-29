# ReverseGeocodeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**City** | **string** |  |
**Country** | **string** |  |
**FormattedAddress** | **string** |  |
**GeocodingProvider** | Pointer to [**NullableGeocodingProviderId**](GeocodingProviderId.md) |  | [optional]
**Lat** | **float64** |  |
**Lng** | **float64** |  |
**ProviderPlaceId** | Pointer to **NullableString** | Opaque place id issued by &#x60;geocoding_provider&#x60;, when the point resolved to a known place. Both fields are present together or not at all. | [optional]

## Methods

### NewReverseGeocodeResponse

`func NewReverseGeocodeResponse(city string, country string, formattedAddress string, lat float64, lng float64, ) *ReverseGeocodeResponse`

NewReverseGeocodeResponse instantiates a new ReverseGeocodeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReverseGeocodeResponseWithDefaults

`func NewReverseGeocodeResponseWithDefaults() *ReverseGeocodeResponse`

NewReverseGeocodeResponseWithDefaults instantiates a new ReverseGeocodeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCity

`func (o *ReverseGeocodeResponse) GetCity() string`

GetCity returns the City field if non-nil, zero value otherwise.

### GetCityOk

`func (o *ReverseGeocodeResponse) GetCityOk() (*string, bool)`

GetCityOk returns a tuple with the City field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCity

`func (o *ReverseGeocodeResponse) SetCity(v string)`

SetCity sets City field to given value.


### GetCountry

`func (o *ReverseGeocodeResponse) GetCountry() string`

GetCountry returns the Country field if non-nil, zero value otherwise.

### GetCountryOk

`func (o *ReverseGeocodeResponse) GetCountryOk() (*string, bool)`

GetCountryOk returns a tuple with the Country field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCountry

`func (o *ReverseGeocodeResponse) SetCountry(v string)`

SetCountry sets Country field to given value.


### GetFormattedAddress

`func (o *ReverseGeocodeResponse) GetFormattedAddress() string`

GetFormattedAddress returns the FormattedAddress field if non-nil, zero value otherwise.

### GetFormattedAddressOk

`func (o *ReverseGeocodeResponse) GetFormattedAddressOk() (*string, bool)`

GetFormattedAddressOk returns a tuple with the FormattedAddress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFormattedAddress

`func (o *ReverseGeocodeResponse) SetFormattedAddress(v string)`

SetFormattedAddress sets FormattedAddress field to given value.


### GetGeocodingProvider

`func (o *ReverseGeocodeResponse) GetGeocodingProvider() GeocodingProviderId`

GetGeocodingProvider returns the GeocodingProvider field if non-nil, zero value otherwise.

### GetGeocodingProviderOk

`func (o *ReverseGeocodeResponse) GetGeocodingProviderOk() (*GeocodingProviderId, bool)`

GetGeocodingProviderOk returns a tuple with the GeocodingProvider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetGeocodingProvider

`func (o *ReverseGeocodeResponse) SetGeocodingProvider(v GeocodingProviderId)`

SetGeocodingProvider sets GeocodingProvider field to given value.

### HasGeocodingProvider

`func (o *ReverseGeocodeResponse) HasGeocodingProvider() bool`

HasGeocodingProvider returns a boolean if a field has been set.

### SetGeocodingProviderNil

`func (o *ReverseGeocodeResponse) SetGeocodingProviderNil(b bool)`

 SetGeocodingProviderNil sets the value for GeocodingProvider to be an explicit nil

### UnsetGeocodingProvider
`func (o *ReverseGeocodeResponse) UnsetGeocodingProvider()`

UnsetGeocodingProvider ensures that no value is present for GeocodingProvider, not even an explicit nil
### GetLat

`func (o *ReverseGeocodeResponse) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *ReverseGeocodeResponse) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *ReverseGeocodeResponse) SetLat(v float64)`

SetLat sets Lat field to given value.


### GetLng

`func (o *ReverseGeocodeResponse) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *ReverseGeocodeResponse) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *ReverseGeocodeResponse) SetLng(v float64)`

SetLng sets Lng field to given value.


### GetProviderPlaceId

`func (o *ReverseGeocodeResponse) GetProviderPlaceId() string`

GetProviderPlaceId returns the ProviderPlaceId field if non-nil, zero value otherwise.

### GetProviderPlaceIdOk

`func (o *ReverseGeocodeResponse) GetProviderPlaceIdOk() (*string, bool)`

GetProviderPlaceIdOk returns a tuple with the ProviderPlaceId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderPlaceId

`func (o *ReverseGeocodeResponse) SetProviderPlaceId(v string)`

SetProviderPlaceId sets ProviderPlaceId field to given value.

### HasProviderPlaceId

`func (o *ReverseGeocodeResponse) HasProviderPlaceId() bool`

HasProviderPlaceId returns a boolean if a field has been set.

### SetProviderPlaceIdNil

`func (o *ReverseGeocodeResponse) SetProviderPlaceIdNil(b bool)`

 SetProviderPlaceIdNil sets the value for ProviderPlaceId to be an explicit nil

### UnsetProviderPlaceId
`func (o *ReverseGeocodeResponse) UnsetProviderPlaceId()`

UnsetProviderPlaceId ensures that no value is present for ProviderPlaceId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
