# SiteAccessZone

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Doors** | [**[]SiteAccessZoneDoor**](SiteAccessZoneDoor.md) | Doors of the site placed in this zone. |
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |
**Path** | [**[]LocalizedString**](LocalizedString.md) | Zone names from the root zone down to this zone, inclusive. |

## Methods

### NewSiteAccessZone

`func NewSiteAccessZone(doors []SiteAccessZoneDoor, id string, name LocalizedString, path []LocalizedString, ) *SiteAccessZone`

NewSiteAccessZone instantiates a new SiteAccessZone object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSiteAccessZoneWithDefaults

`func NewSiteAccessZoneWithDefaults() *SiteAccessZone`

NewSiteAccessZoneWithDefaults instantiates a new SiteAccessZone object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDoors

`func (o *SiteAccessZone) GetDoors() []SiteAccessZoneDoor`

GetDoors returns the Doors field if non-nil, zero value otherwise.

### GetDoorsOk

`func (o *SiteAccessZone) GetDoorsOk() (*[]SiteAccessZoneDoor, bool)`

GetDoorsOk returns a tuple with the Doors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDoors

`func (o *SiteAccessZone) SetDoors(v []SiteAccessZoneDoor)`

SetDoors sets Doors field to given value.


### GetId

`func (o *SiteAccessZone) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SiteAccessZone) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SiteAccessZone) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *SiteAccessZone) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SiteAccessZone) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SiteAccessZone) SetName(v LocalizedString)`

SetName sets Name field to given value.


### GetPath

`func (o *SiteAccessZone) GetPath() []LocalizedString`

GetPath returns the Path field if non-nil, zero value otherwise.

### GetPathOk

`func (o *SiteAccessZone) GetPathOk() (*[]LocalizedString, bool)`

GetPathOk returns a tuple with the Path field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPath

`func (o *SiteAccessZone) SetPath(v []LocalizedString)`

SetPath sets Path field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
