# \DbListApi

All URIs are relative to */api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_db_list_item**](DbListApi.md#add_db_list_item) | **POST** /dblist/{listName}/items | Add an item to a DBList
[**check_db_list_item_exists**](DbListApi.md#check_db_list_item_exists) | **POST** /dblist/{listName}/exists | Check if an item exists in a DBList
[**delete_db_list_item**](DbListApi.md#delete_db_list_item) | **DELETE** /dblist/items/{itemId} | Delete an item from a DBList
[**list_db_list_items**](DbListApi.md#list_db_list_items) | **GET** /dblist/{listName}/items | List items from a specific DBList
[**list_db_lists**](DbListApi.md#list_db_lists) | **GET** /dblist | List all DBList names
[**update_db_list_item**](DbListApi.md#update_db_list_item) | **PUT** /dblist/items/{itemId} | Update an item in a DBList



## add_db_list_item

> String add_db_list_item(list_name, db_list_add_item_request)
Add an item to a DBList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_name** | **String** |  | [required] |
**db_list_add_item_request** | [**DbListAddItemRequest**](DbListAddItemRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## check_db_list_item_exists

> models::DbListItemExistsResponse check_db_list_item_exists(list_name, db_list_item_exists_request)
Check if an item exists in a DBList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_name** | **String** |  | [required] |
**db_list_item_exists_request** | [**DbListItemExistsRequest**](DbListItemExistsRequest.md) |  | [required] |

### Return type

[**models::DbListItemExistsResponse**](DBListItemExistsResponse.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_db_list_item

> delete_db_list_item(item_id)
Delete an item from a DBList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_db_list_items

> std::collections::HashMap<String, serde_json::Value> list_db_list_items(list_name)
List items from a specific DBList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_name** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_db_lists

> Vec<String> list_db_lists()
List all DBList names

### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_db_list_item

> String update_db_list_item(item_id, db_list_add_item_request)
Update an item in a DBList

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**item_id** | **String** |  | [required] |
**db_list_add_item_request** | [**DbListAddItemRequest**](DbListAddItemRequest.md) |  | [required] |

### Return type

**String**

### Authorization

[SessionAuth](../README.md#SessionAuth), [ApiKeyAuth](../README.md#ApiKeyAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

