# Backend

AWS backend for SnipSnap iOS app

The original vision for this app was to have basic Snapchat esc functionality of picture messaging. I had to significantly reduce the scope of the project given school, work, career, personal commitments etc. 

Now it has full functionality for signing in using Sign In With Apple.

The iOS client sends its generated device ID through an AWS endpoint which triggers a Lambda with Rust code that creates a nonce, stores it in DynamoDb, and returns it to the client. The client can then make a login request from their device, which will first interact with Apple servers to create a login token which can then be passed to my serverless endpoints and verified by me, using the nonce to stop replay attacks.

This login system is secure, and when authenticated gives you a relatively high degree of confidence that the request is coming from a geniuine instance of the iOS client. There are technically more ways to verify client hardware integrity, but Sign In With Apple does provide some basic checks. 
