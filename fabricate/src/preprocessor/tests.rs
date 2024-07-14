use super::*;

#[test]
fn test_process() {
    let lines: Vec<&str> = vec!["[start]═─{\"hello world\"}═─[println]"];
    let (stations, start_i, assign_table) = process(&lines, &stdlib::MANIFEST).unwrap();
    assert_eq!(stations.len(), 3);
    assert_eq!(stations[start_i].logic.id, "start");
    assert_eq!(assign_table.len(), 1);
}

#[test]
fn test_process_err() {}
