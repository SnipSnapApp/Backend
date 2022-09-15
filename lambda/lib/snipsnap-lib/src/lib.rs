/*
 * Copyright (c) 2022. Josh Bedwell. All rights reserved.
 */

pub mod database;
pub mod http;

const REGION: &str = "us-west-2";

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
