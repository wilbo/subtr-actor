use super::*;

fn beaten_to_ball_event(player: PlayerId, winner: PlayerId) -> BeatenToBallEvent {
    BeatenToBallEvent {
        time: 1.0,
        frame: 30,
        player,
        winner,
        is_team_0: true,
        player_position: None,
        distance_at_touch: 150.0,
        margin_seconds: 0.1,
        approach_speed: 1200.0,
        velocity_alignment: 0.9,
        dodge_active: false,
        aerial: false,
    }
}

#[test]
fn beaten_to_ball_events_count_per_losing_player() {
    let loser = PlayerId::Steam(1);
    let other_loser = PlayerId::Steam(2);
    let winner = PlayerId::Steam(3);
    let mut stats = WhiffStatsAccumulator::new();

    stats.apply_beaten_to_ball_event(&beaten_to_ball_event(loser.clone(), winner.clone()));
    stats.apply_beaten_to_ball_event(&beaten_to_ball_event(loser.clone(), winner.clone()));
    stats.apply_beaten_to_ball_event(&beaten_to_ball_event(other_loser.clone(), winner.clone()));

    let loser_stats = &stats.player_stats()[&loser];
    assert_eq!(loser_stats.beaten_to_ball_count, 2);
    assert_eq!(loser_stats.whiff_count, 0);
    assert_eq!(stats.player_stats()[&other_loser].beaten_to_ball_count, 1);
    assert!(!stats.player_stats().contains_key(&winner));
}
