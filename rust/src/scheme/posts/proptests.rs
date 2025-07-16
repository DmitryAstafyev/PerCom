use crate::scheme::posts::{Post, PostInput};
use chrono::Utc;
use proptest::{prelude::*, string};
use uuid::Uuid;

/// Implements `Arbitrary` for [`PostInput`] to enable property-based testing using `proptest`.
///
/// This strategy generates randomized `PostInput` values that simulate realistic user input for
/// creating or updating blog posts. The generated data includes:
///
/// - `author`: A randomly generated alphanumeric string between 5 and 20 characters.
/// - `content`: A longer alphanumeric string, between 200 and 2000 characters.
/// - `date`: Always set to the current UTC time using `Utc::now()` at generation time.
///
/// # Panics
/// Panics if the regex used for string generation is invalid (should never happen unless modified).
impl Arbitrary for PostInput {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        (
            string::string_regex("[a-zA-Z0-9]{5,20}").expect("Author is generated"),
            string::string_regex("[a-zA-Z0-9]{200,2000}").expect("Content is generated"),
        )
            .prop_map(|(author, content)| PostInput {
                author,
                content,
                date: Utc::now(),
            })
            .boxed()
    }
}

/// Implements `Arbitrary` for [`Post`] to enable property-based testing using `proptest`.
///
/// This strategy wraps a generated [`PostInput`] and adds a randomly generated UUID (`v4`) as the `id`
/// field. The resulting `Post` represents a realistic, complete blog post as it might exist in the system.
///
/// This implementation allows testing parts of the application that work with fully constructed posts,
/// rather than just inputs.
///
/// # Note
/// The `date` is set to the current UTC time rather than derived from the original input, which may
/// slightly differ from real-life update flows where `date` may be preserved.
impl Arbitrary for Post {
    type Parameters = ();

    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        any::<PostInput>()
            .prop_map(|inputs| Post {
                id: Uuid::new_v4().to_string(),
                author: inputs.author,
                content: inputs.content,
                date: Utc::now(),
            })
            .boxed()
    }
}
