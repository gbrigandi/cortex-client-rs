# JobCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**data** | Option<**String**> | Observable data (e.g. IP, domain) or stringified JSON for responders. For files, use multipart/form-data with an 'attachment' part. | [optional]
**data_type** | Option<**String**> | Type of the observable data (e.g. 'ip', 'file'). | [optional]
**tlp** | Option<**i64**> |  | [optional][default to 2]
**pap** | Option<**i64**> |  | [optional][default to 2]
**message** | Option<**String**> |  | [optional]
**parameters** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Custom parameters for the worker. | [optional][default to {}]
**label** | Option<**String**> |  | [optional]
**force** | Option<**bool**> | Force running the job even if a similar recent job exists in cache. | [optional][default to false]
**attributes** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | (Legacy) Contains dataType, tlp, and other parameters. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


