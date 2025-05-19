# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**String**> | User ID (same as login). | [optional]
**login** | Option<**String**> |  | [optional]
**name** | Option<**String**> | Full name. | [optional]
**roles** | Option<**Vec<String>**> |  | [optional]
**status** | Option<**String**> |  | [optional]
**avatar** | Option<**String**> |  | [optional]
**preferences** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | User preferences as a JSON object. | [optional]
**organization** | Option<**String**> |  | [optional]
**has_key** | Option<**bool**> | Indicates if the user has an API key. | [optional]
**has_password** | Option<**bool**> | Indicates if the user has a password set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


