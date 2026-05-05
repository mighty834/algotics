/// # Best Time to Buy and Sell Stock
///
/// You are given an array `prices` where `prices[i]` is the price of a given stock
/// on the `i`th day.
///
/// You want to maximize your profit by choosing a single day to buy one stock
/// and choosing a different day in the future to sell that stock.
///
/// Return the maximum profit you can achieve from this transaction. If you cannot
/// achieve any profit, return `0`.
///
/// ## Examples
///
/// ### Example 1:
/// ```text
/// Input: prices = [7,1,5,3,6,4]
/// Output: 5
/// Explanation:
/// Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6 - 1 = 5.
/// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.
/// ```
///
/// ### Example 2:
/// ```text
/// Input: prices = [7,6,4,3,1]
/// Output: 0
/// Explanation:
/// In this case, no transactions are done and the max profit = 0.
/// ```
///
/// ## Constraints
///
/// - `1 <= prices.length <= 10^5`
/// - `0 <= prices[i] <= 10^4`
pub struct Solutions;

impl Solutions {
    /// Returns the maximum profit achievable from **exactly one** buy followed by one sell.
    ///
    /// You must buy before you sell. If no profitable transaction exists, returns `0`.
    ///
    /// # Algorithm
    ///
    /// Single pass with two running values:
    ///
    /// - `min`: the lowest price seen so far (best day to buy up to this point)
    /// - `max`: the best profit seen so far
    ///
    /// For each day `i` (starting at 1), compute the profit if selling today after buying at the
    /// cheapest earlier day: `prices[i] - min`. Update `max` if this profit is larger, then update
    /// `min` if `prices[i]` is a new low.
    ///
    /// This works because for any fixed sell day, the best buy day is the minimum price before it;
    /// scanning left-to-right maintains that minimum.
    ///
    /// # Arguments
    ///
    /// * `prices` — Daily prices. Must contain at least one element (per typical constraints).
    ///
    /// # Returns
    ///
    /// The maximum non-negative profit from one transaction.
    ///
    /// # Complexity
    ///
    /// - **Time:** `O(n)` for `n = prices.len()`
    /// - **Extra space:** `O(1)`
    ///
    /// # Panics
    ///
    /// Panics if `prices` is empty, because the implementation reads `prices[0]` to initialize the
    /// running minimum.
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut min, mut max) = (prices[0], 0);
        for i in 1..prices.len() {
            max = max.max(prices[i] - min);
            min = min.min(prices[i]);
        }

        max
    }
}

#[cfg(test)]
mod best_time_to_buy_and_sell_stock_tests {
    use super::*;

    macro_rules! test_case {
        ($prices: expr, $expected: expr, $fn_name: ident) => {
            #[test]
            fn $fn_name() {
                let prices: Vec<i32> = $prices;
                let expected: i32 = $expected;

                let result: i32 = Solutions::max_profit(prices);
                assert_eq!(expected, result);
            }
        };
    }

    test_case!(vec![7, 1, 5, 3, 6, 4], 5, test_case_1);
    test_case!(vec![7, 6, 4, 3, 1], 0, test_case_2);
    test_case!(vec![1], 0, test_case_3);
    test_case!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 8, test_case_4);
    test_case!(vec![9, 8, 7, 6, 5, 6, 5, 4, 3, 2, 1], 1, test_case_5);
    test_case!(vec![50, 40, 30, 100, 90, 80, 100, 70, 60, 50], 70, test_case_6);
    test_case!(vec![0, 100, 0, 110, 0, 120, 0, 90, 0, 100, 50, 60, 0], 120, test_case_7);
    test_case!(vec![0, 120, 0, 50, 40, 60, 20, 110, 90, 130, 100, 80, 50], 130, test_case_8);
}
