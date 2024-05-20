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

    // query {
    //   complexQuery {
    //     field1
    //     field2
    //     subField {
    //       subField1
    //       subField2
    //     }
    //   }
    // }

    pub async fn complex_query(&self) -> ComplexQueryResult {
        ComplexQueryResult {
            field1: "Value1".to_string(),
            field2: 42,
            sub_field: ComplexSubField {
                sub_field1: true,
                sub_field2: "SubValue2".to_string(),
            },
        }
    }

    // query {
    //   getTodo(num: 1){
    //     userId
    //     id
    //     title
    //     completed
    //   }
    // }

    // query {
    //   notSimpleObject {
    //     field1
    //     field2
    //     subField {
    //       subField1
    //       subField2
    //     }
    //   }
    // }

    pub async fn not_simple_object(&self) -> NotSimpleObject {
        NotSimpleObject {
            field1: "value".to_string(),
            field2: 42,
            sub_field: ComplexSubField {
                sub_field1: true,
                sub_field2: "subvalue2".to_string(),
            }
        }
    }


}
#[derive(async_graphql::SimpleObject)]
pub struct ComplexQueryResult {
    pub field1: String,
    pub field2: i32,
    pub sub_field: ComplexSubField,
}

#[derive(async_graphql::SimpleObject)]
pub struct ComplexSubField {
    pub sub_field1: bool,
    pub sub_field2: String,
}

pub struct NotSimpleObject {
    pub field1: String,
    pub field2: i32,
    pub sub_field: ComplexSubField,
}


#[async_graphql::Object]
impl NotSimpleObject {
    pub async fn field1(&self) -> &str {
        &self.field1
    }

    pub async fn field2(&self) -> i32 {
        self.field2
    }

    pub async fn sub_field(&self) -> &ComplexSubField {
        &self.sub_field
    }
}