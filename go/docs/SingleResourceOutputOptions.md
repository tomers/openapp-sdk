# SingleResourceOutputOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IncludeDeleted** | **bool** | If true, include soft-deleted items in results. |
**IncludeMetadata** | **bool** | If true, include storage metadata (e.g. created_at, deleted_at) in results. |

## Methods

### NewSingleResourceOutputOptions

`func NewSingleResourceOutputOptions(includeDeleted bool, includeMetadata bool, ) *SingleResourceOutputOptions`

NewSingleResourceOutputOptions instantiates a new SingleResourceOutputOptions object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSingleResourceOutputOptionsWithDefaults

`func NewSingleResourceOutputOptionsWithDefaults() *SingleResourceOutputOptions`

NewSingleResourceOutputOptionsWithDefaults instantiates a new SingleResourceOutputOptions object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIncludeDeleted

`func (o *SingleResourceOutputOptions) GetIncludeDeleted() bool`

GetIncludeDeleted returns the IncludeDeleted field if non-nil, zero value otherwise.

### GetIncludeDeletedOk

`func (o *SingleResourceOutputOptions) GetIncludeDeletedOk() (*bool, bool)`

GetIncludeDeletedOk returns a tuple with the IncludeDeleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeDeleted

`func (o *SingleResourceOutputOptions) SetIncludeDeleted(v bool)`

SetIncludeDeleted sets IncludeDeleted field to given value.


### GetIncludeMetadata

`func (o *SingleResourceOutputOptions) GetIncludeMetadata() bool`

GetIncludeMetadata returns the IncludeMetadata field if non-nil, zero value otherwise.

### GetIncludeMetadataOk

`func (o *SingleResourceOutputOptions) GetIncludeMetadataOk() (*bool, bool)`

GetIncludeMetadataOk returns a tuple with the IncludeMetadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIncludeMetadata

`func (o *SingleResourceOutputOptions) SetIncludeMetadata(v bool)`

SetIncludeMetadata sets IncludeMetadata field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
