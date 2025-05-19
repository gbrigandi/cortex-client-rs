# UserCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**login** | **String** |  | 
**name** | Option<**String**> |  | [optional]
**roles** | Option<**Vec<String>**> |  | [optional]
**organization** | Option<**String**> |  | [optional]
**password** | Option<**String**> |  | [optional]
**status** | Option<**String**> |  | [optional][default to Ok]
**preferences** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]
**avatar** | Option<**String**> | Base64 representation of user avatar image. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


