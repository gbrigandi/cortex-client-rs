# \AttachmentApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**download_attachment**](AttachmentApi.md#download_attachment) | **GET** /attachment/{hash} | Download an attachment in plain format
[**download_attachment_zip**](AttachmentApi.md#download_attachment_zip) | **GET** /attachment/{hash}/zip | Download an attachment in a password-protected zip file



## download_attachment

> std::path::PathBuf download_attachment(hash, name)
Download an attachment in plain format

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | The hash of the attachment. | [required] |
**name** | Option<**String**> | Desired filename for the download. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/octet-stream, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## download_attachment_zip

> std::path::PathBuf download_attachment_zip(hash, name)
Download an attachment in a password-protected zip file

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**hash** | **String** | The hash of the attachment. | [required] |
**name** | Option<**String**> | Desired filename for the item within the zip. |  |

### Return type

[**std::path::PathBuf**](std::path::PathBuf.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/zip, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

