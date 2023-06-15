# Mulesoft Anypoint Flex Gateway Policy (Custom Redirect Header)

This is a Rust policy for MuleSoft Anypoint Flex. The policy implements header-based redirection. It examines incoming requests for a particular header and redirects them accordingly. The redirection is achieved by responding with a 302 HTTP status code and including a Location header that specifies the new path.

For more informaton check: [Implementing a Flex Gateway Custom Policy in Rust](https://docs.mulesoft.com/gateway/policies-custom-flex-implement-rust)

You can find another example of a Rust policy for Anypoint Flex [here](https://github.com/jrhuerga/mule-flex-data-masking).

