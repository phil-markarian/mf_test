//is_time_within_rangeは下記のようなことをする。
//ある時刻(0時～23時)が、指定した時間の範囲内に含まれるかどうかを調べる
// - ある時刻と、時間の範囲(開始時刻と終了時刻)を受け取る。
// - 時刻は、6時であれば6のような指定でよく、分単位は問わない。
// - 範囲指定は、開始時刻を含み、終了時刻は含まないと判断すること。
// - ただし開始時刻と終了時刻が同じ場合は含むと判断すること。
// - 開始時刻が22時で終了時刻が5時、というような指定をされても動作すること。
fn is_time_within_range(target: u32, start: u32, end: u32) -> bool {
    assert!(target < 24 && start < 24 && end < 24, "時刻は0から23の間でなければなりません");

    if start < end {
        // 開始時刻が終了時刻よりも前の場合、
        // 対象時刻が開始時刻以上かつ終了時刻未満であれば範囲内と判断
        target >= start && target < end
    } else if start > end {
        // 開始時刻が終了時刻よりも後の場合（深夜をまたぐ範囲）、
        // 対象時刻が終了時刻以上かつ開始時刻未満でなければ範囲内と判断
        !(target >= end && target < start)
    } else {
        // 開始時刻と終了時刻が同じ場合、全ての時刻を範囲内と判断
        true
    }

}

fn main() {
    // テストケース
    println!("テストケース 1: 対象時刻が範囲内 (同じ日)");
    println!("結果: {}", is_time_within_range(10, 9, 17)); // 期待される結果: true

    println!("テストケース 2: 対象時刻が範囲外 (同じ日)");
    println!("結果: {}", is_time_within_range(8, 9, 17)); // 期待される結果: false

    println!("テストケース 3: 対象時刻が範囲内 (深夜をまたぐ)");
    println!("結果: {}", is_time_within_range(23, 22, 5)); // 期待される結果: true

    println!("テストケース 4: 対象時刻が範囲外 (深夜をまたぐ)");
    println!("結果: {}", is_time_within_range(6, 22, 5)); // 期待される結果: false

    println!("テストケース 5: 対象時刻が開始時刻と同じ");
    println!("結果: {}", is_time_within_range(9, 9, 17)); // 期待される結果: true

    println!("テストケース 6: 対象時刻が終了時刻と同じ (含まない)");
    println!("結果: {}", is_time_within_range(17, 9, 17)); // 期待される結果: false

    println!("テストケース 7: 開始時刻と終了時刻が同じ (一日中をカバー)");
    println!("結果: {}", is_time_within_range(0, 5, 5)); // 期待される結果: true

    println!("テストケース 8: 開始時刻と終了時刻が同じ (一日中をカバー)、対象時刻が異なる");
    println!("結果: {}", is_time_within_range(23, 5, 5)); // 期待される結果: true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 同じ日内で、対象時刻が範囲内にあるケースをテスト
    fn test_within_range_same_day() {
        assert!(is_time_within_range(10, 9, 17));
    }

    #[test]
    // 同じ日内で、対象時刻が範囲外にあるケースをテスト
    fn test_outside_range_same_day() {
        assert!(!is_time_within_range(8, 9, 17));
    }

    #[test]
    // 深夜をまたぐ範囲で、対象時刻が範囲内にあるケースをテスト
    fn test_within_range_spans_midnight() {
        assert!(is_time_within_range(23, 22, 5));
    }

    #[test]
    // 深夜をまたぐ範囲で、対象時刻が範囲外にあるケースをテスト
    fn test_outside_range_spans_midnight() {
        assert!(!is_time_within_range(6, 22, 5));
    }

    #[test]
    // 対象時刻が開始時刻と同じであるケースをテスト（範囲内とみなす）
    fn test_target_equals_start_time() {
        assert!(is_time_within_range(9, 9, 17));
    }

    #[test]
    // 対象時刻が終了時刻と同じであるケースをテスト（範囲外とみなす）
    fn test_target_equals_end_time_not_inclusive() {
        assert!(!is_time_within_range(17, 9, 17));
    }

    #[test]
    // 開始時刻と終了時刻が同じで、全ての時刻をカバーするケースをテスト
    fn test_start_equals_end_covers_whole_day() {
        assert!(is_time_within_range(0, 5, 5));
    }

    #[test]
    // 開始時刻と終了時刻が同じで、全ての時刻をカバーする別のケースをテスト
    fn test_start_equals_end_covers_whole_day_different_target() {
        assert!(is_time_within_range(23, 5, 5));
    }

}
