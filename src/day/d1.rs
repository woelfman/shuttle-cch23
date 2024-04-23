use axum::{extract::Path, http::StatusCode};

pub async fn num(Path(num): Path<String>) -> Result<String, StatusCode> {
    let num: Result<Vec<i64>, _> = num.split('/').map(str::parse).collect();
    let num = num.map_err(|_| StatusCode::BAD_REQUEST)?;

    let result: i64 = num.iter().fold(0, |acc, &x| acc ^ x).pow(3);
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_day1() {
        assert_eq!(num(Path("4/8".to_string())).await, Ok("1728".to_string()));
        assert_eq!(num(Path("10".to_string())).await, Ok("1000".to_string()));
        assert_eq!(
            num(Path("4/5/8/10".to_string())).await,
            Ok("27".to_string())
        );
    }
}
