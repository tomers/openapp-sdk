# ZonePathItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **string** |  |
**Name** | [**LocalizedString**](LocalizedString.md) |  |

## Methods

### NewZonePathItem

`func NewZonePathItem(id string, name LocalizedString, ) *ZonePathItem`

NewZonePathItem instantiates a new ZonePathItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewZonePathItemWithDefaults

`func NewZonePathItemWithDefaults() *ZonePathItem`

NewZonePathItemWithDefaults instantiates a new ZonePathItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *ZonePathItem) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ZonePathItem) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ZonePathItem) SetId(v string)`

SetId sets Id field to given value.


### GetName

`func (o *ZonePathItem) GetName() LocalizedString`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ZonePathItem) GetNameOk() (*LocalizedString, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ZonePathItem) SetName(v LocalizedString)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
