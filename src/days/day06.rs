use crate::utils::io::read_rows;
use crate::utils::strings::count_characters;

fn find_packet_start(content: String) -> i32 {
    for i in 3..content.len() {
        let mut has_repeats: bool = false;
        for count in count_characters(content[(i - 3)..(i + 1)].to_string()).values() {
            has_repeats = *count > 1;
            if has_repeats {
                break;
            }
        }
        if !has_repeats {
            return (i + 1) as i32;
        }
    }
    -1
}

#[test]
fn test_find_packet_start() {
    assert_eq!(
        find_packet_start("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
        7
    );
    assert_eq!(
        find_packet_start("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
        5
    );
    assert_eq!(
        find_packet_start("nppdvjthqldpwncqszvftbrmjlhg".to_string()),
        6
    );
    assert_eq!(
        find_packet_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
        10
    );
    assert_eq!(
        find_packet_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
        11
    );
}

pub fn pt1(file_path: String) -> i32 {
    let _rows: Vec<String> = read_rows(file_path);
    find_packet_start(_rows[0].to_string())
}

pub fn run(part: i32) {
    let _result: i32 = match part {
        1 => {
            let packet_start: i32 = pt1("data/day06.txt".to_string());
            println!("The packet starts at character #{}", packet_start);
            0
        }
        _ => {
            println!("Part {} not found.", part);
            0
        }
    };
}
