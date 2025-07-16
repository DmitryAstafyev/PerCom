use crate::scheme::users::{User, UserInput};
use proptest::{prelude::*, string};
use uuid::Uuid;

impl Arbitrary for UserInput {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            string::string_regex("[a-zA-Z0-9]{5,20}").expect("Author is generated"),
            string::string_regex("[a-zA-Z0-9]{5,20}").expect("Author is generated"),
            string::string_regex("[a-zA-Z0-9]{5,20}").expect("Author is generated"),
        )
            .prop_map(|(email_name, email_host, nickname)| UserInput {
                email: format!("{email_name}@{email_host}.com"),
                nickname,
            })
            .boxed()
    }
}

impl Arbitrary for User {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<UserInput>()
            .prop_map(|inputs| User {
                id: Uuid::new_v4().to_string(),
                email: inputs.email,
                nickname: inputs.nickname,
            })
            .boxed()
    }
}
