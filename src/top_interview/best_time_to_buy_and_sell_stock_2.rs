/// Calculates the maximum profit from stock prices with multiple transactions allowed.
///
/// You are given a vector `prices` where `prices[i]` is the price of a stock on day `i`.
///
/// On each day, you may choose to buy and/or sell the stock. You can hold at most one
/// share at any time. However, you may perform multiple transactions (buy + sell)
/// across different days, as long as you do not hold more than one share simultaneously.
///
/// The goal is to compute the maximum profit that can be achieved.
///
/// # Arguments
///
/// * `prices` - A vector of stock prices, where each element represents the price on a given day.
///
/// # Returns
///
/// The maximum profit achievable.
///
/// # Examples
///
/// ```text
/// let prices = vec![7, 1, 5, 3, 6, 4];
/// assert_eq!(max_profit(prices), 7);
/// ```
///
/// ```text
/// let prices = vec![1, 2, 3, 4, 5];
/// assert_eq!(max_profit(prices), 4);
/// ```
///
/// ```text
/// let prices = vec![7, 6, 4, 3, 1];
/// assert_eq!(max_profit(prices), 0);
/// ```
///
/// # Explanation
///
/// - Buy at 1, sell at 5 → profit = 4
/// - Buy at 3, sell at 6 → profit = 3
/// - Total profit = 7
///
/// # Constraints
///
/// * `1 <= prices.len() <= 3 * 10^4`
/// * `0 <= prices[i] <= 10^4`
pub struct Solutions;

impl Solutions {
    /// Maximum profit with **unlimited** transactions and at most one share held at a time.
    ///
    /// This is the classic *Best Time to Buy and Sell Stock II* strategy: you may buy and
    /// sell on many days, but never hold more than one share.
    ///
    /// # Algorithm (greedy)
    ///
    /// Walk the price series left to right. Whenever today’s price is **higher** than
    /// yesterday’s, add that positive difference to the total:
    /// `sum += prices[i] - prices[i - 1]` when `prices[i] > prices[i - 1]`.
    ///
    /// **Why this is optimal:** Any profitable move over multiple days can be decomposed
    /// into a sum of day-to-day gains along rising segments. Capturing every positive
    /// daily change is equivalent to buying at each local minimum and selling at each
    /// local maximum, which maximizes profit when there is no limit on the number of
    /// trades and no fee per trade.
    ///
    /// # Complexity
    ///
    /// - **Time:** O(n) for `n = prices.len()`
    /// - **Extra space:** O(1)
    ///
    /// # Panics
    ///
    /// Never panics for valid inputs. With an empty `prices` vector, the loop is skipped
    /// and the result is `0`.
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                result += prices[i] - prices[i - 1];
            }
        }

        return result;
    }
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_2_tests {
    use super::*;

    macro_rules! test_case {
        ($prices: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let prices: Vec<i32> = $prices;
                let expected: i32 = $expected;

                let result: i32 = Solutions::max_profit(prices);
                assert_eq!(result, expected);
            }
        };
    }

    test_case!(vec![1, 2, 3, 4, 5], 4, test_case_1);
    test_case!(vec![5, 4, 3, 2, 1], 0, test_case_2);
    test_case!(vec![1, 10, 2, 20, 3, 30], 54, test_case_3);
    test_case!(vec![8, 9, 7, 8, 6, 7], 3, test_case_4);
    test_case!(vec![1], 0, test_case_5);
    test_case!(vec![50, 60, 50, 60, 50], 20, test_case_6);
}
