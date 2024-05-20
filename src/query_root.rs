pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Query {
    //    hello(name: "foo")
    //}
    pub async fn hello(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    // Query {
    //    howdy
    //}
    pub async fn howdy(&self) -> &'static str {
        "partner"
    }
}