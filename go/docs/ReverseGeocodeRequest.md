# ReverseGeocodeRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Lat** | **float64** |  |
**Lng** | **float64** |  |
**Locale** | Pointer to **NullableString** | Preferred locale (app code or BCP-47) for the resolved text. Defaults to &#x60;en&#x60;. | [optional]

## Methods

### NewReverseGeocodeRequest

`func NewReverseGeocodeRequest(lat float64, lng float64, ) *ReverseGeocodeRequest`

NewReverseGeocodeRequest instantiates a new ReverseGeocodeRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReverseGeocodeRequestWithDefaults

`func NewReverseGeocodeRequestWithDefaults() *ReverseGeocodeRequest`

NewReverseGeocodeRequestWithDefaults instantiates a new ReverseGeocodeRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLat

`func (o *ReverseGeocodeRequest) GetLat() float64`

GetLat returns the Lat field if non-nil, zero value otherwise.

### GetLatOk

`func (o *ReverseGeocodeRequest) GetLatOk() (*float64, bool)`

GetLatOk returns a tuple with the Lat field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLat

`func (o *ReverseGeocodeRequest) SetLat(v float64)`

SetLat sets Lat field to given value.


### GetLng

`func (o *ReverseGeocodeRequest) GetLng() float64`

GetLng returns the Lng field if non-nil, zero value otherwise.

### GetLngOk

`func (o *ReverseGeocodeRequest) GetLngOk() (*float64, bool)`

GetLngOk returns a tuple with the Lng field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLng

`func (o *ReverseGeocodeRequest) SetLng(v float64)`

SetLng sets Lng field to given value.


### GetLocale

`func (o *ReverseGeocodeRequest) GetLocale() string`

GetLocale returns the Locale field if non-nil, zero value otherwise.

### GetLocaleOk

`func (o *ReverseGeocodeRequest) GetLocaleOk() (*string, bool)`

GetLocaleOk returns a tuple with the Locale field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLocale

`func (o *ReverseGeocodeRequest) SetLocale(v string)`

SetLocale sets Locale field to given value.

### HasLocale

`func (o *ReverseGeocodeRequest) HasLocale() bool`

HasLocale returns a boolean if a field has been set.

### SetLocaleNil

`func (o *ReverseGeocodeRequest) SetLocaleNil(b bool)`

 SetLocaleNil sets the value for Locale to be an explicit nil

### UnsetLocale
`func (o *ReverseGeocodeRequest) UnsetLocale()`

UnsetLocale ensures that no value is present for Locale, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
