use crate::context::Context;
use crate::models::test::TestObj;
use crate::usecase;
use async_graphql::{InputObject, MergedObject, Object, SimpleObject};

#[derive(Default)]
pub struct TestQuery;

#[Object]
impl TestQuery {
    async fn obj<'ctx>(&self, ctx: &async_graphql::Context<'ctx>) -> TestObj {
        let ctx = ctx.data_unchecked::<Context>();
        usecase::test::get_obj(ctx)
    }
}

#[derive(Default, MergedObject)]
pub struct TestMutation(CreateTestObjMutation);

#[derive(Default)]
pub struct CreateTestObjMutation;

#[derive(InputObject)]
pub struct CreateTestObjInput {
    #[graphql(validator(max_length = 255))]
    pub name: String,
    #[graphql(validator(minimum = 1))]
    pub num: usize,
}

#[derive(SimpleObject, Debug)]
pub struct CreateTestObjPayload {
    pub obj: TestObj,
}

#[Object]
impl CreateTestObjMutation {
    async fn create_obj(
        &self,
        input: CreateTestObjInput,
    ) -> Result<CreateTestObjPayload, async_graphql::Error> {
        let Ok(obj) = usecase::test::create_obj(input.name, input.num) else {
            return Err(async_graphql::Error {
                message: "yuorei".to_string(),
                source: None,
                extensions: None,
            });
        };

        Ok(CreateTestObjPayload { obj })
    }
}
