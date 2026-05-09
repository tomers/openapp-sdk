# MeApartmentsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Apartments** | [**[]MeApartmentAsset**](MeApartmentAsset.md) |  |
**DndGlobal** | Pointer to **NullableBool** | True when user has global DND enabled (all calls silenced). | [optional]

## Methods

### NewMeApartmentsResponse

`func NewMeApartmentsResponse(apartments []MeApartmentAsset, ) *MeApartmentsResponse`

NewMeApartmentsResponse instantiates a new MeApartmentsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeApartmentsResponseWithDefaults

`func NewMeApartmentsResponseWithDefaults() *MeApartmentsResponse`

NewMeApartmentsResponseWithDefaults instantiates a new MeApartmentsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApartments

`func (o *MeApartmentsResponse) GetApartments() []MeApartmentAsset`

GetApartments returns the Apartments field if non-nil, zero value otherwise.

### GetApartmentsOk

`func (o *MeApartmentsResponse) GetApartmentsOk() (*[]MeApartmentAsset, bool)`

GetApartmentsOk returns a tuple with the Apartments field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApartments

`func (o *MeApartmentsResponse) SetApartments(v []MeApartmentAsset)`

SetApartments sets Apartments field to given value.


### GetDndGlobal

`func (o *MeApartmentsResponse) GetDndGlobal() bool`

GetDndGlobal returns the DndGlobal field if non-nil, zero value otherwise.

### GetDndGlobalOk

`func (o *MeApartmentsResponse) GetDndGlobalOk() (*bool, bool)`

GetDndGlobalOk returns a tuple with the DndGlobal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDndGlobal

`func (o *MeApartmentsResponse) SetDndGlobal(v bool)`

SetDndGlobal sets DndGlobal field to given value.

### HasDndGlobal

`func (o *MeApartmentsResponse) HasDndGlobal() bool`

HasDndGlobal returns a boolean if a field has been set.

### SetDndGlobalNil

`func (o *MeApartmentsResponse) SetDndGlobalNil(b bool)`

 SetDndGlobalNil sets the value for DndGlobal to be an explicit nil

### UnsetDndGlobal
`func (o *MeApartmentsResponse) UnsetDndGlobal()`

UnsetDndGlobal ensures that no value is present for DndGlobal, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
