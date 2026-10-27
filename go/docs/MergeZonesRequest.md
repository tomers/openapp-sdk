# MergeZonesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**SourceZoneIds** | **[]string** | Zones merged into the target (path) zone; each is soft-deleted afterwards. |

## Methods

### NewMergeZonesRequest

`func NewMergeZonesRequest(sourceZoneIds []string, ) *MergeZonesRequest`

NewMergeZonesRequest instantiates a new MergeZonesRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMergeZonesRequestWithDefaults

`func NewMergeZonesRequestWithDefaults() *MergeZonesRequest`

NewMergeZonesRequestWithDefaults instantiates a new MergeZonesRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetSourceZoneIds

`func (o *MergeZonesRequest) GetSourceZoneIds() []string`

GetSourceZoneIds returns the SourceZoneIds field if non-nil, zero value otherwise.

### GetSourceZoneIdsOk

`func (o *MergeZonesRequest) GetSourceZoneIdsOk() (*[]string, bool)`

GetSourceZoneIdsOk returns a tuple with the SourceZoneIds field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceZoneIds

`func (o *MergeZonesRequest) SetSourceZoneIds(v []string)`

SetSourceZoneIds sets SourceZoneIds field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
