fn use_names_for_something_else(_names: Vec<&str>) {
}

fn main() {
    let names = vec!["Jane", "Jill", "Jack", "John"];

    let total_bytes: _ = names.iter()
        .map(|w| w.len())
        .fold(0, |a, l| a+l);

    assert_eq!(total_bytes, 16);
    use_names_for_something_else(names);

    let player_scores = [
        ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19),
    ];

    let players: _ = player_scores
        .iter()
        .map(|&(player, _scores)| player)
        .collect::<Vec<_>>();

    assert_eq!(players, ["Jack", "Jane", "Jill", "John"]);

    let mut teams = [
        [ ("Jack", 20), ("Jane", 23), ("Jill", 18), ("John", 19), ],
        [ ("Bill", 17), ("Brenda", 16), ("Brad", 18), ("Barbara", 17), ]
    ];

    let _teams_in_score_order = teams
        .iter_mut()
        .map(|team| {
            team.sort_by(|&a,&b|
                b.1.cmp(&a.1)
            );
            team
        })
        .collect::<Vec<_>>();


    let data = [1,1];
    let mut data_iter = data.iter();
    let first = data_iter.next().unwrap();
    let second = data_iter.next().unwrap();
    assert_eq!(first, second);
}