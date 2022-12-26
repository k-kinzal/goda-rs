**NOTE: This is a proof-of-concept repository. It is not production ready.**

# GODA (GraphQL Operation Driven Approach)

GODA (GraphQL Operation Driven Approach) is a method for operating GraphQL more safely by focusing on operations within GraphQL.

## Motivation

GraphQL communicates securely with clients by using schema versioning. However, this can be limited by the client's ability to support newer versions, especially for older mobile clients. To address these issues, GODA (GraphQL Operation Driven Approach) offers the following advantages

- Migration of requests and responses for older operations without having to implement resolvers
- Operations can be predefined to prevent denial-of-service attacks
- It can reproduce the operations expected by the client, thus preventing schema degradation as well as CDC testing

![Untitled](https://user-images.githubusercontent.com/1281825/209499484-ac4c7aab-9a15-41b7-9d89-8001eb770d3b.jpg)

## Not yet supported

- Safe migration using static types
- Automatic code generation from operations graphql files
- Automatic generation of test code from operation graphql files
- Middleware support for HTTP server libraries
- Faster static operation registry
- Operation registry using external storage such as Redis
