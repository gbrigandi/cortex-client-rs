# Worker

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**_id** | Option<**String**> | The unique ID of the worker (analyzer/responder). | [optional]
**name** | Option<**String**> |  | [optional]
**version** | Option<**String**> |  | [optional]
**worker_definition_id** | Option<**String**> | ID of the definition this worker is based on. | [optional]
**description** | Option<**String**> |  | [optional]
**author** | Option<**String**> |  | [optional]
**url** | Option<**String**> |  | [optional]
**license** | Option<**String**> |  | [optional]
**command** | Option<**String**> | Command to execute for process-based workers. | [optional]
**docker_image** | Option<**String**> | Docker image for containerized workers. | [optional]
**data_type_list** | Option<**Vec<String>**> | List of data types this worker can process. | [optional]
**configuration** | Option<[**models::WorkerConfiguration**](Worker_configuration.md)> |  | [optional]
**base_config** | Option<**String**> | Key for the base configuration. | [optional]
**rate** | Option<**f64**> |  | [optional]
**rate_unit** | Option<**String**> |  | [optional]
**job_cache** | Option<**f64**> |  | [optional]
**job_timeout** | Option<**f64**> |  | [optional]
**r#type** | Option<**String**> |  | [optional]
**analyzer_definition_id** | Option<**String**> |  | [optional]
**max_tlp** | Option<**f64**> |  | [optional]
**max_pap** | Option<**f64**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


