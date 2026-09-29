# MeListingsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AgeClass** | [**AgeClass**](AgeClass.md) | Derived from parent-declared date of birth. &#x60;adult&#x60; when unset. |
**DndGlobal** | Pointer to **NullableBool** | True when user has global DND enabled (all calls silenced). | [optional]
**Listings** | [**[]MeListingAsset**](MeListingAsset.md) |  |

## Methods

### NewMeListingsResponse

`func NewMeListingsResponse(ageClass AgeClass, listings []MeListingAsset, ) *MeListingsResponse`

NewMeListingsResponse instantiates a new MeListingsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMeListingsResponseWithDefaults

`func NewMeListingsResponseWithDefaults() *MeListingsResponse`

NewMeListingsResponseWithDefaults instantiates a new MeListingsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAgeClass

`func (o *MeListingsResponse) GetAgeClass() AgeClass`

GetAgeClass returns the AgeClass field if non-nil, zero value otherwise.

### GetAgeClassOk

`func (o *MeListingsResponse) GetAgeClassOk() (*AgeClass, bool)`

GetAgeClassOk returns a tuple with the AgeClass field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAgeClass

`func (o *MeListingsResponse) SetAgeClass(v AgeClass)`

SetAgeClass sets AgeClass field to given value.


### GetDndGlobal

`func (o *MeListingsResponse) GetDndGlobal() bool`

GetDndGlobal returns the DndGlobal field if non-nil, zero value otherwise.

### GetDndGlobalOk

`func (o *MeListingsResponse) GetDndGlobalOk() (*bool, bool)`

GetDndGlobalOk returns a tuple with the DndGlobal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDndGlobal

`func (o *MeListingsResponse) SetDndGlobal(v bool)`

SetDndGlobal sets DndGlobal field to given value.

### HasDndGlobal

`func (o *MeListingsResponse) HasDndGlobal() bool`

HasDndGlobal returns a boolean if a field has been set.

### SetDndGlobalNil

`func (o *MeListingsResponse) SetDndGlobalNil(b bool)`

 SetDndGlobalNil sets the value for DndGlobal to be an explicit nil

### UnsetDndGlobal
`func (o *MeListingsResponse) UnsetDndGlobal()`

UnsetDndGlobal ensures that no value is present for DndGlobal, not even an explicit nil
### GetListings

`func (o *MeListingsResponse) GetListings() []MeListingAsset`

GetListings returns the Listings field if non-nil, zero value otherwise.

### GetListingsOk

`func (o *MeListingsResponse) GetListingsOk() (*[]MeListingAsset, bool)`

GetListingsOk returns a tuple with the Listings field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListings

`func (o *MeListingsResponse) SetListings(v []MeListingAsset)`

SetListings sets Listings field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
