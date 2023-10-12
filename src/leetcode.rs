use std::{collections::HashMap, error, fmt};

const DAILY_PROBLEM_QUERY: &str = "query questionOfToday {
    activeDailyCodingChallengeQuestion {
        date
        userStatus
        link
        question {
            acRate
            difficulty
            freqBar
            frontendQuestionId: questionFrontendId
            isFavor
            paidOnly: isPaidOnly
            status
            title
            titleSlug
            hasVideoSolution
            hasSolution
            topicTags {
                name
                id
                slug
            }
        }
    }
}
";
const LEETCODE_URL: &str = "https://leetcode.com";
const GRAPHQL_ENDPOINT: &str = "/graphql";
const QUESTION_KEY: &str = "activeDailyCodingChallengeQuestion";

#[derive(Debug)]
pub struct LeetCodeQuestion {
    pub link: String,
    pub difficulty: String,
    pub title: String,
}

impl fmt::Display for LeetCodeQuestion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "LeetCodeQuestion {{ title={:?}, difficulty={:?}, link={:?} }}",
            self.title, self.difficulty, self.link
        )
    }
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct LeetCodeError;

impl fmt::Display for LeetCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to retrieve daily LeetCode problem")
    }
}

impl error::Error for LeetCodeError {}

pub async fn get_daily_leetcode() -> Result<LeetCodeQuestion> {
    let mut body = HashMap::new();
    body.insert("query", DAILY_PROBLEM_QUERY);
    let client = reqwest::Client::new();
    let raw_data = client
        .post(LEETCODE_URL.to_owned() + GRAPHQL_ENDPOINT)
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let raw_question = raw_data
        .get("data")
        .and_then(|value| value.get(QUESTION_KEY))
        .ok_or(LeetCodeError)?;

    let link = format!(
        "{}{}",
        LEETCODE_URL.to_string(),
        raw_question
            .get("link")
            .ok_or(LeetCodeError)?
            .as_str()
            .ok_or(LeetCodeError)?
    );
    let title = raw_question
        .get("question")
        .and_then(|value| value.get("title"))
        .ok_or(LeetCodeError)?
        .as_str()
        .map(|val| val.to_string())
        .ok_or(LeetCodeError)?;

    let difficulty = raw_question
        .get("question")
        .and_then(|value| value.get("difficulty"))
        .ok_or(LeetCodeError)?
        .as_str()
        .map(|val| val.to_string())
        .ok_or(LeetCodeError)?;

    Ok(LeetCodeQuestion {
        title,
        link,
        difficulty,
    })
}
