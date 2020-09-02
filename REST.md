# REST API v1

## POST /api/v1/recipe

Call this endpoint via a POST request in order to create a new recipe.

Example POST body:

```
{
   "url": "http://test.local/api/rest",
   "rules": [
       {
           "Authenticated":{"key_path":"foo"}
       }
   ],
   "payload": {
       "foo": "bar"
   }
}
```

### url

*Type*: String, required

This is the URL that test code will call in order to be served the payload from this recipe. If the url property is missing or its value cannot be parsed as a URL, the response will use an error status code and its body will contain a String message explaining the details of the problem.

### rules

*Type*: Array of objects, optional.

This is the set of rules that must match the request in order to serve the payload. Each rule is a JSON object with a single property whose name is the type of rule and whose value contains the specifics of the rule. See the following sections for supported rules and examples. For any rules provided, if they do not match the expected shape, the response will use an error status code and its body will contain a String message explaining the details of the problem.

#### Authenticated

**key_path** is a String, it is required. How its value will be interpreted depends on how the server is configured. The key indicated will be used to verify the signature on a JWT in a Bearer Authorization header on the request.

Example:

```
{
    "Authenticated":{"key_path":"foo"}
}
```

#### Subject

**subject** is a String, it is required, its value will be the subject claim in a JWT in a Bearer Authorization header on the request. The rule will match if and only if the subject claim matches this value.

Example:

```
{
    "Subject":{"subject":"foo"}
}
```

#### HttpMethod

**http_method** is a String and must be one of the literal values: `Get`, `Post`, `Put`, or `Delete`. The rule will match if and only if the HTTP method of the request matches the rule's value.

Example:

```
{
    "HttpMethod":{"http_method":"Get"}
}
```

### payload

*Type*: JSON Object, JSON Array, or String. If the payload property cannot be parsed as valid JSON, the response will use an error status code and the body will include a String message explaining the details of the problem.

This is the JSON that is served if the rules all match for this recipe.

## PUT /api/v1/recipe

Use this endpoint with a PUT request to update an existing recipe. The PUT body is the same as the POST body, above, for creating a recipe with the addition of an "id" property whose value, a String, must be parseable as a UUID. "id" is required.

## GET /api/v1/recipe

Use this endpoint to get a page of recipes.

**total**: An integer, the total number of recipes.

**offset**: The zero based index for the first recipe within all of the available recipes.

**limit**: The number of recipes in this page.

**recipes**: An array of JSON objects whose shape matches the PUT body described above.

Example response:

```
{
    total: 29,
    offset: 0,
    limit: 25,
    recipes: [
        {
           "id": "<uuid string>",
           "url": "http://test.local/api/rest",
           "rules": [
               {
                   "Authenticated":{"key_path":"foo"}
               }
           ],
           "payload": {
               "foo": "bar"
           }
        },
        ...
    ]
}
```

## GET /api/v1/recipe/offset/{offset}

Use this endpoint to get a page of recipes, starting at a specific offset. The response body shape is the same as the endpoint without `/offset/{offset}`, above.

## DELETE /api/v1/recipe/{uuid}

Deletes the recipe with the matching `uuid` value.
